use std::os::raw;

use reqwest::{Client, StatusCode};
use scraper::Html;

use crate::{utils::funcs::{get_random_webpage, get_html_content_test}, scraper::get_html_content, modules::webpage::Webpage};

#[test]
fn test_form() {
    if let Some(wp) = get_random_webpage() {
        let res = wp.get_forms();

        assert!(res.is_ok());

        let form = res.unwrap();

        println!("{:?}", form);

        for ar in form.combinations() {
            println!("{:?}", ar);
        }

    }
}

#[tokio::test]
async fn test_form_requester() {
    if let Ok(html) = get_html_content_test() {
        let url = "https://medienorge.uib.no/statistikk/medium/avis/346".to_string();
        let wp = Webpage::from_html(346, url.clone(), html, "avis".to_string());
        let res = wp.get_forms();
        assert!(res.is_ok());

        let form = res.unwrap();
        let client = Client::default();

        let request = client
            .post(url.clone());

        for mut ar in form.combinations() {
            ar.insert("btnSubmit".to_string(), "Vis+tabell".to_string());
            let req = request
                .try_clone().expect("Should not be a stream")
                .form(&ar).build().expect("Should work :)");

            let res = client.execute(req).await;

            assert!(res.is_ok());

            let response = res.unwrap();

            assert!(response.status().is_success());

            let raw_html = response.text().await;

            assert!(raw_html.is_ok());

            let html = Html::parse_document(&raw_html.unwrap());

            let sub_wp = Webpage::from_html(346, url.clone(), html, "avis".to_string());

            let res = sub_wp.get_sok();

            assert!(res.is_ok());

            let sok = res.unwrap();

            assert!(sok.tables.len() != 0);
        }
        
    } else {
        println!("Failed to get webpage");
        assert!(false);
    }
}