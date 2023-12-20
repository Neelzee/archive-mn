use reqwest::Client;
use scraper::Html;
use tokio::sync::mpsc::Sender;
use crate::conc::failed_sending;
use crate::modules::webpage::Link;
use crate::parser::medium::get_links_from_medium;
use crate::scraper::get_html_content;

/// Gets all links in a given medium
pub async fn medium_conc(
    medium: Link,
    link_channel: Sender<String>,
    error: Sender<String>
) {
    println!("Medium Conc: '{:?}'", medium.clone().to_string());
    let client = Client::default();
    let res = get_html_content(&client, medium.to_string()).await;

    if res.is_err() {
        failed_sending(
            error.send(
                format!("Medium: {}, Error: {}", medium.to_string(), res.unwrap_err())
            ).await,
            "error".to_string()
        );
        return;
    }

    let html = res.unwrap();

    match get_links_from_medium(Html::parse_document(&html)) {
        Ok(links) => {
            for l in links {
                failed_sending(
                    link_channel.send(l.create_full().to_string()).await,
                    "link_channel".to_string()
                );
            }
        }
        Err(err) => {
            failed_sending(
                error.send(format!("Medium: {}, Error: {}", medium.to_string(), err)).await,
                "error".to_string()
            );
        }
    }
}