use std::fmt::format;

use super::parser::sok::Sok;
use rust_xlsxwriter::Format;
use rust_xlsxwriter::{Workbook, XlsxError};


impl Sok {
    pub fn save(self) -> Result<(), XlsxError> {

        let mut wb = Workbook::new();

        
        let bold = Format::new().set_bold();
        let wrap_text = Format::new().set_text_wrap();
        
        { // Front sheet

            let front_sheet = wb.add_worksheet();
            front_sheet.set_name("Innhold")?;
            front_sheet.write_with_format(0, 0, &format!("Søk: {}", self.id), &bold)?;
            front_sheet.write_with_format(1, 0, &format!("Tittel: {}", self.title), &bold)?;

            // Text
            let mut i = 2;
            for line in self.text {
                i += 1;
                front_sheet.write_with_format(i, 0, &line, &wrap_text)?;

            }
        }

        // Tables
        for t in self.tables {
            let sheet = wb.add_worksheet();
            sheet.set_name(&t.name)?;
            for r in 0..t.rows.len() {
                //for c in 0..t.columns.len() {
                    //sheet.write(r as u32, c as u16, 0)?;
                //}
            }
        }


        wb.save(format!("{}/{}", self.medium, self.id))
    }
}