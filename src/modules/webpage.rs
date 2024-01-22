use reqwest::Client;
use scraper::Html;

use crate::{error::ArchiveError, scraper::get_html_content, utils::constants::ROOT_URL};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link {
    url: String,
}

impl Link {
    pub fn new(url: String) -> Link {
        Link { url }
    }

    /// Creates a full link out of itself
    pub fn create_full(&self) -> Link {
        if !self.is_partial() {
            self.clone()
        } else {
            Link {
                url: format!("{}{}", ROOT_URL, self.url.clone()),
            }
        }
    }

    pub fn is_metode(&self) -> bool {
        self.url.contains("metode")
    }

    pub fn is_sok(&self) -> bool {
        if let Some(id) = self.url.clone().split("/").last() {
            self.url.contains("statistikk/medium") && id.parse::<u32>().is_ok()
        } else {
            false
        }
    }

    pub fn is_aspekt(&self) -> bool {
        if let Some(id) = self.url.clone().split("/").last() {
            self.url.contains("statistikk/aspekt") && id.parse::<u32>().is_ok()
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        return self.url.clone();
    }

    pub fn is_partial(&self) -> bool {
        !self.url.contains("http")
    }

    pub fn is_external(&self) -> bool {
        !self.url.contains("medienorge") || !self.url.contains("uib")
    }
}

#[derive(Debug, Clone)]
pub struct Webpage {
    url: String,
    id: usize,
    medium: String,
    content: Html,
}

impl Webpage {
    pub fn from_html(id: usize, url: String, content: Html, medium: String) -> Webpage {
        Webpage {
            id,
            url,
            content,
            medium,
        }
    }

    pub fn set_medium(&mut self, medium: String) {
        self.medium = medium;
    }

    pub fn get_id(&self) -> usize {
        self.id.clone()
    }

    pub fn get_medium(&self) -> String {
        self.medium.clone()
    }

    pub async fn from_link(link: Link) -> Result<Webpage, ArchiveError> {
        let url = link.create_full().to_string();
        let raw_content = get_html_content(&Client::default(), url.clone()).await?;

        let content = Html::parse_document(&raw_content);

        let mut id = 0;
        let mut medium = String::new();
        let url_c = url.clone();

        let mut str = url_c.split("/").collect::<Vec<_>>().into_iter().rev();

        if let Some(i) = str.next() {
            id = i.parse::<usize>()?;
        }

        if let Some(m) = str.next() {
            medium = m.to_owned();
        }

        Ok(Webpage {
            id,
            medium,
            url,
            content,
        })
    }

    pub fn get_content(&self) -> Html {
        self.content.clone()
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
