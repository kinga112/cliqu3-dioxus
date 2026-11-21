use crate::{
    modules::{
        docs::db::{DB, Server, ServerMetadata},
        xmtp::xmtp::XMTP_STATE,
    },
    smart_contract::interact::fetch_profiles,
    state::{
        app_state::{AppState, Screen},
        data_types::server::{ServerState, TextChannelState},
    },
};
use dioxus::{html::script::text, prelude::*};

#[component]
pub fn ServerButton(metadata: ServerMetadata) -> Element {
    let app_state = use_context::<AppState>();
    let server_state = use_context::<ServerState>();
    let channel_state = use_context::<TextChannelState>();
    // default style
    let mut active = "h-2 w-0 invisible group-hover:h-5 group-hover:w-3 group-hover:visible";
    if metadata.id == server_state.metadata.read().id
        && matches!(*app_state.current_screen.read(), Screen::Server)
    {
        // active server style
        active = "h-10 w-3";
    }
    // match server_state.metadata.read().clone() {
    //     Some(server_metadata) => {
    //         if metadata.id == server_metadata.id
    //             && matches!(*state.current_screen.read(), Screen::Server)
    //         {
    //             // active server style
    //             active = "h-10 w-3";
    //         }
    //     }
    // None => {}
    // Some(server) => {
    //     if metadata.id == server.metadata.id
    //         && matches!(*state.current_screen.read(), Screen::Server)
    //     {
    //         // active server style
    //         active = "h-10 w-3";
    //     }
    // }
    // None => {}
    // }

    rsx! {
        div {
            class: "pt-1.5",
            div {
                class: "flex relative place-items-center group",
                div {
                    class: "absolute -left-1.5 shrink-0 bg-deep-purple-100 rounded-full duration-300 {active}",
                }
                button {
                    class: "flex flex-col w-12 h-12 bg-deep-purple-300 rounded-xl justify-center place-items-center duration-200 hover:scale-105 ml-4 shrink-0 overflow-hidden select-none",
                    onclick: move |_| select_server(metadata.clone()),
                    if &metadata.pic == "" {
                        "{metadata.name}"
                    } else {
                        img {
                            class: "w-full h-full object-cover shrink-0",
                            src: "{metadata.pic}"
                        }
                    }
                }
            }
        }
    }
}

async fn select_server(metadata: ServerMetadata) {
    let mut app_state = use_context::<AppState>();
    let mut server_state = use_context::<ServerState>();
    let mut channel_state = use_context::<TextChannelState>();
    println!("Server Button Click");
    // if CURRENT_SERVER.read().is_some() && *state.current_screen.read() == Screen::Server {
    //     if CURRENT_SERVER.read().as_ref().unwrap().metadata.id == metadata.id {
    //         println!("Not changing server.. same server!");
    //         return;
    //     }
    // }
    if *app_state.current_screen.read() == Screen::Server
        && server_state.metadata.read().id == metadata.id
    {
        println!("Not changing server.. same server!");
        return;
    }
    // let address = ADDRESS.read().clone().unwrap();
    // let app_state = APP_STATE.lock().await;

    // let db_arc = app_state
    //     .db
    //     .as_ref()
    //     .expect("Database no initialized")
    //     .clone();
    // let db = db_arc.lock().await;
    let db = DB
        .read()
        .expect("failed to read DB")
        .clone()
        .expect("failed to get db");
    let server = db
        .get_server(&metadata.id)
        .await
        .expect("failed to get server");
    println!("SERVER NAME: {:?}", server.metadata.name);
    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here? 10");
    let text_channel = xmtp
        .get_conversation(&server.text_channels[0].id)
        .await
        .expect("failed to get text channel");

    fetch_profiles();
    *channel_state.outbox_messages.write() = Vec::new();
    *channel_state.reply.write() = None;
    *channel_state.messages.write() = text_channel.messages.clone();
    *channel_state.members.write() = text_channel.members.clone();
    *channel_state.metadata.write() = text_channel.metadata.clone();
    *server_state.text_channels.write() = server.text_channels.clone();
    *server_state.voice_channels.write() = server.voice_channels.clone();
    *app_state.current_screen.write() = Screen::Server;
    *server_state.metadata.write() = server.metadata;
}
