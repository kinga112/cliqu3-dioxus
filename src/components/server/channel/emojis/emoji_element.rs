use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::{Content, ReactionAction};

use crate::{
    app_state::APP_STATE,
    components::server::channel::emojis::emoji_enum::Emoji,
    modules::xmtp::xmtp::{Msg, TextChannel},
    states::{
        dm_states::{CURRENT_DIRECT_MESSAGE, DM_MESSAGES},
        global_states::{CURRENT_SCREEN, Screen},
        server_states::{CURRENT_TEXT_CHANNEL, MESSAGES},
        user_states::USER,
    },
};

#[component]
pub fn EmojiElement(emoji: Emoji) -> Element {
    let msg_id = use_context::<String>();
    let channel;
    let messages;
    let mut message = Msg {
        id: String::new(),
        content: Content::Text(String::new()),
        from: String::new(),
        timestamp: 0,
        reactions: HashMap::new(),
    };

    match *CURRENT_SCREEN.read() {
        Screen::Server => {
            channel = CURRENT_TEXT_CHANNEL
                .read()
                .clone()
                .expect("text channel is none?");
            messages = MESSAGES.read().clone();
        }
        Screen::DirectMessages => {
            channel = CURRENT_DIRECT_MESSAGE.read().clone().expect("dm is none?");
            messages = DM_MESSAGES.read().clone();
        }
        Screen::Settings => {
            channel = CURRENT_TEXT_CHANNEL
                .read()
                .clone()
                .expect("text channel is none?");
            messages = MESSAGES.read().clone();
        }
    }
    let id = channel.metadata.id;
    for msg in messages {
        if msg.id == msg_id {
            message = msg;
        }
    }

    rsx! {
        button {
            class: "hover:bg-deep-purple-400 rounded-md",
            onclick: move |_| on_click(emoji.clone(), id.clone(), message.clone()),
            "{emoji.as_str()}"
        }
    }
}

async fn on_click(emoji: Emoji, id: String, msg: Msg) {
    let app_state = APP_STATE.lock().await;
    if msg.reactions.get(emoji.as_str()).is_some() {
        let users = msg.reactions.get(emoji.as_str()).unwrap();
        if !users.contains(&USER.read().inbox_id) {
            println!("DOES NOT CONTAIN, SENDING EMOJI");
            app_state
                .xmtp
                .as_ref()
                .expect("xmtp not initialized")
                .send_reaction(&id, &msg.id, emoji.as_str(), ReactionAction::Added)
                .await;
        }
    } else {
        println!("DOES NOT CONTAIN, SENDING EMOJI");
        app_state
            .xmtp
            .as_ref()
            .expect("xmtp not initialized")
            .send_reaction(&id, &msg.id, emoji.as_str(), ReactionAction::Added)
            .await;
    }
}
