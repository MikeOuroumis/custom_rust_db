use std::collections::HashMap;

pub struct DatabaseEngine {
    storage: HashMap<String, String>
}

impl DatabaseEngine {
    pub fn new()-> Self {
        DatabaseEngine {
            storage: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.storage.insert(key.to_string(), value.to_string());
        println!("Inserting key: {}, value: {}", key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        print!("Retrieving key {}", key);
        self.storage.get(key)
    }
}