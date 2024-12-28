pub enum Event {
    PopupInvalidated,
    HotkeysInvalidated,
}

impl Event {
    pub fn name(&self) -> String {
        let sfx = match &self {
            Event::PopupInvalidated => "popup-invalidated",
            Event::HotkeysInvalidated => "hotkeys-invalidated",
        };

        format!("zettl://{}", sfx)
    }
}

