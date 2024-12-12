use sqlx::prelude::FromRow;

use crate::session::{Context, Session};

pub async fn get_settings() {

}

pub async fn update_settings() {

}

#[derive(serde::Serialize)]
pub struct ListSnippetsRequest {
  pub cursor: Option<i32>,
  pub search: String,
}

#[tauri::command]
pub async fn list_snippets(handle: tauri::AppHandle, request: ListSnippetsRequest) {
  let session = Session::new(handle);
  let pool = session.pool();
  let ctx = &*session.ctx();

  #[derive(FromRow)]
  struct Row {
      content: String,
      content_type: String
  }

  let rows: Vec<Row> = sqlx::query_as::<_, Row>(
      "
          SELECT content, content_type
          FROM snippet
          JOIN snippet_fts ON snippet.id = snippet_fts.rowid
          WHERE pb_item_fts.rowid > ?
          LIMIT 50
      ",
  )
  .bind(filter.cursor.unwrap_or(0))
  .bind(&filter.search)
  .fetch_all(self.pool)
  .await
  .unwrap();


rows.into_iter()
.map(|row| {
    let Row { content, content_type } = row;
    Snippet { content, content_type };
  ctx.lookup.content_type_by_key[]
      })
      .collect()
}

pub async fn paste_content() {
  
}
