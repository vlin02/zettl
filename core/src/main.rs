use app::start;
use sqlx::{query_as, Sqlite, SqlitePool};
use tauri::async_runtime::block_on;

mod app;
pub mod clipboard;
pub mod db;
pub mod detection;
pub mod event;
pub mod lookup;
pub mod pasteboard;
pub mod profile;
pub mod settings;
pub mod shortcuts;
pub mod snippet;
pub mod syntax;
pub mod theme;
pub mod window;

fn main() {
    // let pool = block_on(SqlitePool::connect("/Users/vilin/Library/Application Support/io.zettl.app/zettl.db")).unwrap();
    // let row: (i64,) = block_on(query_as("SELECT count(*) FROM snippet").fetch_one(&pool)).unwrap();
    // println!("{:?}", row);
    start();
}
