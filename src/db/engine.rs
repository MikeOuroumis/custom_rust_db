use std::collections::HashMap;

pub struct DatabaseEngine {
    storage: HashMap<String, String>,
}

impl DatabaseEngine {
    pub fn new() -> Self {
        DatabaseEngine {
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.storage
            .insert(key.trim().to_string(), value.trim().to_string());
        println!("Inserting key: {}, value: {}", key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        print!("Retrieving key {}", key);
        self.storage.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        if let Some(key) = self.storage.remove(key) {
            println!("Removed key {}", key);
        } else {
            println!("Key not found")
        }
    }

    pub fn get_all_entries(&self) -> HashMap<String, String> {
        self.storage.clone()
    }
}
