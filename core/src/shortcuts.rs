use tauri::{plugin::TauriPlugin, AppHandle, Manager, Wry};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use crate::{db, profile::{self, Hotkeys}, window::{self, Window}};

pub async fn build_plugin(handle: &AppHandle) -> TauriPlugin<Wry> {
  let pool = &*handle.state::<db::Pool>();

  let meta_comma = Shortcut::new(Some(Modifiers::META), Code::Comma);
  let Hotkeys { show_popup } = profile::find_hotkeys(pool).await;

  let builder = tauri_plugin_global_shortcut::Builder::new();

  builder
      .with_shortcuts(vec![meta_comma, show_popup])
      .unwrap()
      .with_handler(move |handle, shortcut, event| {
          if event.state != ShortcutState::Released {
              return;
          }

          if shortcut == &show_popup {
              window::render_popup(handle.clone());
          }

          if shortcut == &meta_comma {
              if let Some(popup) = handle.get_webview_window(Window::POPUP.label()) {
                  if popup.is_focused().unwrap() {
                      window::render_settings(handle.clone());
                  }
              }
          }
      })
      .build()
}