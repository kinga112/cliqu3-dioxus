use crate::components::login::WalletConnectButton;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-screen w-screen overflow-hidden bg-off-black-300 justify-center place-items-center",
            div {
                class: "flex flex-col relative gap-5 place-items-center pt-24 bg-deep-purple-300 rounded-4xl overflow-hidden w-1/2 h-2/3",
                div {
                    class: "text-6xl font-thin text-deep-purple-100 font-neuropol",
                    "C L I Q U 3"
                },
                WalletConnectButton {  }
            }
        }
    }
}
