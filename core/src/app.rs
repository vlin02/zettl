use crate::{
    clipboard::Clipboard,
    db,
    event::Event,
    pasteboard::Pasteboard,
    settings, shortcuts,
    snippet::{insert_snippet, list_snippets},
    window::Window,
};

use tauri::{
    async_runtime::{self, block_on},
    generate_handler,
    plugin::Plugin,
    AppHandle, Emitter, Listener, Manager,
};
use tauri_plugin_sql::DbPool;

async fn acquire_pool(handle: &AppHandle) -> db::Pool {
    let instances = &*handle.state::<tauri_plugin_sql::DbInstances>();
    let instances = instances.0.read().await;

    let DbPool::Sqlite(pool) = instances.get(db::URL).unwrap();

    pool.clone()
}

pub fn start() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(db::URL, db::list_migrations())
                .build(),
        )
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let app = app.handle().clone();

            let (pasteboard, copy_rx) = Pasteboard::new();
            let pool = block_on(acquire_pool(&app));
            let clipboard = Clipboard::new(&pool);

            app.manage(clipboard);
            app.manage(pasteboard);
            app.manage(pool);

            async_runtime::block_on(settings::initialize(&app));

            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                let clipboard = &*handle.state::<Clipboard>();
                for content in copy_rx {
                    insert_snippet(&clipboard, &content).await;

                    handle.emit(&Event::PopupInvalidated.name(), 0).unwrap()
                }
            });

            let handle = app.clone();

            let mut shortcuts_plugin = async_runtime::block_on(shortcuts::build_plugin(&handle));
            app.plugin(shortcuts_plugin);

            // app.listen(Event::HotkeysInvalidated.name(), move |_| {
            //     app.remove_plugin(shortcuts_plugin.name());
            //     shortcuts_plugin = async_runtime::block_on(shortcuts::build_plugin(&handle));
            //     app.plugin(shortcuts_plugin);
            // });

            Ok(())
        })
        .invoke_handler(generate_handler![list_snippets])
        .on_window_event(|window, event| {
            if window.label() == Window::POPUP.label() {
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
