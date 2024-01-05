use std::{time::Instant, fs::{self, File}, io::Read, fmt::format};

use itertools::Itertools;
use scraper::Html;

use crate::{error::ArchiveError, modules::{webpage::{Link, Webpage}, sok::{SokCollection, Merknad}}, parser::{wp::{get_sok_collection, get_kilde, get_metode}, medium, get_merknad}, xl::save_sok, app::main_app::{checkmark_sok, write_log}, utils::constants::ROOT_URL};

use std::path::Path;

const OFFLINE_PATH: &str = "arkiv\\offline";

pub fn visit_dirs(dir: &Path) -> std::io::Result<Vec<String>> {
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

fn list_files_and_folders_in(folder_path: &str) -> Vec<(String, bool)> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let is_directory = path.is_dir();

                result.push((file_name, is_directory));
                
                // If it's a directory, recursively call the function to get its contents
                if is_directory {
                    let subfolder_path = path.to_str().unwrap();
                    let subfolder_contents = list_files_and_folders_in(subfolder_path);
                    result.extend(subfolder_contents);
                }
            }
        }
    }

    result
}

pub async fn get_soks_offline() -> Result<(), ArchiveError> {

    let files = list_files_and_folders_in("in");
    
    for (raw_path, is_folder) in files {
        let path = format!("in\\{}", raw_path.clone());
        if !is_folder {
            let mut sok_log: Vec<ArchiveError> = Vec::new();
            
            let res = File::open(path.clone());

            if res.is_err() {
                eprintln!("Skipping: {}, due too: {}", &path, res.unwrap_err());
                continue;
            }

            let mut file = res.unwrap();

            let mut raw_html = String::new();
            let r = file.read_to_string(&mut raw_html);

            if r.is_err() {
                eprintln!("Skipping: {}, due too: {}", &path, r.unwrap_err());
                continue;
            }
            
            let content = Html::parse_document(&raw_html);

            let mut id = 0;
            let mut medium = "unknown".to_string();
            
            let mut str = path.split("\\").collect::<Vec<_>>().into_iter().rev();

            if let Some(i) = str.next() {
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
            println!("Single Page Offline Sok: {}", &id);        
            let time_start = Instant::now();

            let link = wp.get_url();
            let path = format!("{}\\{}", OFFLINE_PATH, medium.clone());

            let mut sokc = SokCollection::new(id, medium);

            let r = fs::create_dir_all(path.clone());
            if r.is_err() {
                println!("Could not create path: {}, got error: {}", path.clone(), r.unwrap_err());
            }

            let time_end = Instant::now();

            sokc.title = wp.get_title()?;

            let mut sok = wp.get_sok()?;
            sok.header_title = sok.title.clone();

            sokc.add_sok(sok);

            // Kilde
            for k in get_kilde(&wp).await? {
                sokc.add_kilde(k.into());
            }

            // Metode
            for m in get_metode(&wp).await? {
                sokc.add_metode(m.into());
            }

            // Merknad
            sokc.add_merknad(Merknad { title: "Merknad".to_string(), content: wp.get_merknad()? });

            // text
            for t in wp.get_text()? {
                sokc.add_text(t);
            }

            // Failed
            if sokc.sok.len() == 0 {
                println!("Sok: {}, had 0 tables.", &sokc.id);
                sok_log.push(ArchiveError::FailedParsing(sokc.id.clone(), link));
                continue;
            }

            match save_sok(sokc, &path) {
                Ok(_) => {
                    println!("Saved sok: {}, Took {}s", &id, (time_end - time_start).as_secs());
                },
                Err(e) => {
                    println!("Failed saving sok: {}, With Error: {}, Took {}s", &id, &e, (time_end - time_start).as_secs());
                    sok_log.push(e);
                }, 
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
        } else {
            let mut id = 0;
            let mut medium = "unknown".to_string();
            let mut str = path.split("\\").collect::<Vec<_>>().into_iter().rev();

            if let Some(i) = str.next() {
                id = i.parse::<usize>()?;
            }

            if let Some(m) = str.next() {
                medium = m.to_owned();
            }

            if medium == "in".to_string() {
                medium = "unknown".to_string();
            }

            let url = format!("{}/{}/{}", ROOT_URL, medium, id);

            let mut sokc = SokCollection::new(id, medium.clone());

            let mut i = 0;
            let mut sok_log: Vec<ArchiveError> = Vec::new();
            let time_start = Instant::now();
            let res = visit_dirs(Path::new(&path));

            if res.is_err() {
                eprintln!("Skipping: {}, due too: {}", &path, res.unwrap_err());
                continue;
            }

            for p in res.unwrap() {
                println!("Path: {}", p.clone());
                let res = File::open(p.clone());

                if res.is_err() {
                    eprintln!("Skipping: {}, due too: {}", &p, res.unwrap_err());
                    continue;
                }

                let mut file = res.unwrap();

                let mut raw_html = String::new();

                let r = file.read_to_string(&mut raw_html);

                if r.is_err() {
                    eprintln!("Skipping: {}, due too: {}", p, r.unwrap_err());
                }
                
                let content = Html::parse_document(&raw_html);

                let wp = Webpage::from_html(id, url.clone(), content, medium.clone());
                let medium = wp.get_medium();
                let id = wp.get_id().clone();
                println!("Multi Page Offline Sok: {}", &id);        

                let path = format!("{}\\{}", OFFLINE_PATH, medium.clone());

                let r = fs::create_dir_all(path.clone());
                if r.is_err() {
                    println!("Could not create path: {}, got error: {}", path.clone(), r.unwrap_err());
                }

                if i == 0 {
                    // Kilde
                    for k in get_kilde(&wp).await? {
                        sokc.add_kilde(k.into());
                    }

                    // Metode
                    for m in get_metode(&wp).await? {
                        sokc.add_metode(m.into());
                    }

                    // Merknad
                    sokc.add_merknad(Merknad { title: "Merknad".to_string(), content: wp.get_merknad()? });

                    // text
                    for t in wp.get_text()? {
                        sokc.add_text(t);
                    }

                    sokc.title = wp.get_title()?;
                }

                i += 1;

                let mut sok = wp.get_sok()?;
                sok.header_title = sok.title.clone();
                sokc.add_sok(sok);
            }

            let time_end = Instant::now();

            // Failed
            if sokc.sok.len() == 0 {
                println!("Sok: {}, had 0 tables.", &id);
                sok_log.push(ArchiveError::FailedParsing(id.clone(), "offline".to_string()));
                continue;
            }

            match save_sok(sokc, OFFLINE_PATH) {
                Ok(_) => {
                    println!("Saved sok: {}, Took {}s", &id, (time_end - time_start).as_secs());
                },
                Err(e) => {
                    println!("Failed saving sok: {}, With Error: {}, Took {}s", &id, &e, (time_end - time_start).as_secs());
                    sok_log.push(e);
                }, 
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
    }

    Ok(())
}
