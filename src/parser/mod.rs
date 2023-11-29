use scraper::{self, Html};

pub struct Webpage {
    id: usize,
    content: scraper::html::Html,
}

pub struct Link {
    parent_id: usize,
    url: String,
}

pub struct Table {
    parent_id: usize,
    rows: Vec<String>,
    columns: Vec<String>
}

pub struct Sok {
    parent_id: usize,
    id: usize,
    tables: Vec<Table>,
    merknad: Vec<String>,
    kilde: Vec<String>,
}

impl Webpage {
    pub fn from_html(id: usize, content: scraper::html::Html) -> Webpage {
        Webpage { id, content }
    }

    pub fn get_links(&self) -> Vec<Link> {
        let mut links = Vec::new();

        if let Ok(a_selector) = scraper::selector::Selector::parse("a") {
            links = self.content
                .select(&a_selector)
                .filter_map(|a| a.attr("href"))
                .map(|a| Link { parent_id: self.id, url: a.to_owned() })
                .collect::<Vec<Link>>();
        }
            
        links
    }

    pub fn get_sok(&self) -> Option<Sok> {
        None
    }
}
