use dioxus::prelude::*;

use crate::state::data_types::server::TextChannelState;

const LOADER: Asset = asset!("/assets/icons/loader2.svg");

#[component]
pub fn OutboxBanner() -> Element {
    let channel_state = use_context::<TextChannelState>();
    if channel_state.outbox_messages.read().is_empty() {
        return rsx! {};
    }

    let outbox_messages = channel_state
        .outbox_messages
        .read()
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, outbox_message)| {
            rsx! {
                OutboxMessage { index: index, text: outbox_message.message_text.clone() }
            }
        });

    rsx! {
        div {
            class: "absolute z-50 -top-3 -translate-y-full w-full h-fit z-20 pr-5 pl-2",
            div {
                class: "bg-off-black-500 rounded-lg shadow-inner shadow-black p-2",
                {outbox_messages}
            }
        }
    }
}

#[component]
pub fn OutboxMessage(index: usize, text: String) -> Element {
    rsx! {
        div {
            if index != 0 {
                div {
                    class: "h-0.5 w-full bg-off-black-400"
                }
            }
            div{
                class: "flex justify-between place-items-center",
                p {
                    class: "truncate",
                    "{text}"
                }
                img {
                    class: "w-8 h-8 animate-spin",
                    src: LOADER,
                }
            }
        }
    }
}
