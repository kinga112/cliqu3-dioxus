use std::time::Duration;

use dioxus::prelude::*;
use tokio::{sync::mpsc::UnboundedReceiver, time::Instant};
use tokio_util::sync::CancellationToken;
use xmtp::MessageEvent;

use crate::{
    app_state::APP_STATE,
    components::{
        dms::DirectMessages,
        server::Server,
        settings::Settings,
        sidenav::{SideNav, options_menu::CreateServerModal},
    },
    modules::xmtp::utilities,
    states::{
        dm_states::{CURRENT_DIRECT_MESSAGE, DM_MESSAGES},
        global_states::{CURRENT_SCREEN, Screen},
        server_states::{CURRENT_TEXT_CHANNEL, MESSAGES, OUTBOX_MESSAGES},
    },
};

/// The Home page components displays screen based by Screen enum by CURRENT_SCREEN signal
#[component]
pub fn Home() -> Element {
    use_future(move || async move {
        loop {
            let mut last_tick = Instant::now();
            let interval = Duration::from_secs(2);
            let (stream_tx, stream_rx) = tokio::sync::mpsc::unbounded_channel::<MessageEvent>();
            let cancellation_token = CancellationToken::new();
            let token = cancellation_token.clone();

            // Run init stream then release lock
            {
                let app_state = APP_STATE.lock().await;
                app_state
                    .xmtp
                    .as_ref()
                    .expect("xmtp is none??")
                    .init_stream(stream_tx, cancellation_token);
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

    rsx! {
        div {
            class: "flex relative bg-off-black-700 h-screen w-screen text-deep-purple-100 overflow-hidden",
            SideNav{}
            div {
                class: "w-full h-full",
                match *CURRENT_SCREEN.read() {
                    Screen::Server => rsx! {
                        Server{}
                    },
                    Screen::DirectMessages => rsx! { DirectMessages{} },
                    Screen::Settings => rsx! { Settings{} },
                }
            }
            CreateServerModal{}
        }

    }
}

pub async fn handle_stream_events(mut stream_rx: UnboundedReceiver<MessageEvent>) {
    while let Some(event) = stream_rx.recv().await {
        let mut current_convo_id = "".to_string();
        match *CURRENT_SCREEN.peek() {
            Screen::DirectMessages => {
                if let Some(dm) = CURRENT_DIRECT_MESSAGE.peek().as_ref() {
                    current_convo_id = dm.metadata.id.clone();
                }
            }
            Screen::Server => {
                if let Some(text_channel) = CURRENT_TEXT_CHANNEL.peek().as_ref() {
                    current_convo_id = text_channel.metadata.id.clone();
                }
            }
            Screen::Settings => {}
        }
        if event.conversation_id == current_convo_id {
            let app_state = APP_STATE.lock().await;
            let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
            let message = xmtp.get_message(&event.message_id);
            drop(app_state); // drop app_state as soon client does not need to be called
            let msg = utilities::get_msg(message).expect("failed to convert message to msg");
            // A delay is needed because sometimes the message event from stream
            // arrives faster than the message_id is added to the Outbox
            // Outbox message is removed by message_id
            let _ = tokio::task::spawn_blocking(move || Duration::from_millis(50)).await;
            match *CURRENT_SCREEN.peek() {
                Screen::DirectMessages => {
                    DM_MESSAGES.write().push(msg);
                }
                Screen::Server => {
                    MESSAGES.write().push(msg);
                }
                Screen::Settings => {}
            }

            let index = OUTBOX_MESSAGES
                .peek()
                .iter()
                .position(|outbox_message| *outbox_message.message_id == event.message_id);

            if index.is_some() {
                OUTBOX_MESSAGES.write().remove(index.unwrap());
            }
        }
    }
}
