use std::collections::HashMap;

use dioxus::prelude::*;
use xmtp::Conversation;

use crate::modules::{
    docs::db::TextChannelMetaData,
    xmtp::xmtp::{Msg, TextChannel},
};

// impl Default for TextChannel {
//     fn default() -> Self {
//         Self {
//             metadata: TextChannelMetaData {
//                 id: String::new(),
//                 name: String::new(),
//             },
//             description: Some(String::new()),
//             messages: Vec::new(),
//             members: HashMap::new(),
//         }
//     }
// }

pub static CURRENT_DIRECT_MESSAGE: GlobalSignal<Option<TextChannel>> = Signal::global(|| None);
// pub static CURRENT_DIRECT_MESSAGE: GlobalSignal<TextChannel> =
//     Signal::global(|| TextChannel::default());
pub static DM_MESSAGES: GlobalSignal<Vec<Msg>> = Signal::global(Vec::new);
