use std::f32::MAX_10_EXP;

use crate::{
    detection::{format::Format, infer_format},
    session::Session,
    syntax::{format_to_scope, highlight_lines},
};

use sqlx::prelude::FromRow;

pub async fn insert_snippet(session: &Session, content: &str) {
    let Session {
        ort,
        lookup,
        pool,
        syntax_set,
        theme_set,
        ..
    } = session;

    let format = infer_format(ort, lookup, &content);
    let scope = format_to_scope(format);
    let syntax = syntax_set.find_syntax_by_scope(scope).unwrap();
    let theme = &theme_set.themes["base16-ocean.dark"];

    let lines = highlight_lines(syntax_set, syntax, theme, content).unwrap();
    let lines = &serde_json::to_string(&lines).unwrap();

    sqlx::query("INSERT INTO snippet (content, format, lines) VALUES (?, ?, jsonb(?))")
        .bind(content)
        .bind(format.key())
        .bind(lines)
        .execute(pool)
        .await
        .unwrap();
}

const PREVIEW_LINE_COUNT: usize = 5;

#[derive(serde::Deserialize)]
pub struct SnippetsQuery {
    pub search: String,
    pub next_id: Option<i32>,
    pub limit: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct Snippet {
    pub id: i32,
    pub format: Format,
    pub preview_html: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SnippetPage {
    pub snippets: Vec<Snippet>,
    pub next_id: Option<i32>,
}

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

fn find_target_line_index(input: &str, target: &str) -> usize {
    for (i, line) in input.lines().enumerate() {
        if line.to_ascii_lowercase().contains(target) {
            return i;
        }
    }

    panic!()
}

pub async fn find_snippets(session: &Session, query: &SnippetsQuery) -> SnippetPage {
    #[derive(FromRow)]
    struct Row {
        id: i32,
        content: String,
        format: String,
        lines: String,
    }

    let Session { pool, lookup, .. } = session;

    let SnippetsQuery {
        search,
        next_id,
        limit,
    } = query;
    let search = search.to_ascii_lowercase();

    let rows: Vec<Row> = sqlx::query_as(
        "
SELECT
  snippet.id,
  snippet.content,
  snippet.format,
  json (snippet.lines) as lines
FROM
  snippet
  JOIN snippet_fts ON snippet.id = snippet_fts.rowid
WHERE
  (
    ?
    OR snippet_fts.rowid <= ?
  )
  AND snippet_fts.content LIKE ? COLLATE NOCASE ESCAPE '\\'
ORDER BY
  snippet.id DESC
LIMIT
  ?
        ",
    )
    .bind(next_id.is_none())
    .bind(next_id.unwrap_or(0))
    .bind(format!("%{}%", escape_like_query(&search)))
    .bind(limit + 1)
    .fetch_all(pool)
    .await
    .unwrap();

    let limit = *limit as usize;
    let next_id = rows.get(limit).map(|x| x.id);

    let snippets: Vec<Snippet> = rows[..limit.min(rows.len())]
        .iter()
        .map(|row| {
            let Row {
                id,
                content,
                format: format_key,
                lines,
            } = row;
            let lines: Vec<String> = serde_json::from_str(lines).unwrap();

            let target_i = find_target_line_index(&content, &search);

            Snippet {
                id: *id,
                format: lookup.format_by_key[format_key],
                preview_html: lines[target_i..(lines.len().min(target_i + PREVIEW_LINE_COUNT))]
                    .join(""),
            }
        })
        .collect();

    return SnippetPage { next_id, snippets };
}
