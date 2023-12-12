/*
Webpage test
*/

use std::{fs::File, io::Read};

use scraper::Html;

use crate::modules::webpage::Webpage;

fn get_webpage() -> Result<Webpage, std::io::Error> {
    let mut content = String::new();

    let mut file = File::open("src\\tests\\sok_346.html")?;
    file.read_to_string(&mut content)?;

    Ok(Webpage::from_html(
        346,
        "https://medienorge.uib.no/statistikk/medium/avis/346".to_string(),
        Html::parse_fragment(&content),
        "avis".to_string()))
}


#[test]
fn test_get_links() {
    if let Ok(wp) = get_webpage() {
        let res = wp.get_links();

        assert!(res.is_ok());

        let links = res.unwrap();

        //assert!(links.len() != 0);

        println!("{:?}", links);
    } else {
        panic!("Could not get webpage to test");
    }
}