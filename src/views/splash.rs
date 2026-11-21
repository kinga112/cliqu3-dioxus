use dioxus::prelude::*;

use crate::{
    modules::{docs::db::DB, xmtp::xmtp::XMTP_STATE},
    smart_contract::interact::fetch_profiles,
    state::{
        app_state::AppState,
        data_types::{
            server::{ServerState, TextChannelState},
            user::AuthState,
        },
        saved_state::SAVED_STATE_HANDLER,
    },
};

#[component]
pub fn Splash() -> Element {
    let mut app_state = use_context::<AppState>();
    let mut server_state = use_context::<ServerState>();
    let mut channel_state = use_context::<TextChannelState>();

    let auth_state = app_state.auth_state.read().clone();
    if auth_state == AuthState::CheckingSavedState {
        use_future(move || async move {
            let mut user = app_state.user.peek().clone();
            let saved_state_handler = SAVED_STATE_HANDLER
                .read()
                .expect("saved state handler is none")
                .clone()
                .expect("failed to read saved state handler ");
            match saved_state_handler.get_state() {
                Ok(saved_state) => {
                    user.audio = saved_state.user_config.audio;
                    user.video = saved_state.user_config.video;
                    user.silence = saved_state.user_config.silence;
                    let db = DB
                        .read()
                        .expect("failed to read DB")
                        .clone()
                        .expect("failed to get db");
                    if saved_state.last_server != "".to_string() {
                        let server = db
                            .get_server(&saved_state.last_server)
                            .await
                            .expect("failed to get server");
                        println!("SERVER NAME: {:?}", server.metadata.name);
                        let xmtp_opt = XMTP_STATE.lock().await;
                        let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here? 10");
                        let text_channel = xmtp
                            .get_conversation(&saved_state.last_text_channel)
                            .await
                            .expect("failed to get text channel");

                        fetch_profiles();
                        *channel_state.messages.write() = text_channel.messages.clone();
                        *channel_state.members.write() = text_channel.members.clone();
                        *channel_state.metadata.write() = text_channel.metadata.clone();
                        *server_state.text_channels.write() = server.text_channels.clone();
                        *server_state.voice_channels.write() = server.voice_channels.clone();
                        *server_state.metadata.write() = server.metadata;
                    }
                    *app_state.current_screen.write() = saved_state.last_screen;
                    *app_state.user.write() = user;
                    *app_state.auth_state.write() = AuthState::Authenticated;
                }
                Err(e) => {
                    *app_state.auth_state.write() = AuthState::Authenticated;
                }
            }
        });
    }

    rsx! {
        div {
            class: "flex justify-center place-items-center w-screen h-screen bg-off-black-500 text-deep-purple-100 text-8xl",
            "C L I Q U 3"
        }
    }
}
