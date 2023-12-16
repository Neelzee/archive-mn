use scraper::{Html, Selector};

use crate::{modules::sok::{Sok, Table}, error::ArchiveError, utils::funcs::{trim_string, has_ancestor}};

pub fn parse_sok(html: Html) -> Result<Sok, ArchiveError> {
    let mut sok = Sok::new();

    let title_selector = Selector::parse(r#"div[id="sokResult"] h4"#)?;

    let table_selector = Selector::parse(r#"div[id="sokResult"] table"#)?;

    let tr_selector = Selector::parse(r#"div[id="sokResult"] tr"#)?;
    let th_selector = Selector::parse(r#"div[id="sokResult"] th"#)?;
    let td_selector = Selector::parse(r#"div[id="sokResult"] td"#)?;

    for t in html.select(&title_selector) {
        sok.title = trim_string(&t.text().collect::<String>());
        break;
    }

    let mut tables: Vec<Table> = Vec::new();

    for table in html.select(&table_selector) {
        let mut cur_table = Table::new();

        // Header
        // Rows
        let mut headers: Vec<Vec<String>> = Vec::new();
        for tr in html.select(&tr_selector) {
            // This row belongs to the current table
            if has_ancestor(*tr, table.id()) {
                // Iterating over all cells
                let mut row: Vec<String> = Vec::new();
                for td in html.select(&th_selector) {
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
        for tr in html.select(&tr_selector) {
            // This row belongs to the current table
            if has_ancestor(*tr, table.id()) {
                // Iterating over all cells
                let mut row: Vec<String> = Vec::new();
                for td in html.select(&td_selector) {
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