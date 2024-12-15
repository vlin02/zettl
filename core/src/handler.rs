use tauri::{AppHandle, Manager};

use crate::{
    app::PasteTx,
    clipboard::Clipboard,
    snippet::{find_snippets, get_content, SnippetPage, SnippetsQuery},
};

pub async fn get_settings() {}

pub async fn update_settings() {}

#[tauri::command]
pub async fn list_snippets(handle: AppHandle, query: SnippetsQuery) -> SnippetPage {
    let session = handle.state::<Clipboard>();
    find_snippets(&session, &query).await
}

#[tauri::command]
pub async fn copy_snippet(handle: AppHandle, id: i32) {
    let PasteTx(paste_tx) = &*handle.state::<PasteTx>();
    let session = handle.state::<Clipboard>();
    let content = get_content(&session.pool, id).await;

    paste_tx.send(content).unwrap();
}

#[tauri::command]
pub async fn close_window(window: tauri::WebviewWindow) {
    window.hide().unwrap();
}
