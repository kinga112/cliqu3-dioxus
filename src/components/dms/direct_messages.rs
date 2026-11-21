use dioxus::prelude::*;

use crate::{
    components::dms::{DirectMessage, NewDirectMessage, nav::DirectMessageNav},
    state::data_types::server::TextChannelState,
};

#[component]
pub fn DirectMessages() -> Element {
    let channel_state = use_context::<TextChannelState>();
    rsx! {
        div {
            class: "flex w-full h-full",
            DirectMessageNav{}
            if channel_state.metadata.read().id != "".to_string() {
                DirectMessage{}
            }else{
                NewDirectMessage{}
            }
        }
    }
}
