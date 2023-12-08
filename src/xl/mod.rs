use std::cmp::min;
use std::fmt::format;

use super::parser::sok::Sok;
use rust_xlsxwriter::{Format, FormatAlign};
use rust_xlsxwriter::{Workbook, XlsxError};


impl Sok {
    /// Path must include `/` or `\` as a affix.
    pub fn save(&self, path: &str) -> Result<(), XlsxError> {

        let mut wb = Workbook::new();

        
        let bold = Format::new().set_bold();
        let wrap_text = Format::new()
            .set_text_wrap()
            .set_align(FormatAlign::Top)
            .set_align(FormatAlign::Left);

        { // Front sheet

            let front_sheet = wb.add_worksheet();
            front_sheet.set_column_width_pixels(0, 1000)?;
            front_sheet.set_name("Innhold")?;
            front_sheet.write_with_format(0, 0, &format!("Søk: {}", self.id), &bold)?;
            front_sheet.write_with_format(1, 0, &format!("Tittel: {}", self.title), &bold)?;

            // Text
            let mut r = 2;
            for line in self.text.clone() {
                r += 1;
                front_sheet.write_with_format(r, 0, &line, &wrap_text)?;
                front_sheet.set_row_height_pixels(r, 70)?;
            }

            // Merknad
            r += 1;
            front_sheet.write_with_format(r, 0, "Merknad", &bold)?;
            r += 1;
            for s in self.merknad.clone() {
                front_sheet.write_with_format(r, 0, s, &wrap_text)?;
                front_sheet.set_row_height_pixels(r, 70)?;
                r += 1;
            }

            // Metode
            r += 1;
            front_sheet.write_with_format(r, 0, "Metode", &bold)?;
            r += 1;
            for s in self.metode.clone() {
                front_sheet.write_with_format(r, 0, s, &wrap_text)?;
                front_sheet.set_row_height_pixels(r, 70)?;
                r += 1;
            }

            // Kilde
            r += 1;
            front_sheet.write_with_format(r, 0, "Kilde", &bold)?;
            r += 1;
            for s in self.kilde.clone() {
                front_sheet.write_with_format(r, 0, s, &wrap_text)?;
                front_sheet.set_row_height_pixels(r, 70)?;
                r += 1;
            }
        }

        // Tables
        for t in self.tables.clone() {
            let sheet = wb.add_worksheet();
            sheet.set_column_width_pixels(0, 120)?;
            //sheet.set_name(&t.name)?;
            let mut r = 0;
            for row in t.rows {
                let mut c = 0;
                for cell in row {
                    sheet.write_with_format(r, c, cell, &wrap_text)?;
                    c += 1;
                }
                r += 1;
            }
        }

        if path.len() != 0 {
            wb.save(path)?;
        } else {
            wb.save(format!("{}\\sok_{}.xlsx", self.medium, self.id))?;
        }

        Ok(())

    }
}


pub fn save_sok(soks: Vec<Sok>, path: &str) -> Result<(), XlsxError> {
    let mut wb = Workbook::new();
    let wb_path: String;
    if path.len() != 0 {
        wb_path = path.to_string();
    } else {
        wb_path = format!("{}\\sok_{}.xlsx", soks.get(0).unwrap().medium.clone(), soks.get(0).unwrap().id.clone());
    }

    let bold = Format::new().set_bold();
        let wrap_text = Format::new()
            .set_text_wrap()
            .set_align(FormatAlign::Top)
            .set_align(FormatAlign::Left);

    { // Front sheet
        let front_sheet = wb.add_worksheet();
        front_sheet.set_column_width_pixels(0, 1000)?;
        front_sheet.set_name("Innhold")?;
        front_sheet.write_with_format(0, 0, &format!("Søk: {}", soks.get(0).unwrap().id), &bold)?;
        front_sheet.write_with_format(1, 0, &format!("Tittel: {}", soks.get(0).unwrap().title), &bold)?;

        // Text
        let mut r = 2;
        for line in soks.get(0).unwrap().text.clone() {
            r += 1;
            front_sheet.write_with_format(r, 0, &line, &wrap_text)?;
            front_sheet.set_row_height_pixels(r, 70)?;
        }

        // Merknad
        r += 1;
        front_sheet.write_with_format(r, 0, "Merknad", &bold)?;
        r += 1;
        for s in soks.get(0).unwrap().merknad.clone() {
            front_sheet.write_with_format(r, 0, s, &wrap_text)?;
            front_sheet.set_row_height_pixels(r, 70)?;
            r += 1;
        }

        // Metode
        r += 1;
        front_sheet.write_with_format(r, 0, "Metode", &bold)?;
        r += 1;
        for s in soks.get(0).unwrap().metode.clone() {
            front_sheet.write_with_format(r, 0, s, &wrap_text)?;
            front_sheet.set_row_height_pixels(r, 70)?;
            r += 1;
        }

        // Kilde
        r += 1;
        front_sheet.write_with_format(r, 0, "Kilde", &bold)?;
        r += 1;
        for s in soks.get(0).unwrap().kilde.clone() {
            front_sheet.write_with_format(r, 0, s, &wrap_text)?;
            front_sheet.set_row_height_pixels(r, 70)?;
            r += 1;
        }
    }

    for sub_sok in soks {
        let sheet = wb.add_worksheet();
        sheet.set_column_width_pixels(0, 120)?;
        let full_name = sub_sok.tables.get(0).unwrap().name.clone();
        let (partial_name, _) = full_name.split_at(min(31, full_name.len()));
        sheet.set_name(partial_name)?;

        // Tables
        let mut r = 0;
        for t in sub_sok.tables {
            if r != 0 {
                r += 1;
            }
            for row in t.rows {
                let mut c = 0;
                for cell in row {
                    sheet.write_with_format(r, c, cell, &wrap_text)?;
                    c += 1;
                }
                r += 1;
            }
        }
    }

    
    wb.save(wb_path)    
}