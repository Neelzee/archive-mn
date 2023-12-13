use std::cmp::max;


#[derive(Debug, Clone)]
pub struct Table {
    pub header: Vec<Vec<String>>,
    pub styles: Vec<Vec<String>>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            header: Vec::new(),
            styles: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Prints the table, adds `' ,'` between every element.
    pub fn show(&self) {
        let header = self.header
        .iter()
        .fold(
            String::new(), 
            |mut acc, e| {
                acc += &e.iter().fold(String::new(), |mut ac, x| {
                    ac += x;
                    ac += ", ";
                    ac
                });
                acc += "\n";
                acc
            });

        let rows = self.rows
            .iter()
            .fold(
                String::new(), 
                |mut acc, e| {
                    acc += &e.iter().fold(String::new(), |mut ac, x| {
                        ac += x;
                        ac += ", ";
                        ac
                    });
                    acc += "\n";
                    acc
                });
        println!("{}", "=".repeat(max(header.len(), rows.len())));
        println!("{}", header);
        println!("{}", rows);
        println!("{}", "=".repeat(max(header.len(), rows.len())));
    }
}

#[derive(Debug, Clone)]
pub struct Sok {
    pub title: String,
    pub tables: Vec<Table>,
}


impl Sok {
    pub fn new() -> Sok {
        Sok {
            title: String::new(),
            tables: Vec::new(),
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
    pub merknad: Vec<String>,
    pub kilde: Vec<String>,
    pub metode: Vec<String>,
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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn add_text(&mut self, text: String) {
        self.text.push(text);
    }

    pub fn add_sok(&mut self, sok: Sok) {
        self.sok.push(sok);
    }

    pub fn add_merknad(&mut self, merknad: String) {
        self.merknad.push(merknad);
    }

    pub fn add_kilde(&mut self, kilde: String) {
        self.kilde.push(kilde);
    }

    pub fn add_metode(&mut self, metode: String) {
        self.metode.push(metode);
    }
}