use scraper::{Selector, error::SelectorErrorKind};

use crate::modules::{webpage::Webpage, sok::{Sok, Table}};


pub fn get_sok(sok_page: Webpage) -> Result<Sok, ()> {
    let mut sok: Sok;

    let mut id: usize = 0;
    let mut medium = String::new();

    let url = sok_page.get_url();

    let split = url.split("/").collect::<Vec<&str>>();

    if let Some(id_str) =  split.last() {
        match id_str.parse::<usize>() {
            Ok(id_) => {
                id = id_;
            }
            Err(_e) => {
                // TODO: Add some error handling
                return Err(());
            },
        }
    }

    if let Some(_medium) = split.get(split.len() - 2) {
        medium = _medium.to_owned().to_owned();
    }

    sok = Sok::new(id, medium);

    // TODO: Add good error handling
    let table = get_table(&sok_page);

    Ok(sok)
}

pub fn get_table(sok_page: &Webpage) -> Result<Vec<Table>, SelectorErrorKind> {

    let mut tables = Vec::new();

    let div_sok_selector = Selector::parse("div[id=sokResult]")?;

    let header_selector = Selector::parse("h4")?;

    let table_selector = Selector::parse("table")?;

    let table_header_selector = Selector::parse("th")?;

    let table_row_selector = Selector::parse("tr")?;

    // Should just be one div with that id, if there are multiple, we dont really care.
    let content = sok_page.get_content();
    
    let div_sok = content.select(&div_sok_selector).next().unwrap();



    // Getting title
    // Should just be one h4, inside this div, if not we dont care.
    let name = div_sok
        .select(&header_selector).next().unwrap()
        .text().collect::<String>()
        .trim().to_owned();

    // Getting rows
    for t in div_sok.select(&table_selector) {
        let mut table = Table::new();
        table.name = name.clone();
        // TODO: This is wrong, as the header-row, is a tr, with th-cells
        table.header.push(
            t.select(&table_header_selector)
                .map(|th| {
                    th.text()
                    .map(|u| u.trim().to_owned())
                    .collect::<String>()
                })
                .collect::<Vec<String>>()
            );

        table.rows = t.select(&table_row_selector)
            .map(|tr| {
                tr.text()
                .map(|u| u.trim().to_owned())
                .filter(|u| u.len() != 0)
                .collect::<Vec<String>>()
            })
            .filter(|v| v.len() != 0)
            .collect::<Vec<Vec<String>>>();
        
        tables.push(table);
    }

    Ok(tables)
}