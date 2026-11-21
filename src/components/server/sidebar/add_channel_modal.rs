use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    components::Modal,
    modules::{
        docs::db::{DB, Server, TextChannelMetaData},
        xmtp::xmtp::{TextChannel, XMTP_STATE},
    },
    state::{
        app_state::AppState,
        data_types::server::{AddChannelModalState, ServerState},
    },
};
// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

#[component]
pub fn AddChannelModal(open_signal: Signal<AddChannelModalState>) -> Element {
    // let server = use_context::<Server>();
    // let server = CURRENT_SERVER.read().clone().expect("no current server??");
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let loading = use_signal(|| false);

    let mut open = use_signal(|| false);
    // if *OPEN_ADD_CHANNEL_MODAL.read() != AddChannelModalState::Closed {
    //     open.set(true);
    // }

    if *open_signal.read() != AddChannelModalState::Closed {
        open.set(true);
    }

    rsx! {
        Modal {
            open,
            div {
                class: "flex flex-col gap-1",
                div {
                    class: "flex justify-center font-light text-4xl p-3",
                    "Add Channel"
                }
                div {
                    class: "flex gap-1",
                    button {
                        class: "rounded-lg p-2",
                        class: if *open_signal.read() == AddChannelModalState::Text {"bg-deep-purple-100 text-deep-purple-300"} else {"bg-deep-purple-300 hover:bg-deep-purple-200"},
                        onclick: move |_| *open_signal.write() = AddChannelModalState::Text,
                        "Text Channel"
                    }
                    button {
                        class: "rounded-lg p-2",
                        class: if *open_signal.read() == AddChannelModalState::Voice {"bg-deep-purple-100 text-deep-purple-300"} else {"bg-deep-purple-300 hover:bg-deep-purple-200"},
                        onclick: move |_| *open_signal.write() = AddChannelModalState::Voice,
                        "Voice Channel"
                    }
                }
                input {
                    class: "bg-deep-purple-400 p-2 rounded-md",
                    placeholder: "name",
                    type: "text",
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value()),
                }
                input {
                    class: "bg-deep-purple-400 p-2 rounded-md",
                    placeholder: "description",
                    type: "text",
                    value: "{description}",
                    oninput: move |evt| description.set(evt.value()),
                }
                div {
                    class: "flex gap-2 pt-2",
                    button {
                        class: "bg-deep-purple-400 text-deep-purple-100 p-2 w-32 rounded-md duration-100 border-2 border-deep-purple-400 hover:border-deep-purple-100",
                        onclick: move |_| add_new_channel(name.read().clone(), description.read().clone(), open_signal),
                        "Add Channel"
                    }
                    button {
                        class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
                        onclick: move |_| *open_signal.write() = AddChannelModalState::Closed,
                        "Cancel"
                    }
                }
            }
        }
    }
}

async fn add_new_channel(
    name: String,
    description: String,
    mut open_signal: Signal<AddChannelModalState>,
) {
    println!("ADDING NEW CHANNEL: {:?}", name);
    let app_state = use_context::<AppState>();
    let mut server_state = use_context::<ServerState>();
    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 4?");
    if name != "" {
        let db = DB
            .read()
            .expect("failed to read DB")
            .clone()
            .expect("failed to get db");
        let address = app_state.user.read().profile.address.clone();
        println!("CURRENT USER ADDRESSS: {:?}", address.clone());
        let members: Vec<Recipient> = vec![Recipient::Address(address)];
        let new_group_id = xmtp
            .create_group(&members, Some(name.to_string()), None, None)
            .expect("failed to create new group");
        match db
            .add_text_channel(&server_state.metadata.peek().id, &name, &new_group_id)
            .await
        {
            Ok(_) => {
                println!("added channel!!!");
                let new_text_channel_metadata = TextChannelMetaData {
                    id: new_group_id,
                    name: name,
                };
                server_state
                    .text_channels
                    .write()
                    .push(new_text_channel_metadata);

                *open_signal.write() = AddChannelModalState::Closed;
            }
            Err(e) => {
                println!("failed to add channel");
            }
        }
    }
    // loading.set(false);
}
