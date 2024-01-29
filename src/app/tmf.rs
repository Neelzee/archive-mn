use std::collections::HashSet;
use std::{fs::OpenOptions, io::Write};

use crate::app::try_save_sok;
use crate::{app::main_fn_tmf, modules::webpage::Link};
use crate::{error::ArchiveError, parse_args, scraper::get_html_content, xl::save_sok};
use itertools::Itertools;
use reqwest::Client;
use scraper::Html;

use crate::app::main_app::write_log;
use crate::app::mf_app::write_failed_sok;
use crate::parser::medium::get_links_from_medium;
use crate::parser::wp::get_sok_collection;

pub async fn run_app(links: Vec<Link>) -> Result<(), ArchiveError> {
    for link in links {
        match main_fn_tmf(&link).await {
            Ok((sokcs, mut sok_log)) => {
                let mut nm: HashSet<String> = HashSet::new();
                for mut sokc in sokcs {
                    while !nm.insert(sokc.title.clone()) {
                        sokc.title = format!("{}1", sokc.title);
                    }
                    let path = format!("arkiv\\{}", sokc.medium);
                    match try_save_sok(&sokc, &path, 2) {
                        Ok(mut log) => {
                            sok_log.append(&mut log);
                            if let Err(e) = write_log(
                                sok_log.clone()
                                    .clone()
                                    .into_iter()
                                    .map(|e| e.to_string())
                                    .collect_vec(),
                                sokc.id,
                            ) {
                                eprintln!("Error writing to logs: {}, dumping log to terminal", e);
                                for l in &sok_log {
                                    eprintln!("{}", l);
                                }
                            } else {
                                println!("Saved sok to {}", path);
                            }
                        }
                        Err(err) => {
                            eprintln!(
                                "Failed saving sok: {};{}, due to errors: {}, dumping it to root",
                                &sokc.id,
                                &sokc.title,
                                err.into_iter().map(|e| e.to_string()).join("\n")
                            );
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed");
                let mut id = 0;
                let url_c = link.to_string().clone();

                let mut str = url_c.split("/").collect::<Vec<_>>().into_iter().rev();

                if let Some(i) = str.next() {
                    id = i.parse::<usize>()?;
                }

                if let Err(e) = write_failed_sok(
                    err.clone().into_iter().map(|e| e.to_string()).join("\n"),
                    &id,
                ) {
                    eprintln!(
                        "Error writing to error logs: {}, dumping log to terminal",
                        e
                    );
                    for l in err {
                        eprintln!("{}", l);
                    }
                }
            }
        }
    }
    Ok(())
}
