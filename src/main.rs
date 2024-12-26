use std::io;

use custom_rust_db::db::DatabaseEngine;

fn main() {
    let mut db = DatabaseEngine::new();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    db.insert("name", &name);

    if let Some(value) = db.get("name") {
        println!("Value {}", value)
    } else {
        println!("Key not found")
    }
}
