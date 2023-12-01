// # Scraper module
// 
// CURLs a given URL for its HTML content

use reqwest::Client;


pub async fn get_html_content(client: &Client, url: String) -> Result<String, reqwest::Error> {
    client
        .get(url)
        .send()
        .await?
        .text()
        .await
}