use std::{env, fs};
use error::ArchiveError;
use itertools::Itertools;
use modules::{sok::{SokCollection, self}, webpage::{Webpage, Link}};
use parser::wp::get_sok_collection;
use reqwest::Client;
use ::scraper::Html;
use scraper::get_html_content;
use xl::save_sok;
use std::fs::File;
use std::io::prelude::*;

use crate::parser::medium::get_links_from_medium;

mod error;
mod modules;
mod parser;
mod scraper;
mod utils;
mod xl;
mod tests;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), ArchiveError> {
    let mut args: Vec<String> = env::args().collect();

    let mut log: Vec<ArchiveError> = Vec::new();

    args.remove(0); // First argument is path to the exe
    if args.len() == 0 {
        println!("Missing URL-argument.");
        return Err(ArchiveError::InvalidURL);
    }

    let medium_links: Vec <Link> = parse_args(args)?;

    let mut sok_collections: Vec<SokCollection> = Vec::new();
    let mut wp_count = 0;
    let mut save_count = 0;
    let mut mediums: Vec<String> = Vec::new();

    let client = Client::default();
    for medium_link in medium_links {
        let raw_html = get_html_content(&client, medium_link.to_string()).await?;
        let html = Html::parse_document(&raw_html);
        for link in get_links_from_medium(html)? {

            let wp = Webpage::from_link(link).await?;

            match get_sok_collection(wp).await {
                Ok(sokc) => {
                    wp_count += 1;

                    let path = format!("src\\out\\{}", sokc.medium.clone());

                    if !mediums.contains(&sokc.medium) {
                        mediums.push(sokc.medium.clone());
                        let r = fs::create_dir_all(path.clone());
                        if r.is_err() {
                            println!("Could not create path: {}", path.clone());
                        }
                    }

                    match save_sok(sokc, &path) {
                        Ok(_) => save_count += 1,
                        Err(e) => log.push(e), 
                    }
                },
                Err(e) => log.push(e)
            }

        }
    }

    println!("Found {} webpages, saved {} of them.", wp_count, save_count);
    if log.len() != 0 {
        println!("{} errors found.", log.len());
        write_log(log.into_iter().map(|e| e.to_string()).collect_vec());
    } else {
        println!("No errors reported.");
    }

    Ok(())
}

fn write_log(logs: Vec<String>) {
    match File::create("log.log") {
        Ok(mut file) => {
            for t in logs {
                let _ = file.write(t.as_bytes());
            }
        }
        Err(e) => eprintln!("Failed writing log: {}", e),
    }
}

fn parse_args(args: Vec<String>) -> Result<Vec<Link>, ArchiveError> {
    let mut links = Vec::new();
    for arg in args {
        let link = Link::new(arg);
        if link.is_external() || link.is_partial() {
            return Err(ArchiveError::InvalidURL);
        }
        links.push(link);
    }

    Ok(links)
}

/*
async fn parse_webpage(url: String) -> Result<SokCollection, ArchiveError<'static>> {
    get_sok_collection(
        &Webpage::from_link(
            Link::new(url).create_full()
        ).await?
    ).await
}
*/