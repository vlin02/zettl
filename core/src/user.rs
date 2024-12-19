use sqlx::FromRow;

use crate::{clipboard::Clipboard, db, theme::populate_default_themes};

const USER_ID: i32 = 1;

async fn is_initialized(pool: &db::Pool) -> bool {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM user LIMIT 1)")
        .fetch_one(pool)
        .await
        .unwrap();

    exists.0
}

pub async fn initialize(clipboard: &Clipboard) {
    let Clipboard { pool, .. } = clipboard;

    if is_initialized(&pool).await {
        return;
    }

    let theme_id = populate_default_themes(pool).await;

    sqlx::query("INSERT INTO snippet (id, popup_width, popup_height, popup_transparent, theme_id, crop_whitespace) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(USER_ID)
        .bind(400)
        .bind(800)
        .bind(true)
        .bind(theme_id)
        .bind(true)
        .execute(pool)
        .await
        .unwrap();
}

#[derive(FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub popup_width: i32,
    pub popup_height: i32,
    pub popup_transparent: bool,
    pub theme_id: i32,
    pub crop_whitespace: bool,
}

pub async fn get_user(pool: &db::Pool) -> User {
    sqlx::query_as(
        "SELECT popup_width, popup_height, popup_transparent, theme_id, crop_whitespace FROM user WHERE id = ?",
    )
    .bind(USER_ID)
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn update_user(pool: &db::Pool, user: User) {
    let User {
        popup_width,
        popup_height,
        popup_transparent,
        theme_id,
        crop_whitespace,
    } = user;

    sqlx::query("UPDATE user SET popup_width=? popup_height=? popup_transparent=? theme_id=?, crop_whitespace=? WHERE id = ?")
    .bind(popup_width)
    .bind(popup_height).bind(popup_transparent).bind(theme_id).bind(crop_whitespace).bind(USER_ID).execute(pool).await.unwrap();
}
