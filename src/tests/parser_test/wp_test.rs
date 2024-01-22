/*
Webpage test
*/

use std::{fs::File, io::Read, iter::zip};

use reqwest::Client;
use scraper::Html;

use crate::{
    modules::webpage::{Link, Webpage},
    parser::{
        medium::get_links_from_medium,
        wp::{get_kilde, get_metode},
    },
    utils::funcs::can_reqwest,
};

fn get_webpage() -> Result<Webpage, std::io::Error> {
    let mut content = String::new();

    let mut file = File::open("src\\tests\\html\\346")?;
    file.read_to_string(&mut content)?;

    Ok(Webpage::from_html(
        346,
        "https://medienorge.uib.no/statistikk/medium/avis/346".to_string(),
        Html::parse_fragment(&content),
        "avis".to_string(),
    ))
}

#[test]
fn test_get_title() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_title();

        assert!(res.is_ok());

        let title = res.unwrap();

        assert!(title.len() != 0);
        assert_eq!("Andel med papiravisabonnement og antall abonnement", &title);
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[test]
fn test_get_forms() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_forms();

        assert!(res.is_ok());

        let form = res.unwrap();

        println!("{:?}", form);
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[tokio::test]
async fn test_get_sok() {
    if !can_reqwest().await {
        return;
    }
    if let Ok(wp) = Webpage::from_link(Link::new(
        "https://medienorge.uib.no/statistikk/medium/ukepresse/336".to_string(),
    ))
    .await
    {
        let res = wp.get_sok().await;

        assert!(res.is_ok());

        let _sok = res.unwrap();
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[test]
fn test_get_merknad() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_merknad();

        assert!(res.is_ok());

        let merknad = res.unwrap();

        assert!(merknad.len() != 0);

        println!("{:?}", merknad);
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[tokio::test]
async fn test_get_metode() {
    if !can_reqwest().await {
        return;
    }
    if let Ok(wp) = get_webpage() {
        let res = get_metode(&wp).await;

        assert!(res.is_ok());

        let merknad = res.unwrap();

        assert!(merknad.len() == 1);

        println!("{:?}", merknad);
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[tokio::test]
async fn test_get_kilde() {
    if !can_reqwest().await {
        return;
    }
    if let Ok(wp) = get_webpage() {
        let res = get_kilde(&wp).await;

        assert!(res.is_ok());

        let kilde = res.unwrap();

        assert!(kilde.len() == 1);

        println!("{:?}", kilde);
    } else {
        eprintln!("Could not get webpage to test");
    }
}

#[tokio::test]
async fn test_get_medium_links() {
    if !can_reqwest().await {
        return;
    }
    let client = Client::default();
    let res = client
        .get("https://medienorge.uib.no/statistikk/medium/boker")
        .send()
        .await;

    assert!(res.is_ok());

    let response = res.unwrap();

    assert!(response.status().is_success());

    let res = response.text().await;

    assert!(res.is_ok());

    let html = Html::parse_document(&res.unwrap());

    let res = get_links_from_medium(html);

    assert!(res.is_ok());

    let links = res.unwrap();

    println!("{:?}", links);
}
