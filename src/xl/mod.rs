use std::cmp::min;

use crate::error::ArchiveError;
use crate::modules::sok::SokCollection;
use crate::utils::capitalize_first;

use super::modules::sok::Sok;
use rust_xlsxwriter::{Format, FormatAlign};
use rust_xlsxwriter::Workbook;

pub const MAX_STR_LEN: usize = 150;

pub fn save_sok(soks: SokCollection, path: &str) -> Result<(), ArchiveError> {
    let mut wb = Workbook::new();
    let wb_path: String;
    if path.len() != 0 {
        wb_path = format!("{}\\{}.xlsx", path.to_string(), soks.title.clone());
    } else {
        wb_path = format!("{}\\{}.xlsx", soks.medium.clone(), soks.title.clone());
    }

    let bold = Format::new().set_bold();
    let wrap_text = Format::new()
        .set_text_wrap()
        .set_align(FormatAlign::Top)
        .set_align(FormatAlign::Left);


    for sub_sok in soks.sok {
        let sheet = wb.add_worksheet();
        let mut r = 0;
        
        // Title
        sheet.write_with_format(r, 0, &format!("Tittel: {}", sub_sok.title), &bold)?;
        r += 1;
        
        // Content
        for line in soks.text.clone() {
            for l in split_string(line) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }
        

        sheet.set_column_width_pixels(0, 120)?;
        let full_name = sub_sok.title.clone();

        if let Some(l) = full_name.split_terminator(",").last() {
            let partial_name = l.trim();
            sheet.set_name(capitalize_first(partial_name))?;
        } else {
            let (partial_name, _) = full_name.split_at(min(31, full_name.len()));
            sheet.set_name(partial_name)?;
        }


        // Tables
        for t in sub_sok.tables {
            r += 1;
            for row in t.rows {
                let mut c = 0;
                for cell in row {
                    sheet.write_with_format(r, c, cell, &wrap_text)?;
                    c += 1;
                }
                r += 1;
            }
        }


        // Merknad
        r += 1;
        sheet.write_with_format(r, 0, "Merknad", &bold)?;
        r += 1;
        for s in soks.merknad.clone() {
            for l in split_string(s) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }

        // Metode
        r += 1;
        sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for s in soks.metode.clone() {
            for l in split_string(s) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }

        // Kilde
        r += 1;
        sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for s in soks.kilde.clone() {
            for l in split_string(s) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }
    }

    
    wb.save(wb_path)?;

    Ok(())    
}

pub fn split_string(input: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let psl = 20; // Punctuation Search Limit

    let mut cur_line = String::new();
    for w in input.split_whitespace() {
        if cur_line.len() + w.len() + 1 <= MAX_STR_LEN { // +1 for the space
            if !cur_line.is_empty() {
                cur_line.push(' ');
            }
            cur_line.push_str(w);

            // Check for breakpoint
            if cur_line.len() >= MAX_STR_LEN - psl && (w.contains(',') || w.contains('.')) {
                result.push(cur_line);
                cur_line = String::new();
            }
        } else {
            if !cur_line.is_empty() {
                result.push(cur_line);
            }
            cur_line = w.to_string();
        }
    }

    if !cur_line.is_empty() {
        result.push(cur_line);
    }

    result
}
