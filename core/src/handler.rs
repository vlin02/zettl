use tauri::{AppHandle, Manager};

use crate::{
    app::PasteTx,
    clipboard::Clipboard,
    snippet::{query, get_content, Page, SnippetsQuery},
    theme::{self, load_theme_css, File},
    profile::{self, Settings, User},
};

#[tauri::command]
pub async fn get_settings(handle: AppHandle) -> Settings {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    profile::get_settings(pool).await
}


#[tauri::command]
pub async fn load_active_theme(handle: AppHandle) -> String {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();

    let settings = profile::get_user(pool).await;
    load_theme_css(&pool, settings.theme_id).await
}

#[tauri::command]
pub async fn import_theme(handle: AppHandle, name: String, text: String) -> Result<i32, String> {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    let file = File { name, text };

    theme::import_theme(pool, &file).await
}

#[tauri::command]
pub async fn set_popup_dimensions(handle: AppHandle, w: i32, h: i32) {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    
}

#[tauri::command]
pub async fn set_popup_dimensions(handle: AppHandle, w: i32, h: i32) {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    
}

#[tauri::command]
pub async fn list_snippets(handle: AppHandle, query: SnippetsQuery) -> Page {
    let clipboard = handle.state::<Clipboard>();

    query(&clipboard, &query).await
}

#[tauri::command]
pub async fn close_window(window: tauri::WebviewWindow) {
    window.hide().unwrap();
}
