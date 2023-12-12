use std::io::{Read, Error};
use std::fs::File;

use scraper::Html;

use crate::parser::get_table;
use crate::modules::webpage::Webpage;

fn get_html_content() -> Result<Html, Error> {
    let mut content = String::new();
    let mut file = File::open("src\\tests\\sok_346.html")?;
    file.read_to_string(&mut content)?;

    Ok(Html::parse_document(&content))
}


#[test]
fn test_get_table() {
    if let Ok(html) = get_html_content() {
        let wb = Webpage::from_html(346, "test.medienorge.uib.no".to_owned(), html, "avis".to_owned());

        let res = get_table(&wb);

        assert!(res.is_ok());

        let tables =  res.unwrap();
        
        for t in tables {
            assert_eq!("Andel med avisabonnement hjemme, fordelt på alle (prosent)", &t.name);
        }

    }
}