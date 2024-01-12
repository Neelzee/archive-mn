use std::collections::HashMap;

use reqwest::Client;
use scraper::Html;

use crate::{
    modules::webpage::Webpage,
    utils::funcs::{can_reqwest, get_html_content_test, get_random_webpage},
};

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

#[test]
fn test_form_titler() {
    if let Some(wp) = get_random_webpage() {
        let res = wp.get_forms();

        assert!(res.is_ok());
    }
}

#[tokio::test]
async fn test_form_requester() {
    if !can_reqwest().await {
        return;
    }
    if let Ok(html) = get_html_content_test() {
        let url = "https://medienorge.uib.no/statistikk/medium/avis/346".to_string();
        let wp = Webpage::from_html(346, url.clone(), html, "avis".to_string());
        let res = wp.get_forms();
        assert!(res.is_ok());

        let form = res.unwrap();
        let client = Client::default();

        let request = client.post(url.clone());

        for form in form.combinations() {
            let mut form_data = HashMap::new();

            for (k, (v, _)) in form {
                form_data.insert(k, v);
            }

            form_data.insert("btnSubmit".to_string(), "Vis+tabell".to_string());
            let req = request
                .try_clone()
                .expect("Should not be a stream")
                .form(&form_data)
                .build()
                .expect("Should work :)");

            let res = client.execute(req).await;

            if res.is_err() {
                continue;
            }

            let response = res.unwrap();

            assert!(response.status().is_success());

            let raw_html = response.text().await;

            assert!(raw_html.is_ok());

            let html = Html::parse_document(&raw_html.unwrap());

            let sub_wp = Webpage::from_html(346, url.clone(), html, "avis".to_string());

            let res = sub_wp.get_sok().await;
            assert!(res.is_ok());

            let sok = res.unwrap();

            assert!(sok.tables.len() != 0);
        }
    } else {
        eprintln!("Failed to get webpage");
    }
}
