use std::{fs::File, io::Write};

use itertools::Itertools;
use reqwest::Client;
use scraper::Html;
use serde_json::{json, Value};

use crate::{app::{main_app::{write_failed_sok, write_log}, main_fn, try_save_sok}, error::ArchiveError, modules::{sok::JsonSok, webpage::Link}, parser::medium::get_links_from_medium, scraper::get_html_content, xl::save_sok};

pub async fn jsonify_soks(medium_links: Vec<Link>) -> Result<(), ArchiveError> {

    let mut log = Vec::new();
    let client = Client::default();
    for medium_link in medium_links {
        let raw_html = get_html_content(&client, medium_link.to_string()).await?;
        let html = Html::parse_document(&raw_html);
        for (m, link) in get_links_from_medium(html)? {
            let medium = medium_link
                .to_string()
                .split("/")
                .last()
                .and_then(|e| Some(e.to_string()))
                .unwrap_or(m);
            match main_fn(&link).await {
                Ok((mut sokc, sok_log)) => {
                    sokc.medium = medium.to_string();
                    let path = format!("json\\{}", medium);
                    
                    match serde_json::to_string_pretty(&JsonSok::new(sokc, sok_log)) {
                        Ok(content) => {
                            let mut file = File::create(path)?;
                            file.write_all(content.as_bytes())?;
                        },
                        Err(e) => {
                            log.push(e);
                        },
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

    Ok(())
}