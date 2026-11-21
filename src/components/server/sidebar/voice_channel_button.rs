use dioxus::prelude::*;
use iroh_live::rooms::RoomTicket;

use crate::{
    components::server::sidebar::VoiceChannelUser,
    modules::{
        docs::db::{TextChannelMetaData, VoiceChannel},
        live::call::CallHandler,
    },
    state::data_types::server::ServerState,
};

const VOLUME: Asset = asset!("/assets/icons/volume.svg");

#[component]
// pub fn VoiceChannelButton(name: String) -> Element {
pub fn VoiceChannelButton(voice_channel: VoiceChannel) -> Element {
    // let server_state = use_context::<ServerState>();
    // let voice_channels = VOICE_CHANNELS.read().clone();
    // let voice_channels_ui =
    //     server_state
    //         .voice_channels
    //         .read()
    //         .clone()
    //         .into_iter()
    //         .map(|(id, voice_channel)| {
    //             let users = voice_channel
    //                 .active_users
    //                 .iter()
    //                 .map(|(public_key, display_name)| {
    //                     rsx! {VoiceChannelUser{public_key, display_name}}
    //                 });
    //             rsx! {
    //                 div {
    //                     class: "flex flex-col",
    //                     {users}
    //                     // VoiceChannelUser{public_key: String::new(), display_name: "Test User 1"}
    //                     // VoiceChannelUser{public_key: String::new(), display_name: "Test User 2"}
    //                 }
    //             }
    //         });
    //
    let users = voice_channel
        .active_users
        .iter()
        .map(|(public_key, display_name)| {
            rsx! {VoiceChannelUser{public_key, display_name}}
        });

    rsx! {
        div {
            class: "w-full overflow-y-auto px-2",
            button {
                class: "flex w-full h-8 place-items-center p-0.5 rounded-lg hover:bg-off-black-400",
                onclick: move |_| {
                    // let name_clone = name.clone();
                    let name_clone = voice_channel.name.clone();
                    spawn(async move {
                       join_call(name_clone).await;
                    });
                },
                div {
                    class: "flex w-full justify-between",
                    div {
                        class: "flex flex-row gap-2 overflow-hidden place-items-center",
                        img {
                            src: VOLUME,
                            height: 20,
                            width: 20,
                        }
                        p {
                            class: "truncate text-deep-purple-100",
                            "{voice_channel.name}"
                        }
                    }
                }
            }
            // {voice_channels_ui}
            div {
                class: "flex flex-col",
                {users}
            }
            // div {
            //     class: "flex flex-col",
            //     // {users}
            //     VoiceChannelUser{public_key: String::new(), display_name: "Test User 1"}
            //     VoiceChannelUser{public_key: String::new(), display_name: "Test User 2"}
            // }
        }
    }
}

// async fn join_call(name: String, ticket_str: String) {
async fn join_call(name: String) {
    let server_state = use_context::<ServerState>();
    println!("joining call");
    // let voice_channels = VOICE_CHANNELS.read();
    // let voice_channel_opt = voice_channels.get(&name.clone());

    let voice_channel_opt = {
        // let voice_channels = VOICE_CHANNELS.read();
        server_state.voice_channels.peek().get(&name).cloned() // Option<VoiceChannel> — owned, no borrow left
    };

    let call_handler = CallHandler::new()
        .await
        .expect("failed to create call handler");

    match voice_channel_opt {
        Some(voice_channel) => {
            let ticket: RoomTicket = voice_channel
                .ticket
                .parse()
                .expect("failed to get room ticket from string");
            match call_handler
                .join_room(name.clone(), ticket, server_state.voice_channels)
                .await
            {
                Ok(()) => {
                    // CallHandler::add_call(name.clone(), voice_channel.ticket.clone());
                    println!("JOIN: succeeded");
                }
                Err(e) => {
                    println!("JOIN FAILED: {e:?}");
                }
            }
        }
        None => match call_handler
            .create_room(name.clone(), server_state.voice_channels)
            .await
        {
            Ok(ticket) => {
                // CallHandler::add_call(name.clone(), ticket);
                println!("CREATE: succeeded");
            }
            Err(e) => {
                println!("CREATE FAILED: {e:?}");
            }
        },
    }

    // if ticket_str.is_empty() {
    //     match call_handler.create_room(name).await {
    //         Ok(()) => {
    //             println!("CREATE: succeeded");
    //         }
    //         Err(e) => {
    //             println!("CREATE FAILED: {e:?}");
    //         }
    //     }
    // } else {
    //     let ticket: RoomTicket = ticket_str
    //         .parse()
    //         .expect("failed to get room ticket from string");
    //     match call_handler.join_room(name.clone(), ticket).await {
    //         Ok(()) => {
    //             CallHandler::add_call(name, ticket_str);
    //             println!("JOIN: succeeded");
    //         }
    //         Err(e) => {
    //             println!("JOIN FAILED: {e:?}");
    //         }
    //     }
    // }
}
