use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Local};
use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    components::server::channel::message::{DisplayName, Interactions, Reactions},
    modules::xmtp::xmtp::Msg,
    states::{
        dm_states::DM_MESSAGES,
        global_states::{CURRENT_SCREEN, Screen},
        server_states::MESSAGES,
        user_states::USER,
    },
};

#[component]
pub fn BasicContent(children: Element) -> Element {
    // let message = use_context::<Signal<Arc<Msg>>>();
    // let message = use_context::<Signal<Msg>>();
    let msg_id = use_context::<String>();
    // let messages = MESSAGES.read().clone();
    let messages;
    match *CURRENT_SCREEN.read() {
        Screen::Server => {
            messages = MESSAGES.read().clone();
        }
        Screen::DirectMessages => {
            messages = DM_MESSAGES.read().clone();
        }
        Screen::Settings => {
            messages = MESSAGES.read().clone();
        }
    }

    let mut message = Msg {
        id: "".to_string(),
        content: Content::Text("".to_string()),
        from: "".to_string(),
        timestamp: 10000000000,
        reactions: HashMap::new(),
    };

    for msg in messages {
        if msg.id == msg_id {
            message = msg;
        }
    }
    // let msg = message.read().clone();
    let previous_message_timestamp = use_context::<Option<i64>>();
    let mut within_five = false;
    // println!("MESSAGE TIMESTAMP: {:?}", message.timestamp.clone());
    if previous_message_timestamp.is_some() {
        if message.timestamp - previous_message_timestamp.unwrap() <= 300_000_000_000 {
            within_five = true;
        }
    }

    // convert timestamp
    let date = DateTime::from_timestamp_nanos(message.timestamp);
    let local: DateTime<Local> = DateTime::from(date);
    let mut formated_date = local.format("%B %d, %Y %l:%M %p").to_string();

    let user = USER.read().clone();
    let profile = user.profile.expect("user profile is null");

    if within_five {
        formated_date = local.format("%l:%M %p").to_string();
        rsx! {
            div {
                class: "flex place-items-start gap-2 w-full h-full",
                div {
                    class: "flex w-14 h-6 justify-center place-items-center shrink-0",
                    div {
                        class: "text-xxs font-semibold select-none invisible group-hover:visible",
                        "{formated_date}"
                    }
                }
                div {
                    class: "flex flex-col w-full",
                    {children}
                    Reactions{}
                    Interactions{}
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "flex place-items-start gap-2 w-full h-full",
                img {
                    // should Avatar have rounded-xl??
                    class: "w-14 h-14 bg-deep-purple-100 rounded-xl object-cover shrink-0 select-none pointer-events-none",
                    src: "https://png.pngtree.com/thumb_back/fh260/background/20230727/pngtree-aesthetic-liquid-purple-background-image_12761619.jpg",

                }
                div {
                    class: "flex flex-col w-full",
                    div {
                        class: "flex flex-row place-items-center gap-2",
                        DisplayName{profile: profile}
                        div {
                            class: "text-xxs font-semibold select-none",
                            "{formated_date}"
                        }
                    }
                    {children}
                    Reactions{}
                }
            }
        }
    }
}
