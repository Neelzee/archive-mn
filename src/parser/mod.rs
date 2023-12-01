pub mod webpage;


pub struct Link {
    parent_id: usize,
    url: String,
}

pub struct Table {
    pub parent_id: usize,
    pub name: String,
    pub rows: Vec<Vec<String>>,
    pub columns: Vec<Vec<String>>
}

pub struct Sok {
    pub parent_id: usize,
    pub id: usize,
    pub medium: String,
    pub title: String,
    pub text: Vec<String>,
    pub tables: Vec<Table>,
    pub merknad: Vec<String>,
    pub kilde: Vec<String>,
}


impl Sok {
    pub fn new(parent_id: usize, medium: String) -> Sok {
        Sok {
            parent_id,
            id: 0,
            medium,
            title: String::from("No title"),
            text: Vec::new(),
            tables: Vec::new(),
            merknad: Vec::new(),
            kilde: Vec::new(),
        }
    }
}