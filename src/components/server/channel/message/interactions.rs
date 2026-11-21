use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    components::server::channel::emojis::EmojiModal,
    modules::xmtp::xmtp::Msg,
    state::data_types::server::{ReplyState, ServerState, TextChannelState},
};

const ARROW: Asset = asset!("/assets/icons/reply.svg");
const EMOJI: Asset = asset!("/assets/icons/emoji2.svg");

#[component]
pub fn Interactions() -> Element {
    let mut server_state = use_context::<ServerState>();
    let mut channel_state = use_context::<TextChannelState>();
    let message = use_context::<Msg>();
    // let open_emojis = use_signal(|| false);
    // let message = use_context::<Signal<Msg>>();
    // let mut open = use_context::<Signal<Option<String>>>();
    // let id = msg.id; // assumes Copy (u64, Uuid...); otherwise clone as needed
    // let msg_id = use_context::<String>();
    // let id = msg_id.clone();
    // let is_open = use_memo(move || open() == Some(id));
    // let is_open = {
    //     let id = message.id.clone();
    //     use_memo(move || open.read().as_deref() == Some(id.as_str()))
    // };
    let id = message.id.clone();
    // let toggle_id = message.id.clone();

    // let messages = server_s.read().clone();
    // let mut message = Msg {
    //     id: "".to_string(),
    //     content: Content::Text("".to_string()),
    //     from: "".to_string(),
    //     timestamp: 10000000000,
    //     reactions: HashMap::new(),
    // };

    // for msg in channel_state.messages.read().clone() {
    //     if msg.id == msg_id {
    //         message = msg;
    //     }
    // }
    let mut open_emoji_modal = use_signal(|| false);
    let text = match message.content.clone() {
        Content::Text(text) => text,
        Content::Reply(reply) => {
            let content_bytes = reply.content.content;
            let text = String::from_utf8(content_bytes).expect("failed to convert to string");
            text
        }
        Content::Attachment(attachment) => {
            let name = attachment
                .filename
                .unwrap_or_else(|| "File: {attachment.mime_type}".to_string());
            name
        }
        _ => "".to_string(),
    };
    // let message_text = message.content
    rsx! {
        div {
            class: "absolute right-5 -top-5 w-20 h-10 bg-deep-purple-300 rounded-lg z-10 invisible group-hover:visible",
            div {
                class: "flex h-10 p-1.5 relative gap-2 place-items-center justify-center",
                button {
                    class: "text-deep-purple-100 border-2 border-deep-purple-100 rounded-lg",
                    onclick: move |_| *channel_state.reply.write() = Some(ReplyState {reference: id.clone(), text: text.clone()}),
                    img {
                        class: "select-none",
                        src: ARROW,
                        width: 25,
                        height: 25,
                    }
                }
                button {
                    class: "text-deep-purple-300",
                    onclick: move |_| open_emoji_modal.set(true),
                    // onclick: move |_| {
                    //     let same = open.read().as_deref() == Some(&toggle_id);
                    //     open.set(if same { None } else { Some(toggle_id.clone()) });
                    // },
                    img {
                        class: "select-none",
                        src: EMOJI,
                        width: 30,
                        height: 30,
                    }
                }
                // EmojiModal{open: open_emoji_modal}
                // if is_open() {
                    // EmojiModal { on_close: move |_| open.set(None) }
                // }
                EmojiModal { open: open_emoji_modal }
            }
        }
    }
}
