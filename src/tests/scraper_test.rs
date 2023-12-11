use reqwest::Client;
use once_cell::sync::Lazy;

use crate::scraper::get_html_content;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[tokio::test]
async fn get_html_content_test() {
    assert!(get_html_content(&CLIENT, "https://www.uib.no".to_owned()).await.is_ok());
    assert!(get_html_content(&CLIENT, "".to_owned()).await.is_err());
}