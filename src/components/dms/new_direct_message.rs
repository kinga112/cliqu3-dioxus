use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    components::server::channel::{BottomBar, Messages},
    states::dm_states::CURRENT_DIRECT_MESSAGE,
};

#[component]
pub fn NewDirectMessage() -> Element {
    let mut recipient = use_signal(|| String::new());

    rsx! {
        div {
            class: "relative flex flex-col overflow-hidden h-full w-full bg-off-black-500",
            div {
                class: "flex gap-1 h-14 border-b z-10 p-2 border-off-black-700 justify-start shadow-md shadow-off-black-700 place-items-center",
                input {
                    class: "flex-1 p-2 h-full rounded-lg focus:outline-none bg-off-black-600 min-w-0",
                    placeholder: "Recipient Address - [Enter] to select user",
                    oninput: move |evt| recipient.set(evt.value()),
                    onkeydown: {
                        move |evt| {
                            if evt.key() == Key::Enter && !evt.modifiers().shift() {
                                evt.prevent_default();
                                // let id = id.clone();
                                spawn(create_new_dm(recipient.read().clone()));
                            }
                        }
                    }
                    // onchange: ,
                }
            }
            div {
                class: "flex flex-col overflow-hidden",
                // Messages{}
                // BottomBar {id: recipient.read()}
            }
        }
    }
}

async fn create_new_dm(address: String) {
    println!("Creating new id with address: {:?}", address.clone());
    let app_state = APP_STATE.lock().await;
    let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
    let recipient = Recipient::Address(address);
    let convo = xmtp.client.dm(&recipient).expect("failed to create new dm");
    let dm = xmtp
        .get_conversation(&convo.id())
        .await
        .expect("failed to get dm text channel");
    *CURRENT_DIRECT_MESSAGE.write() = Some(dm);
    // let a = self.client.dm
}
