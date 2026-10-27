use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum SettingsContent {
    UpdateProfile,
    Audio,
    Archives,
    TestItem1,
    TestItem2,
    TestItem3,
}

pub static SETTINGS_CONTENT: GlobalSignal<SettingsContent> =
    Signal::global(|| SettingsContent::UpdateProfile);
