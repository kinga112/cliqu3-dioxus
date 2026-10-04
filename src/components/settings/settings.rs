use dioxus::prelude::*;

use crate::{
    components::settings::{
        SettingsNav,
        content::{Audio, UpdateProfile},
    },
    states::settings_states::{SETTINGS_CONTENT, SettingsContent},
};

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "flex w-full h-full ",
            SettingsNav{}
            match *SETTINGS_CONTENT.read() {
                SettingsContent::UpdateProfile => {rsx!{UpdateProfile{}}}
                SettingsContent::Audio => {rsx! {Audio{}}}
                SettingsContent::TestItem1 => {rsx! {div{"Test Item 1"}}}
                SettingsContent::TestItem2 => {rsx! {div{"Test Item 2"}}}
                SettingsContent::TestItem3 => {rsx! {div{"Test Item 3"}}}
            }
        }
    }
}
