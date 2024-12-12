use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use detection::infer_content_type;
use handler::list_snippets;
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;
use session::{Context, Session};
use syntect::parsing::SyntaxSet;
use tauri::{generate_handler, Manager};

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
        let content_type = infer_content_type(ort, lookup, &content);

        sqlx::query("INSERT INTO snippet (content, content_type) VALUES (?, ?)")
            .bind(content)
            .bind(content_type.key())
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
        .setup(|app| {
            
            app.manage(Context {
                ort: ort::session::Session::builder()
                    .unwrap()
                    .commit_from_memory(include_bytes!("model.onnx"))
                    .unwrap(),
                syntax_set: SyntaxSet::load_defaults_newlines(),
                lookup: lookup::Table::new(),
                paste_tx,
            });

            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let session = Session::new(handle);

                start_monitoring(&session, copy_rx).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();

    Ok(())
}
