mod wp_test;

use std::io::{Read, Error};
use std::fs::File;


use reqwest::Client;
use scraper::Html;

use crate::parser::get_table;
use crate::modules::webpage::Webpage;
use crate::scraper::get_html_content;

#[tokio::test]
async fn test_get_table() {
    if let Ok(raw_html) = get_html_content(&Client::default(), "src\\tests\\html\\346".to_string()).await {
        let html = Html::parse_document(&raw_html);
        let wb = Webpage::from_html(346, "test.medienorge.uib.no".to_owned(), html, "avis".to_owned());

        let res = get_table(&wb);

        assert!(res.is_ok());

        let tables =  res.unwrap();
        

    }
}