use std::{io::Cursor, str::FromStr};

use crate::db;
use sqlx::FromRow;
use syntect::{
    dumps::{dump_binary, from_binary},
    highlighting::{Color, Theme, ThemeItem, ThemeSet},
    html::{css_for_theme_with_class_style, ClassStyle},
    parsing::{MatchPower, ScopeStack},
};

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

pub type ThemePreview = Vec<Color>;

pub fn preview_theme(tm_plist: &str) -> Option<ThemePreview> {
    let mut cursor = Cursor::new(tm_plist.as_bytes());

    if let Ok(theme) = ThemeSet::load_from_reader(&mut cursor) {
        return Some(get_preview_colors(&theme));
    }

    None
}

const DEFAULT_THEMES: [(&str, &str); 7] = [
    ("one-dark", "One Dark"),
    ("dracula", "Dracula"),
    ("github-dark", "Github Dark"),
    ("gruvbox", "Gruvbox"),
    ("monokai", "Monokai"),
    ("solarized-dark", "Solarized Dark"),
    ("solarized-light", "Solarized Light"),
];

pub async fn populate_default_themes(pool: &db::Pool) -> i32 {
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

pub async fn load_theme_styles(pool: &db::Pool, id: i32) -> String {
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
pub struct ThemeListing {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
    pub preview_colors: ThemePreview,
}

pub async fn list_themes(pool: &db::Pool) -> Vec<ThemeListing> {
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

    rows.into_iter()
        .map(|row| {
            let Row {
                id,
                name,
                is_default,
                dump,
            } = row;

            let theme: Theme = from_binary(&dump);

            ThemeListing {
                id,
                name,
                is_default,
                preview_colors: get_preview_colors(&theme),
            }
        })
        .collect()
}

#[derive(serde::Deserialize)]
pub struct ThemeBuilder {
    pub name: String,
    pub tm_plist: String,
}

pub async fn add_theme(pool: &db::Pool, builder: &ThemeBuilder) -> i32 {
    let ThemeBuilder { name, tm_plist } = builder;

    let mut cursor = Cursor::new(tm_plist.as_bytes());
    let theme = ThemeSet::load_from_reader(&mut cursor).unwrap();

    #[derive(FromRow)]
    struct Row {
        id: i32,
    }

    let Row { id } = sqlx::query_as(
        "
INSERT INTO
  theme (name, dump, is_default)
VALUES
  (?, ?, ?, ?) RETURNING id
      ",
    )
    .bind(name)
    .bind(dump_binary(&theme))
    .bind(false)
    .fetch_one(pool)
    .await
    .unwrap();

    id
}

pub async fn delete_theme(pool: &db::Pool, id: i32) {
    sqlx::query("DELETE FROM theme WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
