use std::{thread, time::Duration};

use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

pub enum Window {
    POPUP,
    SETTINGS,
}

impl Window {
    pub fn label(&self) -> &str {
        match self {
            Window::POPUP => "popup",
            Window::SETTINGS => "settings",
        }
    }

    pub fn get(&self, handle: &AppHandle) -> Option<WebviewWindow> {
        handle.get_webview_window(self.label())
    }
}

const TRANSITION_DELAY_MS: u64 = 10;

pub fn render_popup(handle: AppHandle) {
    let window = if let Some(window) = Window::POPUP.get(&handle) {
        window
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            &handle,
            "popup",
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
    };

    thread::spawn(move || {
        let pos = handle.cursor_position().unwrap();

        window.set_position(pos).unwrap();
        thread::sleep(Duration::from_millis(TRANSITION_DELAY_MS));
        window.show().unwrap();
        window.set_focus().unwrap();
    });
}

pub fn render_settings(handle: AppHandle) {
    if let Some(window) = Window::SETTINGS.get(&handle) {
        window.set_focus().unwrap();
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            &handle,
            "settings",
            tauri::WebviewUrl::App("src/settings/index.html".into()),
        )
        .inner_size(600.0, 600.0)
        .visible(false)
        .transparent(true)
        .focused(true)
        .build()
        .unwrap();

        apply_vibrancy(
            &window,
            NSVisualEffectMaterial::ContentBackground,
            None,
            None,
        )
        .unwrap();

        window.move_window(Position::Center).unwrap();
    }
}
