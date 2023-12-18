use std::{env, fs::{self, OpenOptions}, io, collections::HashMap, ops::Index, future::Future};
use error::ArchiveError;
use itertools::Itertools;
use logger::{error_logger::error_logger, sok_logger::sok_logger};
use modules::{sok::{SokCollection, self, Sok, Merknad}, webpage::{Webpage, Link}};
use parser::{wp::{get_sok_collection, get_kilde, get_metode}, parse_sok::parse_sok, get_text, get_merknad};
use reqwest::{Client, Request};
use ::scraper::Html;
use scraper::{get_html_content, html_scraper::scrape_html, request_scraper::execute_request};
use tokio::sync::mpsc::{self, Receiver, Sender, error::SendError};
use xl::save_sok;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant, Duration};
use crate::{parser::medium::get_links_from_medium, app::{main_app::run_app, single_app::get_soks, offline_app::get_soks_offline}};

mod error;
mod logger;
mod modules;
mod parser;
mod scraper;
mod utils;
mod xl;
mod app;
mod tests;

fn setup() -> io::Result<()> {
    File::create("sok.log")?;
    File::create("log.log")?;
    File::create("failed_sok.log")?;
    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), ArchiveError> {
    let _ = setup();
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // First argument is path to the exe
    if args.len() == 0 {
        args.append(
            &mut vec![
                "https://medienorge.uib.no/statistikk/medium/avis".to_string(),
                "https://medienorge.uib.no/statistikk/medium/fagpresse".to_string(),
                "https://medienorge.uib.no/statistikk/medium/ukepresse".to_string(),
                "https://medienorge.uib.no/statistikk/medium/boker".to_string(),
                "https://medienorge.uib.no/statistikk/medium/radio".to_string(),
                "https://medienorge.uib.no/statistikk/medium/fonogram".to_string(),
                "https://medienorge.uib.no/statistikk/medium/tv".to_string(),
                "https://medienorge.uib.no/statistikk/medium/kino".to_string(),
                "https://medienorge.uib.no/statistikk/medium/video".to_string(),
                "https://medienorge.uib.no/statistikk/medium/ikt".to_string()
            ]
        );
    }

    if args.contains(&"-err".to_string()) {
        // TODO: Add stop on first error code
    }

    if args.contains(&"-offline".to_string()) {
        let time_start = Instant::now();
        let r = get_soks_offline().await;
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!("That took: {} seconds", duration.as_secs());
        return r;
    }

    if args.contains(&"-sok".to_string()) {
        let mut j = 0;
        for i in 0..args.len() {
            if args.get(i).unwrap() == &"-sok".to_string() {
                j = i;
                break;
            }
        }

        args.swap_remove(j);

        match parse_args(args) {
            Ok(links) => get_soks(links).await?,
            Err(err) => println!("Failed parsing args: {}", err),
        }
        
        return Ok(());
    }


    let time_start = Instant::now();
    let r = run_app(args).await;
    let time_end = Instant::now();
    let duration = time_end - time_start;
    println!("That took: {} seconds", duration.as_secs());
    r
}

fn parse_args(args: Vec<String>) -> Result<Vec<Link>, ArchiveError> {
    let mut links = Vec::new();
    for arg in args {
        let link = Link::new(arg.clone());
        if link.is_external() || link.is_partial() {
            println!("Invalid URL: {}", arg);
            return Err(ArchiveError::InvalidURL);
        }
        links.push(link);
    }

    Ok(links)
}
