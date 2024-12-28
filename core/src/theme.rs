use std::{io::Cursor, str::FromStr};

use crate::{
    db,
    profile::{self, Appearance},
};
use sqlx::FromRow;
use syntect::{
    dumps::{dump_binary, from_binary},
    highlighting::{Color, Theme, ThemeItem, ThemeSet},
    html::{css_for_theme_with_class_style, ClassStyle},
    parsing::{MatchPower, ScopeStack},
    LoadingError,
};
use tauri::{AppHandle, Manager};

const PREVIEW_SCOPES: [&str; 5] = [
    "keyword.control.conditional",
    "entity.name.class",
    "variable.parameter",
    "constant.numeric.integer.binary",
    "string.quoted.double",
];

fn get_preview_colors(theme: &Theme) -> Vec<Color> {
    let mut colors = Vec::new();

    let Theme {
        scopes, settings, ..
    } = theme;

    for name in PREVIEW_SCOPES {
        let stack = ScopeStack::from_str(name).unwrap();

        let mut max_power: f64 = 0.0;
        let mut best_color: Option<Color> = None;

        for item in scopes {
            let ThemeItem { style, scope } = item;
            if let Some(MatchPower(power)) = scope.does_match(stack.as_slice()) {
                if let Some(color) = style.foreground {
                    if max_power < power {
                        max_power = power;
                        best_color = Some(color);
                    }
                }
            }
        }

        if let Some(color) = best_color {
            colors.push(color);
        }
    }

    if let Some(color) = settings.foreground {
        colors.push(color);
    }

    colors
}

pub type Preview = Vec<Color>;

const DEFAULT_THEMES: [(&str, &str); 7] = [
    ("one-dark", "One Dark"),
    ("dracula", "Dracula"),
    ("github-dark", "Github Dark"),
    ("gruvbox", "Gruvbox"),
    ("monokai", "Monokai"),
    ("solarized-dark", "Solarized Dark"),
    ("solarized-light", "Solarized Light"),
];

pub async fn populate_defaults(pool: &db::Pool) -> i32 {
    let ThemeSet { themes } = from_binary(include_bytes!("./data/default.themedump"));

    #[derive(FromRow)]
    struct Row {
        id: i32,
    }

    let mut default_id: Option<i32> = None;

    for (key, name) in DEFAULT_THEMES {
        let theme = dump_binary(&themes[key]);

        let Row { id } = sqlx::query_as(
            "
INSERT INTO
  theme (name, dump, is_default)
VALUES
  (?, ?, ?) RETURNING id
            ",
        )
        .bind(name)
        .bind(theme)
        .bind(true)
        .fetch_one(pool)
        .await
        .unwrap();

        if default_id == None {
            default_id = Some(id);
        }
    }

    default_id.unwrap()
}

#[tauri::command]
pub async fn load_active_theme(handle: AppHandle, id: i32) -> String {
    let pool = &*handle.state::<db::Pool>();

    #[derive(FromRow)]
    struct Row {
        dump: Vec<u8>,
    }

    let Row { dump } = sqlx::query_as("SELECT dump FROM theme WHERE theme.id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();

    let theme: Theme = from_binary(&dump);
    css_for_theme_with_class_style(&theme, ClassStyle::Spaced).unwrap()
}

#[derive(serde::Serialize)]
pub struct Listing {
    pub id: i32,
    pub active: bool,
    pub name: String,
    pub can_delete: bool,
    pub preview: Preview,
}

pub async fn list_all(pool: &db::Pool) -> Vec<Listing> {
    #[derive(FromRow)]
    struct Row {
        id: i32,
        name: String,
        is_default: bool,
        dump: Vec<u8>,
    }

    let rows: Vec<Row> = sqlx::query_as("SELECT id, name, dump, is_default FROM theme")
        .fetch_all(pool)
        .await
        .unwrap();

    let Appearance { theme_id, .. } = profile::find_appearance(pool).await;

    rows.into_iter()
        .map(|row| {
            let Row {
                id,
                name,
                is_default,
                dump,
            } = row;

            let theme: Theme = from_binary(&dump);

            Listing {
                id,
                active: theme_id == id,
                name,
                can_delete: !is_default && theme_id != id,
                preview: get_preview_colors(&theme),
            }
        })
        .collect()
}

#[derive(serde::Deserialize)]
pub struct File {
    pub name: String,
    pub text: String,
}

const TM_THEME_EXT: &str = ".tmtheme";

#[tauri::command]
pub async fn import_theme(handle: AppHandle, file: File) -> Result<i32, String> {
    let pool = &*handle.state::<db::Pool>();

    let File { name, text } = file;

    let parts: Vec<&str> = name.split(".").collect();
    let valid = parts.len() == 2 && parts[1].to_ascii_lowercase() == "tmtheme";

    if !valid {
        return Err(String::from("File does not end in .tmtheme"));
    }

    let name = parts[0];

    let mut cursor = Cursor::new(text.as_bytes());
    let theme = ThemeSet::load_from_reader(&mut cursor);

    if let Err(LoadingError::ReadSettings(_)) = theme {
        return Err(String::from("Invalid syntax for .tmtheme"));
    }

    let theme = theme.unwrap();

    #[derive(FromRow)]
    struct Row {
        id: i32,
    }

    let Row { id } = sqlx::query_as(
        "
INSERT INTO
  theme (name, dump, is_default)
VALUES
  (?, ?, ?) RETURNING id
      ",
    )
    .bind(name)
    .bind(dump_binary(&theme))
    .bind(false)
    .fetch_one(pool)
    .await
    .unwrap();

    Ok(id)
}

#[tauri::command]
pub async fn set_active(handle: AppHandle, id: i32) {
    let pool = &*handle.state::<db::Pool>();

    sqlx::query("UPDATE FROM theme WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

#[tauri::command]
pub async fn delete(handle: AppHandle, id: i32) {
    let pool = &*handle.state::<db::Pool>();

    sqlx::query("DELETE FROM theme WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
