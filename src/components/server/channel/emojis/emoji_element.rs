use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::{Content, ReactionAction};

use crate::{
    components::server::channel::emojis::emoji_enum::Emoji,
    modules::xmtp::xmtp::{Msg, XMTP_STATE},
    state::{app_state::AppState, data_types::server::TextChannelState},
};

#[component]
pub fn EmojiElement(emoji: Emoji) -> Element {
    // let message = use_context::<Msg>();

    rsx! {
        button {
            class: "hover:bg-deep-purple-400 rounded-md",
            // onclick: move |_| on_click(emoji.clone(), message.clone()),
            onclick: move |_| on_click(emoji.clone()),
            "{emoji.as_str()}"
        }
    }
}

// async fn on_click(emoji: Emoji, message: Msg) {
async fn on_click(emoji: Emoji) {
    let app_state = use_context::<AppState>();
    let channel_state = use_context::<TextChannelState>();
    let message = use_context::<Msg>();

    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 4?");

    match message.reactions.get(emoji.as_str()) {
        Some(users) => {
            if !users.contains(&app_state.user.read().inbox_id) {
                println!("DOES NOT CONTAIN, SENDING EMOJI");
                xmtp.send_reaction(
                    &channel_state.metadata.read().id,
                    &message.id,
                    emoji.as_str(),
                    ReactionAction::Added,
                )
                .await;
            }
        }
        None => {
            println!("DOES NOT CONTAIN, SENDING EMOJI");
            xmtp.send_reaction(
                &&channel_state.metadata.read().id,
                &message.id,
                emoji.as_str(),
                ReactionAction::Added,
            )
            .await;
        }
    }
}
