use tauri_plugin_sql::{Migration, MigrationKind};

pub fn list_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "",
        sql: "CREATE TABLE pasteboard_item (id INTEGER PRIMARY KEY, name TEXT);",
        kind: MigrationKind::Up,
    }]
}

pub const DB_URL: &str = "sqlite:mydatabase.db";
