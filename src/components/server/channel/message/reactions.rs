use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::{Content, ReactionAction};

use crate::{
    app_state::APP_STATE,
    modules::xmtp::xmtp::{Msg, TextChannel},
    states::{
        server_states::{CURRENT_TEXT_CHANNEL, MESSAGES},
        user_states::USER,
    },
};

#[component]
pub fn Reactions() -> Element {
    // Is there a better way to deal with this?
    let msg_id = use_context::<String>();
    let messages = MESSAGES.read().clone();
    let mut message = Msg {
        id: "".to_string(),
        content: Content::Text(("".to_string())),
        from: "".to_string(),
        timestamp: 10000000000,
        reactions: HashMap::new(),
    };

    for msg in messages {
        if msg.id == msg_id {
            message = msg;
        }
    }
    let reactions = message.reactions.clone();
    let reactions = reactions.iter().map(|(emoji, user_list)| {
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
        // let message = use_context::<Signal<Arc<Msg>>>();
        // let message = use_context::<Signal<Msg>>();
        let msg_id = use_context::<String>();
        // let text_channel = use_context::<TextChannel>();
        let text_channel = CURRENT_TEXT_CHANNEL
            .read()
            .clone()
            .expect("text channel is none?");

        let app_state = APP_STATE.lock().await;
        // let users = message.reactions.get(emoji.as_str()).unwrap();
        if user_list.contains(&USER.read().inbox_id) {
            println!(
                "USER ALREADY REACTED WITH {:?}, NOT SENDING EMOJI",
                emoji_content
            );
            app_state
                .xmtp
                .as_ref()
                .expect("xmtp not initialized")
                .send_reaction(
                    &text_channel.metadata.id,
                    &msg_id,
                    &emoji_content,
                    ReactionAction::Removed,
                )
                .await;
        } else {
            println!("SENDING EMOJI {:?}", emoji_content);
            app_state
                .xmtp
                .as_ref()
                .expect("xmtp not initialized")
                .send_reaction(
                    &text_channel.metadata.id,
                    &msg_id,
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
