use dioxus::prelude::*;

use crate::components::server::channel::message::content::BasicContent;

#[component]
pub fn AttachmentContent(filename: Option<String>, mime_type: String, data: Vec<u8>) -> Element {
    let name = filename.unwrap_or_else(|| "File: {mime_type}".to_string());
    rsx! {
        BasicContent {
            button {
                class: "flex gap-2 p-2 bg-red-500",
                "{name}"
            }
        }
    }
}
