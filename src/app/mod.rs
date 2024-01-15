use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;

use crate::{
    error::ArchiveError,
    modules::{
        sok::SokCollection,
        webpage::{Link, Webpage},
    },
    parser::wp::{get_sok_collection, get_sok_collection_tmf},
    xl::save_sok,
    ALLOW_DUPS, CHECKED_SOK_ID, CHECKED_SOK_TITLE,
};

pub mod main_app;

pub mod single_app;

pub mod offline_app;

pub mod interactive_app;

pub mod mf_app;

pub async fn main_fn(link: &Link) -> Result<(SokCollection, Vec<ArchiveError>), Vec<ArchiveError>> {
    let mut sok_log: Vec<ArchiveError> = Vec::new();

    let res = Webpage::from_link(link.clone()).await;

    if res.is_err() {
        return Err(vec![res.unwrap_err()]);
    }

    let wp = res.unwrap();

    let medium = wp.get_medium();

    let id = wp.get_id().clone();

    // Checks if dups are allowed
    if let Ok(allow_dup) = ALLOW_DUPS.lock() {
        if let Ok(title) = wp.get_title() {
            if !*allow_dup {
                if skip_sok(Some(&id), Some(&title)) {
                    println!("Skipping sok: {}, is duplicate", id);
                    return Err(vec![ArchiveError::DuplicateSok]);
                } else {
                    add_sok(Some(id), Some(title));
                }
            }
        }
    }

    println!("Sok: {}", id);

    match get_sok_collection_tmf(wp).await {
        Ok((sokc, mut errs)) => {
            sok_log.append(&mut errs);

            let path = format!("arkiv\\{}", medium.clone());
            if let Err(r) = fs::create_dir_all(path.clone()) {
                sok_log.push(r.into());
            }

            return Ok((sokc, sok_log));
        }
        Err(err) => {
            return Err(vec![err]);
        }
    }
}

pub fn try_save_sok(
    sokc: &SokCollection,
    path: &str,
    attempts: usize,
) -> Result<Vec<ArchiveError>, Vec<ArchiveError>> {
    match save_sok(&sokc, &path) {
        Ok(log) => Ok(log),
        Err(err) => {
            if attempts == 0 {
                let mut errs = save_sok(sokc, ".").unwrap_or_else(|e| vec![e]);
                errs.push(err);
                Err(errs)
            } else {
                match try_save_sok(sokc, path, attempts - 1) {
                    Ok(mut log) => {
                        log.push(err);
                        Ok(log)
                    }
                    Err(mut errs) => {
                        errs.push(err);
                        Err(errs)
                    }
                }
            }
        }
    }
}

pub fn skip_sok(id: Option<&usize>, title: Option<&str>) -> bool {
    if let Some(id) = id
        && let Ok(map) = CHECKED_SOK_ID.lock()
        && map.contains_key(id)
    {
        return true;
    }

    if let Some(title) = title
        && let Ok(map) = CHECKED_SOK_TITLE.lock()
        && map.contains_key(title)
    {
        return true;
    }

    return false;
}

pub fn add_sok(id: Option<usize>, title: Option<String>) {
    if let Some(id) = id
        && let Ok(mut map_id) = CHECKED_SOK_ID.lock()
    {
        map_id.insert(id, true);
    }

    if let Some(title) = title
        && let Ok(mut map_title) = CHECKED_SOK_TITLE.lock()
    {
        map_title.insert(title, true);
    }
}
