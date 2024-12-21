use tauri::{AppHandle, Emitter};

pub enum Event {
    ThemeUpdated,
    Copied,
}

impl Event {
    fn name(&self) -> String {
        let sfx = match &self {
            Event::ThemeUpdated => "theme-updated",
            Event::Copied => "copied",
        };

        format!("zettl://{}", sfx)
    }
}

fn fmt_event_name(name: &str) -> String {
    format!("zettl://{}", name)
}

pub struct Dispatcher {
    handle: AppHandle,
}

impl Dispatcher {
    pub fn new(handle: &AppHandle) -> Dispatcher {
        Dispatcher {
            handle: handle.clone()
        }
    }

    pub fn emit_theme_updated(&self, id: i32) {
        self.handle.emit(&Event::ThemeUpdated.name(), id).unwrap();
    }

    pub fn emit_copied(&self, content: &str) {
      self.handle.emit(&Event::Copied.name(), content).unwrap();
  }
}
