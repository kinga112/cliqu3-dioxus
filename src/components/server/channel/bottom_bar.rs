use dioxus::prelude::*;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use xmtp::content::Content;

use crate::{
    app_state::APP_STATE,
    components::server::channel::{OutboxBanner, emojis::EmojiModal},
    modules::xmtp::xmtp::{Msg, TextChannel},
    states::{
        server_states::{MESSAGES, OUTBOX_MESSAGES, REPLY},
        user_states::USER,
    },
};

const GIF: Asset = asset!("/assets/icons/gif.svg");
const EMOJI: Asset = asset!("/assets/icons/emoji2.svg");
const GALLERY: Asset = asset!("/assets/icons/gallery.svg");
const CLOSE: Asset = asset!("/assets/icons/close.svg");

#[component]
pub fn BottomBar(id: String) -> Element {
    // let text_channel = use_context::<TextChannel>();
    let mut text_input = use_signal(|| String::new());
    // let text_input_context = use_context_provider(|| text_input);
    let mut open_emoji_modal = use_signal(|| false);
    // let id = text_channel.metadata.id;
    let show_reply_banner = REPLY.read().is_some();

    rsx! {
        div {
            class: "flex shrink-0 w-full justify-center z-20 pb-3",
            div {
                // when no scroll bar h-4
                // class: "relative h-[calc(58px)] w-full mx-3",
                class: "relative h-full w-full mx-3",
                div {
                    // when no scroll bar: bottom-4
                    class: "flex w-full",
                    if show_reply_banner {
                        div {
                            class: "absolute z-50 -top-5 rounded-e-md rounded-tl-md p-1 bg-deep-purple-300",
                            div {
                                class: "flex gap-3",
                                p {
                                    class: "truncate max-w-96",
                                    "Replying to: {REPLY.read().as_ref().unwrap().text.clone()}"
                                }
                                button {
                                    class: "hover:bg-red-700 rounded-full w-6 h-6",
                                    onclick:  move |_| *REPLY.write() = None,
                                    img {
                                        src: CLOSE,
                                    }
                                }
                            }
                        }
                    }
                    textarea {
                        class: "z-0 w-full bg-off-black-600 rounded-lg px-2 py-4 focus:outline-none pr-56 resize-none",
                        // placeholder: "Send a message to #{text_channel.metadata.name}",
                        placeholder: "Send a message...",
                        value: "{text_input.read()}",
                        rows: 1,
                        oninput: move |evt| text_input.set(evt.value()),
                        onkeydown: {
                            let id = id.clone();
                            move |evt| {
                                if evt.key() == Key::Enter && !evt.modifiers().shift() {
                                    evt.prevent_default();
                                    // let id = id.clone();
                                    spawn(send_message(id.clone(), text_input));
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute right-[calc(68px)] top-2 bg-deep-purple-300 px-2 py-1 h-10 rounded-md",
                        div {
                            class: "flex place-items-center h-full w-full gap-1",
                            button {
                                img {
                                    src: GALLERY,
                                    height: 30,
                                    width: 30,
                                }
                            }
                            button {
                                img {
                                    src: GIF,
                                    height: 35,
                                    width: 35,
                                }
                            }
                            button {
                                onclick: move |_| open_emoji_modal.set(true),
                                img {
                                    src: EMOJI,
                                    height: 30,
                                    width: 30,
                                }
                            }
                            EmojiModal{open: open_emoji_modal}
                        }
                    }
                    button {
                        class: "absolute right-2 top-2 bg-deep-purple-300 px-2 py-1 h-10 rounded-md hover:bg-deep-purple-200",
                        onclick: {
                            let id = id.clone();
                            move |_| {
                                spawn(send_message(id.clone(), text_input));
                            }
                        },
                        "Send"
                    }
                }
                // Outbox Banner
                OutboxBanner{}
            }
        }
    }
}

async fn send_message(id: String, mut text_input: Signal<String>) {
    let text = text_input.read().clone();
    text_input.set("".to_string());
    OUTBOX_MESSAGES.write().push(text.clone());
    let app_state = APP_STATE.lock().await;
    // let reply = REPLY.read().is_none();
    if REPLY.read().is_none() {
        app_state
            .xmtp
            .as_ref()
            .expect("xmtp not initialized")
            .send_text(&id, text)
            .await;
    } else {
        println!("SENDNING A REPLY 1");
        // let reply = REPLY.read().as_ref().expect("");/
        let reference = REPLY
            .read()
            .as_ref()
            .expect("reply is none??")
            .reference
            .clone();
        app_state
            .xmtp
            .as_ref()
            .expect("xmtp not initialized")
            .send_text_reply(&id, text, reference)
            .await;
    }
}
