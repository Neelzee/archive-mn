use reqwest::Client;
use scraper::{Selector, Html};

use crate::{modules::{webpage::{Webpage, Link, self}, form::Form, sok::{Sok, Table, SokCollection, self, Merknad}}, error::ArchiveError, utils::funcs::{trim_string, has_ancestor, format_form_to_title}, scraper::get_html_content};

// TODO: Change these from methods to functions
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

    pub fn get_forms(&self) -> Result<Form, ArchiveError> {
        let select_selector = Selector::parse("select")?;
        let option_selector = Selector::parse("option")?;

        let mut form = Form::new();

        for select in self.get_content().select(&select_selector) {
            if let Some(option_name) = select.attr("name") {
                let mut options: Vec<(String, String)> = Vec::new();

                for option in self.get_content().select(&option_selector) {
                    if let Some(p) = option.parent() {
                        if p.id() != select.id() {
                            continue;
                        }

                        if let Some(v) = option.attr("value") {
                            options.push((v.to_string(), trim_string(&option.text().collect::<String>())));
                        }
                    }
                }

                form.add_options(option_name.to_string(), options);
            }
        }

        Ok(form)
    }

    pub fn get_sok(&self) -> Result<Sok, ArchiveError> {
        let mut sok = Sok::new();

        let title_selector = Selector::parse(r#"div[id="sokResult"] h4"#)?;

        let table_selector = Selector::parse(r#"div[id="sokResult"] table"#)?;

        let tr_selector = Selector::parse(r#"div[id="sokResult"] tr"#)?;
        let th_selector = Selector::parse(r#"div[id="sokResult"] th"#)?;
        let td_selector = Selector::parse(r#"div[id="sokResult"] td"#)?;

        for t in self.get_content().select(&title_selector) {
            sok.title = trim_string(&t.text().collect::<String>());
            break;
        }

        let mut tables: Vec<Table> = Vec::new();

        for table in self.get_content().select(&table_selector) {
            let mut cur_table = Table::new();

            // Header
            // Rows
            let mut headers: Vec<Vec<String>> = Vec::new();
            for tr in self.get_content().select(&tr_selector) {
                // This row belongs to the current table
                if has_ancestor(*tr, table.id()) {
                    // Iterating over all cells
                    let mut row: Vec<String> = Vec::new();
                    for td in self.get_content().select(&th_selector) {
                        // This cell belongs to the current row
                        if has_ancestor(*td, tr.id()) {
                            let txt = trim_string(&td.text().collect::<String>());

                            if txt.len() == 0 || txt.contains('\u{a0}') {
                                continue;
                            }

                            row.push(txt);
                        }
                    }

                    if row.len() != 0 {
                        headers.push(row);
                    }
                }
            }

            cur_table.header = headers;

            // Rows
            let mut rows: Vec<Vec<String>> = Vec::new();
            for tr in self.get_content().select(&tr_selector) {
                // This row belongs to the current table
                if has_ancestor(*tr, table.id()) {
                    // Iterating over all cells
                    let mut row: Vec<String> = Vec::new();
                    for td in self.get_content().select(&td_selector) {
                        // This cell belongs to the current row
                        if has_ancestor(*td, tr.id()) {
                            let txt = trim_string(&td.text().collect::<String>());

                            if txt.len() == 0 || txt.contains('\u{a0}') {
                                continue;
                            }

                            row.push(txt);
                        }
                    }
                    if row.len() != 0 {
                        rows.push(row);
                    }
                }
            }

            cur_table.rows = rows;

            // TODO: Get style aswell.

            tables.push(cur_table);

        }

        sok.tables = tables;

        Ok(sok)
    }

    pub fn get_merknad(&self) -> Result<Vec<String>, ArchiveError> {
        let mut merknad = Vec::new();

        let merknad_selector = Selector::parse(r#"p[class="merknadTekst"]"#)?;

        for p in self.get_content().select(&merknad_selector) {
            merknad.push(trim_string(&p.text().collect::<String>()));
        }

        Ok(merknad)
    }
}


pub async fn get_metode(wp: &Webpage) -> Result<Vec<(String, Vec<String>)>, ArchiveError> {
    let mut metoder: Vec<(String, Vec<String>)> = Vec::new();

    let mut links = Vec::new();

    let merknad_head_selector = Selector::parse(".merknadHeader")?;
    let p_selector = Selector::parse("p")?;
    let h3_selector = Selector::parse("h3")?;

    // METODE
    for a in wp.get_content().select(&merknad_head_selector).filter_map(|e| e.parent()) {
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

    for l in links {
        let url = l.create_full().to_string();
        let mut title = String::new();
        let content = get_html_content(&Client::default(), url).await?;
        for h in Html::parse_document(&content).select(&h3_selector) {
            title = trim_string(&h.text().collect::<String>());
            break;   
        }

        metoder.push(
            (
                title,
                Html::parse_document(&content)
                    .select(&p_selector)
                    .map(|p| trim_string(&p.text().collect::<String>()))
                    .collect::<Vec<String>>()
            )
        );

    }


    Ok(metoder)
}

pub async fn get_kilde(wp: &Webpage) -> Result<Vec<(String, Vec<String>)>, ArchiveError> {
    let mut kilder: Vec<(String, Vec<String>)> = Vec::new();

    let mut links = Vec::new();

    let a_selector = Selector::parse("a.bold-text[href][onclick]")?;
    let div_selector = Selector::parse(".brodtekst")?;
    let h2_selector = Selector::parse("h2")?;

    // Kilde
    for el in wp.get_content().select(&a_selector) {
        if let Some(a) = el.attr("href") {
            links.push(Link::new(a.to_owned()));
        }
    }

    links.sort();
    links.dedup();

    for l in links {
        let url = l.create_full().to_string();
        let mut title = String::new();
        let content = get_html_content(&Client::default(), url).await?;
        for h in Html::parse_document(&content).select(&h2_selector) {
            title = trim_string(&h.text().collect::<String>());
            break;   
        }

        kilder.push(
            (
                title,
                Html::parse_document(&content)
                    .select(&div_selector)
                    .map(|p| trim_string(&p.text().collect::<String>()))
                    .collect::<Vec<String>>()
            )
        );

    }


    Ok(kilder)
}

pub async fn get_sok_collection(wp: Webpage) -> Result<SokCollection, ArchiveError> {
    let mut sok_collection = SokCollection::new(wp.get_id(), wp.get_medium());

    sok_collection.add_sok(wp.get_sok()?);

    let client = Client::default();

    let request = client
        .post(wp.get_url());

    for mut form in wp.get_forms()?.combinations() {
        let title = format_form_to_title(form.clone());
        form.insert("btnSubmit".to_string(), "Vis+tabell".to_string());

        let req = request
                .try_clone().expect("Should not be a stream")
                .form(&form).build()?;

        match client.execute(req).await {
            Ok(response) => {
                if response.status().is_success() {
                    let raw_html = response.text().await?;
            
                    let html = Html::parse_document(&raw_html);
            
                    let sub_wp = Webpage::from_html(346, wp.get_url(), html, wp.get_medium());
            
                    let mut sok = sub_wp.get_sok()?;
                    sok.title = title;
                    sok_collection.add_sok(sok);
                } else {
                    println!("Code: {:?}", response.status());
                }
            }
            Err(err) => println!("Error: {:?}", err),
        }

    }

    sok_collection.title = wp.get_title()?;
    let _ = wp.get_text()?
        .into_iter()
        .map(|e| sok_collection.add_text(e))
        .collect::<Vec<_>>();

    for metode in get_metode(&wp).await? {
        sok_collection.add_metode(metode.into());
    }

    for kilde in get_kilde(&wp).await? {
        sok_collection.add_kilde(kilde.into());
    }

    sok_collection.add_merknad(Merknad { title: "Merknad".to_string(), content: wp.get_merknad()? });




    Ok(sok_collection)
}