use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use xmtp::content::{Content, ReactionAction};

use crate::{
    app_state::APP_STATE,
    components::server::channel::emojis::emoji_enum::Emoji,
    modules::xmtp::xmtp::{Msg, TextChannel},
    states::{
        server_states::{CURRENT_TEXT_CHANNEL, MESSAGES},
        user_states::USER,
    },
};

#[component]
pub fn EmojiElement(emoji: Emoji) -> Element {
    // let message = use_context::<Signal<Arc<Msg>>>();
    // let message = use_context::<Signal<Msg>>();
    let msg_id = use_context::<String>();
    let messages = MESSAGES.read().clone();
    let mut message = Msg {
        id: "".to_string(),
        content: Content::Text(("".to_string())),
        from: "".to_string(),
        timestamp: 10000000000,
        reactions: HashMap::new(),
    };

    for msg in messages {
        if msg.id == msg_id {
            message = msg;
        }
    }
    // let message = messages
    //     .iter()
    //     .find(|m| m.id == msg_id)
    //     .expect("did not find message!");
    // let text_channel = use_context::<TextChannel>();
    let text_channel = CURRENT_TEXT_CHANNEL
        .read()
        .clone()
        .expect("text channel is none?");
    let id = text_channel.metadata.id;
    // let text_input = use_context::<Signal<String>>();
    // println!("MESSAGE ARC in EMOJI ELEMENT??? {:?}", message.id);

    // fn on_click(emoji: Emoji, mut text_input: Signal<String>) {
    //     // if reference.is_some() {
    //     // Send Reaction
    //     // } else {
    //     let input = text_input.read().clone();
    //     text_input.set(format!("{}{}", input, emoji.as_str()));
    //     // }
    // }
    async fn on_click(emoji: Emoji, id: String, msg: Msg) {
        let app_state = APP_STATE.lock().await;
        // let msg = message.read();
        if msg.reactions.get(emoji.as_str()).is_some() {
            let users = msg.reactions.get(emoji.as_str()).unwrap();
            if !users.contains(&USER.read().inbox_id) {
                println!("DOES NOT CONTAIN, SENDING EMOJI");
                app_state
                    .xmtp
                    .as_ref()
                    .expect("xmtp not initialized")
                    .send_reaction(&id, &msg.id, emoji.as_str(), ReactionAction::Added)
                    .await;
            }
        } else {
            println!("DOES NOT CONTAIN, SENDING EMOJI");
            app_state
                .xmtp
                .as_ref()
                .expect("xmtp not initialized")
                .send_reaction(&id, &msg.id, emoji.as_str(), ReactionAction::Added)
                .await;
        }

        // let msg = app_state
        //     .xmtp
        //     .as_ref()
        //     .expect("xmtp not init")
        //     .client
        //     .message_by_id("58f0fdb92cc1cd1f4b1fa91566c60a7b7f5222171e6c4896f9f9dbf717718994")
        //     .expect("FAILED 1")
        //     .expect("FAILED 2");
        // println!("MESSAGE: {:?}", msg.conversation_id);
        // let b = msg.sender_inbox_id.clone();
        // println!("msg sender of reaction sender: {:?}", b);
        // match msg.decode().expect("failed 3") {
        //     Content::Reaction(reaction) => {
        //         println!("Message is a reaction");
        //         let a = reaction.content;
        //         println!("reaction content: {:?}", a);
        // let b = reaction.reference;
        // let msg = app_state
        //     .xmtp
        //     .as_ref()
        //     .expect("xmtp not init")
        //     .client
        //     .message_by_id(&b)
        //     .expect("FAILED 3")
        //     .expect("FAILED 4");
        // let c = msg.;
        // println!("convo id of referenced message of reaction: {:?}", c);
        // println("")
        //     }
        //     _ => {
        //         println!("Message is not a reaction");
        //     }
        // }
        // app_state
        //     .xmtp
        //     .as_ref()
        //     .expect("xmtp not initialized")
        //     .send_reaction(&id, &message_id, emoji.as_str(), ReactionAction::Added)
        //     .await;
    }

    rsx! {
        button {
            class: "hover:bg-deep-purple-400 rounded-md",
            onclick: move |_| on_click(emoji.clone(), id.clone(), message.clone()),
            "{emoji.as_str()}"
        }
    }
}
