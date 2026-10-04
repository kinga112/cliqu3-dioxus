use dioxus::prelude::*;
use xmtp::content::Content;

use crate::{
    app_state::APP_STATE, components::dms::nav::DirectMessageNavButton, states::user_states::USER,
};

const ADD: Asset = asset!("/assets/icons/add-cropped.svg");

#[component]
pub fn DirectMessageNav() -> Element {
    let mut dms = use_signal(|| Vec::new());

    use_effect(move || {
        spawn(async move {
            let app_state = APP_STATE.lock().await;
            let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
            dms.set(xmtp.get_dms().expect("failed to get dms"));
        });
    });

    let dms_list = dms.read();
    let dm_items = dms_list.iter().map(|convo| {
        let mut from = String::new();
        let members = convo.members().expect("failed to get members");
        for member in members {
            let address = member
                .account_identifiers
                .get(0)
                .map(|id| id.to_string())
                .expect("failed to get address");
            if USER
                .read()
                .clone()
                .profile
                .expect("failed to get user profile")
                .address
                != address
            {
                from = address;
            }
        }
        let last_message = convo
            .last_message()
            .expect("couldnt get last message")
            .expect("no last message");
        let content = match last_message
            .decode()
            .expect("failed to decode last message")
        {
            Content::Text(text) => text,
            Content::Reply(reply) => {
                let content_string = String::from_utf8(reply.content.content)
                    .expect("failed to convert reply content to string");
                content_string
            }
            Content::Reaction(reaction) => reaction.content,
            Content::Attachment(attachment) => {
                format!("Attachment: {}", attachment.mime_type)
            }
            Content::Markdown(markdown) => "markdown".to_string(),
            Content::RemoteAttachment(remote_attachment) => "remote attachment".to_string(),
            Content::ReadReceipt => "read receipt".to_string(),
            Content::Unknown { content_type, raw } => {
                println!("CONTENT TYPE: {:?}", content_type);
                format!("{}", content_type)
            }
        };
        rsx! { DirectMessageNavButton{id: convo.id(), from: from, message: content}}
    });

    rsx! {
        div {
            class: "flex flex-col gap-2 w-56 shrink-0 bg-off-black-600 overflow-hidden",
            div {
                class: "flex justify-between place-items-center h-14 border-b z-10 border-off-black-700 shadow-md shadow-off-black-700 shrink-0 p-2",
                div {
                    class: "text-2xl font-light",
                    "Direct Messages"
                }
                button {
                    class: "bg-off-black-400 hover:bg-off-black-300 p-2 rounded-md",
                    img {
                        src: ADD,
                        width: 15,
                        height: 15,
                    }
                }
            }
            div{
                class: "px-2",
                {dm_items}
            }
        }
    }
}
