use std::{env, fs::{self, OpenOptions, File}, io::{self, Write}, time::Instant};

use itertools::Itertools;
use scraper::Html;
use crate::{error::ArchiveError, xl::save_sok, scraper::get_html_content, parse_args};
use crate::modules::webpage::{Webpage, Link};
use crate::parser::wp::get_sok_collection;
use reqwest::Client;

use std::io::prelude::*;

use crate::parser::medium::get_links_from_medium;


pub async fn run_app() -> Result<(), ArchiveError> {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // First argument is path to the exe
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
            
            
            let wp = Webpage::from_link(link.clone()).await?;
            let medium = wp.get_medium();
            let id = wp.get_id().clone();
            if checked_sok.contains(&wp.get_id()) {
                continue;
            }
            checked_sok.push(wp.get_id());
            let time_start = Instant::now();

            if let Some(com) = wp.get_forms().ok() {
                let count = com.combinations().collect::<Vec<_>>().len();
                if count <= 30 {
                    println!("Sok: {}", wp.get_id());
                } else {
                    println!("Sok: {}", wp.get_id());
                    println!("Form Combo: {:?}", count);
                    continue;
                }
            }

            match get_sok_collection(wp).await {
                Ok((sokc, mut errs)) => {
                    wp_count += 1;

                    sok_log.append(&mut errs);

                    let path = format!("src\\out\\{}", medium.clone());

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
                        continue;
                    }

                    match save_sok(sokc, &path) {
                        Ok(_) => {
                            save_count += 1;
                            checkmark_sok(&id);
                            println!("Saved sok: {}, Took {}s", &id, (time_end - time_start).as_secs());
                        },
                        Err(e) => {
                            println!("Failed saving sok: {}, With Error: {}, Took {}s", &id, &e, (time_end - time_start).as_secs());
                            sok_log.push(e);
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