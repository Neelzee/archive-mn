use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Form {
    /// List of (option name, all options)
    options: Vec<FormOption>
}

#[derive(Debug)]
pub struct FormOption {
    option_name: String,
    /// Request Name, Display Name
    options: Vec<(String, String)>
}

impl FormOption {
    pub fn new(option_name: String, options: Vec<(String, String)>) -> FormOption {
        FormOption {
            option_name,
            options,
        }
    }
}

impl Form {
    pub fn new() -> Form {
        Form { options: Vec::new() }
    }

    pub fn add_options(&mut self, option_name: String, options: Vec<(String, String)>) {
        self.options.push(FormOption { option_name, options });
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