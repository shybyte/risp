use std::collections::HashMap;
use types::*;

pub struct Environment {
    data: HashMap<String, RispType>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { data: HashMap::new() }
    }

    pub fn insert(&mut self, key: &str, value: RispType) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> RispType {
        self.data[key].clone()
    }
}


