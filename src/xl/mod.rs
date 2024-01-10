use std::cmp::min;

use crate::error::ArchiveError;
use crate::modules::sok::{Kilde, Merknad, Metode, SokCollection};
use crate::utils::funcs::{capitalize_first, validify_excel_string};

use once_cell::sync::Lazy;
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Url, Worksheet};
use rust_xlsxwriter::{Workbook, XlsxError};

pub const MAX_STR_LEN: usize = 150;
const BOLD: Lazy<Format> = Lazy::new(|| Format::new().set_bold());

pub fn save_sok(soks: &SokCollection, path: &str) -> Result<Vec<ArchiveError>, ArchiveError> {
    let mut sheets: Vec<(String, String)> = Vec::new();
    let mut wb = Workbook::new();
    let wb_path: String;
    let mut errors: Vec<ArchiveError> = Vec::new();
    let id = soks.id.clone();
    let title = validify_excel_string(&soks.title.clone());
    wb_path = format!("{}\\{}.xlsx", path.to_string(), title);

    {
        let sheet = wb.add_worksheet();
        sheet.set_name("Framside")?;
        sheet.write_with_format(0, 0, "Innhald", &BOLD)?;
    }

    // Table row format
    let row_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left);

    let header_format = Format::new()
        .set_border(FormatBorder::Thick)
        .set_bold()
        .set_align(FormatAlign::Left);

    let mut i = 0;
    for sub_sok in soks.sok.clone() {
        let mut sheet = Worksheet::new();
        let mut r = 0;

        // Title
        sheet.write_with_format(r, 0, &sub_sok.title, &BOLD)?;
        r += 1;

        // Content
        for line in soks.text.clone() {
            for l in split_string(line) {
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }

        let full_name = sub_sok.header_title.clone().trim().to_string();
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

        // This is garbage code
        let mut sheet_name = capitalize_first(&validify_excel_string(&name));

        if sheets.clone().into_iter().any(|(_, dp)| dp == sheet_name) {
            errors.push(ArchiveError::XlSheetError(
                format!(
                    "Skipping: {}, {}. '{}' already a sheetname",
                    sub_sok.title, sub_sok.header_title, &sheet_name
                ),
                id.clone().to_string(),
            ));
            continue;
        }
        if !&sheet_name.is_empty() {
            if wb
                .worksheets()
                .into_iter()
                .map(|e| e.name())
                .collect::<Vec<String>>()
                .contains(&sheet_name)
            {
                errors.push(ArchiveError::XlSheetError(
                    format!(
                        "Error: {}, {}. '{}' already a sheetname",
                        sub_sok.title, sub_sok.header_title, &sheet_name
                    ),
                    id.clone().to_string(),
                ));
                sheet_name = format!("Sheet{}", i);
                i += 1;
            }
            sheets.push((sheet_name.clone(), full_name));
        } else {
            let sheet_name = format!("Sheet{}", i);
            sheets.push((sheet_name.clone(), full_name));
            i += 1;
        }

        sheet.set_name(sheet_name)?;

        // Title
        sheet.write_with_format(r, 0, &sub_sok.title, &BOLD)?;
        sheet.set_column_width_pixels(0, 180)?;
        r += 1;
        // Tables
        for t in sub_sok.tables {
            r += 1;
            // Header
            for row in t.header {
                let mut c = 0;
                for cell in row {
                    // Try to parse as int, header is most likley some year
                    match cell.parse::<i32>() {
                        Ok(i) => {
                            sheet.write_number_with_format(r, c, i, &header_format)?;
                        }
                        Err(_) => {
                            // Lets try again with trim
                            let s = cell.clone();
                            let res = s.split_whitespace().collect::<Vec<&str>>().join("");
                            match res.parse::<i32>() {
                                Ok(i) => {
                                    sheet.write_number_with_format(r, c, i, &header_format)?;
                                }
                                Err(_) => {
                                    sheet.write_with_format(r, c, cell, &header_format)?;
                                }
                            }
                        }
                    }
                    c += 1;
                }
                r += 1;
            }
            // Data
            for row in t.rows {
                let mut c = 0;
                for cell in row {
                    // Try to parse as float
                    match cell.parse::<f32>() {
                        Ok(i) => {
                            sheet.write_number_with_format(r, c, i, &row_format)?;
                        }
                        Err(_) => {
                            // Lets try again with trim
                            let s = cell.clone();
                            let res = s.split_whitespace().collect::<Vec<&str>>().join("");
                            match res.parse::<f32>() {
                                Ok(i) => {
                                    sheet.write_number_with_format(r, c, i, &row_format)?;
                                }
                                Err(_) => {
                                    sheet.write_with_format(r, c, cell, &row_format)?;
                                }
                            }
                        }
                    }
                    c += 1;
                }
                r += 1;
            }
        }
        r += 1;
        let (sheet, _) = write_metode(
            sheet,
            soks.metode.clone(),
            soks.kilde.clone(),
            soks.merknad.clone(),
            r,
        )?;

        wb.push_worksheet(sheet);
    }

    // Info sheet
    {
        let mut info_sheet = Worksheet::new();
        let sheet_name = String::from("Informasjon");
        info_sheet.set_name(&sheet_name)?;
        sheets.push((sheet_name.clone(), sheet_name));

        let (info_sheet, _) = write_metode(
            info_sheet,
            soks.metode.clone(),
            soks.kilde.clone(),
            soks.merknad.clone(),
            0,
        )?;

        wb.push_worksheet(info_sheet);
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

            sheet.write_with_format(0, 0, soks.title.clone(), &BOLD)?;

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
        Ok(_) => Ok(errors),
        Err(err) => {
            let p = format!("arkiv\\{}.xlsx", soks.id);
            match wb.save(p.clone()) {
                Ok(_) => {
                    errors.push(ArchiveError::XlSaveError(err.to_string(), wb_path));
                    Ok(errors)
                }
                Err(e) => Err(ArchiveError::XlSaveError(e.to_string(), p)),
            }
        }
    }
}

pub fn write_metode(
    mut sheet: Worksheet,
    metoder: Vec<Metode>,
    kilder: Vec<Kilde>,
    merknader: Vec<Merknad>,
    mut r: u32,
) -> Result<(Worksheet, u32), XlsxError> {
    // Merknad
    sheet.write_with_format(r, 0, "Merknad", &BOLD)?;
    r += 1;
    for merknad in merknader {
        for long_line in merknad.content {
            if long_line.trim().is_empty() {
                continue;
            }
            for l in split_string(long_line) {
                if l.trim().is_empty() {
                    continue;
                }
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }
    }

    // Kilde
    sheet.write_with_format(r, 0, "Kilde", &BOLD)?;
    r += 1;
    for kilde in kilder {
        sheet.write_with_format(r, 0, kilde.title, &BOLD)?;
        r += 1;
        for long_line in kilde.content {
            if long_line.trim().is_empty() {
                continue;
            }
            for l in split_string(long_line) {
                if l.trim().is_empty() {
                    continue;
                }
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }
        r += 1;
    }

    // Metode
    for metode in metoder {
        sheet.write_with_format(r, 0, metode.title, &BOLD)?;
        r += 1;
        for long_line in metode.content {
            if long_line.trim().is_empty() {
                continue;
            }
            for l in split_string(long_line) {
                if l.trim().is_empty() {
                    continue;
                }
                sheet.write(r, 0, l)?;
                r += 1;
            }
            r += 1;
        }
        r += 1;
    }

    Ok((sheet, r))
}

pub fn split_string(input: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let psl = 20; // Punctuation Search Limit

    let mut cur_line = String::new();
    for w in input.split_whitespace() {
        if cur_line.len() + w.len() + 1 <= MAX_STR_LEN {
            // +1 for the space
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
