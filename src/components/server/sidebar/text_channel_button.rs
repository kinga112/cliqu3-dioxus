use dioxus::prelude::*;

use crate::{
    app_state::APP_STATE,
    modules::docs::db::TextChannelMetaData,
    states::server_states::{CURRENT_TEXT_CHANNEL, MESSAGES, OUTBOX_MESSAGES, REPLY},
};

const HASHTAG: Asset = asset!("/assets/icons/hashtag.svg");

#[component]
pub fn TextChannelButton(metadata: TextChannelMetaData) -> Element {
    let current_id = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("failed to get current text channel")
        .metadata
        .id;
    println!("current text channel id: {:?}", current_id.clone());
    println!("this text channel id: {:?}", metadata.clone().id);
    rsx! {
        div {
            class: "w-full overflow-y-auto px-2",
            button {
                class: "flex w-full h-8 place-items-center p-0.5 mb-0.5 rounded-lg",
                class: if CURRENT_TEXT_CHANNEL
                    .read().as_ref()
                    .unwrap()
                    .metadata
                    .id == metadata.id {"bg-deep-purple-300"} else {"hover:bg-off-black-400"},
                onclick: move |_| select_text_channel(metadata.clone().id),
                div {
                    class: "flex w-full justify-between",
                    div {
                        class: "flex flex-row gap-2 overflow-hidden place-items-center",
                        img {
                            src: HASHTAG,
                            height: 20,
                            width: 20,
                        }
                        p {
                            class: "truncate text-deep-purple-100",
                            "{metadata.name}"
                        }
                    }
                }
            }
        }
    }
}

async fn select_text_channel(id: String) {
    println!("Setting text channel");
    let app_state = APP_STATE.lock().await;
    let xmtp = app_state
        .xmtp
        .as_ref()
        .expect("xmtp client not initialized?");
    let text_channel = xmtp
        .get_conversation(&id)
        .expect("failed to get text channel");
    println!("SETTING TEXT CHANNEL: {:?}", text_channel.metadata.name);

    *MESSAGES.write() = Vec::new();
    *OUTBOX_MESSAGES.write() = Vec::new();
    *REPLY.write() = None;
    *CURRENT_TEXT_CHANNEL.write() = Some(text_channel.clone());
    *MESSAGES.write() = text_channel.messages;
}
