use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    components::Modal,
    modules::docs::db::Server,
    states::{
        global_states::OPEN_CREATE_SERVER_MODAL,
        server_states::{CURRENT_SERVER, OPEN_ADD_MEMBER_MODAL, SERVER_LIST},
        user_states::USER,
    },
};
// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

const TOGGLE: &'static str = "group peer bg-deep-purple-100 rounded-full
        duration-300 w-8 h-4 ring-1 ring-off-black-100 after:duration-300
        after:bg-off-black-100 peer-checked:after:bg-green-500
        peer-checked:ring-green-500 after:rounded-full after:absolute after:h-3
        after:w-3 after:top-1.5 after:left-0.5 after:flex after:justify-center
        after:items-center peer-checked:after:translate-x-4 peer-hover:after:scale-95";

#[component]
pub fn AddMemberModal() -> Element {
    // let server = use_context::<Server>();
    let server = CURRENT_SERVER.read().clone().expect("no current server??");
    let mut address_text = use_signal(|| String::new());
    let loading = use_signal(|| false);

    let text_channels_list = server.text_channels.into_iter().map(|metadata| {
        rsx! {
            div {
                class: "flex justify-between p-2 border-b-1 border-deep-purple-200",
                div {
                    "{metadata.name}"
                }
                label {
                    class:"relative inline-flex items-center cursor-pointer",
                    input {
                        type:"checkbox",
                        class: "sr-only peer",
                        value:"",
                    }
                    div {
                        class: TOGGLE
                    }
                }
            }
        }
    });

    rsx! {
        Modal {
            open: OPEN_ADD_MEMBER_MODAL.signal(),
            div {
                class: "space-y-1",
                div {
                    class: "font-light text-4xl",
                    "Add New Member"
                }
                div {
                    class: "flex gap-2",
                    div {
                        class: "flex flex-col gap-1",
                        input {
                            class: "bg-deep-purple-400 p-2 rounded-md",
                            placeholder: "user address",
                            type: "text",
                            value: "{address_text}",
                            oninput: move |evt| address_text.set(evt.value()),
                        }
                    }
                }
                {text_channels_list}
                div {
                    class: "flex gap-2 pt-2",
                    button {
                        class: "bg-deep-purple-400 text-deep-purple-100 p-2 w-32 rounded-md duration-100 border-2 border-deep-purple-400 hover:border-deep-purple-100",
                        // onclick: move |_| create_server(name_text, description_text, image_text, loading),
                        "Add Member"
                    }
                    button {
                        class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
                        onclick: move |_| *OPEN_ADD_MEMBER_MODAL.write() = false,
                        "Cancel"
                    }
                }
            }
        }
    }
}
