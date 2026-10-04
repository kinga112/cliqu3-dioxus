use dioxus::prelude::*;

use crate::{
    components::server::channel::{OutboxBanner, message::Message},
    modules::xmtp::xmtp::TextChannel,
    states::{
        dm_states::DM_MESSAGES,
        global_states::{CURRENT_SCREEN, Screen},
        server_states::{MESSAGES, OUTBOX_MESSAGES},
    },
};

#[component]
pub fn Messages() -> Element {
    let padding_bottom = 36 * OUTBOX_MESSAGES.read().len() + 24;
    let messages;
    // if *CURRENT_SCREEN.read() == Screen::DirectMessages {
    //     println!("USING DM MESSAGES");
    //     messages = DM_MESSAGES.read();
    // }
    match *CURRENT_SCREEN.read() {
        Screen::Server => {
            messages = MESSAGES.read();
        }
        Screen::DirectMessages => {
            messages = DM_MESSAGES.read();
        }
        Screen::Settings => {
            messages = MESSAGES.read();
        }
    }

    let rendered_messages = messages.iter().enumerate().map(|(i, msg)| {
        let previous_message_timestamp = if i > 0 && messages[i].from == messages[i - 1].from {
            Some(messages[i - 1].timestamp)
        } else {
            None
        };

        rsx! {
            Message {
                key: "{msg.id}",
                message: msg.clone(),
                previous_message_timestamp: previous_message_timestamp,
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col-reverse w-full h-full overflow-y-auto overflow-x-hidden px-5",
            div {
                class: "flex flex-col pt-4 w-full ",
                style: "padding-bottom: {padding_bottom}px;",
                // style: "padding-bottom: 52px;",
                {rendered_messages}
            }
        }
        // div {
            // class: "h-3 shrink-0"
        // }
    }
}
