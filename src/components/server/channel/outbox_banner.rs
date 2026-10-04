use dioxus::prelude::*;

use crate::states::server_states::OUTBOX_MESSAGES;

#[component]
pub fn OutboxBanner() -> Element {
    let clone = OUTBOX_MESSAGES.read().clone();
    let outbox_messages = clone.iter().enumerate().map(|(index, text)| {
        rsx! {
            OutboxMessage { index: index, text: text }
        }
    });

    if OUTBOX_MESSAGES.read().is_empty() {
        return rsx! {};
    }

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
                div {
                    class: "bg-red-500 w-3 h-3 animate-spin"
                }
            }
        }
    }
}
