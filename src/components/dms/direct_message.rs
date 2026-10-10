use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    components::server::channel::{BottomBar, Messages},
    states::{dm_states::CURRENT_DIRECT_MESSAGE, user_states::USER},
};

#[component]
pub fn DirectMessage() -> Element {
    let dm_option = CURRENT_DIRECT_MESSAGE.read().clone();
    let id = dm_option.expect("dm is none?").metadata.id;

    let members = CURRENT_DIRECT_MESSAGE
        .read()
        .clone()
        .expect("failed to get current dm")
        .members
        .into_iter()
        .map(|(inbox_id, member)| {
            println!("MEMBER IN DM: {:?}", member.clone());
            rsx! {
                if USER.read().clone().profile.expect("failed to get user profile").address != member.address{
                    div {
                        class: "flex place-items-center text-xl h-14 p-2 border-b-2 border-off-black-300",
                        "{member.address}"
                    }
                }
            }
        });

    rsx! {
        div {
            class: "flex flex-col overflow-hidden h-full w-full bg-off-black-500",
            div {
                class: "h-fit",
                {members}
            }
            div {
                class: "flex flex-col overflow-hidden h-full",
                Messages{}
                BottomBar {id: id}
            }
        }
    }
}
