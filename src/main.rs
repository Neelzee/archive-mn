use std::{env, fs};
use error::ArchiveError;
use itertools::Itertools;
use modules::{sok::{SokCollection, self}, webpage::{Webpage, Link}};
use parser::wp::get_sok_collection;
use scraper::get_html_content;
use xl::save_sok;
use std::fs::File;
use std::io::prelude::*;

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

    let links: Vec <Link> = parse_args(args)?;

    let mut sok_collections: Vec<SokCollection> = Vec::new();

    let mut webpages: Vec<Webpage> = Vec::new();

    for link in links {
        // TODO: The given links, should be links to the different Mediums
        // TODO: And not specific Soks
        webpages.push(Webpage::from_link(link).await?);
    }

    let mut wp_count = 0;
    
    for wp in webpages {
        match get_sok_collection(wp).await {
            Ok(sc) => {
                sok_collections.push(sc);
                wp_count += 1;
            },
            Err(e) => log.push(e)
        }
    }

    let mut mediums: Vec<String> = Vec::new();
    
    let mut save_count = 0;
    for sokc in sok_collections {
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
    }

    println!("Found {} webpages, saved {} of them.", wp_count, save_count);

    write_log(log.into_iter().map(|e| e.to_string()).collect_vec());

    Ok(())
}

fn write_log(logs: Vec<String>) {
    match File::create("log.txt") {
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