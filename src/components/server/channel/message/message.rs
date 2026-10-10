use chrono::{DateTime, Local};
use std::sync::Arc;

use dioxus::prelude::*;
use xmtp::content::{Attachment, Content, Reaction, ReactionAction, RemoteAttachment, Reply};

use crate::{
    components::server::channel::message::{
        Interactions,
        content::{AttachmentContent, BasicContent, InviteContent, ReplyContent},
    },
    modules::xmtp::xmtp::{Msg, TextChannel},
    states::{server_states::MESSAGES, user_states::USER},
};

#[component]
pub fn Message(message: Msg, previous_message_timestamp: Option<i64>) -> Element {
    // let mut message_context = use_context_provider(|| Signal::new(message));
    provide_context(message.id);
    // use_effect(use_reactive(&message, move |message| {
    //     message_context.set(message);
    // }));
    //
    use_context_provider(move || previous_message_timestamp);

    // return content component
    let content = match message.content.clone() {
        Content::Text(text) => {
            // println!("content text: {:?}", text);
            // rsx! { BasicContent{ pre{"{text}"}} }
            rsx! { BasicContent{"{text}"}}
        }
        Content::Attachment(attachment) => {
            // let a = Attachment { filename: (), mime_type: (), data: () };
            rsx! { AttachmentContent{
                filename: attachment.filename,
                mime_type: attachment.mime_type,
                data: attachment.data,
                }
            }
        }
        Content::Reaction(reaction) => {
            println!("GOT A REACTION 2: {:?}", reaction.content);
            rsx! {}
        }
        Content::Reply(reply) => {
            let content_string = String::from_utf8(reply.content.content)
                .expect("failed to convert reply content to string");
            rsx! {
                ReplyContent{
                    content: content_string,
                    reference: reply.reference,
                    reference_inbox_id: reply.reference_inbox_id
                }
            }
        }
        Content::Markdown(markdown) => {
            println!("GOT A MARKDOWN");
            rsx! { div { "blank" }}
        }
        Content::RemoteAttachment(remote_attachment) => {
            println!("GOT A REMOTE ATTACHMENT");
            rsx! { div { "blank" }}
        }
        Content::ReadReceipt => {
            println!("GOT A READ REC");
            rsx! { div { "blank" }}
        }
        Content::Unknown { content_type, raw } => {
            println!("GOT A UNKNOWN");
            if content_type == "cliqu3.com/invite:1.0".to_string() {
                rsx! { InviteContent{raw} }
            } else {
                rsx! { BasicContent{"Start of new Direct Message!"} }
            }
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
