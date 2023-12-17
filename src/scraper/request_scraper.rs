use std::{time::Duration, collections::HashMap};

use reqwest::{Request, Client};
use scraper::Html;
use tokio::{sync::mpsc::{Receiver, Sender}, time::sleep};

use crate::{error::ArchiveError, utils::funcs::sending_error};


pub async fn execute_request(
    mut req_channel: Receiver<HashMap<String, String>>,
    html_channel: Sender<String>,
    error_channel: Sender<ArchiveError>
) {
    let client = Client::new();
    println!("Execute Request function enter");
    while let Some(form_data) = req_channel.recv().await {
        let req = todo!(); // TODO: Create request
        match client.execute(req).await {
            // We get a response!
            Ok(response) => {
                let status = response.status().clone();
                // 200
                if status.is_success() {
                    let raw_html = response.text().await;
                    // Manage response html error
                    if raw_html.is_err() {
                        sending_error(error_channel
                            .send(ArchiveError::RequestError(raw_html.unwrap_err().to_string())).await);
                        continue; // Failed to parse the html data, or something
                    } else { // Done with executing the request
                        sending_error(html_channel.send(raw_html.unwrap()).await);
                    }
                } else if status.as_u16() == 403 { // If 403, most likley a restrict, so sleep 5 min?
                    println!("Sleeping: {}", status.as_str());
                    sleep(Duration::from_secs(60 * 5)).await;
                } else { // Error in the response
                    let error = status.as_str();
                    let r = error_channel
                            .send(ArchiveError::RequestError(error.to_string())).await;
                    sending_error(r);
                }
            },
            // Mostlikley end up here, due to how we combine the parameters in the post-requests, (see Form.combinations())
            Err(err) => {
                let r = error_channel.send(ArchiveError::RequestError(err.to_string())).await;
                sending_error(r);
            },
        }
    }
    println!("Execute Request function exiting");
}