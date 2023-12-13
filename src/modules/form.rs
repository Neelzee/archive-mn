use itertools::Itertools;

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

/*
impl Iterator for Form {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.options.pop()
    }
}
*/

impl Form {
    pub fn new() -> Form {
        Form { options: Vec::new() }
    }

    pub fn add_options(&mut self, option_name: String, options: Vec<(String, String)>) {
        self.options.push(FormOption { option_name, options });
    }

    /*
    pub fn all_combinations(self) -> impl Iterator {
        let n = self.options.len();

        self.options
            .into_iter()
            .combinations(n)
    }
     */
}