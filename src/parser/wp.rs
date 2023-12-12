use scraper::Selector;

use crate::{modules::webpage::{Webpage, Link}, error::ArchiveError};

impl Webpage {
    pub fn get_links(&self) -> Result<Vec<Link>, ArchiveError> {
        let mut links = Vec::new();

        let merknad_head_selector = Selector::parse(".merknadHeader")?;
        let merknad_text_selector = Selector::parse(".merknadTekst")?;
        let a_selector = Selector::parse("a")?;

        // TODO: Fix
        /*
        When using select on classname, only the elements with that class-name is recieved,
        to check chilren, we have to use .children() method
        */

        //println!("{:?}", self.get_content().select(&merknad_head_selector).flat_map(|e| e.children().map(|c| c.value())).collect::<Vec<String>>());
        println!("");
        println!("");
        println!("{:?}", self.get_content().select(&merknad_text_selector).map(|e| e.html()).collect::<Vec<String>>());


        links = self.get_content()
            .select(&merknad_head_selector) 
            .flat_map(|e| e.select(&a_selector))
            .filter_map(|a| a.attr("href"))
            .map(|a| Link::new(a.to_owned()))
            .collect::<Vec<Link>>();

        links.append(
            &mut self.get_content()
                .select(&merknad_text_selector)
                .flat_map(|e| e.select(&a_selector))
                .filter_map(|a| a.attr("href"))
                .map(|a| Link::new(a.to_owned()))
                .collect::<Vec<Link>>()
        );

        links.sort();
        links.dedup_by(|a, b| a == b);    
        
        Ok(links)
    }

    pub fn get_title(&self) -> Result<String, ArchiveError> {
        let title_selector = Selector::parse(r#"h1[class="searchTitle"]"#)?;

        match self.get_content().select(&title_selector).next() {
            Some(title) => Ok(title.text().collect::<String>()),
            None => Err(ArchiveError::MissingTitle),
        }
    }

    pub fn get_text(&self) -> Result<Vec<String>, ArchiveError> {
        let mut text = Vec::new();

        let text_selector = Selector::parse(r#"div[id="forklaringTxt"]"#)?;

        let p_selector = Selector::parse(r#"p"#)?;

        for el in self.get_content().select(&text_selector) {
            for p in el.select(&p_selector) {
                text.push(p.text().collect::<String>());
            }
        }

        Ok(text)
    }
}