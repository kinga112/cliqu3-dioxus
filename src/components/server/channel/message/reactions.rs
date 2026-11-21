use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::{Content, ReactionAction};

use crate::{
    modules::xmtp::xmtp::{Msg, TextChannel, XMTP_STATE},
    state::{app_state::AppState, data_types::server::TextChannelState},
};

#[component]
pub fn Reactions() -> Element {
    // let state = use_context::<NewAppState>();
    // let server_state = use_context::<ServerState>();
    // let mut channel_state = use_context::<TextChannelState>();
    // Is there a better way to deal with this?
    let message = use_context::<Msg>();
    // let messages = MESSAGES.read().clone();
    // let messages;
    // match *state.current_screen.read() {
    //     Screen::Server => {
    //         messages = channel_state.messages.read().clone();
    //     }
    //     Screen::DirectMessages => {
    //         messages = DM_MESSAGES.read().clone();
    //     }
    //     Screen::Settings => {
    //         messages = channel_state.messages.read().clone();
    //     }
    // }
    // let mut message = Msg {
    //     id: String::new(),
    //     content: Content::Text(String::new()),
    //     from: String::new(),
    //     timestamp: 0,
    //     reactions: HashMap::new(),
    // };

    // for msg in channel_state.messages.read().iter() {
    //     if msg.id == msg_id {
    //         message = msg.clone();
    //     }
    // }
    // let reactions = message.reactions.clone();
    let reactions = message.reactions.iter().map(|(emoji, user_list)| {
        rsx! {ReactionElement {key: "{emoji}", emoji_content: emoji, user_list: user_list.clone()}}
    });

    rsx! {
        div {
            class: "flex gap-1",
            {reactions}
        }
    }
}

#[component]
pub fn ReactionElement(emoji_content: String, user_list: Vec<String>) -> Element {
    let users = user_list.iter().enumerate().map(|(i, user)| {
        let (s1, s2) = user.split_at(5);
        rsx! {
            if i != user_list.len() - 1{
                p {
                    "{s1},"
                }
            }else {
                p {
                    "{s1}"
                }
            }
        }
    });

    async fn on_click(emoji_content: String, user_list: Vec<String>) {
        let state = use_context::<AppState>();
        // let server_state = use_context::<ServerState>();
        let channel_state = use_context::<TextChannelState>();
        // let message = use_context::<Signal<Arc<Msg>>>();
        let message = use_context::<Msg>();
        // let msg_id = use_context::<String>();
        // let text_channel = use_context::<TextChannel>();
        // let id;
        // match *state.current_screen.read() {
        //     Screen::Server => id = channel_state.metadata.read().id.clone(),
        //     Screen::DirectMessages => {
        //         id = CURRENT_DIRECT_MESSAGE
        //             .read()
        //             .clone()
        //             .expect("dm is none?")
        //             .metadata
        //             .id
        //             .clone();
        //     }
        //     Screen::Settings => id = channel_state.metadata.read().id.clone(),
        // }
        // let id = channel.metadata.id;

        // let app_state = APP_STATE.lock().await;
        // let users = message.reactions.get(emoji.as_str()).unwrap();
        let xmtp_opt = XMTP_STATE.lock().await;
        let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 4?");
        if user_list.contains(&state.user.read().inbox_id) {
            println!(
                "USER ALREADY REACTED WITH {:?}, NOT SENDING EMOJI",
                emoji_content
            );
            // app_state
            xmtp
                // .as_ref()
                // .expect("xmtp not initialized")
                .send_reaction(
                    &channel_state.metadata.peek().id,
                    &message.id,
                    &emoji_content,
                    ReactionAction::Removed,
                )
                .await;
        } else {
            println!("SENDING EMOJI {:?}", emoji_content);
            // app_state
            xmtp
                // .as_ref()
                // .expect("xmtp not initialized")
                .send_reaction(
                    &channel_state.metadata.peek().id,
                    &message.id,
                    &emoji_content,
                    ReactionAction::Added,
                )
                .await;
        }
    }

    if user_list.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "group/inner",
            button{
                class: "flex gap-0.5 rounded-sm w-fit py-0.5 px-1 bg-deep-purple-300 hover:bg-off-black-100 text-sm select-none",
                onclick: move |_| on_click(emoji_content.clone(), user_list.clone()),
                p {
                    "{emoji_content}"
                }
                p {
                    "{user_list.len()}"
                }
            }
            div {
                class: "invisible absolute -bottom-4.5 flex flex-row bg-deep-purple-300 rounded-sm p-0.5 gap-1 group-hover/inner:visible z-10 text-xs",
                {users}
            }
        }
    }
}
