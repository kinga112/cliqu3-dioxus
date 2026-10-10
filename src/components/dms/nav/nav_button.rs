use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    app_state::APP_STATE,
    states::dm_states::{CURRENT_DIRECT_MESSAGE, DM_MESSAGES},
};

#[component]
pub fn DirectMessageNavButton(id: String, from: String, message: String) -> Element {
    rsx! {
        button {
            class: "bg-off-black-500 px-2 py-1 w-full shrink hover:bg-off-black-400 rounded-lg",
            onclick: move |_| set_current_dm(id.clone()),
            div {
                class: "flex gap-4 place-items-center",
                div {
                    class: "w-12 h-12 bg-blue-300 rounded-md shrink-0 object-cover select-none",
                }
                // img {
                //     class: "w-12 h-12 rounded-md shrink-0 object-cover select-none",
                //     src: ""
                // }
                div {
                    class: "flex flex-col w-32",
                    p { class: "truncate w-full text-xl text-left",
                        "{from}"
                    }
                    p { class: "truncate font-light w-full text-white text-opacity-50 text-left",
                        "{message}"
                    }
                }
            }
        }
    }
}

async fn set_current_dm(id: String) {
    let app_state = APP_STATE.lock().await;
    let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
    // let convo = xmtp
    //     .client
    //     .conversation(&id)
    //     .expect("failed to get convo by id")
    //     .expect("convo with id does not exist");
    let dm = xmtp
        .get_conversation(&id)
        .await
        .expect("failed to get dm text channel");

    println!("NEW DM MEMBERS: {:?}", dm.members.clone());

    *DM_MESSAGES.write() = dm.messages.clone();
    *CURRENT_DIRECT_MESSAGE.write() = Some(dm);
    // let a = xmtp.get_conversation(&id).expect("failed to get text channel");
}
