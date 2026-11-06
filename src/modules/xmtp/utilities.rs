use crate::modules::xmtp::xmtp::Msg;
use std::collections::HashMap;
use xmtp::{
    Message,
    content::{Content, ReactionAction},
};

///
/// Utility functions for Xmtp that do not require client calls
///

// Get Msg type used in Cliqu3 UI
// (Reactions are HashMap, not a vec in Xmtp Message and removed other bloat)
pub fn get_msg(message: Message) -> Result<Msg, String> {
    let msg: Msg;
    let timestamp = message.sent_at_ns;
    let content = message.decode().expect("failed to decode message");
    let mut reactions: HashMap<String, Vec<String>> = HashMap::new();
    for reaction in message.reactions {
        println!("REACTION: {:?}", reaction.content);
        if reactions.get(&reaction.content).is_some() {
            if reaction.action == ReactionAction::Added {
                reactions
                    .entry(reaction.content)
                    .or_insert(Vec::new())
                    .push(reaction.sender_inbox_id);
            } else {
                if let Some(users) = reactions.get_mut(&reaction.content) {
                    users.retain(|user| user != &reaction.sender_inbox_id);
                }
            }
        } else {
            reactions.insert(reaction.content, vec![reaction.sender_inbox_id]);
        }
    }
    msg = Msg {
        id: message.id,
        content: content,
        from: message.sender_inbox_id,
        reactions: reactions,
        timestamp: timestamp,
    };
    Ok(msg)
}

// Get the String of the Msg Content, used for UI when Content needs to be broken down to just the text
pub fn get_msg_text(content: Content) -> String {
    match content {
        Content::Text(text) => text,
        Content::Reply(reply) => {
            let content_string = String::from_utf8(reply.content.content)
                .expect("failed to convert reply content to string");
            content_string
        }
        Content::Reaction(reaction) => reaction.content,
        Content::Attachment(attachment) => attachment.filename.expect("no file name?"),
        Content::RemoteAttachment(attachment) => attachment.filename.expect("no remote file name?"),
        Content::Markdown(markdown) => markdown,
        Content::Unknown { content_type, raw } => content_type,
        Content::ReadReceipt => "".to_string(),
    }
}
