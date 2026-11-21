use dioxus::prelude::*;

use crate::{
    components::settings::{SettingsNavButton, settings::SettingsContent},
    state::app_state::AppState,
};

#[component]
pub fn SettingsNav() -> Element {
    rsx! {
        div {
            class: "flex flex-col p-2 w-56 bg-off-black-600 border-r-1 border-off-black-400 shrink-0 h-full gap-2 justify-between",
            div {
                class: "flex flex-col gap-2",
                SettingsNavButton{content: SettingsContent::UpdateProfile}
                SettingsNavButton{content: SettingsContent::Audio}
                SettingsNavButton{content: SettingsContent::Archives}
                SettingsNavButton{content: SettingsContent::TestItem1}
                SettingsNavButton{content: SettingsContent::TestItem2}
                SettingsNavButton{content: SettingsContent::TestItem3}
            }
            button {
                class: "text-2xl font-extralight hover:text-red-500 mb-5",
                onclick: move |_| logout(),
                "Logout"
            }
        }
    }
}

pub async fn logout() {
    let mut app_state = use_context::<AppState>();
    let logout_result = app_state.logout().await;
}
