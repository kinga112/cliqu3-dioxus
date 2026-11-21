use dioxus::prelude::*;
use dioxus_desktop::tao::event::{Event, WindowEvent};
use dioxus_desktop::{use_window, use_wry_event_handler};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{sync::mpsc::UnboundedReceiver, time::Instant};
use tokio_util::sync::CancellationToken;
use xmtp::MessageEvent;

use crate::state::app_state::Screen;
use crate::state::data_types::server::{ServerState, TextChannelState};
use crate::state::saved_state::{SavedAppState, UserConfig};
use crate::{
    components::{dms::DirectMessages, server::Server, settings::Settings, sidenav::SideNav},
    modules::{
        docs::db::DB,
        xmtp::{utilities, xmtp::XMTP_STATE},
    },
    smart_contract::interact::fetch_profiles,
    state::{app_state::AppState, saved_state::SAVED_STATE_HANDLER},
    // states::global_states::Screen,
};

/// The Home page components displays screen based by Screen enum by AppState signal
/// Home also starts/restarts the thread to handle xmtp stream events
/// Close app event listener also here
#[component]
pub fn Home() -> Element {
    let mut app_state = use_context::<AppState>();
    let mut server_state = use_context::<ServerState>();
    let mut channel_state = use_context::<TextChannelState>();

    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            println!(
                "
                *
                *
                *
                *
                *
                CLOSE EVENT???
                *
                *
                *
                *
                *
                "
            );
            let saved_state_handler = SAVED_STATE_HANDLER
                .read()
                .expect("failed 1")
                .clone()
                .expect("failed 2");

            let user = app_state.user.peek();
            println!("USER STATE BEFORE CLOSE: {:?}", user);
            let config = UserConfig {
                audio: user.audio,
                video: user.video,
                silence: user.silence,
            };
            let last_screen;
            match *app_state.current_screen.peek() {
                Screen::Server => {
                    last_screen = Screen::Server;
                }
                Screen::DirectMessages => {
                    last_screen = Screen::DirectMessages;
                }
                Screen::Settings => {
                    last_screen = Screen::Settings;
                }
            }

            let state = SavedAppState {
                user_config: config,
                last_screen: last_screen,
                last_server: server_state.metadata.peek().id.clone(),
                last_text_channel: channel_state.metadata.peek().id.clone(),
            };
            saved_state_handler.store_state(&state);
        }
    });

    use_effect(move || {
        use_future(move || async move {
            // let mut app_state = use_context::<AppState>();
            // let mut server_state = use_context::<ServerState>();
            // let mut channel_state = use_context::<TextChannelState>();
            let saved_state_handler = SAVED_STATE_HANDLER
                .read()
                .expect("state handler doesnt exist?")
                .clone()
                .expect("failed to read state handler");
            match saved_state_handler.get_state() {
                Ok(saved_state) => {
                    let db = DB
                        .read()
                        .expect("failed to read DB")
                        .clone()
                        .expect("failed to get db");
                    if saved_state.last_server != "".to_string() {
                        // let id = saved_state.last_server;
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
                }
                Err(e) => {}
            }
        });
    });

    use_effect(move || {
        use_future(move || async move {
            loop {
                let mut last_tick = Instant::now();
                let interval = Duration::from_secs(2);
                let (stream_tx, stream_rx) = tokio::sync::mpsc::unbounded_channel::<MessageEvent>();
                let cancellation_token = CancellationToken::new();
                let token = cancellation_token.clone();

                // Run init stream then release lock
                {
                    let xmtp_opt = XMTP_STATE.lock().await;
                    if let Some(xmtp) = xmtp_opt.as_ref() {
                        xmtp.init_stream(stream_tx, cancellation_token);
                    }
                }

                // Needs dioxus spawn to handle stream events to update message signals
                spawn(async {
                    handle_stream_events(stream_rx).await;
                });
                loop {
                    tokio::time::sleep(interval).await;
                    let now = Instant::now();
                    let elapsed = now.duration_since(last_tick);

                    // If 10 seconds passed on a 2-second loop, the PC slept
                    if elapsed > Duration::from_secs(10) {
                        token.cancel();
                        break;
                    }

                    last_tick = now;
                }
            }
        });
    });

    rsx! {
        div {
            class: "flex relative bg-off-black-700 h-screen w-screen text-deep-purple-100 overflow-hidden",
            SideNav{}
            div {
                class: "w-full h-full",
                match *app_state.current_screen.read() {
                    Screen::Server => rsx! {
                        Server{}
                    },
                    Screen::DirectMessages => rsx! { DirectMessages{} },
                    Screen::Settings => rsx! { Settings{} },
                }
            }
            // CreateServerModal{}
        }
    }
}

pub async fn handle_stream_events(mut stream_rx: UnboundedReceiver<MessageEvent>) {
    println!(
        "*
        *
        *
        STARTING XMTP STREAM HANDLE
        *
        *
        *"
    );
    let mut channel_state = use_context::<TextChannelState>();
    while let Some(event) = stream_rx.recv().await {
        println!("got stream event!: {:?}", event.message_id.clone());
        let current_convo_id = channel_state.metadata.peek().id.clone();
        if event.conversation_id == current_convo_id {
            let xmtp_opt = XMTP_STATE.lock().await;
            let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here?");
            let message = xmtp.get_message(&event.message_id);
            let msg = utilities::get_msg(message).expect("failed to convert message to msg");
            // A delay is needed because sometimes the message event from stream
            // arrives faster than the message_id is added to the Outbox
            // Outbox message is removed by message_id
            let _ = tokio::task::spawn_blocking(move || Duration::from_millis(50)).await;
            channel_state.messages.write().push(msg);

            let index = channel_state
                .outbox_messages
                .peek()
                .iter()
                .position(|outbox_message| *outbox_message.message_id == event.message_id);

            if let Some(i) = index {
                channel_state.outbox_messages.write().remove(i);
            }
        }
    }
}
