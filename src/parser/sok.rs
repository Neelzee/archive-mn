
#[derive(Debug, Clone)]
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
            // TODO: Change to Vec<String>
            name: String::new(),
            header: Vec::new(),
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
        println!("{}", self.name);
        println!("{}", header);
        println!("{}", rows);
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
    pub metode: Vec<String>,
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
            metode: Vec::new()
        }
    }
}