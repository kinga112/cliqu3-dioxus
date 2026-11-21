use dioxus::prelude::*;
use xmtp::content::{Attachment, Content, Reaction, ReactionAction, RemoteAttachment, Reply};

use crate::{
    components::server::channel::message::{
        Interactions,
        content::{
            AttachmentContent, GenericContent, GenericShortContent, InviteContent, ReplyContent,
        },
    },
    modules::xmtp::xmtp::Msg,
};

#[component]
pub fn Message(message: Msg, previous_message_timestamp: Option<i64>) -> Element {
    use_context_provider(|| message.clone());

    let mut within_five = false;
    if previous_message_timestamp.is_some() {
        if message.timestamp - previous_message_timestamp.unwrap() <= 300_000_000_000 {
            within_five = true;
        }
    }

    let content = match message.content.clone() {
        Content::Text(text) => {
            if within_five {
                rsx! { GenericShortContent {"{text}"}}
            } else {
                rsx! { GenericContent {"{text}"}}
            }
        }
        Content::Attachment(attachment) => {
            if within_five {
                rsx! {
                    GenericShortContent {
                        AttachmentContent{
                            filename: attachment.filename,
                            mime_type: attachment.mime_type,
                            data: attachment.data,
                        }
                    }
                }
            } else {
                rsx! {
                    GenericContent {
                        AttachmentContent{
                            filename: attachment.filename,
                            mime_type: attachment.mime_type,
                            data: attachment.data,
                        }
                    }
                }
            }
        }
        // Content::Reaction(reaction) => {
        //     println!("GOT A REACTION 2: {:?}", reaction.content);
        //     rsx! {}
        // }
        Content::Reply(reply) => {
            let text = String::from_utf8(reply.content.content)
                .expect("failed to convert reply content to string");
            rsx! {
                ReplyContent {
                    text: "".to_string(),
                    reference: reply.reference,
                    reference_inbox_id: reply.reference_inbox_id
                }
                if within_five {
                    GenericShortContent {"{text}"}
                } else {
                    GenericContent {"{text}"}
                }
                // TextContent{text, previous_message_timestamp}
            }
        }
        // Content::Markdown(markdown) => {
        //     println!("GOT A MARKDOWN");
        //     rsx! { div { "blank" }}
        // }
        // Content::RemoteAttachment(remote_attachment) => {
        //     println!("GOT A REMOTE ATTACHMENT");
        //     rsx! { div { "blank" }}
        // }
        // Content::ReadReceipt => {
        //     println!("GOT A READ REC");
        //     rsx! { div { "blank" }}
        // }
        Content::Unknown { content_type, raw } => {
            println!("GOT A UNKNOWN");
            if content_type == "cliqu3.com/invite:1.0".to_string() {
                rsx! { InviteContent{raw} }
            } else {
                if within_five {
                    rsx! { GenericShortContent {"Start of new Direct Message!"}}
                } else {
                    rsx! { GenericContent {"Start of new Direct Message!"}}
                }
            }
        }
        _ => {
            rsx! { GenericContent {"Nothing to show here!"}}
        }
    };

    rsx! {
        div {
            class: "flex-col w-full place-items-start relative p-1 hover:bg-off-black-300 gap-2 group rounded text-wrap break-words whitespace-pre-wrap",
            {content}
            Interactions{}
        }
    }
}
