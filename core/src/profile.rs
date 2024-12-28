use sqlx::FromRow;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

use crate::db;

const PROFILE_ID: i32 = 1;

pub async fn initialize(pool: &db::Pool, theme_id: i32) {
    let alt_p = Shortcut::new(Some(Modifiers::ALT), Code::KeyP);

    sqlx::query(
        "
        INSERT INTO profile (
          id,
          popup_width,
          popup_height,
          popup_transparent,
          theme_id,
          crop_whitespace,
          show_popup_shortcut
        )
        VALUES
          (?, ?, ?, ?, ?, ?, ?)
        ",
    )
    .bind(PROFILE_ID)
    .bind(400)
    .bind(800)
    .bind(true)
    .bind(theme_id)
    .bind(true)
    .bind(serde_json::to_string(&alt_p).unwrap())
    .execute(pool)
    .await
    .unwrap();
}

#[derive(FromRow, serde::Serialize, serde::Deserialize)]
pub struct Appearance {
    pub popup_width: i32,
    pub popup_height: i32,
    pub popup_transparent: bool,
    pub crop_whitespace: bool,
    pub theme_id: i32,
}

pub async fn find_appearance(pool: &db::Pool) -> Appearance {
    sqlx::query_as(
        "
        SELECT
          popup_width,
          popup_height,
          popup_transparent,
          theme_id,
          crop_whitespace
        FROM
          profile
        WHERE
          id = ?
        ",
    )
    .bind(PROFILE_ID)
    .fetch_one(pool)
    .await
    .unwrap()
}

pub struct Hotkeys {
    pub show_popup: Shortcut,
}

pub async fn find_hotkeys(pool: &db::Pool) -> Hotkeys {
    #[derive(FromRow)]
    struct Row {
        show_popup_shortcut: Vec<u8>,
    }

    let Row {
        show_popup_shortcut: show_popup,
    } = sqlx::query_as(
        "
        SELECT
        show_popup_shortcut
        FROM
        profile
        WHERE
        id = ?
        ",
    )
    .bind(PROFILE_ID)
    .fetch_one(pool)
    .await
    .unwrap();

    Hotkeys {
        show_popup: serde_json::from_slice(&show_popup).unwrap(),
    }
}

enum Hotkey {
    ShowPopup,
}

pub async fn set_hotkey(pool: &db::Pool, hotkey: &Hotkey, shortcut: &Shortcut) {
    let col = match hotkey {
        Hotkey::ShowPopup => "show_popup_shortcut",
    };

    sqlx::query_as(
        "
        UPDATE profile
        SET ? = ?
        WHERE id = ?
        ",
    )
    .bind(col)
    .bind(serde_json::to_string(shortcut).unwrap())
    .bind(PROFILE_ID)
    .fetch_one(pool)
    .await
    .unwrap()
}
