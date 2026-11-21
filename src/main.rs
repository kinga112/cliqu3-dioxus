use crate::state::app_state::AppState;
use crate::state::data_types::server::{ServerState, TextChannelState};
use crate::state::data_types::user::AuthState;
use crate::views::{Home, Login, Splash};
use dioxus::prelude::*;
use dioxus_desktop::wry::dpi::Size;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod components;
mod config;
mod modules;
mod smart_contract;
mod state;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // dioxus::launch(App);
    let size = LogicalSize::new(1200.00, 600.00);
    let min_size = Size::Logical(size);
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_title("cliqu3")
                    .with_min_inner_size(min_size),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut app_state = use_context_provider(|| AppState::default());
    let _server_state = use_context_provider::<ServerState>(|| ServerState::default());
    let _channel_state = use_context_provider::<TextChannelState>(|| TextChannelState::default());

    use_effect(move || {
        spawn(async move {
            app_state.check_saved_user().await;
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "overflow-hidden",
            match *app_state.auth_state.read() {
                AuthState::Unauthenticated => rsx! { Login {} },
                AuthState::Authenticated => rsx! { Home {} },
                _ => rsx! { Splash {} },
            }
        }
    }
}
