use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{modules::xmtp::xmtp::XMTP_STATE, state::data_types::server::TextChannelState};

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
    let mut channel_state = use_context::<TextChannelState>();
    println!("Creating new id with address: {:?}", address.clone());
    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 4?");
    let recipient = Recipient::Address(address);
    let convo = xmtp.client.dm(&recipient).expect("failed to create new dm");
    let dm = xmtp
        .get_conversation(&convo.id())
        .await
        .expect("failed to get dm text channel");
    // *CURRENT_DIRECT_MESSAGE.write() = Some(dm);
    // *CURRENT_DIRECT_MESSAGE.write() = Some(dm);
    *channel_state.outbox_messages.write() = Vec::new();
    *channel_state.reply.write() = None;
    *channel_state.messages.write() = dm.messages.clone();
    *channel_state.members.write() = dm.members.clone();
    *channel_state.metadata.write() = dm.metadata.clone();
    // let a = self.client.dm
}
