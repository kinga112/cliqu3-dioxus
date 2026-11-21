use chrono::{DateTime, Local};
use dioxus::prelude::*;

use crate::{
    components::server::channel::message::{DisplayName, Reactions},
    modules::xmtp::xmtp::Msg,
    state::data_types::{server::TextChannelState, user::MemberProfile},
};

#[component]
pub fn GenericContent(children: Element) -> Element {
    let channel_state = use_context::<TextChannelState>();
    let message = use_context::<Msg>();

    let date = DateTime::from_timestamp_nanos(message.timestamp);
    let local: DateTime<Local> = DateTime::from(date);
    let formated_date = local.format("%B %d, %Y %l:%M %p").to_string();

    let avatar = "https://png.pngtree.com/thumb_back/fh260/background/20230727/pngtree-aesthetic-liquid-purple-background-image_12761619.jpg";

    let profile = match channel_state.members.peek().get(&message.from) {
        Some(profile) => profile.clone(),
        None => MemberProfile {
            address: message.from.clone(),
            name: "".to_string(),
            avatar: avatar.to_string(),
            description: "".to_string(),
        },
    };

    rsx! {
        div {
            class: "flex place-items-start gap-2 w-full h-full",
            img {
                // should Avatar have rounded-xl??
                class: "w-14 h-14 bg-deep-purple-100 rounded-xl object-cover shrink-0 select-none pointer-events-none",
                src: if profile.avatar != "".to_string() {
                    "{profile.avatar}"
                }else{
                    "{avatar}"
                }
            }
            div {
                class: "flex flex-col w-full",
                div {
                    class: "flex flex-row place-items-center gap-2",
                    DisplayName{profile: profile.clone()}
                    div {
                        class: "text-xxs font-semibold select-none",
                        "{formated_date}"
                    }
                }
                {children}
                Reactions{}
                // Interactions{}
            }
        }
    }
}

/// This Content is a standard Message block that does not display
/// user profile or display name
#[component]
pub fn GenericShortContent(children: Element) -> Element {
    let message = use_context::<Msg>();

    let date = DateTime::from_timestamp_nanos(message.timestamp);
    let local: DateTime<Local> = DateTime::from(date);
    let formated_date = local.format("%l:%M %p").to_string();

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
                // Interactions{}
            }
        }
    }
}
