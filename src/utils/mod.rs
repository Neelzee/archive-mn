pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => s.to_string(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}