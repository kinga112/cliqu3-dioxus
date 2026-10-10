use std::collections::HashMap;

use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    app_state::APP_STATE,
    components::server::channel::message::content::BasicContent,
    modules::xmtp::xmtp::Msg,
    states::{server_states::CURRENT_TEXT_CHANNEL, user_states::MemberProfile},
};

const CARROT: Asset = asset!("/assets/icons/carrot.svg");

#[component]
pub fn ReplyContent(
    content: String,
    reference: String,
    reference_inbox_id: Option<String>,
) -> Element {
    let mut msg_state: Signal<Option<Msg>> = use_signal(|| None);

    use_effect(move || {
        let owned_ref = reference.to_owned();
        spawn(async move {
            let app_state = APP_STATE.lock().await;
            let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
            let referenced_message = xmtp
                .client
                .message_by_id(&owned_ref)
                .expect("no message with that reference")
                .expect("failed to get message");
            let msg = xmtp
                .get_msg(referenced_message)
                .expect("couldnt get Msg from Message");
            msg_state.set(Some(msg));
        });
    });

    let empty_msg = Msg {
        id: "".to_string(),
        content: Content::Text("".to_string()),
        from: "".to_string(),
        reactions: HashMap::new(),
        timestamp: 0,
    };

    let msg = msg_state
        .read()
        .as_ref()
        .unwrap_or_else(|| &empty_msg)
        .clone();

    let text_channel = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("text channel is none?");

    let temp_member_profile = MemberProfile {
        address: String::new(),
        avatar: String::new(),
        name: String::new(),
        description: String::new(),
    };

    let profile = text_channel
        .members
        .get(&msg.from.clone())
        .unwrap_or_else(|| &temp_member_profile);

    let content_string = match msg.content {
        Content::Text(text) => text,
        _ => "Other Text???".to_string(),
    };

    rsx! {
        div { class: "flex flex-col bg-off-black-400 rounded-sm p-1 mb-1",
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
        BasicContent{"{content}"}
    }
}
