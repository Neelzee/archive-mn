use std::cmp::min;

use crate::error::ArchiveError;
use crate::modules::sok::{SokCollection, Merknad};
use crate::utils::funcs::capitalize_first;

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
    let number_format = Format::new()
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
        let full_name = sub_sok.header_title.clone();
        let name: String;

        if let Some(l) = full_name.split_terminator(",").last() {
            let partial_name = l.trim();
            let (n, _) = partial_name.split_at(min(31, partial_name.len()));
            name = n.to_owned();
        } else {
            let (partial_name, _) = full_name.split_at(min(31, full_name.len()));
            name = partial_name.trim().to_owned();
        }
        sheet.set_name(capitalize_first(&name))?;


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
                            sheet.write_number_with_format(r, c, i, &number_format)?;
                        },
                        Err(_) => {
                            sheet.write_with_format(r, c, cell, &number_format)?;
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
                            sheet.write_number_with_format(r, c, i, &number_format)?;
                        },
                        Err(_) => {
                            sheet.write_with_format(r, c, cell, &number_format)?;
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
        r += 1;
        sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for metode in soks.metode.clone() {
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
        r += 1;
        sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for kilde in soks.kilde.clone() {
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

    // Info
    {
        let info_sheet = wb.add_worksheet();
        info_sheet.set_name("Informasjon")?;

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
        r += 1;
        info_sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for metode in soks.metode.clone() {
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
        r += 1;
        info_sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for kilde in soks.kilde.clone() {
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

    
    match wb.save(wb_path.clone()) {
        Ok(_) => Ok(()),
        Err(err) => {
            Err(ArchiveError::XlSaveError(err, wb_path))
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
