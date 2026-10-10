use crate::{
    app_state::APP_STATE,
    modules::docs::db::{Server, ServerMetadata},
    states::{
        global_states::{CURRENT_SCREEN, Screen},
        server_states::{CURRENT_SERVER, CURRENT_TEXT_CHANNEL, MESSAGES},
    },
};
use dioxus::prelude::*;

#[component]
pub fn ServerButton(metadata: ServerMetadata) -> Element {
    // default style
    let mut active = "h-2 w-0 invisible group-hover:h-5 group-hover:w-3 group-hover:visible";
    match CURRENT_SERVER.read().clone() {
        Some(server) => {
            if metadata.id == server.metadata.id && matches!(*CURRENT_SCREEN.read(), Screen::Server)
            {
                // active server style
                active = "h-10 w-3";
            }
        }
        None => {}
    }

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
    println!("Server Button Click");
    if CURRENT_SERVER.read().is_some() && *CURRENT_SCREEN.read() == Screen::Server {
        if CURRENT_SERVER.read().as_ref().unwrap().metadata.id == metadata.id {
            println!("Not changing server.. same server!");
            return;
        }
    }
    // let address = ADDRESS.read().clone().unwrap();
    let app_state = APP_STATE.lock().await;
    let db_arc = app_state
        .db
        .as_ref()
        .expect("Database no initialized")
        .clone();
    let db = db_arc.lock().await;
    let server = db
        .get_server(&metadata.id)
        .await
        .expect("failed to get server");
    println!("SERVER NAME: {:?}", server.metadata.name);
    let xmtp = app_state
        .xmtp
        .as_ref()
        .expect("xmtp client not initialized?");
    let text_channel = xmtp
        .get_conversation(&server.text_channels[0].id)
        .await
        .expect("failed to get text channel");

    *CURRENT_TEXT_CHANNEL.write() = Some(text_channel.clone());
    *MESSAGES.write() = text_channel.messages;
    // *MESSAGES.write() = text_channel.messages.clone();
    // *CURRENT_TEXT_CHANNEL.write() = Some(text_channel);
    *CURRENT_SCREEN.write() = Screen::Server;
    *CURRENT_SERVER.write() = Some(server);
    // *CURRENT_TEXT_CHANNEL.write() = server.text_channels[0];
}
