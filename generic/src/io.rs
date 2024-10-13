use std::collections::HashMap;

pub fn get_args() -> HashMap<String, String> {
    let mut res = HashMap::new();
    let args = std::env::args().collect::<Vec<String>>();
    let mut key = "".to_string();
    for arg in args {
        if arg.starts_with("--") {
            key = arg.trim_start_matches("--").to_string();
        } else if !key.is_empty() {
            res.insert(key.clone(), arg);
        }
    }
    res
}

pub struct Help {
    descriptions: Vec<(String, String)>,
}

impl Help {
    pub fn new() -> Self {
        Self {
            descriptions: Vec::new(),
        }
    }

    pub fn add(&mut self, key: &str, description: &str) {
        self.descriptions.push((key.to_string(), description.to_string()));
    }

    pub fn print(&self) {
        println!("Usage:");
        for (key, description) in &self.descriptions {
            println!("\t--{}: {}", key, description);
        }
    }
}
