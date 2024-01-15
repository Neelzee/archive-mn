use scraper::{Html, Selector};

use crate::{error::ArchiveError, utils::funcs::trim_string};

pub mod medium;
pub mod parse_sok;
pub mod wp;

pub fn get_merknad(html: &Html) -> Result<Vec<String>, ArchiveError> {
    let mut merknad = Vec::new();

    let merknad_selector = Selector::parse(r#"p[class="merknadTekst"]"#)?;

    for p in html.select(&merknad_selector) {
        merknad.push(trim_string(&p.text().collect::<String>()));
    }

    Ok(merknad)
}

pub fn get_text(html: &Html) -> Result<Vec<String>, ArchiveError> {
    let text_selector = Selector::parse(r#"div[id="forklaringTxt"] p"#)?;

    Ok(html
        .select(&text_selector)
        .map(|e| trim_string(&e.text().collect::<String>()))
        .collect::<Vec<String>>())
}

pub fn get_title(html: &Html) -> Result<String, ArchiveError> {
    let title_selector = Selector::parse(".searchTitle")?;

    for el in html.select(&title_selector) {
        if el.value().name() == "h1" {
            return Ok(el.text().collect::<String>().trim().to_owned());
        }
    }

    return Err(ArchiveError::MissingTitle);
}
