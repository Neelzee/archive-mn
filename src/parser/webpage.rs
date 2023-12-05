use scraper::{self, Html, selector::Selector, error::SelectorErrorKind};

use super::sok::Sok;

pub struct Link {
    parent_id: usize,
    url: String,
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

    pub fn get_links(&self) -> Vec<Link> {
        let mut links = Vec::new();

        if let Ok(a_selector) = Selector::parse("a") {
            links = self.content
                .select(&a_selector)
                .filter_map(|a| a.attr("href"))
                .map(|a| Link { parent_id: self.id, url: a.to_owned() })
                .collect::<Vec<Link>>();
        }
            
        links
    }

    pub fn get_sok(&self) -> Result<Sok, SelectorErrorKind> {
        let mut sok = Sok::new(self.id, self.medium.clone());

        let div_sok_selector = Selector::parse("div[id=sokResult]")?;

        let header_selector = Selector::parse("h4")?;

        // Should just be one div with that id, if there are multiple, we dont really care.
        let div_sok = self.content.select(&div_sok_selector).next().unwrap();

        // Getting title
        // Should just be one h4, inside this div, if not we dont care.
        sok.title = div_sok.select(&header_selector).next().unwrap().text().collect::<String>();

        // Getting tables
        // TODO

        Ok(sok)
    }
}

