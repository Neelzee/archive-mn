use scraper::{Html, Selector};

use crate::{modules::webpage::Link, error::ArchiveError};

pub fn get_links_from_medium(html: Html) -> Result<Vec<Link>, ArchiveError> {
    let mut links = Vec::new();

    let a_selector = Selector::parse(r#"a[class="d-inline"]"#)?;

    for a in html.select(&a_selector) {
        if let Some(l) = a.attr("href") {
            links.push(Link::new(l.to_string()).create_full());
        }
    }

    Ok(links)
}