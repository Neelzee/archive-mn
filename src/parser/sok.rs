
pub struct Table {
    pub parent_id: usize,
    pub name: String,
    pub header: Vec<Vec<String>>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            parent_id: 0,
            name: String::new(),
            header: Vec::new(),
            rows: Vec::new(),
        }
    }
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