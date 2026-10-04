use dioxus::prelude::*;

use crate::states::settings_states::{SETTINGS_CONTENT, SettingsContent};

const BUTTON_STYLE: &'static str = "h-10 w-full rounded-lg shrink";

#[component]
pub fn SettingsNavButton(content: SettingsContent) -> Element {
    let mut active = false;
    if *SETTINGS_CONTENT.read() == content {
        active = true;
    }

    let name;
    match content.clone() {
        SettingsContent::UpdateProfile => name = "Update Profile".to_string(),
        SettingsContent::Audio => name = "Audio".to_string(),
        SettingsContent::TestItem1 => name = "Test Item 1".to_string(),
        SettingsContent::TestItem2 => name = "Test Item 2".to_string(),
        SettingsContent::TestItem3 => name = "Test Item 3".to_string(),
    }

    rsx! {
        button {
            class: if !active {"{BUTTON_STYLE} hover:bg-off-black-400"},
            class: if active {"{BUTTON_STYLE} bg-deep-purple-300 cursor-default"},
            onclick: move |_| *SETTINGS_CONTENT.write() = content.clone(),
            "{name}"
        }
    }
}
