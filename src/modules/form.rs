use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Form {
    /// List of (option name, all options)
    options: Vec<FormOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormOption {
    option_name: String,
    /// Request Name, Display Name
    options: Vec<(String, String)>,
    multiple: bool,
}

impl FormOption {
    pub fn new(option_name: String, options: Vec<(String, String)>) -> FormOption {
        FormOption {
            option_name,
            options,
            multiple: false,
        }
    }

    pub fn add_options(&mut self, option: (String, String)) {
        self.options.push(option);
    }

    pub fn contains_req(&self, req_name: String) -> bool {
        self.options().into_iter().any(|(e, _)| e == req_name)
    }

    pub fn show(&self) {
        println!("Option Name: {}", self.option_name);
        for (req, dis) in self.options.clone() {
            println!(
                "Request Name: {}, Display Name: {}, Multiple: {}",
                req, dis, self.multiple
            );
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
}

impl Form {
    pub fn new() -> Self {
        Form {
            options: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.options.clear();
    }

    pub fn get_option(&self, option: String) -> Option<FormOption> {
        for op in self.options() {
            if op.option_name == option {
                return Some(op);
            }
        }
        None
    }

    pub fn missing_options(&self, form: &Form) -> bool {
        self.options()
            .into_iter()
            .any(|op| form.options.contains(&op))
    }

    pub fn fill_form_data(&self, form: &Form) -> Form {
        let mut form_data = form.clone();
        for op in self.options() {
            if !form_data.options.contains(&op) {
                form_data.add_options(op);
            }
        }
        return form_data;
    }

    pub fn contains_option(&self, option: String) -> bool {
        self.options
            .clone()
            .into_iter()
            .any(|e| e.option_name == option)
    }

    pub fn contains_choice(&self, choice: String) -> bool {
        self.options
            .clone()
            .into_iter()
            .any(|e| e.options.into_iter().any(|c| c.0 == choice))
    }

    pub fn order(&mut self) {
        let mut vec = Vec::new();
        for el in self.options() {
            if el.option_name == "variabel" {
                vec.push(el);
                break;
            }
        }
        for el in self.options() {
            if el.option_name != "variabel" {
                vec.push(el);
            }
        }
        self.options = vec;
    }

    pub fn options(&self) -> Vec<FormOption> {
        self.options.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// Overwrites old fo
    pub fn add_options(&mut self, fo: FormOption) {
        for i in 0..self.options().len() {
            if self.options.get(i).unwrap().option_name == fo.option_name {
                self.options.swap_remove(i);
                break;
            }
        }
        self.options.push(fo);
    }

    pub fn combinations(
        self,
    ) -> impl Iterator<Item = (HashMap<String, (String, String)>, Vec<String>)> {
        let options_vec: Vec<Vec<(String, String)>> =
            self.options.iter().map(|opt| opt.options.clone()).collect();

        let product_iter = options_vec.into_iter().multi_cartesian_product();

        product_iter.map(move |product| {
            let mut form_data: HashMap<String, (String, String)> = HashMap::new();
            let mut disp: Vec<String> = Vec::new();
            for (i, option) in self.options.iter().enumerate() {
                form_data.insert(
                    option.option_name.clone(),
                    (product[i].0.clone(), product[i].1.clone()),
                );
                disp.push(product[i].1.clone());
            }
            (form_data, disp)
        })
    }

    /// Returns the display of the given option
    pub fn get_display(&self, req: String) -> String {
        for op in self.options() {
            for (r, d) in op.options() {
                if r == req {
                    return d;
                }
            }
        }
        return "MISSING".to_string();
    }
}
