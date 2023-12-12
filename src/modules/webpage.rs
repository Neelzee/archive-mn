use scraper::{self, Html, selector::Selector, error::SelectorErrorKind};

use super::sok::Sok;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    url: String,
}

impl Link {
    pub fn new(url: String) -> Link {
        Link { url }
    }

    pub fn is_partial(&self) -> bool {
        self.url.contains("www") || self.url.contains("http")
    }

    pub fn is_external(&self) -> bool {
        !self.url.contains("medienorge") || !self.url.contains("uib")
    }

    pub fn is_internal(&self) -> bool {
        !self.is_external()
    }

    pub fn is_sok(&self) -> bool {
        self.url.contains("medium") || self.url.contains("statestikk")
    }
}


pub struct Webpage {
    id: usize,
    url: String,
    content: Html,
    medium: String
}


impl Webpage {
    pub fn from_html(id: usize, url: String, content: Html, medium: String) -> Webpage {
        Webpage { id, url, content, medium }
    }

    pub fn get_content(&self) -> Html {
        self.content.clone()
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

