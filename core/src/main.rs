use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use detection::infer_format;
use handler::list_snippets;
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;
use session::{Context, Session};
use sqlx::{
    prelude::FromRow,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use syntect::parsing::SyntaxSet;
use tauri::{async_runtime::block_on, generate_handler, Manager};

mod db;
mod detection;
mod handler;
mod highlight;
pub mod lookup;
mod session;

const PASTEBOARD_POOL_MS: u64 = 100;

fn listen_pasteboard(copy_tx: Sender<String>, paste_rx: Receiver<String>) {
    thread::spawn(move || {
        thread::scope(|s| {
            s.spawn(|| {
                let general = unsafe { NSPasteboard::generalPasteboard() };
                let mut cnt = unsafe { general.changeCount() };

                loop {
                    let new_cnt = unsafe { general.changeCount() };

                    if cnt != new_cnt {
                        cnt = new_cnt;

                        let content = unsafe { general.stringForType(NSStringPboardType) };

                        if let Some(content) = content {
                            copy_tx.send(content.to_string()).unwrap();
                        }
                    }

                    thread::sleep(Duration::from_millis(PASTEBOARD_POOL_MS));
                }
            });

            s.spawn(|| {
                let general = unsafe { NSPasteboard::generalPasteboard() };

                for content in paste_rx {
                    let content = NSString::from_str(&content);
                    unsafe { general.setString_forType(&content, NSStringPboardType) };
                }
            });
        });
    });
}

async fn start_monitoring(session: &Session, copy_rx: Receiver<String>) {
    let Context { ort, lookup, .. } = &*session.ctx();
    let pool = session.pool();

    for content in copy_rx {
        let format = infer_format(ort, lookup, &content);

        sqlx::query("INSERT INTO snippet (content, format) VALUES (?, ?)")
            .bind(content)
            .bind(format.key())
            .execute(&pool)
            .await
            .unwrap();
    }
}

fn main() -> Result<(), ort::Error> {
    let (copy_tx, copy_rx) = channel::<String>();
    let (paste_tx, paste_rx) = channel::<String>();

    listen_pasteboard(copy_tx, paste_rx);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(db::URL, db::list_migrations())
                .build(),
        )
        .invoke_handler(generate_handler![list_snippets])
        .run(tauri::generate_context!())
        .unwrap();

    Ok(())
}
