use crate::components::login::WalletConnectModal;
use dioxus::prelude::*;

#[component]
pub fn WalletConnectButton() -> Element {
    let mut open_modal = use_signal(|| false);

    rsx! {
        button {
            class: "h-16 w-42 rounded-lg bg-deep-purple-100 text-deep-purple-300 hover:text-deep-purple-500 text-xl cursor-pointer",
            onclick: move |_| open_modal.set(true),
            "Connect Wallet"
        }
        WalletConnectModal { open: open_modal }
    }
}
