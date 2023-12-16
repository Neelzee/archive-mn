// # Scraper module
// 
// `CURL`s a given URL for its HTML content

use reqwest::Client;
use scraper::Html;

pub mod html_scraper;
pub mod request_scraper;

use crate::{modules::webpage::Webpage, error::ArchiveError};


pub async fn get_html_content(client: &Client, url: String) -> Result<String, reqwest::Error> {
    client
        .get(url)
        .send()
        .await?
        .text()
        .await
}


impl Webpage {
    /// Creates a webpage struct from the given url
    pub async fn from_url(client: &Client, url: String) -> Result<Webpage, ArchiveError> {
        let raw_content = get_html_content(client, url.clone()).await?;

        let content = Html::parse_fragment(&raw_content);

        let mut url_path = url.split("/").collect::<Vec<&str>>();
        url_path.reverse();

        let id = url_path.pop().unwrap().parse::<usize>()?;

        let medium = url_path.pop().unwrap().to_owned();

        Ok(Webpage::from_html(id, url, content, medium))
    }
}