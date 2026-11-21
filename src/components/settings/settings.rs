use dioxus::prelude::*;

use crate::components::settings::{
    SettingsNav,
    content::{Archives, Audio, UpdateProfile},
};

#[derive(Clone, PartialEq, Debug)]
pub enum SettingsContent {
    UpdateProfile,
    Audio,
    Archives,
    TestItem1,
    TestItem2,
    TestItem3,
}

#[component]
pub fn Settings() -> Element {
    let settings_content = use_signal(|| SettingsContent::UpdateProfile);
    let settings_content_state = use_context_provider(|| settings_content);
    rsx! {
        div {
            class: "flex w-full h-full ",
            SettingsNav{}
            match *settings_content_state.read() {
                SettingsContent::UpdateProfile => {rsx!{UpdateProfile{}}}
                SettingsContent::Audio => {rsx! {Audio{}}}
                SettingsContent::Archives => {rsx! {Archives{}}}
                SettingsContent::TestItem1 => {rsx! {div{"Test Item 1"}}}
                SettingsContent::TestItem2 => {rsx! {div{"Test Item 2"}}}
                SettingsContent::TestItem3 => {rsx! {div{"Test Item 3"}}}
            }
        }
    }
}
