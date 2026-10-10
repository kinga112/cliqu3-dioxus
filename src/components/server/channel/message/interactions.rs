use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    components::server::channel::emojis::EmojiModal,
    modules::xmtp::xmtp::Msg,
    states::server_states::{MESSAGES, REPLY, ReplyState},
};

const ARROW: Asset = asset!("/assets/icons/reply.svg");
const EMOJI: Asset = asset!("/assets/icons/emoji2.svg");

#[component]
pub fn Interactions() -> Element {
    // let message = use_context::<Signal<Arc<Msg>>>();
    // let message = use_context::<Signal<Msg>>();
    let msg_id = use_context::<String>();
    let messages = MESSAGES.read().clone();
    let mut message = Msg {
        id: "".to_string(),
        content: Content::Text("".to_string()),
        from: "".to_string(),
        timestamp: 10000000000,
        reactions: HashMap::new(),
    };

    for msg in messages {
        if msg.id == msg_id {
            message = msg;
        }
    }
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
                    onclick: move |_| *REPLY.write() = Some(ReplyState {reference: message.id.clone(), text: text.clone()}),
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
                    img {
                        class: "select-none",
                        src: EMOJI,
                        width: 30,
                        height: 30,
                    }
                }
                EmojiModal{open: open_emoji_modal}
            }
        }
    }
}
