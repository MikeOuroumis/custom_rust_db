use std::io;

use custom_rust_db::db::DatabaseEngine;

fn main() {
    let mut db = DatabaseEngine::new();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let all_entries = db.get_all_entries();
    println!("{:?}", all_entries);
}
