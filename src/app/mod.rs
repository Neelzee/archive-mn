use std::fs;

use crate::{
    app::main_app::write_failed_sok,
    error::ArchiveError,
    modules::{
        sok::SokCollection,
        webpage::{Link, Webpage},
    },
    parser::wp::{get_sok_collection, get_sok_collection_tmf},
    xl::save_sok,
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

    let mut wp = res.unwrap();

    let medium = wp.get_medium();
    let id = wp.get_id().clone();

    wp.set_medium(medium.clone());

    println!("Sok: {}", wp.get_id());

    if let Some(com) = wp.get_forms().ok() {
        let count = com.combinations().collect::<Vec<_>>().len();
        if count >= 30 {
            println!("Form Combo: {:?}", count);

            match get_sok_collection_tmf(wp).await {
                Ok((sokc, mut errs)) => {
                    sok_log.append(&mut errs);

                    let path = format!("error\\{}", medium.clone());
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
    }

    match get_sok_collection(wp).await {
        Ok((sokc, mut errs)) => {
            sok_log.append(&mut errs);

            let path = format!("arkiv\\{}", medium.clone());
            if let Err(r) = fs::create_dir_all(path.clone()) {
                sok_log.push(r.into());
            }

            // Failed
            if sokc.sok.len() == 0 {
                println!("Sok: {}, had 0 tables.", &sokc.id);
                sok_log.push(ArchiveError::FailedParsing(
                    sokc.id.clone(),
                    link.to_string().clone(),
                ));
            }

            Ok((sokc, sok_log))
        }
        Err(e) => {
            sok_log.push(e);
            Err(sok_log)
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
