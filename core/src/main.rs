use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use detection::{infer_content_type, LookupTable};
use highlight::generate_html;
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;
use session::Session;
use syntect::parsing::SyntaxSet;
use tauri::Manager;
use tauri_plugin_sql::DbPool;

mod db;
mod detection;
mod highlight;
mod session;

const PASTEBOARD_POOL_MS: u64 = 500;

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

async fn start_monitoring(session: Session<'_>, copy_rx: Receiver<String>) {
    let Session { ort, lookup, syntax_set, pool } = session;

    for content in copy_rx {
        let content_type = infer_content_type(&ort, &lookup, &content);
        let html = generate_html(&syntax_set, &content, content_type);

        sqlx::query("INSERT INTO snippet (content, content_type, html) VALUES (?, ?, ?)")
            .bind(content)
            .bind(content_type.name())
            .bind(html)
            .execute(pool)
            .await
            .unwrap();
    }
}

fn main() -> Result<(), ort::Error> {
    let (copy_tx, copy_rx) = mpsc::channel::<String>();
    let (paste_tx, paste_rx) = mpsc::channel::<String>();

    listen_pasteboard(copy_tx, paste_rx);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(db::URL, db::list_migrations())
                .build(),
        )
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let instances = &*handle.try_state::<tauri_plugin_sql::DbInstances>().unwrap();
                let instances = instances.0.read().await;
                let DbPool::Sqlite(pool) = instances.get(db::URL).unwrap();

                let session = Session {
                    ort: ort::session::Session::builder()
                        .unwrap()
                        .commit_from_memory(include_bytes!("model.onnx"))
                        .unwrap(),
                    syntax_set: SyntaxSet::load_defaults_newlines(),
                    lookup: LookupTable::new(),
                    pool
                };
                
                start_monitoring(session, copy_rx);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();

    Ok(())
}
