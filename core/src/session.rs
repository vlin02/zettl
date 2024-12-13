use std::sync::mpsc::Sender;

use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::DbPool;

use crate::{db, lookup};

pub struct Session {
    handle: AppHandle,
}

pub struct Context {
    pub ort: ort::session::Session,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub lookup: lookup::Table,
    pub paste_tx: Sender<String>,
}

impl Context {
    pub fn new(paste_tx: Sender<String>) -> Context {
        Context {
            ort: ort::session::Session::builder()
                .unwrap()
                .commit_from_memory(include_bytes!("model.onnx"))
                .unwrap(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            lookup: lookup::Table::new(),
            paste_tx,
        }
    }
}

impl Session {
    pub fn new(handle: AppHandle) -> Session {
        Session { handle }
    }

    pub async fn pool(&self) -> db::Pool {
        let instances = &*self.handle.state::<tauri_plugin_sql::DbInstances>();
        let instances = instances.0.read().await;

        let DbPool::Sqlite(pool) = instances.get(db::URL).unwrap();
        pool.clone()
    }

    pub fn ctx(&self) -> tauri::State<'_, Context> {
        self.handle.state::<Context>()
    }
}
