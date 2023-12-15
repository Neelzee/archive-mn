use reqwest::Client;
use scraper::Html;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{modules::webpage::Link, error::ArchiveError};

use super::get_html_content;

pub async fn scrape_html(mut url_channel: Receiver<Link>, html_channel: Sender<Html>, error_channel: Sender<ArchiveError>) {
    let client = Client::default();
    // Links that are recieved are the links to the sok
    // So we should get all sub sok, while we are here.
    // TODO: Some of the code in get_sok_collection should be here
    while let Some(link) = url_channel.recv().await {
        match get_html_content(&client, link.to_string()).await {
            Ok(raw_html) => {
                let html = Html::parse_document(&raw_html);
                html_channel.send(html).await
                    .is_err()
                    .then(|| println!("Failed sending html content for url: {}", link.to_string()));
            },
            Err(err) => {
                let aerr: ArchiveError = err.into();
                error_channel
                    .send(aerr.clone())
                    .await
                    .is_err()
                    .then(|| println!("Failed sending error to ErrorLogger: {}", aerr));
            },
        }
    }
}