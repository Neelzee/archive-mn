use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Error};
use std::path::Path;
use rand::Rng;
use rand::distributions::Uniform;
use ego_tree::{NodeId, NodeRef};
use scraper::Html;
use scraper::{Node, ElementRef};
use tokio::sync::mpsc::error::SendError;

use crate::modules::sok::Merknad;
use crate::modules::webpage::Webpage;
use crate::parser::wp::get_kilde;
use crate::parser::{get_merknad, get_text};

use super::constants::ROOT_URL;

// Might cause issues
pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => s.to_string(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn get_html_content_test() -> Result<Html, Error> {
    let mut content = String::new();
    let mut file = File::open("src\\tests\\html\\346")?;
    file.read_to_string(&mut content)?;

    Ok(Html::parse_document(&content))
}

pub fn validify_excel_string(str: &str) -> String {
    let mut vstr = str.replace(":", "");

    vstr = vstr.replace("{", "");
    vstr = vstr.replace("}", "");
    vstr = vstr.replace(";", "");
    vstr = vstr.replace(",", "");
    vstr = vstr.replace(".", "");
    vstr = vstr.replace("]", "");
    vstr = vstr.replace("[", "");
    vstr = vstr.replace("*", "");
    vstr = vstr.replace("?", "");
    vstr = vstr.replace("/", "-");
    vstr = vstr.replace("\\", "");

    vstr
}

pub fn trim_string(str: &str) -> String {
    str.split_whitespace().collect::<Vec<&str>>().join(" ")
} 

pub fn has_ancestor(node: NodeRef<Node>, id: NodeId) -> bool {

    if node.ancestors().collect::<Vec<_>>().len() == 0 {
        return false;
    }

    for ancestor in node.ancestors().into_iter() {
        if ancestor.id() == id {
            return true;
        }
    }

    node.ancestors().into_iter().flat_map(|e| e.ancestors()).any(|e| has_ancestor(e, id))
}

pub fn get_random_webpage() -> Option<Webpage> {
    if let Ok((file_name, raw_content)) = get_random_file_and_contents("src\\tests\\html".to_string()) {

        let url = format!("{}/{}", ROOT_URL, file_name.clone());
        let content = Html::parse_document(&raw_content);

        let mut id = 0;
        let medium = String::from("MEDIUM");

        if let Ok(i) = file_name.parse::<usize>() {
            id = i;
        } else {
            return None;
        }

        Some(Webpage::from_html(id, url, content, medium))
    } else {
        None
    }
}

pub fn get_random_file_and_contents(folder_path: String) -> io::Result<(String, String)> {
    // Read the contents of the folder
    let entries = fs::read_dir(folder_path)?;

    // Collect file paths into a vector
    let files: Vec<_> = entries.filter_map(|entry| {
        entry.ok().and_then(|e| {
            let path = e.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
    }).collect();

    // Choose a random file
    if !files.is_empty() {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0, files.len());
        let random_file = files.get(rng.sample(&dist)).unwrap();

        // Read file contents
        let contents = fs::read_to_string(random_file)?;

        // Get file name as String
        let file_name = random_file.file_name().unwrap().to_str().unwrap().to_string();

        Ok((file_name, contents))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No files found in the folder"))
    }
}

pub fn format_form_to_title(k: String, v: String) -> String {
    return format!("{} {}", k, v)
}

pub fn sending_error<T, E>(res: Result<T, SendError<E>>)
where
    T: std::fmt::Debug,
{
    if res.is_err() {
        println!("Failed sending error: {:?}", res.unwrap_err());
    }
}