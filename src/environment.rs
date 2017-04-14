use std::collections::HashMap;
use types::*;

#[derive(Default)]
pub struct Environment {
    data: HashMap<String, RispType>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { data: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: RispType) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<RispType> {
        self.data.get(key).cloned()
    }
}


