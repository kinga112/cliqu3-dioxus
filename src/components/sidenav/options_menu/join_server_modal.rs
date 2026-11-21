use dioxus::prelude::*;
use xmtp::Recipient;

use crate::components::Modal;
// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

#[component]
pub fn JoinServerModal(open_modal: Signal<bool>) -> Element {
    let loading = use_signal(|| false);

    rsx! {
        Modal {
            open: open_modal,
            div {
                class: "font-light text-4xl",
                "Join New Server"
            }
            div {
                class: "flex gap-2",
                div {
                    "Join Server Stuff"
                }
            }
            div {
                class: "flex gap-2 pt-2",
                button {
                    class: "bg-deep-purple-400 text-deep-purple-100 p-2 w-32 rounded-md duration-100 border-2 border-deep-purple-400 hover:border-deep-purple-100",
                    onclick: move |_| join_server(),
                    "Join Server"
                }
                button {
                    class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
                    "Close"
                }
            }
        }
    }
}

async fn join_server() {}
