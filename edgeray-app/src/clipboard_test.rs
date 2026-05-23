use dioxus::prelude::*;

pub fn test_clip() {
    let mut cb = use_clipboard();
    cb.set("test".to_string());
}
