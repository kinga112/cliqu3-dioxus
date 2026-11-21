use crate::{
    components::server::channel::OutboxBanner,
    modules::xmtp::xmtp::XMTP_STATE,
    state::data_types::server::{OutboxMessageType, TextChannelState},
};
use dioxus::prelude::*;
use uuid::Uuid;

const GIF: Asset = asset!("/assets/icons/gif.svg");
const EMOJI: Asset = asset!("/assets/icons/emoji2.svg");
const GALLERY: Asset = asset!("/assets/icons/gallery.svg");
const CLOSE: Asset = asset!("/assets/icons/close.svg");

#[component]
pub fn BottomBar(id: String) -> Element {
    let mut channel_state = use_context::<TextChannelState>();
    let mut text_input = use_signal(|| String::new());
    let mut open_emoji_modal = use_signal(|| false);

    rsx! {
        div {
            class: "flex shrink-0 w-full justify-center z-20 pb-3",
            div {
                class: "relative h-full w-full mx-3",
                div {
                    class: "flex w-full",
                    if let Some(reply) = channel_state.reply.read().as_ref() {
                        div {
                            class: "absolute z-50 -top-5 rounded-e-md rounded-tl-md p-1 bg-deep-purple-300",
                            div {
                                class: "flex gap-3",
                                p {
                                    class: "truncate max-w-96",
                                    "Replying to: {reply.text}"
                                }
                                button {
                                    class: "hover:bg-red-700 rounded-full w-6 h-6",
                                    onclick:  move |_| *channel_state.reply.write() = None,
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
                            // EmojiModal{open: open_emoji_modal}
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
                OutboxBanner{}
            }
        }
    }
}

async fn send_message(id: String, mut text_input: Signal<String>) {
    let mut channel_state = use_context::<TextChannelState>();
    let text = text_input.read().clone();
    if text == "".to_string() {
        return;
    }
    let tmp_id = Uuid::new_v4().to_string();
    let outbox_message = OutboxMessageType {
        tmp_id: tmp_id.clone(),
        message_id: "".to_string(),
        message_text: text.clone(),
    };
    channel_state.outbox_messages.write().push(outbox_message);
    text_input.set("".to_string());

    spawn(async move {
        let xmtp_opt = XMTP_STATE.lock().await;
        let message_id;
        match channel_state.reply.peek().as_ref() {
            Some(reply) => {
                println!("SENDNING A REPLY 1");
                let reference = reply.reference.clone();
                message_id = tokio::task::spawn_blocking(move || {
                    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 6?");
                    xmtp.send_text_reply(&id, text, reference)
                        .expect("failed to send text")
                })
                .await
                .expect("failed send text reply");
            }
            None => {
                message_id = tokio::task::spawn_blocking(move || {
                    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 6?");
                    xmtp.send_text(&id, text).expect("failed to send text")
                })
                .await
                .expect("failed send text");
            }
        }
        *channel_state.reply.write() = None;
        for outbox_message in channel_state.outbox_messages.write().iter_mut() {
            if *outbox_message.tmp_id == tmp_id {
                outbox_message.message_id = message_id.clone();
            }
        }
    });
}
