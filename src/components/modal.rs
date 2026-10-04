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
pub fn Modal(open: Signal<bool>, children: Element) -> Element {
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
                    class: "flex flex-col max-w-lg items-center gap-5 bg-deep-purple-300 p-10 rounded-2xl overflow-hidden",
                    onclick: move |evt| evt.stop_propagation(),
                    {children}
                }
            }
        }
    }
}
