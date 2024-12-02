use objc::runtime::{Class, Object};
use objc_id::Id;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

// read the core [clipboard-change-count] which can be used as a state
pub fn clipboard_change_count() -> i64 {
    use objc::{msg_send, sel, sel_impl};

    let cls = match Class::get("NSPasteboard") {
        Some(cls) => cls,
        None => return -1,
    };
    let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };

    if pasteboard.is_null() {
        return -1;
    }

    let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };

    let change_count = unsafe { msg_send![pasteboard, changeCount] };
    change_count
}