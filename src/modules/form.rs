use itertools::Itertools;
use std::collections::HashMap;

macro_rules! sort_by_vec {
    ($input:expr, $order:expr) => {{
        let mut zipped: Vec<_> = $input.into_iter().zip($order.into_iter()).collect();
        zipped.sort_by(|(_, a), (_, b)| a.cmp(b));
        zipped
            .into_iter()
            .map(|(tuple, _)| tuple)
            .collect::<Vec<_>>()
    }};
}

#[derive(Debug, Clone)]
pub struct Form {
    /// List of (option name, all options)
    options: Vec<FormOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormOption {
    pub option_name: String,
    /// Request Name, Display Name
    pub options: Vec<(String, String)>,
    pub multiple: bool,
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

    pub fn form_data(&self) -> HashMap<String, (String, String)> {
        let mut form_data: HashMap<String, (String, String)> = HashMap::new();
        for op in self.options() {
            form_data.insert(
                op.option_name(),
                (
                    op.options().into_iter().map(|(r, _)| r).join(","),
                    op.options().into_iter().map(|(_, d)| d).join(" "),
                ),
            );
        }
        form_data
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
        self.options = sort_by_vec!(
            self.options()
                .into_iter()
                .map(|fo| (fo.option_name(), fo.options())),
            vec!["fordeling", "min_pro", "pro_ant", "kroner_prosent", "prosent_antall", "info", "variabel", "aar"]
        )
        .into_iter()
        .map(|(on, ops)| FormOption::new(on, ops))
        .collect_vec();
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

    /// Splits a large Form into several smaller once.
    pub fn split(mut self) -> impl Iterator<Item = Self> {
        let mut vec: Vec<Self> = Vec::new();

        self.order();

        if let Some(op) = self.options().last() {
            let nm = op.option_name();
            for fo in op.clone().options {
                let mut form = Self::new();

                form.add_options(FormOption::new(nm.clone(), vec![fo]));

                for ops in self.options() {
                    if ops.option_name == nm {
                        continue;
                    }
                    form.add_options(ops);
                }
                vec.push(form);
            }
        }

        vec.into_iter()
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

    pub fn show(&self) {
        for op in &self.options {
            println!("[{}, {}]", op.option_name, op.options().into_iter().map(|(d, _)| d).collect_vec().join(", "));
        }
    }
}
