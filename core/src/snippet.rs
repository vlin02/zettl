use sqlx::prelude::FromRow;

use crate::{db, session::Session};

#[derive(Debug, serde::Deserialize)]
pub struct SnippetsQuery {
    pub search: String,
}

#[derive(serde::Serialize)]
pub struct Snippet {
    pub content: String,
    pub preview_html: String,
}

const PREVIEW_LINE_COUNT: i32 = 5;

fn escape_like_query(s: &str) -> String {
    let mut escaped = String::new();

    for c in s.chars() {
        if c == '\\' || c == '_' || c == '%' {
            escaped.push('\\');
        }

        escaped.push(c);
    }
    escaped
}

pub fn get_snippets(ctx: Session, pool: &db::Pool, query: SnippetsQuery) {
    #[derive(FromRow)]
    struct Row {
        content: String,
        format: String,
    }

    let SnippetsQuery { search } = query;
    let search = search.to_ascii_lowercase();

    let rows: Vec<Row> = sqlx::query_as(
        "
          SELECT snippet.content, format
          FROM snippet
          JOIN snippet_fts ON snippet.id = snippet_fts.rowid
          WHERE snippet_fts.content LIKE ? COLLATE NOCASE
    ",
    )
    .bind(format!("%{}%", escape_like_query(&search)))
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
            let scope = format_to_scope(format);

            let preview = preview_target_in_content(&content, &search, PREVIEW_LINE_COUNT);
            let syntax = syntax_set.find_syntax_by_scope(scope).unwrap();
            let theme = &theme_set.themes["base16-ocean.dark"];
            let preview_html = highlight_as_html(&syntax_set, &syntax, theme, &preview).unwrap();

            println!("{}", content);

            Snippet {
                content,
                preview_html,
            }
        })
        .collect()
}
