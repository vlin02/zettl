use sqlx::SqlitePool;
use tauri_plugin_sql::{Migration, MigrationKind};

pub const URL: &str = "sqlite:zettl.db";

pub type Pool = SqlitePool;

pub fn list_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "",
        sql: "
CREATE TABLE
  pb_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );

CREATE VIRTUAL TABLE pb_item_fts USING fts5 (content);

CREATE TRIGGER trigger_insert_pb_item AFTER INSERT ON pb_item
BEGIN
    INSERT INTO pb_item_fts(rowid, content) VALUES (new.id, new.content);
END;

CREATE TRIGGER trigger_delete_pb_item AFTER DELETE ON pb_item
BEGIN
    DELETE FROM pb_item_fts WHERE rowid = old.id;
END;
",
        kind: MigrationKind::Up,
    }]
}
