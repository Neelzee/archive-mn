use std::cmp::min;
use std::collections::{HashMap, HashSet};

use crate::error::ArchiveError;
use crate::modules::form::{self, Form};
use crate::modules::sok::{IsEmpty, Kilde, Merknad, Metode, Sok, SokCollection, Table};
use crate::utils::funcs::{capitalize_first, validify_excel_string};

use itertools::Itertools;
use once_cell::sync::Lazy;
use rust_xlsxwriter::{Color, DocProperties, Format, FormatAlign, FormatBorder, Worksheet};
use rust_xlsxwriter::{FormatUnderline, Url};
use rust_xlsxwriter::{Workbook, XlsxError};

pub const MAX_STR_LEN: usize = 150;

const MAX_COL_WIDTH: f64 = 50.0;
const DEFAULT_COL_WIDTH: f64 = 8.43;
const MAX_SHEET_NAME: usize = 30;

const BOLD: Lazy<Format> = Lazy::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::Left)
        .set_font_size(FONT_SIZE)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::White)
});

const FONT_SIZE: f64 = 12.0;

const HEADER_FORMAT: Lazy<Format> = Lazy::new(|| {
    Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xA2CAD6))
        .set_font_size(FONT_SIZE)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0xA2CAD6))
});

const ROW_FORMAT_EVEN: Lazy<Format> = Lazy::new(|| {
    Format::new()
        .set_align(FormatAlign::Right)
        .set_num_format("#,##0") // Should give 10 000 formatting
        .set_background_color(Color::RGB(0xcee8f1))
        .set_font_size(FONT_SIZE)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0xcee8f1))
});

const ROW_FORMAT_ODD: Lazy<Format> = Lazy::new(|| {
    Format::new()
        .set_align(FormatAlign::Right)
        .set_num_format("#,##0") // Should give 10 000 formatting
        .set_background_color(Color::RGB(0xe6f3f8))
        .set_font_size(FONT_SIZE)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::RGB(0xe6f3f8))
});

const URL_FORMAT: Lazy<Format> = Lazy::new(|| {
    Format::new()
        .set_font_size(FONT_SIZE)
        .set_font_color(Color::Blue)
        .set_underline(FormatUnderline::Single)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::White)
});

const PLAIN_FORMAT: Lazy<Format> = Lazy::new(|| Format::new().set_font_size(FONT_SIZE).set_border(FormatBorder::Thin).set_border_color(Color::White));

const ROW_HEIGHT: f64 = 17.0;

const COLUMN_FORMAT: Lazy<Format> = Lazy::new(|| Format::new().set_border(FormatBorder::Thin).set_border_color(Color::White));

pub fn save_sok(soks: &SokCollection, path: &str) -> Result<Vec<ArchiveError>, ArchiveError> {
    let mut sheets: Vec<String> = Vec::new();
    let mut wb = Workbook::new();
    
    let wb_path: String;
    let mut errors: Vec<ArchiveError> = Vec::new();
    let id = soks.id.clone();
    wb_path = format!("{}\\{}.xlsx", path.to_string(), validify_excel_string(&soks.title.clone()));

    // Creates TOC Sheet, ensures its infront
    {
        let mut sheet = wb.add_worksheet();

        format_sheet(&mut sheet)?;

        sheet.set_name("Framside")?;
        sheet.write_with_format(0, 0, "Innhold", &BOLD)?;
    }

    let mut i = 0;
    for sub_sok in soks.sok.clone() {
        let mut sheet = Worksheet::new();
        format_sheet(&mut sheet)?;
        let mut r = 0;

        // Title
        sheet.write_with_format(r, 0, &soks.title, &BOLD)?;
        r += 1;

        // Content
        for line in soks.text.clone() {
            for l in split_string(line) {
                sheet.write_with_format(r, 0, l, &PLAIN_FORMAT)?;
                r += 1;
            }
            r += 1;
        }

        // Prefer using display names
        let mut full_name = sub_sok.display_names.clone().join(" ").trim().to_string();

        // But can settle for header_title
        if full_name.is_empty() {
            full_name = sub_sok.header_title.clone();
        }

        // Ensures that full_name is valid.
        full_name = validify_excel_string(&full_name);

        // I hate Excel
        full_name = full_name.to_lowercase();

        // Ensures its of the correct length
        if full_name.len() > MAX_SHEET_NAME {
            // Can't use split, since there are non-ascii-chars
            let mut fl = String::new();
            for c in full_name.clone().chars() {
                if fl.len() == MAX_SHEET_NAME - 1 {
                    break;
                }
                fl.push(c);
            }
            full_name = fl;
        } 

        let mut i = 1;
        // Checks if this sheet name has been used before
        while sheets.contains(&full_name) {
            if full_name.len() >= MAX_SHEET_NAME {
                // Can't use split, since there are non-ascii-chars
                let mut fl = String::new();
                for c in full_name.clone().chars() {
                    if fl.len() <= MAX_SHEET_NAME - 2 {
                        break;
                    }
                    fl.push(c);
                }
                full_name = fl;
            } 

            let mut chrs = full_name.chars().collect::<Vec<_>>();
            chrs.pop();
            full_name = format!("{}{}", chrs.into_iter().fold(String::new(), |mut acc, c| {acc.push(c); acc}), i);
            i += 1;
        }
    
        sheet.set_name(&full_name)?;
        sheets.push(full_name.clone());

        // Table Title
        for title in &sub_sok.titles {
            sheet.write_with_format(r, 0, title, &BOLD)?;
            r += 1;
        }
        // Tables
        let (sheet, mut r) = write_tables(sub_sok.clone(), r, sheet)?;
        r += 1;
        let (mut sheet, r) = write_mkm(sheet, sub_sok.metode, sub_sok.kilde, sub_sok.merknad, r)?;

        if r >= 30 {
            for i in 30..r {
                sheet.set_row_height(i.into(), ROW_HEIGHT);
            }
        }

        wb.push_worksheet(sheet);
    }

    // Info sheet
    if !soks.metode.is_empty() && !soks.metode.clone().into_iter().all(|e| e.is_empty()) {
        let mut info_sheet = Worksheet::new();
        format_sheet(&mut info_sheet)?;
        let sheet_name = String::from("Metode");
        info_sheet.set_name(&sheet_name)?;
        sheets.push(sheet_name);
        let mut r = 0;
        // Metode
        for metode in soks.metode.clone() {
            info_sheet.write_with_format(r, 0, metode.title, &BOLD)?;
            r += 1;
            for long_line in metode.content {
                if long_line.trim().is_empty() || long_line.trim() == "Alle data kan fritt benyttes såfremt både originalkilde og Medienorge oppgis som kilder." {
                continue;
            }
                for l in split_string(long_line) {
                    if l.trim().is_empty() {
                        continue;
                    }
                    info_sheet.write_with_format(r, 0, l, &PLAIN_FORMAT)?;
                    r += 1;
                }
                r += 1;
            }
            r += 1;
        }

        if r > 30 {
            for i in 30..r {
                info_sheet.set_row_height(i, ROW_HEIGHT);
            }
        }

        wb.push_worksheet(info_sheet);
    }

    // Table of Contents
    {
        let mut temp = Vec::new();
        for nm in sheets {
            if wb.worksheet_from_name(&nm).is_ok() {
                temp.push(nm);
            }
        }

        if let Ok(sheet) = wb.worksheet_from_name("Framside") {
            let mut r = 1;
            let mut has_merk = false;
            sheet.write_with_format(0, 0, soks.title.clone(), &BOLD)?;

            for nm in temp {
                if nm.contains("Metode") {
                    has_merk = true;
                    continue;
                }
                let link: &str = &format!("internal:'{}'!A1", &nm);
                sheet.write_with_format(r, 0, Url::new(link).set_text(nm), &URL_FORMAT)?;
                r += 1;
            }

            if has_merk {
                let link: &str = &format!("internal:'{}'!A1", "Metode");
                sheet.write_with_format(r, 0, Url::new(link).set_text("Metode"), &URL_FORMAT)?;
            }
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

/// A header cell should be left aligned if it's column contains only text.
/// A header cell should be right aligned if it's column contains numbers or '-'.
fn header_format(table: &Table) -> Vec<Format> {
    let mut formats: Vec<Format> = Vec::new();

    if table.header.is_empty() {
        return formats;
    }

    if table.rows.is_empty() {
        return formats;
    }

    // From 0 to end of a row
    for col_ind in 0..table.rows.get(0).unwrap().len() {
        // Get the column at the index
        if let Some(col) = table.get_col(col_ind.clone())
            && let Some(cell) = col.get(0)
        {
            if is_num(cell) {
                formats.push(HEADER_FORMAT.clone().set_align(FormatAlign::Right));
            } else {
                formats.push(HEADER_FORMAT.clone().set_align(FormatAlign::Left));
            }
        }
    }

    formats
}

fn is_num(cell: &str) -> bool {
    match cell.parse::<f64>() {
        Ok(_) => true,
        Err(_) => cell.contains("-") && cell.trim().chars().count() == 1,
    }
}

/// Metode, Kilde, Merknad
pub fn write_mkm(
    mut sheet: Worksheet,
    metoder: Vec<Metode>,
    kilder: Vec<Kilde>,
    merknader: Vec<Merknad>,
    mut r: u32,
) -> Result<(Worksheet, u32), XlsxError> {
    // Merknad
    if !merknader.is_empty() && !merknader.clone().into_iter().all(|e| e.is_empty()) {
        sheet.write_with_format(r, 0, "Merk", &BOLD)?;
        r += 1;
    }
    for merknad in merknader {
        for long_line in merknad.content {
            if long_line.trim().is_empty() || long_line.trim() == "Alle data kan fritt benyttes såfremt både originalkilde og Medienorge oppgis som kilder." {
                continue;
            }
            for l in split_string(long_line) {
                if l.trim().is_empty() {
                    continue;
                }
                sheet.write_with_format(r, 0, l, &PLAIN_FORMAT)?;
                r += 1;
            }
            r += 1;
        }
    }

    // Kilde
    if !kilder.is_empty() && !kilder.clone().into_iter().all(|e| e.is_empty()) {
        sheet.write_with_format(r, 0, "Kilde", &BOLD)?;
        r += 1;
    }
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
                sheet.write_with_format(r, 0, l, &PLAIN_FORMAT)?;
                r += 1;
            }
            r += 1;
        }
    }

    sheet.write_with_format(
        r,
        0,
        "Alle data kan fritt benyttes såfremt både originalkilde og Medienorge oppgis som kilder.",
        &PLAIN_FORMAT.clone().set_italic(),
    )?;

    Ok((sheet, r + 1))
}

fn write_tables(
    sok: Sok,
    mut r: u32,
    mut sheet: Worksheet,
) -> Result<(Worksheet, u32), ArchiveError> {
    let mut column_width: HashMap<u16, f64> = HashMap::new();
    for t in sok.tables {
        let mut header_format = header_format(&t);
        let mut prev_form = HEADER_FORMAT.clone();
        r += 1;
        // Header
        for row in t.header {
            let mut c = 0;
            let rl = row.len();
            for cell in row {
                if let Some(width) = column_width.get(&c) {
                    if width.clone() as usize <= cell.len() {
                        column_width.insert(c, (cell.len() as f64) + 3.0);
                    }
                } else {
                    column_width.insert(c, (cell.len() as f64) + 3.0);
                }

                let format: Format;

                if c as usize == rl - 2 {
                    format = HEADER_FORMAT.clone().set_align(FormatAlign::Right);
                } else {
                    format = match header_format.pop() {
                        Some(format) => format,
                        _ => prev_form.clone(),
                    };
                }

                prev_form = format.clone();

                // Try to parse as int, header is most likley some year
                match cell.parse::<f64>() {
                    Ok(i) => {
                        sheet.write_number_with_format(r, c, i, &format)?;
                    }
                    Err(_) => {
                        sheet.write_with_format(r, c, cell, &format)?;
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
                if let Some(width) = column_width.get(&c) {
                    if width.clone() as usize <= cell.len() {
                        column_width.insert(c, cell.len() as f64);
                    }
                } else {
                    column_width.insert(c, cell.len() as f64);
                }
                let mut row_format = ROW_FORMAT_ODD;
                if r == 0 || r % 2 == 0 {
                    row_format = ROW_FORMAT_EVEN;
                }
                // Try to parse as int
                match cell.parse::<i32>() {
                    Ok(i) => {
                        if i > 999 {
                            update_width(&mut column_width, &c, (cell.len() + 2) as f64);
                        } else if i > 999_999 {
                            update_width(&mut column_width, &c, (cell.len() + 3) as f64);
                        }
                        sheet.write_number_with_format(r, c, i, &row_format)?;
                    }
                    Err(_) => {
                        // Trying again, but with trimming, since it could be "- 42"
                        let s = cell.clone();
                        match s.split_whitespace().join("").parse::<i32>() {
                            Ok(i) => {
                                sheet.write_number_with_format(r, c, i, &row_format)?;
                                if i > 999 {
                                    update_width(&mut column_width, &c, (cell.len() + 2) as f64);
                                } else if i > 999_999 {
                                    update_width(&mut column_width, &c, (cell.len() + 3) as f64);
                                }
                            }
                            Err(_) => {
                                // Lets try again with trim, and replace . with ,
                                let s = cell.clone();
                                let res = s
                                    .split_whitespace()
                                    .collect::<Vec<&str>>()
                                    .join("")
                                    .replace(",", ".");
                                match res.parse::<f32>() {
                                    Ok(i) => {
                                        if i > 999.0 {
                                            update_width(
                                                &mut column_width,
                                                &c,
                                                (cell.len() + 2) as f64,
                                            );
                                        } else if i > 999_999.0 {
                                            update_width(
                                                &mut column_width,
                                                &c,
                                                (cell.len() + 3) as f64,
                                            );
                                        }
                                        sheet.write_number_with_format(
                                            r,
                                            c,
                                            i,
                                            &row_format.clone().set_num_format("#,##0.0"),
                                        )?;
                                    }
                                    Err(_) => {
                                        // Could be a `-` char, and if so, its alignment should be right aligned
                                        if cell.clone().trim() == "-" {
                                            sheet.write_with_format(
                                                r,
                                                c,
                                                cell,
                                                &row_format.clone().set_align(FormatAlign::Right),
                                            )?;
                                        } else {
                                            sheet.write_with_format(
                                                r,
                                                c,
                                                cell,
                                                &row_format.clone().set_align(FormatAlign::Left),
                                            )?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c += 1;
            }
            r += 1;
        }
    }
    for (k, v) in column_width {
        if v > MAX_COL_WIDTH {
            sheet.set_column_width(k, MAX_COL_WIDTH)?;
        } else if v > DEFAULT_COL_WIDTH {
            sheet.set_column_width(k, v)?;
        }
    }

    Ok((sheet, r))
}

fn update_width(widths: &mut HashMap<u16, f64>, key: &u16, val: f64) {
    if let Some(ref width) = widths.get(key) {
        if width <= &&val {
            widths.insert(*key, val);
        }
    } else {
        widths.insert(*key, val);
    }
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

fn format_sheet(sheet: &mut Worksheet) -> Result<(), ArchiveError> {
    sheet.set_print_gridlines(false);
    for i in 0..75u32 {
        sheet.set_column_format((i as u16), &COLUMN_FORMAT)?;
        sheet.set_row_height(i.into(), ROW_HEIGHT)?;
    }

    Ok(())
}