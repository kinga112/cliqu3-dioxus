use crate::{
    app_state::APP_STATE,
    components::server::sidebar::{
        AddChannelModal, AddMemberModal, TextChannelButton, VoiceChannelButton,
    },
    modules::docs::db::{Server, TextChannelMetaData},
    states::server_states::{
        AddChannelModalState, CURRENT_SERVER, OPEN_ADD_CHANNEL_MODAL, OPEN_ADD_MEMBER_MODAL,
    },
};
use dioxus::prelude::*;
use prost::Message;
use xmtp::content::{Content, EncodedContent};

const ADD: Asset = asset!("/assets/icons/add-cropped.svg");
const CARROT: Asset = asset!("/assets/icons/carrot.svg");

#[component]
pub fn SideBar() -> Element {
    let server = CURRENT_SERVER.read().clone().expect("no current server??");
    // let server = use_context::<Server>();
    println!("SERVER ID INSIDE SIDEBAR: {:?}", server.metadata.id);
    // let mut open_add_text_channel_modal = use_signal(|| false);
    let mut open_server_options_menu = use_signal(|| false);
    // let text_channel = server_state.read().text_channels;
    // let server_clone = server.clone();
    // let text_channel_id_clone = server.text_channels[0].id.clone();
    let text_channels = server.text_channels.into_iter().map(|metadata| {
        rsx! {TextChannelButton {metadata: metadata}}
    });

    let voice_channels = server
        .voice_channels
        .into_iter()
        .map(|(id, voice_channel)| {
            rsx! {VoiceChannelButton {name: voice_channel.name}}
        });

    rsx! {
        div {
            class: "flex flex-col gap-5 w-56 bg-off-black-600 border-r-1 border-off-black-400 shrink-0 overflow-hidden",
            div {
                class: "relative",
                button {
                    class: "flex justify-between bg-off-black-600 w-full h-14 z-20 absolute border-b border-off-black-700 place-items-center px-2 shadow-md shadow-off-black-700 text-xl font-light",
                    onclick: move |_| open_server_options_menu.toggle(),
                    "{server.metadata.name}"
                }
                div {
                    class: "flex flex-col absolute top-0 w-full gap-2 bg-off-black-600 rounded-b-lg p-2 transition-transform duration-300 ease-in-out shadow-md shadow-off-black-700",
                    class: if *open_server_options_menu.read() {"translate-y-14"} else {"-translate-y-16"},
                    button {
                        class: "w-full h-10 bg-off-black-300 rounded hover:bg-off-black-400",
                        // onclick: move |_| add_member(text_channel_id_clone.clone(), server_clone.clone()),
                        onclick: move |_| *OPEN_ADD_MEMBER_MODAL.write() = true,
                        "Add Members"
                    }
                    button {
                        class: "w-full h-10 bg-off-black-300 rounded hover:bg-off-black-400",
                        "Server Settings"
                    }
                }
            }
            // Text Channels
            div {
                class: "flex flex-col gap-1 overflow-y-auto pt-14",
                div {
                    class: "flex justify-between items-center group px-2 pointer-events-none",
                    div {
                        class: "flex gap-1 items-center",
                        button {
                            class: "hover:underline group-hover:underline pointer-events-auto",
                            "text channels"
                        }
                        img {
                            class: "w-3 h-3",
                            src: CARROT,
                        }
                    }
                    button {
                        class: "hover:bg-off-black-300 p-0.5 h-5 w-5 rounded group-hover:underline pointer-events-auto",
                        onclick: move |_| *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Text,
                        img {
                            class: "w-full h-full",
                            src: ADD,
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    {text_channels}
                }
            }
            // Voice Channels
            div {
                class: "flex flex-col gap-1 overflow-y-auto py-5",
                div {
                    class: "flex justify-between items-center group px-2 pointer-events-none",
                    div {
                        class: "flex gap-1 items-center",
                        button {
                            class: "hover:underline group-hover:underline pointer-events-auto",
                            "voice channels"
                        }
                        img {
                            class: "w-3 h-3",
                            src: CARROT,
                        }
                    }
                    button {
                        class: "hover:bg-off-black-300 p-0.5 h-5 w-5 rounded group-hover:underline pointer-events-auto",
                        onclick: move |_| *OPEN_ADD_CHANNEL_MODAL.write() = AddChannelModalState::Voice,
                        img {
                            class: "w-full h-full",
                            src: ADD,
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    {voice_channels}
                }
            }
        }
        AddMemberModal{}
        AddChannelModal{}
    }
}

async fn add_member(id: String, server: Server) {
    // let text_channel_id = text_channels[0].id.clone();
    println!("SENDING REAL INVITE TEST");
    let app_state = APP_STATE.lock().await;
    let server_json = serde_json::to_string(&server).expect("failed to serialize server to json");
    // let _ = app_state
    //     .xmtp
    //     .as_ref()
    //     .expect("xmtp not initialized")
    //     .send_invite(
    //         &id,
    //         // "This is fake metadata for an invite.. just for testing".to_string(),
    //         server_json,
    //     )
    //     .await;

    // fetch message here ->
    // let msg = app_state
    //     .xmtp
    //     .as_ref()
    //     .expect("xmtp not initialized")
    //     .client
    //     .message_by_id("c1b625eb550169a49ed5996997bdfbbe44d37eeae5e035d4218b61a7c5d20a52")
    //     .expect("failed to get message 1")
    //     .expect("failed to get message 2");
    // let content = msg.decode().expect("failed to get msg content");
    // match content {
    //     Content::Unknown { content_type, raw } => {
    //         println!("UNKOWN with type: {:?}", content_type);
    //         // let content = String::from_utf8(raw).expect("Found invalid UTF-8 data");
    //         let ec = EncodedContent::decode(&*raw).expect("failed to decode encoded content");
    //         let content = String::from_utf8(ec.content).expect("Found invalid UTF-8 data");
    //         println!("CONTENT: {:?}", content);
    //     }
    //     _ => {
    //         println!("NEW MESSAGE IS NOT UNKOWN");
    //     }
    // }
}
