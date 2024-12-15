use session::Session;
use snippet::{find_snippets, SnippetsQuery};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use tauri::async_runtime::block_on;

pub mod db;
pub mod detection;
pub mod handler;
pub mod lookup;
pub mod session;
pub mod snippet;
pub mod syntax;

fn main() {
    // Create a connection pool
    let pool = block_on(SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("/Users/vilin/Library/Application Support/io.zettl.app/zettl.db")
            .create_if_missing(true),
    ))
    .unwrap();

    //     let create_table_query = "
    // CREATE TABLE
    //   snippet (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     content TEXT,
    //     format TEXT,
    //     lines BLOB,
    //     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    //   );

    // CREATE VIRTUAL TABLE snippet_fts USING fts5 (content, tokenize=\"trigram\");

    // CREATE TRIGGER trigger_insert_snippet AFTER INSERT ON snippet
    // BEGIN
    //     INSERT INTO snippet_fts(rowid, content) VALUES (new.id, new.content);
    // END;

    // CREATE TRIGGER trigger_delete_snippet AFTER DELETE ON snippet
    // BEGIN
    //     DELETE FROM snippet_fts WHERE rowid = old.id;
    // END;
    // ";

    //     block_on(sqlx::query(create_table_query).execute(&pool)).unwrap();

    let session = Session {
        ort: ort::session::Session::builder()
            .unwrap()
            .commit_from_memory(include_bytes!("model.onnx"))
            .unwrap(),
        syntax_set: SyntaxSet::load_defaults_newlines(),
        theme_set: ThemeSet::load_defaults(),
        lookup: lookup::LookupTable::new(),
        pool,
    };

    let q = SnippetsQuery {
        search: String::from("c"),
        next_id: Some(10),
        limit: 10,
    };

    println!("{:?}", block_on(find_snippets(&session, &q)));

    // block_on(insert_snippet(
    //     &session,
    //     "
    
    // CREATE TRIGGER trigger_insert_snippet AFTER INSERT ON snippet
    // BEGIN
    //     INSERT INTO snippet_fts(rowid, content) VALUES (new.id, new.content);
    // END;

    // CREATE TRIGGER trigger_delete_snippet AFTER DELETE ON snippet
    // BEGIN
    //     DELETE FROM snippet_fts WHERE rowid = old.id;
    // END;
    // ",
    // ))
}
