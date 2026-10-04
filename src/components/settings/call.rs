use std::str::FromStr;

use dioxus::{prelude::*, subsecond::call};
use iroh_gossip::TopicId;
use iroh_live::rooms::RoomTicket;
use xmtp::content::Content;

use crate::{
    app_state::APP_STATE,
    modules::live::{self, call::CallHandler},
    states::{
        dm_states::{CURRENT_DIRECT_MESSAGE, DM_MESSAGES},
        server_states::VOICE_CHANNELS,
    },
};

#[component]
pub fn Call() -> Element {
    let mut ticket_str = use_signal(|| String::new());

    let voice_channels = VOICE_CHANNELS.read().clone();
    // messages.iter().enumerate().map(|(i, msg)| {

    let voice_channels_ui = voice_channels.into_iter().map(|(ticket, voice_channel)| {
        // let users = voice_channel.into_iter().map(|(public_key, display_name)| {
        let users = voice_channel
            .active_users
            .iter()
            .map(|(public_key, display_name)| {
                rsx! {
                    div {
                        "DISPLAY NAME: {display_name}"
                    }
                }
            });
        rsx! {
            div{
                class: "flex flex-col bg-orange-500",
                div {
                    "TICKET: {ticket}"
                }
                div {
                    class: "flex flex-col bg-blue-300",
                    {users}
                }
            }
        }
    });

    rsx! {
        div{
            class: "flex flex-col gap-5 w-full h-full bg-yellow-500",
            div {
                class: "flex gap-5 w-full h-full bg-blue-300",
                input {
                    class: "bg-red-200 h-20 w-full",
                    placeholder: "Ticket for Call",
                    oninput: move |evt| ticket_str.set(evt.value()),
                }
                button {
                    class: "bg-blue-300 px-2 py-1 w-full shrink hover:bg-off-black-400 rounded-lg",
                    onclick: move |_| {
                        spawn(async move {
                           // let a = join_call(ticket_str.read().clone()).await;
                        });
                    },
                    "JOIN CALL"
                }
            }
            div {
                class: "h-56 bg-white p-5 text-red-500",
                {voice_channels_ui}
            }
        }
    }
}

// async fn join_call(ticket_str: String) {
//     println!("joining call");
//     let call_handler = CallHandler::new()
//         .await
//         .expect("failed to create call handler");
//     if ticket_str.is_empty() {
//         // match live::test::create_room().await {
//         match call_handler.create_room().await {
//             Ok(()) => {
//                 println!("CREATE: succeeded");
//             }
//             Err(e) => {
//                 println!("CREATE FAILED: {e:?}");
//             }
//         }
//     } else {
//         // let ticket =
//         //     RoomTicket::from_str(&ticket_str).expect("failed to get room ticket from string");
//         let ticket: RoomTicket = ticket_str
//             .parse()
//             .expect("failed to get room ticket from string");
//         // match live::test::join_room(ticket).await {
//         match call_handler.join_room(ticket).await {
//             Ok(()) => {
//                 CallHandler::add_call(ticket_str);
//                 println!("JOIN: succeeded");
//             }
//             Err(e) => {
//                 println!("JOIN FAILED: {e:?}");
//             }
//         }
//     }
// }
