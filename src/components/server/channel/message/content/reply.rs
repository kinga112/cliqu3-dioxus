use std::collections::HashMap;

use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    modules::xmtp::{
        utilities,
        xmtp::{Msg, XMTP_STATE},
    },
    state::data_types::{server::TextChannelState, user::MemberProfile},
};

const CARROT: Asset = asset!("/assets/icons/carrot.svg");

#[component]
pub fn ReplyContent(
    // message: Msg,
    text: String,
    reference: String,
    reference_inbox_id: Option<String>,
) -> Element {
    // let state = use_context::<NewAppState>();
    // let server_state = use_context::<ServerState>();
    let channel_state = use_context::<TextChannelState>();
    let mut referenced_message: Signal<Option<Msg>> = use_signal(|| None);

    use_effect(move || {
        let owned_ref = reference.to_owned();
        spawn(async move {
            // let app_state = APP_STATE.lock().await;
            // let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
            let xmtp_opt = XMTP_STATE.lock().await;
            let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 4?");
            let message = xmtp
                .client
                .message_by_id(&owned_ref)
                .expect("no message with that reference")
                .expect("failed to get message");
            let msg = utilities::get_msg(message).expect("couldnt get Msg from Message");
            referenced_message.set(Some(msg));
        });
    });

    let empty_msg = Msg {
        id: "".to_string(),
        content: Content::Text("".to_string()),
        from: "".to_string(),
        reactions: HashMap::new(),
        timestamp: 0,
    };

    let msg = referenced_message
        .read()
        .as_ref()
        .unwrap_or_else(|| &empty_msg)
        .clone();

    // let members;
    // match *state.current_screen.read() {
    //     Screen::Server => {
    //         members = channel_state.members.peek().clone();
    //     }
    //     Screen::DirectMessages => {
    //         let text_channel = CURRENT_DIRECT_MESSAGE
    //             .peek()
    //             .clone()
    //             .expect("text channel is none?");
    //         members = text_channel.members.clone()
    //     }
    //     Screen::Settings => {
    //         members = channel_state.members.peek().clone();
    //     }
    // }

    let mut profile = MemberProfile {
        address: msg.from.clone(),
        avatar: String::new(),
        name: String::new(),
        description: String::new(),
    };

    // let profile = channel_state
    //     .members
    //     .peek()
    //     .get(&msg.from.clone())
    //     .unwrap_or_else(|| &temp_member_profile);
    if let Some(p) = channel_state.members.peek().get(&msg.from) {
        profile = p.clone();
    }

    let content_string = match msg.content {
        Content::Text(text) => text,
        _ => "Other Text???".to_string(),
    };

    rsx! {
        div {
            class: "flex flex-col bg-off-black-400 rounded-sm p-1 mb-1",
            div { class: "flex items-center gap-1 text-xs",
                div {
                    class: "w-14 justify-center place-items-center",
                    img {
                        class: "select-none",
                        src: CARROT,
                        height: 10,
                        width: 15,
                    }
                }
                span {
                    class: "font-medium text-deep-purple-200 hover:underline cursor-pointer",
                    "{profile.address}"
                }
                span {
                    class: "text-zinc-400 truncate max-w-[200px] sm:max-w-xs",
                    "{content_string}"
                }
            }
        }
    }
}
