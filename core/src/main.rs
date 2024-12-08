use std::{thread, time::Duration};
use syntect::parsing::{SyntaxReference, SyntaxSet};

mod schema;

fn a() {
    // Your code for function `a` goes here.
    println!("Function `a` was called");
}

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    
    
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(schema::DB_URL, schema::list_migrations())
                .build(),
        )
        .setup(|_| {
            let handle = thread::spawn(|| {
                loop {
                    a();
                    thread::sleep(Duration::from_millis(1000)); // Sleep for 100 milliseconds
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // println!("here2");
}
