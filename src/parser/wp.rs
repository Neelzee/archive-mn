use std::collections::HashMap;

use itertools::{Combinations, Itertools};
use reqwest::Client;
use scraper::{Html, Selector};

use crate::{
    error::ArchiveError,
    modules::{
        form::{Form, FormOption},
        sok::{Kilde, Merknad, Metode, Sok, SokCollection, Table},
        webpage::{Link, Webpage},
    },
    scraper::get_html_content,
    utils::{constants::ROOT_URL, funcs::trim_string},
};

// TODO: Change these from methods to functions
impl Webpage {
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
        let text_selector = Selector::parse(r#"div[id="forklaringTxt"]"#)?;

        Ok(self
            .get_content()
            .select(&text_selector)
            .map(|e| trim_string(&e.text().collect::<String>()))
            .collect::<Vec<String>>())
    }

    pub fn get_forms(&self) -> Result<Form, ArchiveError> {
        let select_selector = Selector::parse("select")?;
        let option_selector = Selector::parse("option")?;

        let mut form = Form::new();

        for select in self.get_content().select(&select_selector) {
            if let Some(option_name) = select.attr("name") {
                let mut options: Vec<(String, String)> = Vec::new();
                for option in self.get_content().select(&option_selector) {
                    if let Some(p) = option.parent()
                        && p.id() == select.id()
                        && let Some(v) = option.attr("value")
                    {
                        options.push((
                            v.to_string(),
                            trim_string(&option.text().collect::<String>()),
                        ));
                    }
                }
                let mut fo = FormOption::new(option_name.to_string(), options);

                // Checks if it can has multiple
                if let Some(v) = select.attr("multiple")
                    && v == "multiple"
                {
                    fo.multiple();
                }

                form.add_options(fo);
            }
        }

        Ok(form)
    }

    pub async fn get_sok(&self) -> Result<Sok, ArchiveError> {
        let mut sok = Sok::new();

        let title_selector = Selector::parse(r#"div[id="sokResult"] h4"#)?;

        let table_selector = Selector::parse(r#"div[id="sokResult"] table"#)?;

        let tr_selector = Selector::parse("tr")?;
        let th_selector = Selector::parse("th")?;
        let td_selector = Selector::parse("td")?;

        for t in self.get_content().select(&title_selector) {
            sok.title = trim_string(&t.text().collect::<String>());
            break;
        }

        let mut tables: Vec<Table> = Vec::new();

        for table in self.get_content().select(&table_selector) {
            let mut cur_table = Table::new();
            // Header
            let mut headers: Vec<Vec<String>> = Vec::new();

            for tr in table.select(&tr_selector) {
                // Iterating over all cells
                let mut row: Vec<String> = Vec::new();
                for th in tr.select(&th_selector) {
                    // This cell belongs to the current row
                    let txt = trim_string(&th.text().collect::<String>());

                    if txt.contains('\u{a0}') {
                        row.push(" ".into());
                        continue;
                    }

                    row.push(txt);
                }

                if row.len() != 0 {
                    headers.push(row);
                }
            }

            cur_table.header = headers;

            // Rows
            let mut rows: Vec<Vec<String>> = Vec::new();
            for tr in table.select(&tr_selector) {
                // This row belongs to the current table
                // Iterating over all cells
                let mut row: Vec<String> = Vec::new();
                for td in tr.select(&td_selector) {
                    // This cell belongs to the current row
                    let txt = trim_string(&td.text().collect::<String>());

                    if txt.contains('\u{a0}') {
                        row.push(" ".into());
                        continue;
                    }

                    row.push(txt);
                }
                if row.len() != 0 {
                    rows.push(row);
                }
            }

            cur_table.rows = rows;

            tables.push(cur_table);
        }

        sok.tables = tables;
        sok.metode = get_metode(self)
            .await?
            .into_iter()
            .map(|e| Into::<Metode>::into(e))
            .collect_vec();

        sok.kilde = get_kilde(self)
            .await?
            .into_iter()
            .map(|e| Into::<Kilde>::into(e))
            .collect_vec();

        sok.merknad = vec![Merknad {
            title: "Merk".to_string(),
            content: self.get_merknad()?,
        }];

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
    let merknad_fragment = Selector::parse(".metode-tekst")?;
    let h3_selector = Selector::parse("h3")?;

    // METODE
    for a in wp
        .get_content()
        .select(&merknad_head_selector)
        .filter_map(|e| e.parent())
    {
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

    // TODO: Fix?
    for l in &links {
        let url = l.create_full().to_string();
        if !l.is_metode() {
            continue;
        }
        let mut title = String::new();
        let content = get_html_content(&Client::default(), url).await?;
        for h in Html::parse_document(&content).select(&h3_selector) {
            title = trim_string(&h.text().collect::<String>());
            break;
        }

        metoder.push((
            title.clone(),
            Html::parse_document(&content)
                .select(&merknad_fragment)
                .into_iter()
                .filter(|e| !e.text().collect::<String>().contains(&title))
                .map(|p| trim_string(&p.text().collect::<String>()))
                .collect::<Vec<String>>(),
        ));
    }

    // TODO: Probably not needed
    if metoder.len() >= 20 {
        return Err(ArchiveError::InvalidMetode {
            link: links,
            id: wp.get_id(),
        });
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

        if url.trim() == ROOT_URL {
            continue;
        }

        let mut title = String::new();
        let content = get_html_content(&Client::default(), url).await?;
        for h in Html::parse_document(&content).select(&h2_selector) {
            title = trim_string(&h.text().collect::<String>());
            break;
        }

        kilder.push((
            title,
            Html::parse_document(&content)
                .select(&div_selector)
                .map(|p| trim_string(&p.text().collect::<String>()))
                .collect::<Vec<String>>(),
        ));
    }

    Ok(kilder)
}

#[deprecated]
pub async fn get_sok_collection(
    wp: Webpage,
) -> Result<(SokCollection, Vec<ArchiveError>), ArchiveError> {
    let mut sok_collection = SokCollection::new(wp.get_id(), wp.get_medium());

    let mut errors: Vec<ArchiveError> = Vec::new();

    let client = Client::default();

    let request = client.post(wp.get_url());

    let mut forms = wp.get_forms()?;

    if forms.is_empty() {
        let mut sok = wp.get_sok().await?;
        sok.header_title = sok.title.clone();
        sok_collection.add_sok(sok);
    } else {
        forms.order();
        for (form, disps) in forms.combinations() {
            let mut form_data: HashMap<String, String> = HashMap::new();
            let mut title = String::new();
            for (k, (v, d)) in form {
                title += &d;
                title += " ";
                form_data.insert(k, v);
            }
            form_data.insert("btnSubmit".to_string(), "Vis+tabell".to_string());

            title = title.split_whitespace().collect::<Vec<&str>>().join(" ");

            let req = request
                .try_clone()
                .expect("Should not be a stream")
                .form(&form_data)
                .build()?;

            match client.execute(req).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let raw_html = response.text().await?;

                        let html = Html::parse_document(&raw_html);

                        let sub_wp = Webpage::from_html(346, wp.get_url(), html, wp.get_medium());

                        match sub_wp.get_sok().await {
                            Ok(mut sok) => {
                                sok.display_names = disps;
                                sok.header_title = title.trim().to_string();
                                sok_collection.add_sok(sok);
                            }
                            Err(err) => {
                                errors.push(err.into());
                            }
                        }
                    } else {
                        errors.push(ArchiveError::ResponseError(response.status().to_string()));
                    }
                }
                // TODO: This happens because some of the requests are invalid (most likley due to incorrect mixing of args)
                Err(err) => {
                    errors.push(err.into());
                }
            }
        }
    }

    sok_collection.title = wp.get_title()?;
    let _ = wp
        .get_text()?
        .into_iter()
        .map(|e| sok_collection.add_text(e))
        .collect::<Vec<_>>();

    for metode in get_metode(&wp).await? {
        sok_collection.add_metode(metode.into());
    }

    for kilde in get_kilde(&wp).await? {
        sok_collection.add_kilde(kilde.into());
    }

    sok_collection.add_merknad(Merknad {
        title: "Merknad".to_string(),
        content: wp.get_merknad()?,
    });

    Ok((sok_collection, errors))
}

/// Creates on req
pub async fn get_sok_collection_tmf(
    wp: Webpage,
) -> Result<(SokCollection, Vec<ArchiveError>), ArchiveError> {
    let mut sok_collection = SokCollection::new(wp.get_id(), wp.get_medium());

    let mut errors: Vec<ArchiveError> = Vec::new();

    let client = Client::default();

    let request = client.post(wp.get_url());

    let forms = wp.get_forms()?;

    // No need to send requests if form is empty, but we have to get the first sok.
    if forms.is_empty() {
        let mut sok = wp.get_sok().await?;
        sok.header_title = sok.title.clone();
        sok_collection.add_sok(sok);
    } else {
        let mut form_data: HashMap<String, String> = HashMap::new();
        let mut new_fo = Form::new();

        for fo in forms.options() {
            if fo.get_multiple() {
                form_data.insert(
                    fo.option_name(),
                    fo.options()
                        .into_iter()
                        .map(|(e, _)| e.trim().to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                );
            } else {
                new_fo.add_options(fo);
            }
        }

        println!(
            "New Form Combo: {} singles, {} multiple",
            new_fo.clone().combinations().count(),
            form_data.len()
        );

        new_fo.order();

        let mut combinations = new_fo.combinations().collect::<Vec<_>>();

        if combinations.is_empty() {
            combinations.push((
                forms.form_data(),
                vec![forms.options().pop().unwrap().option_name()],
            ));
        }

        for (form, disps) in combinations {
            let mut title = String::new();
            for (k, (v, d)) in form {
                title += &d;
                title += " ";
                form_data.insert(k, v);
            }
            form_data.insert("btnSubmit".to_string(), "Vis+tabell".to_string());

            title = title.split_whitespace().collect::<Vec<&str>>().join(" ");

            let req = request
                .try_clone()
                .expect("Should not be a stream")
                .form(&form_data)
                .build()?;

            match client.execute(req).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let raw_html = response.text().await?;

                        let html = Html::parse_document(&raw_html);

                        let sub_wp =
                            Webpage::from_html(wp.get_id(), wp.get_url(), html, wp.get_medium());

                        match sub_wp.get_sok().await {
                            Ok(mut sok) => {
                                sok.display_names = disps;
                                sok.header_title = title.trim().to_string();
                                sok_collection.add_sok(sok);
                            }
                            Err(err) => {
                                errors.push(err.into());
                            }
                        }
                    } else {
                        errors.push(ArchiveError::ResponseError(response.status().to_string()));
                    }
                }
                // TODO: This happens because some of the requests are invalid (most likley due to incorrect mixing of args)
                Err(err) => {
                    errors.push(err.into());
                }
            }
        }
    }

    sok_collection.title = wp.get_title()?;
    let _ = wp
        .get_text()?
        .into_iter()
        .map(|e| sok_collection.add_text(e))
        .collect::<Vec<_>>();

    for metode in get_metode(&wp).await? {
        sok_collection.add_metode(metode.into());
    }

    for kilde in get_kilde(&wp).await? {
        sok_collection.add_kilde(kilde.into());
    }

    sok_collection.add_merknad(Merknad {
        title: "Merk".to_string(),
        content: wp.get_merknad()?,
    });

    Ok((sok_collection, errors))
}

pub async fn get_sok_collection_form(
    wp: Webpage,
    forms: Form,
) -> Result<(SokCollection, Vec<ArchiveError>), ArchiveError> {
    let mut sok_collection = SokCollection::new(wp.get_id(), wp.get_medium());

    let mut errors: Vec<ArchiveError> = Vec::new();

    let client = Client::default();

    let request = client.post(wp.get_url());

    if forms.is_empty() {
        let mut sok = wp.get_sok().await?;
        sok.header_title = sok.title.clone();
        sok_collection.add_sok(sok);
    } else {
        let mut form_data: HashMap<String, String> = HashMap::new();
        let mut new_fo = Form::new();

        for fo in forms.options() {
            if fo.get_multiple() {
                form_data.insert(
                    fo.option_name(),
                    fo.options()
                        .into_iter()
                        .map(|(e, _)| e.trim().to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                );
            } else {
                new_fo.add_options(fo);
            }
        }

        println!(
            "New Form Combo: {} singles, {} multiple",
            new_fo.clone().combinations().count(),
            form_data.len()
        );

        new_fo.order();

        for (form, disps) in new_fo.combinations() {
            let mut title = String::new();
            for (k, (v, d)) in form {
                title += &d;
                title += " ";
                form_data.insert(k, v);
            }
            form_data.insert("btnSubmit".to_string(), "Vis+tabell".to_string());

            title = title.split_whitespace().collect::<Vec<&str>>().join(" ");

            let req = request
                .try_clone()
                .expect("Should not be a stream")
                .form(&form_data)
                .build()?;

            match client.execute(req).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let raw_html = response.text().await?;

                        let html = Html::parse_document(&raw_html);

                        let sub_wp =
                            Webpage::from_html(wp.get_id(), wp.get_url(), html, wp.get_medium());

                        match sub_wp.get_sok().await {
                            Ok(mut sok) => {
                                sok.display_names = disps;
                                sok.header_title = title.trim().to_string();
                                sok_collection.add_sok(sok);
                            }
                            Err(err) => {
                                errors.push(err.into());
                            }
                        }
                    } else {
                        errors.push(ArchiveError::ResponseError(response.status().to_string()));
                    }
                }
                // TODO: This happens because some of the requests are invalid (most likley due to incorrect mixing of args)
                Err(err) => {
                    errors.push(err.into());
                }
            }
        }
    }

    sok_collection.title = wp.get_title()?;
    let _ = wp
        .get_text()?
        .into_iter()
        .map(|e| sok_collection.add_text(e))
        .collect::<Vec<_>>();

    for metode in get_metode(&wp).await? {
        sok_collection.add_metode(metode.into());
    }

    for kilde in get_kilde(&wp).await? {
        sok_collection.add_kilde(kilde.into());
    }

    sok_collection.add_merknad(Merknad {
        title: "Merk".to_string(),
        content: wp.get_merknad()?,
    });

    Ok((sok_collection, errors))
}
