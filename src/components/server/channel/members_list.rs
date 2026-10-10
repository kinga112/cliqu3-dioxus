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

    let default_avatar = "https://png.pngtree.com/thumb_back/fh260/background/20230727/pngtree-aesthetic-liquid-purple-background-image_12761619.jpg";
    let members = text_channel.members.into_iter().map(|(inbox_id, member)| {
        let mut open_user_info = use_signal(|| false);
        let avatar = member.clone().avatar;
        rsx! {
            div {
                class: "relative",
                button {
                    class: "flex place-items-center gap-2 hover:bg-deep-purple-300 rounded w-full p-2",
                    onclick: move |_| open_user_info.set(true),
                    img {
                        // should Avatar have rounded-xl??
                        class: "w-8 h-8 bg-deep-purple-100 rounded-lg object-cover shrink-0 select-none pointer-events-none",
                        src: if avatar != "" {
                            "{avatar}"
                        }else{
                            "{default_avatar}"
                        }
                    }
                    p {
                        class: "truncate",
                        "{member.address}"
                    }
                }
                div{
                    class: "absolute right-44 top-0",
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
