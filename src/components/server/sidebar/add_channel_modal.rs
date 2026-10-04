use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    components::Modal,
    modules::{
        docs::db::{Server, TextChannelMetaData},
        xmtp::xmtp::TextChannel,
    },
    states::{
        server_states::{AddChannelModalState, CURRENT_SERVER, OPEN_ADD_CHANNEL_MODAL},
        user_states::USER,
    },
};
// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

#[component]
pub fn AddChannelModal() -> Element {
    // let server = use_context::<Server>();
    // let server = CURRENT_SERVER.read().clone().expect("no current server??");
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let loading = use_signal(|| false);

    let mut open = use_signal(|| false);
    if *OPEN_ADD_CHANNEL_MODAL.read() != AddChannelModalState::Closed {
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
                        class: if *OPEN_ADD_CHANNEL_MODAL.read() == AddChannelModalState::Text {"bg-deep-purple-100 text-deep-purple-300"} else {"bg-deep-purple-300 hover:bg-deep-purple-200"},
                        onclick: move |_| *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Text,
                        "Text Channel"
                    }
                    button {
                        class: "rounded-lg p-2",
                        class: if *OPEN_ADD_CHANNEL_MODAL.read() == AddChannelModalState::Voice {"bg-deep-purple-100 text-deep-purple-300"} else {"bg-deep-purple-300 hover:bg-deep-purple-200"},
                        onclick: move |_| *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Voice,
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
                        onclick: move |_| add_new_channel(name.read().clone(), description.read().clone()),
                        "Add Channel"
                    }
                    button {
                        class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
                        onclick: move |_| *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Closed,
                        "Cancel"
                    }
                }
            }
        }
    }
}

async fn add_new_channel(name: String, description: String) {
    println!("ADDING NEW CHANNEL: {:?}", name);
    let mut server = CURRENT_SERVER.read().clone().expect("no current server??");
    let app_state = APP_STATE.lock().await;
    // let db_arc = app_state
    //     .db
    //     .as_ref()
    //     .expect("Database no initialized")
    //     .clone();
    // let db = db_arc.lock().await;
    if name != "" {
        if app_state.xmtp.is_some() {
            let db_arc = app_state
                .db
                .as_ref()
                .expect("Database no initialized")
                .clone();
            let db = db_arc.lock().await;
            let address = USER
                .read()
                .profile
                .as_ref()
                .expect("No User Profile")
                .address
                .clone();
            println!("CURRENT USER ADDRESSS: {:?}", address.clone());
            let members: Vec<Recipient> = vec![Recipient::Address(address)];
            let new_group_id = app_state
                .xmtp
                .as_ref()
                .expect("failed to get xmtp instance")
                .create_group(&members, Some(name.to_string()), None, None)
                .expect("failed to create new group");
            match db
                .add_text_channel(&server.metadata.id, &name, &new_group_id)
                .await
            {
                Ok(result) => {
                    println!("added channel!!!");
                    let updated_server = db
                        .get_server(&server.metadata.id)
                        .await
                        .expect("failed to get server");
                    *CURRENT_SERVER.write() = Some(updated_server);

                    let new_text_channel = TextChannelMetaData {
                        id: new_group_id,
                        name: name,
                    };
                    server.text_channels.push(new_text_channel);

                    *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Closed;
                    // SERVER_LIST.write().insert(0, metadata);
                }
                Err(e) => {
                    println!("failed to add channel");
                }
            }
        }
    }
    // loading.set(false);
}
