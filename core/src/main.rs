use std::{thread, time::Duration};
use magika::{sess}

mod schema;

fn a() {
    // Your code for function `a` goes here.
    println!("Function `a` was called");
}

fn main() {
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
                    thread::sleep(Duration::from_millis(1000));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
