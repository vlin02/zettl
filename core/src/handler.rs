use tauri::{AppHandle, Manager};

use crate::{
    app::PasteTx,
    clipboard::Clipboard,
    snippet::{find_snippets, get_content, SnippetPage, SnippetsQuery},
    theme::{self, add_theme, load_theme_styles, ThemeBuilder, ThemeListing, ThemePreview},
    user::{self, User},
};

#[tauri::command]
pub async fn get_user(handle: AppHandle) -> User {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    user::get_user(pool).await
}

#[tauri::command]
pub async fn update_user(handle: AppHandle, user: User) {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    user::update_user(pool, user).await;
}

#[tauri::command]
pub async fn load_current_theme(handle: AppHandle) -> String {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();

    let settings = user::get_user(pool).await;
    load_theme_styles(&pool, settings.theme_id).await
}

#[tauri::command]
pub async fn list_themes(handle: AppHandle) -> Vec<ThemeListing> {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();

    theme::list_themes(pool).await
}

#[tauri::command]
pub fn preview_theme(tm_plist: String) -> Option<ThemePreview> {
    theme::preview_theme(&tm_plist)
}

#[tauri::command]
pub async fn import_theme(handle: AppHandle, name: String, tm_plist: String) -> i32 {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    let builder = ThemeBuilder { name, tm_plist };

    add_theme(pool, &builder).await
}

#[tauri::command]
pub async fn delete_theme(handle: AppHandle, id: i32) {
    let Clipboard { pool, .. } = &*handle.state::<Clipboard>();
    theme::delete_theme(pool, id).await
}

#[tauri::command]
pub async fn list_snippets(handle: AppHandle, query: SnippetsQuery) -> SnippetPage {
    let clipboard = handle.state::<Clipboard>();

    find_snippets(&clipboard, &query).await
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
