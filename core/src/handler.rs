use tauri::{AppHandle, Manager};

use crate::{
    session::Session,
    snippet::{find_snippets, Snippet, SnippetsQuery},
};

pub async fn get_settings() {}

pub async fn update_settings() {}

#[tauri::command]
pub async fn list_snippets(handle: AppHandle, query: SnippetsQuery)  {
    let session = handle.state::<Session>();
    // get_snippets(&session, &query).await
}
