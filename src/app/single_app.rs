use std::{time::Instant, fs};

use itertools::Itertools;

use crate::{error::ArchiveError, modules::webpage::{Link, Webpage}, parser::wp::get_sok_collection, xl::save_sok, app::main_app::{checkmark_sok, write_log}};

pub async fn get_soks(links: Vec<Link>) -> Result<(), ArchiveError> {
    for link in links {
        let mut sok_log: Vec<ArchiveError> = Vec::new();
    
        
        let wp = Webpage::from_link(link.clone()).await?;
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
                    sok_log.push(ArchiveError::FailedParsing(sokc.id.clone(), link.to_string().clone()));
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