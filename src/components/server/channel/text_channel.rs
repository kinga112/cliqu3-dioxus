use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    components::server::channel::{BottomBar, MembersList, Messages},
    modules::xmtp::xmtp::TextChannel as TextChannelType,
    smart_contract,
    states::{
        server_states::{CURRENT_SERVER, CURRENT_TEXT_CHANNEL, MEMBERS},
        user_states::MemberProfile,
    },
};

const USERS: Asset = asset!("/assets/icons/users.svg");

#[component]
pub fn TextChannel() -> Element {
    let text_channel = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("text channel is none?");

    // // Add Cliqu3 Profiles to HashMap in Background for quicker load
    // spawn(async {
    //     let mut members_map = HashMap::new();
    //     let members = MEMBERS.peek().clone();
    //     for (inbox_id, member) in members.iter() {
    //         match smart_contract::interact::get_profile(&member.address).await {
    //             Ok(profile) => {
    //                 members_map.insert(inbox_id.clone(), profile);
    //             }
    //             Err(_) => {
    //                 members_map.insert(inbox_id.clone(), member.clone());
    //             }
    //         }
    //     }

    //     *MEMBERS.write() = members_map;
    // });

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
