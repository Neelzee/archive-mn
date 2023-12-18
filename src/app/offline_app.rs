use std::{time::Instant, fs::{self, File}, io::Read};

use itertools::Itertools;
use scraper::Html;

use crate::{error::ArchiveError, modules::webpage::{Link, Webpage}, parser::{wp::get_sok_collection, medium}, xl::save_sok, app::main_app::{checkmark_sok, write_log}, utils::constants::ROOT_URL};

use std::path::Path;

fn visit_dirs(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursive call for sub-folders
                paths.append(&mut visit_dirs(&path)?);
            } else {
                // Process each file in the sub-folder
                paths.push(path.as_os_str().to_str().expect("Should be a valid string").to_owned());
            }
        }
    }

    Ok(paths)
}

pub async fn get_soks_offline() -> Result<(), ArchiveError> {
    for path in visit_dirs(Path::new("in"))? {
        let mut sok_log: Vec<ArchiveError> = Vec::new();
        
        let mut file = File::open(path.clone())?;

        let mut raw_html = String::new();
        file.read_to_string(&mut raw_html)?;
        
        let content = Html::parse_document(&raw_html);

        let mut id = 0;
        let mut medium = "unknown".to_string();
        
        let mut str = path.split("\\").collect::<Vec<_>>().into_iter().rev();

        if let Some(i) = str.next() {
            println!("{}", i);
            id = i.parse::<usize>()?;
        }

        if let Some(m) = str.next() {
            medium = m.to_owned();
        }

        if medium == "in".to_string() {
            medium = "unknown".to_string();
        }

        let url = format!("{}/{}/{}", ROOT_URL, medium, id);

        let wp = Webpage::from_html(id, url, content, medium);
        let medium = wp.get_medium();
        let id = wp.get_id().clone();
        
        let time_start = Instant::now();

        if let Some(com) = wp.get_forms().ok() {
            let count = com.combinations().collect::<Vec<_>>().len();
            if count <= 30 {
                println!("Sok: {}", wp.get_id());
            } else {
                println!("Sok: {}", wp.get_id());
                println!("Form Combo: {:?}", count);
                continue;
            }
        }
        let link = wp.get_url();
        match get_sok_collection(wp).await {
            Ok((sokc, mut errs)) => {
                sok_log.append(&mut errs);
                let path = format!("arkiv\\{}", medium.clone());

                let r = fs::create_dir_all(path.clone());
                if r.is_err() {
                    println!("Could not create path: {}, got error: {}", path.clone(), r.unwrap_err());
                }

                let time_end = Instant::now();

                // Failed
                if sokc.sok.len() == 0 {
                    println!("Sok: {}, had 0 tables.", &sokc.id);
                    sok_log.push(ArchiveError::FailedParsing(sokc.id.clone(), link));
                    continue;
                }

                match save_sok(sokc, &path) {
                    Ok(_) => {
                        checkmark_sok(&id);
                        println!("Saved sok: {}, Took {}s", &id, (time_end - time_start).as_secs());
                    },
                    Err(e) => {
                        println!("Failed saving sok: {}, With Error: {}, Took {}s", &id, &e, (time_end - time_start).as_secs());
                        sok_log.push(e);
                    }, 
                }
            },
            Err(e) => sok_log.push(e)
        }
        if sok_log.len() != 0 {
            println!("{} error(s) found.", sok_log.len());
            let r = write_log(sok_log.clone().into_iter().map(|e| e.to_string()).collect_vec(), id);
            if r.is_err() {
                println!("Failed writing Error Log for Sok: {}, due too {}, dumping log.", id, r.unwrap_err());
                for err in sok_log {
                    println!("Sok {} Error: {:?}", id, err);
                }
            }
        } else {
            println!("No errors reported.");
        }
    }
    Ok(())
}