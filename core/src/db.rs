use sqlx::SqlitePool;
use tauri_plugin_sql::{Migration, MigrationKind};

pub type Pool = SqlitePool;

pub const URL: &str = "sqlite:zettl.db";

pub fn list_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "",
        sql: "
CREATE TABLE
  snippet (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT,
    format TEXT,
    lines BLOB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );

CREATE VIRTUAL TABLE snippet_fts USING fts5 (content, tokenize=\"trigram\");

CREATE TRIGGER trigger_insert_snippet AFTER INSERT ON snippet
BEGIN
    INSERT INTO snippet_fts(rowid, content) VALUES (new.id, new.content);
END;

CREATE TRIGGER trigger_delete_snippet AFTER DELETE ON snippet
BEGIN
    DELETE FROM snippet_fts WHERE rowid = old.id;
END;

CREATE TABLE
  theme (
    name TEXT,
    is_dark BOOL,
    dump BLOB,
    is_default BOOL
  );

CREATE TABLE
  user (
    popup_width INTEGER,
    popup_height INTEGER,
    poll_interval INTEGER,
    crop_whitespace BOOL,
    theme_id INTEGER,
    FOREIGN KEY (theme_id) REFERENCES theme (id)
  );
",
        kind: MigrationKind::Up,
    }]
}
