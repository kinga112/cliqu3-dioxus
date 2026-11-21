use crate::{
    components::server::channel::{BottomBar, Messages},
    state::{app_state::AppState, data_types::server::TextChannelState},
};
use dioxus::prelude::*;

#[component]
pub fn DirectMessage() -> Element {
    let state = use_context::<AppState>();
    let channel_state = use_context::<TextChannelState>();
    // let dm_option = CURRENT_DIRECT_MESSAGE.read().clone();
    // let id = dm_option.expect("dm is none?").metadata.id;

    let members = channel_state.members
        .peek()
        .clone()
        // .expect("failed to get current dm")
        .into_iter()
        .map(|(inbox_id, member)| {
            // println!("MEMBER IN DM: {:?}", member.clone());
            rsx! {
                if state.user.read().profile.address != member.address {
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
                BottomBar {id: channel_state.metadata.read().id.clone()}
            }
        }
    }
}
