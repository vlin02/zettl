use sqlx::FromRow;

use crate::db;

const PROFILE_ID: i32 = 1;

pub async fn initialize(pool: &db::Pool, theme_id: i32) {
    sqlx::query(
        "
INSERT INTO profile (
  id,
  popup_width,
  popup_height,
  popup_transparent,
  theme_id,
  crop_whitespace
)
VALUES
  (?, ?, ?, ?, ?, ?)
        ",
    )
    .bind(PROFILE_ID)
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
pub struct Profile {
    pub popup_width: i32,
    pub popup_height: i32,
    pub popup_transparent: bool,
    pub crop_whitespace: bool,
    pub theme_id: i32,
}

pub async fn current(pool: &db::Pool) -> Profile {
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
