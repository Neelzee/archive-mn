use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Form {
    /// List of (option name, all options)
    options: Vec<FormOption>
}

#[derive(Debug, Clone)]
pub struct FormOption {
    option_name: String,
    /// Request Name, Display Name
    options: Vec<(String, String)>,
    multiple: bool,
    has_all: bool,
}

impl FormOption {
    pub fn new(option_name: String, options: Vec<(String, String)>) -> FormOption {
        FormOption {
            option_name,
            options,
            multiple: false,
            has_all: false,
        }
    }

    pub fn show(&self) {
        println!("Option Name: {}", self.option_name);
        for (req, dis) in self.options.clone() {
            println!("Request Name: {}, Display Name: {}", req, dis);
        }
    }

    pub fn get_multiple(&self) -> bool {
        self.multiple
    }

    pub fn options(&self) -> Vec<(String, String)> {
        self.options.clone()
    }

    pub fn option_name(&self) -> String {
        self.option_name.clone()
    }

    /// Toggles multiple
    pub fn multiple(&mut self) {
        self.multiple = !self.multiple;
    }

    /// Toggles has all
    pub fn has_all(&mut self) {
        self.has_all = !self.has_all;
    }

    pub fn request_names(&self) -> Vec<String> {
        self.options
            .clone()
            .into_iter()
            .map(|(req, _)| req)
            .collect::<Vec<String>>()
    }
}

impl Form {
    pub fn new() -> Form {
        Form { options: Vec::new() }
    }

    pub fn options(&self) -> Vec<FormOption> {
        self.options.clone()
    }
    
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    pub fn add_options(&mut self, fo: FormOption) {
        self.options.push(fo);
    }

    pub fn combinations(self) -> impl Iterator<Item = HashMap<String, (String, String)>> {
        let options_vec: Vec<Vec<(String, String)>> = self.options.iter()
            .map(|opt| opt.options.clone())
            .collect();

        let product_iter = options_vec.into_iter().multi_cartesian_product();

        product_iter.map(move |product| {
            let mut form_data: HashMap<String, (String, String)> = HashMap::new();
            for (i, option) in self.options.iter().enumerate() {
                form_data.insert(option.option_name.clone(), (product[i].0.clone(), product[i].1.clone()));
            }
            form_data
        })
    }
}