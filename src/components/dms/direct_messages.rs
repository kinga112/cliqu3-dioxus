use dioxus::prelude::*;

use crate::{
    components::dms::{DirectMessage, NewDirectMessage, nav::DirectMessageNav},
    states::dm_states::CURRENT_DIRECT_MESSAGE,
};

#[component]
pub fn DirectMessages() -> Element {
    rsx! {
        div {
            class: "flex w-full h-full",
            DirectMessageNav{}
            if CURRENT_DIRECT_MESSAGE.read().is_some(){
                DirectMessage{}
            }else{
                NewDirectMessage{}
            }
        }
    }
}
