mod wp_test;




use reqwest::Client;
use scraper::Html;

use crate::parser::medium::get_links_from_medium;
use crate::scraper::get_html_content;


#[tokio::test]
async fn get_links_from_medium_test() {
    if let Ok(raw_html) = get_html_content(&Client::default(), "https://medienorge.uib.no/statistikk/medium/avis".to_string()).await {
        let html = Html::parse_document(&raw_html);

        let res = get_links_from_medium(html);

        assert!(res.is_ok());

        let links = res.unwrap();

        assert!(links.len() != 0);

        for l in links {
            println!("{:?}", l);
        }
    }
}