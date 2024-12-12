use sqlx::prelude::FromRow;

use crate::{
    highlight::generate_html,
    session::{Context, Session},
};

pub async fn get_settings() {}

pub async fn update_settings() {}

#[derive(serde::Deserialize)]
pub struct ListSnippetsRequest {
    pub search: String,
}

#[derive(serde::Serialize)]
pub struct Snippet {
    pub content: String,
    pub html: String,
}

#[tauri::command]
pub async fn list_snippets(handle: tauri::AppHandle, request: ListSnippetsRequest) -> Vec<Snippet> {
    let session = Session::new(handle);
    let pool = session.pool();
    let Context {
        syntax_set, lookup, ..
    } = &*session.ctx();

    #[derive(FromRow)]
    struct Row {
        content: String,
        format: String,
    }

    let ListSnippetsRequest { search } = request;

    let rows: Vec<Row> = sqlx::query_as(
        "
            SELECT snippet.content, format
            FROM snippet
            JOIN snippet_fts ON snippet.id = snippet_fts.rowid
            WHERE snippet_fts.content LIKE ?
      ",
    )
    .bind(format!("%{search}%"))
    .fetch_all(&pool)
    .await
    .unwrap();

    rows.into_iter()
        .map(|row| {
            let Row {
                content,
                format: format_key,
            } = row;

            let format = lookup.format_by_key[&format_key];
            let html = generate_html(syntax_set, &content, format);

            Snippet { content, html }
        })
        .collect()
}
