use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;
use rand::distributions::Uniform;
use ego_tree::{NodeId, NodeRef};
use scraper::{Node, ElementRef};

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => s.to_string(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


pub fn trim_string(str: &str) -> String {
    return str.split_ascii_whitespace().collect::<Vec<&str>>().join(" ");
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