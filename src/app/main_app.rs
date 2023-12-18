use std::{env, fs::{self, OpenOptions, File}, io::{self, Write}, time::Instant};

use itertools::Itertools;
use scraper::Html;
use crate::{error::ArchiveError, xl::save_sok, scraper::get_html_content, parse_args, modules::sok::{SokCollection, Merknad}, parser::wp::{get_kilde, get_metode}};
use crate::modules::webpage::{Webpage, Link};
use crate::parser::wp::get_sok_collection;
use reqwest::Client;

use std::io::prelude::*;

use crate::parser::medium::get_links_from_medium;


pub async fn run_app(args: Vec<String>) -> Result<(), ArchiveError> {
    if args.len() == 0 {
        println!("Missing URL-argument.");
        return Err(ArchiveError::InvalidURL);
    }

    let medium_links: Vec <Link> = parse_args(args)?;

    let mut wp_count = 0;
    let mut save_count = 0;
    let mut mediums: Vec<String> = Vec::new();
    let mut checked_sok = get_checked_soks();

    let client = Client::default();
    for medium_link in medium_links {
        let raw_html = get_html_content(&client, medium_link.to_string()).await?;
        let html = Html::parse_document(&raw_html);
        for link in get_links_from_medium(html)? {
            let mut sok_log: Vec<ArchiveError> = Vec::new();
            
            
            let mut wp = Webpage::from_link(link.clone()).await?;
            let medium = wp.get_medium();
            let id = wp.get_id().clone();
            if checked_sok.contains(&wp.get_id()) {
                continue;
            }
            checked_sok.push(wp.get_id());
            let time_start = Instant::now();

            wp.set_medium(medium.clone());

            if let Some(com) = wp.get_forms().ok() {
                let count = com.combinations().collect::<Vec<_>>().len();
                if count <= 30 {
                    println!("Sok: {}", wp.get_id());
                } else {
                    println!("Sok: {}", wp.get_id());
                    println!("Form Combo: {:?}", count);
                    let _ = write_failed_sok(format!("Had to many forms: {}", count), &id);
                    
                    let mut sokc = SokCollection::new(
                        wp.get_id(),
                        wp.get_medium()
                    );

                    sokc.add_sok(wp.get_sok()?);

                    sokc.title = wp.get_title()?;
                    let _ = wp.get_text()?
                        .into_iter()
                        .map(|e| sokc.add_text(e))
                        .collect::<Vec<_>>();

                    for metode in get_metode(&wp).await? {
                        sokc.add_metode(metode.into());
                    }

                    for kilde in get_kilde(&wp).await? {
                        sokc.add_kilde(kilde.into());
                    }

                    sokc.add_merknad(Merknad { title: "Merknad".to_string(), content: wp.get_merknad()? });

                    let path = format!("arkiv\\{}", medium.clone());
                    if !mediums.contains(&medium) {
                        mediums.push(medium.clone());
                        let r = fs::create_dir_all(path.clone());
                        if r.is_err() {
                            println!("Could not create path: {}, got error: {}", path.clone(), r.unwrap_err());
                        }
                    }

                    match save_sok(sokc, &path) {
                        Ok(_) => {
                            save_count += 1;
                            checkmark_sok(&id);
                            println!("Saved sok: {}", &id);
                        },
                        Err(e) => {
                            println!("Failed saving sok: {}, With Error: {}", &id, &e);
                            sok_log.push(e.clone());
                            let _ = write_failed_sok(e.to_string(), &id);
                        }, 
                    }
                }
            }


            match get_sok_collection(wp).await {
                Ok((mut sokc, mut errs)) => {
                    wp_count += 1;

                    sok_log.append(&mut errs);

                    let path = format!("arkiv\\{}", medium.clone());
                    if !mediums.contains(&medium) {
                        mediums.push(medium.clone());
                        let r = fs::create_dir_all(path.clone());
                        if r.is_err() {
                            println!("Could not create path: {}, got error: {}", path.clone(), r.unwrap_err());
                        }
                    }

                    let time_end = Instant::now();

                    // Failed
                    if sokc.sok.len() == 0 {
                        println!("Sok: {}, had 0 tables.", &sokc.id);
                        sok_log.push(ArchiveError::FailedParsing(sokc.id.clone(), link.to_string().clone()));
                        let _ = write_failed_sok("0 tables".to_string(), &id);
                        sokc.title = sokc.title + &format!("_{}", sokc.id.clone());
                    }

                    match save_sok(sokc, &path) {
                        Ok(_) => {
                            save_count += 1;
                            checkmark_sok(&id);
                            println!("Saved sok: {}, Took {}s", &id, (time_end - time_start).as_secs());
                        },
                        Err(e) => {
                            println!("Failed saving sok: {}, With Error: {}, Took {}s", &id, &e, (time_end - time_start).as_secs());
                            sok_log.push(e.clone());
                            let _ = write_failed_sok(e.to_string(), &id);
                        }, 
                    }
                },
                Err(e) => sok_log.push(e)
            }
            if sok_log.len() != 0 {
                println!("{} error(s) found.", sok_log.len());
                let r = write_log(sok_log.clone().into_iter().map(|e| e.to_string()).collect_vec(), id);
                if r.is_err() {
                    println!("Failed writing Error Log for Sok: {}, due too {}, dumping log.", id, r.unwrap_err());
                    for err in sok_log {
                        println!("Sok {} Error: {:?}", id, err);
                    }
                }
            } else {
                println!("No errors reported.");
            }
        }
    }

    println!("Found {} webpages, saved {} of them.", wp_count, save_count);
    
    Ok(())
}

pub fn write_failed_sok(error: String, id: &usize) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("failed_sok.log")
        .expect("failed_sok.log File should exist");

    writeln!(file, "Sok: {}, Error: {}", id, error)
}

pub fn write_log(logs: Vec<String>, id: usize) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.log")
        .expect("log.log File should exist");

    writeln!(file, "Sok: {}", id)?;
    
    for line in logs {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn get_checked_soks() -> Vec<usize> {
    let file = File::open("sok.log").expect("sok.log File should exist");

    let reader = io::BufReader::new(file);

    return reader.lines()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.parse::<usize>().ok())
        .collect::<Vec<usize>>();
}

pub fn checkmark_sok(id: &usize) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sok.log")
        .expect("sok.log File should exist");

    writeln!(file, "{}", id).expect("Should be able to write to file");
}