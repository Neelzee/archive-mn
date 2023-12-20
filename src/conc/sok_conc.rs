use std::cell::Cell;
use std::sync::{Arc, RwLock};

use tokio::sync::mpsc::{Receiver, Sender};
use crate::conc::failed_sending;
use crate::modules::webpage::{Link, Webpage};
use crate::parser::wp::get_sok_collection;
use crate::xl::save_sok;


/// # Sokc Concurrent
///
/// Parses soks
pub async fn sok_conc(
    medium: String,
    mut link_channel: Receiver<String>,
    logger: Sender<String>,
    error: Sender<String>
) {
    println!("Started sok_conc: {}", &medium);
    while let Some(link) = link_channel.recv().await {
        match Webpage::from_link(Link::new(link.clone()).create_full()).await {
            Ok(wp) => {
                let id = wp.get_id();
                match get_sok_collection(wp).await {
                    Ok((sokc, log)) => {
                        // Logging
                        for l in log {
                            failed_sending(
                                logger.send(format!("Sok: {}, Error: {}", &id, l)).await,
                                "logger".to_string()
                            );
                        }
                        match save_sok(sokc, &format!("arkiv\\{}", &medium)) {
                            Ok(log) => {
                                for l in log {
                                    failed_sending(
                                        logger.send(format!("Sok: {}, Error: {}", &id, l)).await,
                                        "logger".to_string()
                                    );
                                }
                            }
                            Err(err) => {
                                failed_sending(
                                    error.send(format!("Sok: {}, Error: {}", &id, err)).await,
                                    "error".to_string()
                                );
                            }
                        }
                    }
                    Err(err) => {
                        failed_sending(
                            error.send(format!("Sok: {}, Error: {}", &id, err)).await,
                            "error".to_string()
                        );
                    }
                }
            }
            Err(err) => {
                failed_sending(
                    error.send(format!("Link: {}, Error: {}", link, err)).await,
                    "error".to_string()
                );
            }
        }
    }
    println!("Exited sok_conc: {}", medium);
}