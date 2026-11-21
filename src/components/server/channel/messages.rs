use crate::{
    components::server::channel::message::NewMessage, state::data_types::server::TextChannelState,
};
use dioxus::prelude::*;

#[component]
pub fn Messages() -> Element {
    // use_context_provider(|| Signal::new(None::<String>));
    let channel_state = use_context::<TextChannelState>();
    // let fake_signal = use_signal(|| true);
    let padding_bottom = 36 * channel_state.outbox_messages.read().len() + 24;
    let messages = channel_state.messages.read();
    let rendered_messages = messages.iter().enumerate().map(|(i, msg)| {
        let previous_message_timestamp = if i > 0 && messages[i].from == messages[i - 1].from {
            Some(messages[i - 1].timestamp)
        } else {
            None
        };

        rsx! {
            // Message {
            //     key: "{msg.id}",
            //     message: msg.clone(),
            //     previous_message_timestamp: previous_message_timestamp,
            // }
            NewMessage {
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
                {rendered_messages}
            }
        }
    }
}
