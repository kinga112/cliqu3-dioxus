use crate::app_state::APP_STATE;
use crate::modules::walletconnect::walletconnect::WalletConnectHandler;
use crate::states::user_states::AUTH_STATE;
use crate::states::user_states::AuthState;
use crate::states::user_states::USER;

use dioxus::prelude::*;
use rand::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;
use {
    alloy::hex,
    sha2::{Digest, Sha256},
};

use qrcode_generator::{
    Renderer,
    qr::{Encoder, ErrorCorrection},
};

use base64::{Engine, engine::general_purpose::STANDARD};

// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

#[component]
pub fn WalletConnectModal(open: Signal<bool>) -> Element {
    let mut svg = use_signal(String::new);
    // let mut state = use_context::<Signal<AppState>>();

    use_effect(move || {
        if open() {
            spawn(async move {
                // QR generation (sync, but fine inside the async block)
                let mut raw_key = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut raw_key);
                let sym_key_hex = hex::encode(raw_key);
                let mut hasher = Sha256::new();
                hasher.update(&raw_key);
                let pairing_topic_hex = hex::encode(hasher.finalize());
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let expiry = now + 3600;
                let methods = "[wc_sessionPropose]";
                let uri = format!(
                    "wc:{}@2?expiryTimestamp={}&relay-protocol=irn&symKey={}&methods={}",
                    pairing_topic_hex, expiry, sym_key_hex, methods
                );

                let symbol = Encoder::new(ErrorCorrection::Low)
                    .encode_text(uri.clone())
                    .unwrap();
                let svg_string = Renderer::new(&symbol, 512)
                    .to_svg_string(None::<&str>)
                    .unwrap();
                let encoded = STANDARD.encode(svg_string.as_bytes());
                svg.set(format!("data:image/svg+xml;base64,{encoded}"));

                // Now the actual WalletConnect async flow
                let mut wc = WalletConnectHandler::new(uri);
                let (xmtp_tx, xmtp_rx) = oneshot::channel();
                wc.init(xmtp_tx).await;

                let xmtp = xmtp_rx.await.expect("failed to get xmtp in receiver");
                let id = xmtp.client.inbox_id().expect("couldnt get xmtp inbox id");
                println!("GOT XMTP INBOX ID: {id:?}");

                let mut app_state = APP_STATE.lock().await;
                app_state.xmtp = Some(xmtp);
                let login_result = app_state.login();
                // state.xmtp = Some(Arc::new(xmtp));
                // state.write().xmtp = Some(Arc::new(xmtp));
                *AUTH_STATE.write() = AuthState::Authenticated;
                // USER.write().authorized = AuthState::Authenticated;
            });
        }
    });

    if !open() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "relative z-50 select-none",
            // backdrop
            div {
                class: "fixed inset-0 bg-black/50",
                onclick: move |_| open.set(false),
            }
            // center wrapper
            div {
                class: "fixed inset-0 flex w-screen items-center justify-center",
                onclick: move |_| open.set(false),
                // modal
                div {
                    class: "flex flex-col h-3/4 max-w-lg items-center gap-5 bg-deep-purple-200 p-10 rounded-4xl overflow-hidden",
                    onclick: move |evt| evt.stop_propagation(),
                    div {
                        class: "text-deep-purple-100 text-5xl text-center font-extralight",
                        "Scan with mobile wallet to Connect"
                    }
                    img {
                        class: "rounded-xl bg-white max-w-full max-h-full w-auto h-auto object-contain min-h-0 flex-1",
                        src: "{svg()}",
                    }
                }
            }
        }
    }
}
