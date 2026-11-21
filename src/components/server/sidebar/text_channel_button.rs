use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    modules::{docs::db::TextChannelMetaData, xmtp::xmtp::XMTP_STATE},
    smart_contract::interact::fetch_profiles,
    state::data_types::server::TextChannelState,
};

const HASHTAG: Asset = asset!("/assets/icons/hashtag.svg");

#[component]
pub fn TextChannelButton(metadata: TextChannelMetaData) -> Element {
    let channel_state = use_context::<TextChannelState>();
    rsx! {
        div {
            class: "w-full overflow-y-auto px-2",
            button {
                class: "flex w-full h-8 place-items-center p-0.5 mb-0.5 rounded-lg",
                class: if channel_state.metadata.read().id == metadata.id {"bg-deep-purple-300"} else {"hover:bg-off-black-400"},
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
    let mut channel_state = use_context::<TextChannelState>();
    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 8?");
    let text_channel = xmtp
        .get_conversation(&id)
        .await
        .expect("failed to get text channel");
    println!("SETTING TEXT CHANNEL: {:?}", text_channel.metadata.name);

    fetch_profiles();
    *channel_state.outbox_messages.write() = Vec::new();
    *channel_state.reply.write() = None;
    *channel_state.messages.write() = text_channel.messages.clone();
    *channel_state.members.write() = text_channel.members.clone();
    *channel_state.metadata.write() = text_channel.metadata.clone();
}
