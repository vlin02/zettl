use std::{sync::mpsc, thread, time};

use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use objc2_foundation::NSString;

pub struct Pasteboard {
    paste_tx: mpsc::Sender<String>,
}

const PASTEBOARD_POLL_MS: u64 = 100;

impl Pasteboard {
    pub fn new() -> (Pasteboard, mpsc::Receiver<String>) {
        let (copy_tx, copy_rx) = mpsc::channel::<String>();
        let (paste_tx, paste_rx) = mpsc::channel::<String>();

        thread::spawn(move || {
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

                thread::sleep(time::Duration::from_millis(PASTEBOARD_POLL_MS));
            }
        });

        thread::spawn(|| {
            let general = unsafe { NSPasteboard::generalPasteboard() };

            for content in paste_rx {
                let content = NSString::from_str(&content);
                unsafe {
                    general.clearContents();
                    general.setString_forType(&content, NSStringPboardType);
                };
            }
        });

        (Pasteboard { paste_tx }, copy_rx)
    }

    pub fn copy(&self, s: &str) {
        self.paste_tx.send(s.to_string()).unwrap()
    }
}
