use std::cmp::min;

use crate::error::ArchiveError;
use crate::modules::sok::{SokCollection, Merknad};
use crate::utils::funcs::{capitalize_first, validify_excel_string};

use rust_xlsxwriter::{Format, FormatAlign, Url, FormatBorder};
use rust_xlsxwriter::Workbook;

pub const MAX_STR_LEN: usize = 150;

pub fn save_sok(soks: SokCollection, path: &str) -> Result<(), ArchiveError> {
    let mut sheets: Vec<(String, String)> = Vec::new();
    let mut wb = Workbook::new();
    let wb_path: String;
    if path.len() != 0 {
        wb_path = format!("{}\\{}.xlsx", path.to_string(), soks.title.clone());
    } else {
        wb_path = format!("{}\\{}.xlsx", soks.medium.clone(), soks.title.clone());
    }

    let bold = Format::new().set_bold();

    {
        let sheet = wb.add_worksheet();
        sheet.set_name("Framside")?;
        sheet.write_with_format(0, 0, "Innhald", &bold)?;
    }

    // Table row format
    let row_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left);

    let header_format = Format::new()
        .set_border(FormatBorder::Thick)
        .set_bold()
        .set_align(FormatAlign::Left);

    for sub_sok in soks.sok {
        let sheet = wb.add_worksheet();
        let mut r = 0;
        
        // Title
        sheet.write_with_format(r, 0, &sub_sok.title, &bold)?;
        r += 1;
        
        // Content
        for line in soks.text.clone() {
            for l in split_string(line) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }

        let full_name = sub_sok.header_title.clone();
        let name: String;

        if let Some(l) = full_name.split_terminator(",").last() {
            let partial_name = l.trim();
            let mut n = String::new();
            let split_point = min(31, partial_name.chars().count());
            for c in partial_name.chars() {
                if n.chars().count() + c.len_utf16() <= split_point {
                    n.push(c);
                }
            } 
            name = n;
        } else {
            let mut n = String::new();
            let split_point = min(31, full_name.chars().count());
            for c in full_name.chars() {
                if n.chars().count() + c.len_utf16() <= split_point {
                    n.push(c);
                }
            }
            name = n.trim().to_owned();
        }

        let sheet_name = capitalize_first(&validify_excel_string(&name));
        
        if sheets.clone().into_iter().any(|(_, dp)| dp == sheet_name) {
            println!(
                "Skipping: {}, {}. '{}' already a sheetname",
                sub_sok.title,
                sub_sok.header_title,
                &sheet_name);
            continue;
        }

        sheet.set_name(&sheet_name)?;
        sheets.push((sheet_name, full_name));

        // Title
        sheet.write_with_format(r, 0, &sub_sok.title, &bold)?;
        sheet.set_column_width_pixels(0, 180)?;
        r += 1;
        // Tables
        for t in sub_sok.tables {

            r += 1;
            // Header
            for row in t.header {
                let mut c = 0;
                for cell in row {
                    // Try to parse as int
                    match cell.parse::<i32>() {
                        Ok(i) => {
                            sheet.write_number_with_format(r, c, i, &header_format)?;
                        },
                        Err(_) => {
                            sheet.write_with_format(r, c, cell, &header_format)?;
                        },
                    }
                    c += 1;
                }
                r += 1;
            }
            // Data
            for row in t.rows {
                let mut c = 0;
                for cell in row {
                    // Try to parse as int
                    match cell.parse::<i32>() {
                        Ok(i) => {
                            sheet.write_number_with_format(r, c, i, &row_format)?;
                        },
                        Err(_) => {
                            sheet.write_with_format(r, c, cell, &row_format)?;
                        },
                    }
                    
                    c += 1;
                }
                r += 1;
            }
        }

        // Merknad
        r += 1;
        sheet.write_with_format(r, 0, "Merknad", &bold)?;
        r += 1;
        for merknad in soks.merknad.clone() {
            for long_line in merknad.content {
                for l in split_string(long_line) {
                    sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }

        // Metode
        sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for metode in soks.metode.clone() {
            sheet.write_with_format(r, 0, metode.title, &bold)?;
            r += 1;
            for long_line in metode.content {
                for l in split_string(long_line) {
                    sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }

        // Kilde
        sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for kilde in soks.kilde.clone() {
            sheet.write_with_format(r, 0, kilde.title, &bold)?;
            r += 1;
            for long_line in kilde.content {
                for l in split_string(long_line) {
                    sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }
    }

    // TODO: Rewrite this, its duplicate code, dipshit
    // Info
    {
        let info_sheet = wb.add_worksheet();
        let sheet_name = String::from("Informasjon");
        info_sheet.set_name(&sheet_name)?;
        sheets.push((sheet_name.clone(), sheet_name));
        // Merknad
        let mut r = 0;
        info_sheet.write_with_format(r, 0, "Merknad", &bold)?;
        r += 1;
        for merknad in soks.merknad.clone() {
            for long_line in merknad.content {
                for l in split_string(long_line) {
                    info_sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }

        // Metode
        info_sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for metode in soks.metode.clone() {
            info_sheet.write_with_format(r, 0, metode.title, &bold)?;
            r += 1;
            for long_line in metode.content {
                for l in split_string(long_line) {
                    info_sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }

        // Kilde
        info_sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for kilde in soks.kilde.clone() {
            info_sheet.write_with_format(r, 0, kilde.title, &bold)?;
            r += 1;
            for long_line in kilde.content {
                for l in split_string(long_line) {
                    info_sheet.write(r, 0, l)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }
    }

    // Table of Contents
    {
        let mut temp = Vec::new();
        for (nm, dp) in sheets {
            if wb.worksheet_from_name(&nm).is_ok() {
                temp.push((nm, dp));
            }
        }

        if let Ok(sheet) = wb.worksheet_from_name("Framside") {
            let mut r = 1;

            sheet.write_with_format(0, 0, soks.title, &bold)?;

            for (nm, dp) in temp {
                if dp.contains("Informasjon") {
                    continue;
                }
                let link: &str = &format!("internal:'{}'!A1", nm);
                sheet.write_url_with_text(r, 0, link, format!("Fordelt på: {}", dp))?;
                r += 1;
            }

            let link: &str = &format!("internal:'{}'!A1", "Informasjon");
            sheet.write_url_with_text(r, 0, link, "Informasjon")?;
        }
        

    }
    
    match wb.save(wb_path.clone()) {
        Ok(_) => Ok(()),
        Err(err) => {
            Err(ArchiveError::XlSaveError(err.to_string(), wb_path))
        },
    }
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
