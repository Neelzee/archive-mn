use std::borrow::BorrowMut;

use scraper::{Html, Selector};

use crate::{error::ArchiveError, modules::webpage::Link};

pub fn get_links_from_medium(html: Html) -> Result<Vec<(String, Link)>, ArchiveError> {
    let mut links = Vec::new();

    let div_sel = Selector::parse(".listBox a")?;

    for a in html.select(&div_sel) {
        if let Some(l) = a.attr("href") {
            let l = Link::new(l.to_string()).create_full();

            if !l.is_sok() && !l.is_aspekt() {
                continue;
            }

            let mut iter = l
                .clone()
                .to_string()
                .split("/")
                .map(|e| e.to_string())
                .collect::<Vec<String>>();
            iter.pop();
            links.push((iter.pop().unwrap_or("UNKNOWN".to_string()), l));
        }
    }

    Ok(links)
}
