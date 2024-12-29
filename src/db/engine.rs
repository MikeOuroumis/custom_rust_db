use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct DatabaseEngine {
    storage: BTreeMap<String, String>,
}

impl DatabaseEngine {
    pub fn new() -> Self {
        DatabaseEngine {
            storage: BTreeMap::new(),
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
        if let Some(value) = self.storage.remove(key) {
            println!("Removed key: {}, value: {}", key, value);
        } else {
            println!("Key not found")
        }
    }

    pub fn get_all_entries(&self) -> BTreeMap<String, String> {
        self.storage.clone()
    }

    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        // Serialize the storage and map the error to std::io::Error
        let serialized = bincode::serialize(&self.storage).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Serialization error: {}", e),
            )
        })?;

        let mut file = File::create(file_path)?;
        file.write_all(&serialized)?;
        println!("Database saved to {}", file_path);
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        self.storage = bincode::deserialize(&contents).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Deserialization error: {}", e),
            )
        })?;
        println!("Database loaded from {}", file_path);
        Ok(())
    }
}
