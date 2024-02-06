use std::io::{self, Write};

use std::thread;
use std::time::{Duration, Instant};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use crate::{error::ArchiveError, modules::webpage::{Link, Webpage}};

use crate::{
    parse_args, parser::wp::get_sok_collection_tmf, parser::wp::get_sokc_n,
    scraper::get_html_content, xl::save_sok,
};
use crate::parser::medium;
use itertools::Itertools;
use reqwest::Client;
use scraper::Html;

use std::io::prelude::*;

use crate::parser::medium::get_links_from_medium;


pub async fn pretty_run(args: Vec<String>) -> Result<(), ArchiveError> {
    let client = Client::new();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!("Getting all links");


    let mut mediums: Vec<(String, Link)> = Vec::new();

    let m = MultiProgress::new();
    let handles: Vec<_> = args
        .into_iter()
        .map(|medium_link| async {
            match get_html_content(&client, medium_link.clone().to_string()).await {
                Ok(raw_html) => {
                    let html = Html::parse_document(&raw_html);
                    // TODO: Fix
                    match get_links_from_medium(html) {
                        Ok(unvalidated_links) => {
                            let pb = m.add(ProgressBar::new(unvalidated_links.len() as u64));
                            pb.set_style(spinner_style.clone());
                            if unvalidated_links.is_empty() {
                                pb.set_prefix(format!("[{}/?]", 0));
                            } else {
                                pb.set_prefix(format!("[{}/?]", unvalidated_links[0].0));
                            }
                            return thread::spawn(move || {
                                
                                for (m, l) in unvalidated_links {
                                    pb.set_message(format!("{}: Validating {}", &m, &l.to_string()));
                                    
                                    if l.is_sok() {
                                        pb.inc(1);
                                        pb.set_message(format!("{}: {} validated", &m, &l.to_string()));
                                    } else {
                                        pb.set_message(format!("{}: {} not valid", &m, &l.to_string()));
                                    }
        
                                }
                                pb.finish_with_message("Waiting...");
                            });
                        }
                        Err(err) => return thread::spawn(move || {
                            eprintln!("Failed invalidating links from '{}' got error: {}", medium_link, err);
                        }),
                    }
                },
                Err(err) => return thread::spawn(move || {
                    eprintln!("Failed invalidating links from '{}' got error: {}", medium_link, err);
                }),
            }
        })
        .collect();

    for h in handles {
        let _ = h.await.join();
    }

    m.clear().unwrap();

    println!("Finished validating all links.");

    
    Ok(())
}