use std::f32::consts::E;

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