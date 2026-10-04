use alloy::hex;
use base64::prelude::*;
use dioxus::prelude::*;
use prost::Message;
use std::collections::HashMap;
use xmtp::content::{Content, EncodedContent};

use crate::{
    app_state::APP_STATE,
    components::server::channel::message::content::BasicContent,
    modules::{docs::db::Server, walletconnect::crypto, xmtp::xmtp::Msg},
    states::user_states::USER,
};

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, Generate, Key, KeyInit},
};

#[derive(Clone, PartialEq)]
enum InviteState {
    Loading,
    Ready(Server),
    Failed,
}

#[component]
pub fn InviteContent(raw: Vec<u8>) -> Element {
    let ec = EncodedContent::decode(&*raw.to_owned()).expect("failed to decode encoded content");
    let mut server_state = use_signal(|| InviteState::Loading);

    use_effect(move || {
        let content = ec.content.clone();
        spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                let key_bytes =
                    hex::decode("FC52BD39629CC1030A3C08974403A350F79A337C6C99F72EE3B7EADD55551B15")
                        .expect("Invalid hex key");
                let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
                let cipher = Aes256Gcm::new(key);
                let (nonce_bytes, ciphertext) = content.split_at(12);
                let nonce = Nonce::from_slice(nonce_bytes);
                let decrypted_server_data = cipher
                    .decrypt(nonce, ciphertext)
                    .unwrap_or_else(|_| Vec::new());
                let server_data_result = serde_json::from_slice::<Server>(&decrypted_server_data);
                let server = match server_data_result {
                    Ok(server) => Some(server),
                    Err(e) => None,
                };
                server
            })
            .await;

            match result {
                Ok(Some(server)) => server_state.set(InviteState::Ready(server)),
                _ => server_state.set(InviteState::Failed),
            }
        });
    });

    match server_state.read().clone() {
        InviteState::Loading => rsx! {
            BasicContent { "decrypting invite..." }
        },
        InviteState::Failed => rsx! {
            BasicContent { "failed to decrypt Cliqu3 invite" }
        },
        InviteState::Ready(server) => rsx! {
            BasicContent {
                div {
                    class: "flex justify-between items-center bg-off-black-400 rounded-lg p-2",
                    div {
                        class: "flex gap-2",
                        img {
                            class: "w-14 h-14 rounded-xl object-cover",
                            src: "{server.metadata.pic}",
                        }
                        div {
                            class: "text-lg font-light",
                            "{server.metadata.name}"
                        }
                    }
                    button {
                        class: "flex p-4 h-10 bg-deep-purple-300 rounded-lg items-center",
                        onclick: move |_| join_server(),
                        "Join Server"
                    }
                }
            }
        },
    }
}

async fn join_server() {}
