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
    start();
}
