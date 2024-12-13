use std::sync::mpsc::Sender;

use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::DbPool;

use crate::{db, lookup};

pub struct Session {
    pub ort: ort::session::Session,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub lookup: lookup::Table,
    pub paste_tx: Sender<String>,
    pub pool: db::Pool,
}

async fn create_pool_from_handle(handle: AppHandle) -> db::Pool {
    let instances = &*handle.state::<tauri_plugin_sql::DbInstances>();
    let instances = instances.0.read().await;

    let DbPool::Sqlite(pool) = instances.get(db::URL).unwrap();

    pool.clone()
}

impl Session {
    pub async fn new(handle: AppHandle, paste_tx: Sender<String>) -> Session {
        Session {
            ort: ort::session::Session::builder()
                .unwrap()
                .commit_from_memory(include_bytes!("model.onnx"))
                .unwrap(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            lookup: lookup::Table::new(),
            pool: create_pool_from_handle(handle).await,
            paste_tx,
        }
    }
}
