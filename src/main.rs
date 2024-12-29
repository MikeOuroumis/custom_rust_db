use std::io;

use custom_rust_db::db::DatabaseEngine;

fn main() {
    let mut db = DatabaseEngine::new();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    db.insert("name", &name);

    let entries = db.get_all_entries();
    println!("All entries:");
    for (key, value) in entries {
        println!("{}: {}", key, value);
    }
}
