use std::{fs::OpenOptions, io::Write};

use crate::app::try_save_sok;
use crate::{app::main_fn, modules::webpage::Link};
use crate::{error::ArchiveError, parse_args, scraper::get_html_content, xl::save_sok};
use itertools::Itertools;
use reqwest::Client;
use scraper::Html;

use crate::parser::medium::get_links_from_medium;

pub async fn run_app(args: Vec<String>) -> Result<(), ArchiveError> {
    if args.len() == 0 {
        println!("Missing URL-argument.");
        return Err(ArchiveError::InvalidURL);
    }

    let medium_links: Vec<Link> = parse_args(args)?;

    let mut wp_count = 0;
    let mut save_count = 0;

    let client = Client::default();
    for medium_link in medium_links {
        let raw_html = get_html_content(&client, medium_link.to_string()).await?;
        let html = Html::parse_document(&raw_html);
        for (m, link) in get_links_from_medium(html)? {
            let medium = medium_link.to_string().split("/").last().unwrap_or(&m);
            wp_count += 1;
            match main_fn(&link).await {
                Ok((mut sokc, mut sok_log)) => {
                    sokc.medium = medium.to_string();
                    let path = format!("arkiv\\{}", medium);
                    match try_save_sok(&sokc, &path, 2) {
                        Ok(mut log) => {
                            sok_log.append(&mut log);
                            save_count += 1;
                            if let Err(e) = write_log(
                                sok_log
                                    .clone()
                                    .into_iter()
                                    .map(|e| e.to_string())
                                    .collect_vec(),
                                sokc.id,
                            ) {
                                eprintln!("Error writing to logs: {}, dumping log to terminal", e);
                                for l in sok_log {
                                    eprintln!("{}", l);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!(
                                "Failed saving sok: {};{}, due too errors: {}, dumping it to root",
                                &sokc.id,
                                &sokc.title,
                                err.into_iter().map(|e| e.to_string()).join("\n")
                            );
                        }
                    }
                }
                Err(err) => {
                    let mut id = 0;
                    let url_c = link.to_string().clone();

                    let mut str = url_c.split("/").collect::<Vec<_>>().into_iter().rev();

                    if let Some(i) = str.next() {
                        id = i.parse::<usize>()?;
                    }

                    if let Err(e) = write_failed_sok(
                        err.clone().into_iter().map(|e| e.to_string()).join("\n"),
                        &id,
                        "UNKNOWN".to_string(),
                    ) {
                        eprintln!(
                            "Error writing too error logs: {}, dumping log to terminal",
                            e
                        );
                        for l in err {
                            eprintln!("{}", l);
                        }
                    }
                }
            }
        }
    }

    println!("Found {} webpages, saved {} of them.", wp_count, save_count);

    Ok(())
}

pub fn write_failed_sok(error: String, id: &usize, title: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("failed_sok.log")
        .expect("failed_sok.log File should exist");

    writeln!(file, "Sok: {};{}, Error: {}", id, title, error)
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

pub fn checkmark_sok(id: &usize, title: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sok.log")
        .expect("sok.log File should exist");

    writeln!(file, "{};{}", id, title).expect("Should be able to write to file");
}
