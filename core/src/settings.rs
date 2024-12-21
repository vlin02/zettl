use tauri::{AppHandle, Manager};

use crate::{db, profile, theme};

async fn is_initialized(pool: &db::Pool) -> bool {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM profile LIMIT 1)")
        .fetch_one(pool)
        .await
        .unwrap();

    exists.0
}

#[derive(serde::Serialize)]
pub struct Settings {
    pub popup_width: i32,
    pub popup_height: i32,
    pub popup_transparent: bool,
    pub crop_whitespace: bool,
    pub themes: Vec<theme::Listing>,
}

pub async fn get_settings(pool: &db::Pool) -> Settings {
    let profile::Profile {
        popup_height,
        popup_width,
        popup_transparent,
        crop_whitespace,
        ..
    } = profile::current(pool).await;

    let themes = theme::list_all(pool).await;

    return Settings {
        popup_height,
        popup_width,
        popup_transparent,
        crop_whitespace,
        themes,
    };
}

pub async fn initialize(handle: &AppHandle) {
    let pool = &*handle.state::<db::Pool>();
    if is_initialized(&pool).await {
        return;
    }

    let theme_id = theme::populate_defaults(pool).await;
    profile::initialize(pool, theme_id).await;
}

pub async fn set_active_theme(pool: &db::Pool, id: i32) {
    sqlx::query("UPDATE FROM theme WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

#[tauri::command]
pub async fn delete_theme(id: i32) {
    

    sqlx::query("DELETE FROM theme WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
