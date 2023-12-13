use scraper::Selector;

use crate::{modules::webpage::{Webpage, Link}, error::ArchiveError, utils::funcs::trim_string};

impl Webpage {
    pub fn get_links(&self) -> Result<Vec<Link>, ArchiveError> {
        let mut links = Vec::new();

        let merknad_head_selector = Selector::parse(".merknadHeader")?;
        let a_selector = Selector::parse("a.bold-text[href][onclick]")?;

        // TODO: Fix

        // METODE
        for a in self.get_content().select(&merknad_head_selector).filter_map(|e| e.parent()) {
            for child in a.children() {
                if let Some(el) = child.value().as_element() {
                    if el.name() == "a" {
                        if let Some(a) = el.attr("href") {
                            links.push(Link::new(a.to_owned()));
                        }
                    }
                }
            }
        }

        // Kilder?
        for el in self.get_content().select(&a_selector) {
            if let Some(a) = el.attr("href") {
                links.push(Link::new(a.to_owned()));
            }
        }

        links.sort();
        links.dedup();
        
        Ok(links)
    }

    pub fn get_title(&self) -> Result<String, ArchiveError> {
        let title_selector = Selector::parse(".searchTitle")?;

        for el in self.get_content().select(&title_selector) {
            if el.value().name() == "h1" {
                return Ok(el.text().collect::<String>().trim().to_owned());
            }
        }

        return Err(ArchiveError::MissingTitle);
    }

    pub fn get_text(&self) -> Result<Vec<String>, ArchiveError> {
        let text_selector = Selector::parse(r#"div[id="forklaringTxt"] p"#)?;

        Ok(
            self
                .get_content()
                .select(&text_selector)
                .map(|e| trim_string(&e.text().collect::<String>()))
                .collect::<Vec<String>>()
        )
    }
}
