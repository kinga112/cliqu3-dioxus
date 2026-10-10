use dioxus::prelude::*;

use crate::{
    components::user::UserInfo,
    states::user_states::{MemberProfile, USER},
};

#[component]
pub fn DisplayName(profile: MemberProfile) -> Element {
    // let user = USER.read().clone();
    // let profile = user.profile.expect("user profile is null");
    let mut open_user_info = use_signal(|| false);
    rsx! {
        div {
            class: "relative",
            button {
                class: "text-lg font-semibold hover:underline hover:text-deep-purple-200",
                onclick: move |_| open_user_info.set(true),
                if profile.name.clone() != "" {
                    "{profile.name}"
                }else{
                    "{profile.address}"
                }
            }
            div{
                class: "absolute left-full top-0 ml-2",
                UserInfo{profile: profile, open: open_user_info}
            }
        }
    }
}
