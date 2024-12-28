use tauri::{AppHandle, Manager};

use crate::{db, profile, theme};

async fn is_initialized(pool: &db::Pool) -> bool {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM profile LIMIT 1)")
        .fetch_one(pool)
        .await
        .unwrap();

    exists.0
}

pub async fn initialize(handle: &AppHandle) {
    let pool = &*handle.state::<db::Pool>();

    if is_initialized(&pool).await {
        return;
    }

    let theme_id = theme::populate_defaults(pool).await;
    profile::initialize(pool, theme_id).await;
}

#[derive(serde::Serialize)]
pub struct Settings {
    pub popup_width: i32,
    pub popup_height: i32,
    pub popup_transparent: bool,
    pub crop_whitespace: bool,
    pub themes: Vec<theme::Listing>,
}

#[tauri::command]
pub async fn get_settings(handle: AppHandle) -> Settings {
    let pool = handle.state::<db::Pool>();

    let profile::Appearance {
        popup_height,
        popup_width,
        popup_transparent,
        crop_whitespace,
        ..
    } = profile::find_appearance(&pool).await;

    let themes = theme::list_all(&pool).await;

    return Settings {
        popup_height,
        popup_width,
        popup_transparent,
        crop_whitespace,
        themes,
    };
}
