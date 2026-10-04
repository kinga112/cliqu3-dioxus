use dioxus::prelude::*;

use crate::{
    components::user::UserInfo, modules::xmtp::xmtp::TextChannel,
    states::server_states::CURRENT_TEXT_CHANNEL,
};

#[component]
pub fn MembersList() -> Element {
    // let text_channel = use_context::<TextChannel>();
    let text_channel = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("text channel is none?");
    let members = text_channel.members.into_iter().map(|(inbox_id, member)| {
        let mut open_user_info = use_signal(|| false);
        rsx! {
            div {
                class: "relative",
                button {
                    class: "flex place-items-center gap-1 hover:bg-deep-purple-300 rounded w-full p-2",
                    onclick: move |_| open_user_info.set(true),
                    p {
                        class: "truncate",
                        "{inbox_id}"
                    }
                }
                div{
                    class: "absolute right-44 top-0 mr-96",
                    UserInfo{profile: member, open: open_user_info}
                }
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col z-10 w-48 bg-off-black-500 border-l-2 border-off-black-400 shrink-0 px-2 text-deep-purple-100",
            button {
                class: "flex flex-row text-sm pt-2 select-none group pointer-events-none place-items-center select",
                // img {
                //     class: "pointer-events-auto",

                // }
                div {
                    class: "group-hover:underline pointer-events-auto",
                    "members"
                }
                div {
                    class: "group-hover:underline pointer-events-auto",
                    " - {members.len()}"
                }
            }
            {members}
        }
    }
}
