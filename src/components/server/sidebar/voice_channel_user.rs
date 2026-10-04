use dioxus::prelude::*;

#[component]
pub fn VoiceChannelUser(public_key: String, display_name: String) -> Element {
    rsx! {
        div {
            class: "hover:bg-off-black-400 p-1 rounded-lg pl-7",
            "{display_name}"
        }
    }
}
