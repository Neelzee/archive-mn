use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};

use crate::{modules::sok::{Sok, Table}, error::ArchiveError, utils::funcs::trim_string};

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

        let stringify = |n: ElementRef| {
            let txt = trim_string(&n.text().collect::<String>());

            if txt.len() == 0 || txt.contains('\u{a0}') {
                return "".to_string();
            }

            return txt;
        };

        // Header
        // Rows
        for tr in html.select(&tr_selector) {
            cur_table.header.push(tr.select(&th_selector).into_iter().map(|n| stringify(n)).collect_vec());
            cur_table.rows.push(tr.select(&td_selector).into_iter().map(|n| stringify(n)).collect_vec());
        }


        tables.push(cur_table);

    }

    sok.tables = tables;

    Ok(sok)
}