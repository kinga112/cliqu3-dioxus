use dioxus::prelude::*;

use crate::{
    components::server::channel::{BottomBar, MembersList, Messages},
    modules::xmtp::xmtp::TextChannel as TextChannelType,
    states::server_states::{CURRENT_SERVER, CURRENT_TEXT_CHANNEL},
};

const USERS: Asset = asset!("/assets/icons/users.svg");

#[component]
pub fn TextChannel() -> Element {
    let text_channel = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("text channel is none?");
    // let text_channel = use_context_provider(|| {
    //     CURRENT_TEXT_CHANNEL
    //         .read()
    //         .clone()
    //         .expect("text channel is none?")
    // });

    let mut show_users = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col h-screen w-full bg-off-black-500",
            div {
                class: "flex h-14 border-b z-10 border-off-black-700 shadow-md shadow-off-black-700 justify-between place-items-center px-3 shrink-0 text-4xl font-extralight",
                "{text_channel.metadata.name}"
                div {
                    class: "flex",
                    button {
                        class: "flex justify-center place-items-center h-10 w-10 rounded-md bg-deep-purple-300",
                        onclick: move |_| show_users.toggle(),
                        img {
                            src: USERS,
                            height: 30,
                            width: 30,
                        }
                    }
                }
            }
            div {
                class: "flex w-full h-full",
                div {
                    class: "flex flex-col h-[calc(100vh-56px)] w-full overflow-hidden",
                    Messages{}
                    BottomBar{id: text_channel.metadata.id}
                }
                if *show_users.read() {
                    MembersList{}
                }
            }
        }
        // if *show_users.read() {
        //     MembersList{}
        // }
    }
}
