use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use detection::infer_format;
use handler::list_snippets;
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;
use session::Session;
use snippet::{find_snippets, SnippetsQuery};
use sqlx::sqlite::SqlitePoolOptions;
use syntax::highlight_lines;
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use tauri::{async_runtime::block_on, generate_handler, window::Color, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

mod db;
mod detection;
mod handler;
mod lookup;
mod session;
mod snippet;
mod syntax;

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
    let Session {
        ort, lookup, pool, ..
    } = session;

    for content in copy_rx {
        let format = infer_format(ort, lookup, &content);

        sqlx::query("INSERT INTO snippet (content, format) VALUES (?, ?)")
            .bind(content)
            .bind(format.key())
            .execute(pool)
            .await
            .unwrap();
    }
}

fn main() -> Result<(), ort::Error> {
    let database_url = "sqlite:/Users/vilin/Library/Application Support/io.zettl.app/zettl.db";

    // Create a connection pool
    // let pool = block_on(
    //     SqlitePoolOptions::new()
    //         .max_connections(5) // Set the maximum number of connections
    //         .connect(database_url),
    // )
    // .unwrap();

    // let session = Session {
    //     ort: ort::session::Session::builder()
    //         .unwrap()
    //         .commit_from_memory(include_bytes!("model.onnx"))
    //         .unwrap(),
    // syntax_set: SyntaxSet::load_defaults_newlines(),
    // theme_set: ThemeSet::load_defaults(),
    //     lookup: lookup::Table::new(),
    //     pool,
    // };

    // let q = SnippetsQuery { search: String::from("copy the") };
    // block_on(get_snippets(&session, &q));
    // let elapsed = std::time::Instant::now();

    // block_on(get_snippets(&session, &q));
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("js").unwrap();

    let theme_set = ThemeSet::load_defaults();

    highlight_lines(
        &syntax_set,
        &syntax,
        &theme_set.themes["base16-ocean.dark"],
        "import { invoke } from \"@tauri-apps/api/core\"

export type Snippet = {
  content: string
  preview_html: string
}

export async function listSnippets(query: { search: string }): Promise<Snippet[]> {
  return invoke(\"list_snippets\", { query })
}",
    );

    // println!("Elapsed time: {:?}", elapsed.elapsed());

    // let (copy_tx, copy_rx) = channel::<String>();
    // let (paste_tx, paste_rx) = channel::<String>();

    // listen_pasteboard(copy_tx, paste_rx);

    // tauri::Builder::default()
    //     .plugin(
    //         tauri_plugin_sql::Builder::default()
    //             .add_migrations(db::URL, db::list_migrations())
    //             .build(),
    //     )
    //     .setup(|app| {
    //         let handle = app.handle().clone();
    //         let session = Session::new(&handle);
    //         app.manage(Session::new(&handle));

    //         handle.plugin(tauri_plugin_positioner::init())?;

    //         let popover_window = tauri::WebviewWindowBuilder::new(
    //             app,
    //             "popover",
    //             tauri::WebviewUrl::App("index.html".into()),
    //         )
    //         .inner_size(400.0, 800.0)
    //         .always_on_top(true)
    //         .background_color(Color(205, 254, 194, 1))
    //         .shadow(false)
    //         .transparent(true)
    //         .build()?;
    //         apply_vibrancy(&popover_window, NSVisualEffectMaterial::Popover, None, None)?;

    //         popover_window.show()?;

    //         let alt_p = Shortcut::new(Some(Modifiers::ALT), Code::KeyP);
    //         let esc = Shortcut::new(None, Code::Escape);

    //         handle.plugin(
    //             tauri_plugin_global_shortcut::Builder::new()
    //                 .with_handler(move |app, shortcut, event| {
    //                     if event.state != ShortcutState::Released {
    //                         return;
    //                     }

    //                     if shortcut == &alt_p {
    //                         let pos = app.cursor_position().unwrap();
    //                         popover_window.set_position(pos);
    //                         popover_window.show();
    //                         popover_window.set_focus();
    //                     }

    //                     // if shortcut == &esc {
    //                     //     if popover_window.is_visible().unwrap() {
    //                     //         popover_window.hide();
    //                     //     }
    //                     // }
    //                 })
    //                 .build(),
    //         )?;

    //         handle.global_shortcut().register(alt_p)?;
    //         handle.global_shortcut().register(esc)?;

    //         tauri::async_runtime::spawn(async move {
    //             start_monitoring(&session, copy_rx).await;
    //         });

    //         Ok(())
    //     })
    //     .invoke_handler(generate_handler![list_snippets])
    //     .on_window_event(|window, event| {
    //         if window.label() == "popover" {
    //             match event {
    //                 tauri::WindowEvent::Focused(is_focused) => {
    //                     if !is_focused {
    //                         // window.hide().unwrap();
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     })
    //     .run(tauri::generate_context!())
    //     .unwrap();

    Ok(())
}
