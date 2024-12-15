use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use crate::{db, handler::list_snippets, session::Session, snippet::insert_snippet};
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;

use tauri::{generate_handler, AppHandle, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

const PASTEBOARD_POLL_MS: u64 = 100;

fn monitor_pasteboard(pasteboard: &NSPasteboard, copy_tx: &Sender<String>) {
    let mut cnt = unsafe { pasteboard.changeCount() };

    loop {
        let new_cnt = unsafe { pasteboard.changeCount() };

        if cnt != new_cnt {
            cnt = new_cnt;

            let content = unsafe { pasteboard.stringForType(NSStringPboardType) };

            if let Some(content) = content {
                copy_tx.send(content.to_string()).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(PASTEBOARD_POLL_MS));
    }
}

fn paste_content(pasteboard: &NSPasteboard, content: &str) {
    let content = NSString::from_str(&content);
    unsafe { pasteboard.setString_forType(&content, NSStringPboardType) };
}

fn render_popup(handle: &AppHandle) -> WebviewWindow {
    let window = tauri::WebviewWindowBuilder::new(
        handle,
        "popover",
        tauri::WebviewUrl::App("src/popup/index.html".into()),
    )
    .title("Zettl")
    .inner_size(400.0, 800.0)
    .always_on_top(true)
    .shadow(false)
    .transparent(true)
    .visible(false)
    .build()
    .unwrap();

    apply_vibrancy(&window, NSVisualEffectMaterial::Popover, None, None).unwrap();

    window
}

fn render_settings(handle: &AppHandle) -> WebviewWindow {
    tauri::WebviewWindowBuilder::new(
        handle,
        "settings",
        tauri::WebviewUrl::App("src/settings/index.html".into()),
    )
    .inner_size(600.0, 600.0)
    .visible(false)
    .build()
    .unwrap()
}

pub fn start() {
    let (copy_tx, copy_rx) = channel::<String>();
    let (paste_tx, paste_rx) = channel::<String>();

    thread::spawn(|| {
        thread::scope(|s| {
            s.spawn(move || {
                let general = unsafe { NSPasteboard::generalPasteboard() };
                monitor_pasteboard(&*general, &copy_tx);
            });

            s.spawn(|| {
                let general = unsafe { NSPasteboard::generalPasteboard() };

                for content in paste_rx {
                    paste_content(&general, &content);
                }
            });
        });
    });

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(db::URL, db::list_migrations())
                .build(),
        )
        .setup(|app| {
            let base_handle = app.handle();

            let handle = base_handle.clone();
            handle.manage(Session::from_handle(&handle));
            handle.plugin(tauri_plugin_positioner::init())?;

            let alt_p = Shortcut::new(Some(Modifiers::ALT), Code::KeyP);
            let esc = Shortcut::new(None, Code::Escape);

            let popup = render_popup(&handle);
            // let settings = render_settings(&handle);

            handle.plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |handle, shortcut, event| {
                        if event.state != ShortcutState::Released {
                            return;
                        }

                        if shortcut == &alt_p {
                            let popup = popup.clone();
                            let pos = handle.cursor_position().unwrap();

                            thread::spawn(move || {
                                popup.set_position(pos).unwrap();
                                thread::sleep(Duration::from_millis(PASTEBOARD_POLL_MS));
                                popup.show().unwrap();
                                popup.set_focus().unwrap();
                            });
                        }

                        // if shortcut == &esc {
                        //     if popover_window.is_visible().unwrap() {
                        //         popover_window.hide();
                        //     }
                        // }
                    })
                    .build(),
            )?;

            handle.global_shortcut().register(alt_p)?;
            handle.global_shortcut().register(esc)?;

            // let handle = base_handle.clone();
            tauri::async_runtime::spawn(async move {
                let session = &*handle.state::<Session>();

                for content in copy_rx {
                    insert_snippet(&session, &content).await;
                }
            });

            Ok(())
        })
        .invoke_handler(generate_handler![list_snippets])
        .on_window_event(|window, event| {
            if window.label() == "popover" {
                match event {
                    tauri::WindowEvent::Focused(is_focused) => {
                        if !is_focused {
                            // window.hide().unwrap()
                        }
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .unwrap();
}
