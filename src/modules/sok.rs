use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Table {
    pub header: Vec<Vec<String>>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            header: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// No head?
    pub fn get_col(&self, c: usize) -> Option<Vec<&str>> {
        let mut col = Vec::new();

        for r in &self.rows {
            if let Some(cell) = r.get(c) {
                col.push(cell);
            } else {
                return None;
            }
        }

        return Some(col);
    }
}

#[derive(Debug, Clone)]
pub struct Sok {
    pub title: String,
    pub titles: Vec<String>,
    pub header_title: String,
    pub tables: Vec<Table>,
    pub display_names: Vec<String>,
    pub merknad: Vec<Merknad>,
    pub kilde: Vec<Kilde>,
    pub metode: Vec<Metode>,
}

impl Sok {
    pub fn new() -> Sok {
        Sok {
            title: String::new(),
            titles: Vec::new(),
            header_title: String::new(),
            tables: Vec::new(),
            display_names: Vec::new(),
            merknad: Vec::new(),
            kilde: Vec::new(),
            metode: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SokCollection {
    pub id: usize,
    pub medium: String,
    pub title: String,
    pub text: Vec<String>,
    pub sok: Vec<Sok>,
    pub merknad: Vec<Merknad>,
    pub kilde: Vec<Kilde>,
    pub metode: Vec<Metode>,
}

impl SokCollection {
    pub fn new(id: usize, medium: String) -> SokCollection {
        SokCollection {
            id,
            medium,
            title: String::new(),
            text: Vec::new(),
            sok: Vec::new(),
            merknad: Vec::new(),
            kilde: Vec::new(),
            metode: Vec::new(),
        }
    }
    pub fn add_text(&mut self, text: String) {
        self.text.push(text);
    }

    pub fn add_sok(&mut self, sok: Sok) {
        self.sok.push(sok);
    }

    pub fn add_merknad(&mut self, merknad: Merknad) {
        self.merknad.push(merknad);
    }

    pub fn add_kilde(&mut self, kilde: Kilde) {
        self.kilde.push(kilde);
    }

    pub fn add_metode(&mut self, metode: Metode) {
        self.metode.push(metode);
    }
}

#[derive(Debug, Clone)]
pub struct Merknad {
    pub title: String,
    pub content: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Kilde {
    pub title: String,
    pub content: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Metode {
    pub title: String,
    pub content: Vec<String>,
}

type ArchiveContent = (String, Vec<String>);

macro_rules! impl_ac {
    ($struct_name:ident) => {
        impl From<ArchiveContent> for $struct_name {
            fn from((title, content): (String, Vec<String>)) -> Self {
                $struct_name { title, content }
            }
        }
    };
}

macro_rules! impl_ie {
    ($struct_name:ident) => {
        impl IsEmpty for $struct_name {
            fn is_empty(&self) -> bool {
                self.content.is_empty()
                    || self
                        .content
                        .clone()
                        .into_iter()
                        .all(|e| e.is_empty() || e.split_whitespace().count() == 0)
            }
        }
    };
}

impl_ac!(Merknad);

impl_ac!(Metode);
impl_ie!(Metode);
impl_ac!(Kilde);
impl_ie!(Kilde);

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Merknad {
    fn is_empty(&self) -> bool {
        self.content.is_empty()
        || self
            .content
            .clone()
            .into_iter()
            .all(|e| e.is_empty() || e.split_whitespace().count() == 0)
        || (self.content.len() == 1 && self.content.clone().pop().unwrap().trim() == "Alle data kan fritt benyttes såfremt både originalkilde og Medienorge oppgis som kilder.")
    }
}
