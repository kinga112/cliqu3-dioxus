use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;

use crate::modules::{
    docs::db::{Server, ServerMetadata, VoiceChannel},
    xmtp::xmtp::{Msg, TextChannel},
};

pub struct ReplyState {
    pub reference: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddChannelModalState {
    Text,
    Voice,
    Closed,
}

pub static CURRENT_SERVER: GlobalSignal<Option<Server>> = Signal::global(|| None);
pub static CURRENT_TEXT_CHANNEL: GlobalSignal<Option<TextChannel>> = Signal::global(|| None);
pub static SERVER_LIST: GlobalSignal<Vec<ServerMetadata>> = Signal::global(Vec::new);
// pub static MESSAGES: GlobalSignal<Vec<Arc<Msg>>> = Signal::global(Vec::new);
pub static MESSAGES: GlobalSignal<Vec<Msg>> = Signal::global(Vec::new);
pub static OUTBOX_MESSAGES: GlobalSignal<Vec<String>> = Signal::global(Vec::new);
pub static REPLY: GlobalSignal<Option<ReplyState>> = Signal::global(|| None);
//
// Hashmap < Voice channel name, Voice Channel>
//
// pub static VOICE_CHANNELS: GlobalSignal<HashMap<String, HashMap<String, String>>> =
//     Signal::global(|| HashMap::new());
pub static VOICE_CHANNELS: GlobalSignal<HashMap<String, VoiceChannel>> =
    Signal::global(|| HashMap::new());

pub static OPEN_ADD_MEMBER_MODAL: GlobalSignal<bool> = Signal::global(|| false);
pub static OPEN_ADD_CHANNEL_MODAL: GlobalSignal<AddChannelModalState> =
    Signal::global(|| AddChannelModalState::Closed);
