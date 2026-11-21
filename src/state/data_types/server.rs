use dioxus::prelude::*;
use std::collections::HashMap;

use crate::{
    modules::{
        docs::db::{ServerMetadata, TextChannelMetaData, VoiceChannel},
        xmtp::xmtp::Msg,
    },
    state::data_types::user::MemberProfile,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AddChannelModalState {
    Text,
    Voice,
    Closed,
}

pub struct ReplyState {
    pub reference: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OutboxMessageType {
    pub tmp_id: String,
    pub message_id: String,
    pub message_text: String,
}

#[derive(PartialEq, Clone, Copy)]
pub struct ServerState {
    pub metadata: Signal<ServerMetadata>,
    pub text_channels: Signal<Vec<TextChannelMetaData>>,
    pub voice_channels: Signal<HashMap<String, VoiceChannel>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            metadata: Signal::new(ServerMetadata {
                id: String::new(),
                ticket: String::new(),
                name: String::new(),
                pic: String::new(),
                creator_address: String::new(),
            }),
            text_channels: Signal::new(Vec::new()),
            voice_channels: Signal::new(HashMap::new()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct TextChannelState {
    pub metadata: Signal<TextChannelMetaData>,
    pub description: Signal<Option<String>>,
    pub messages: Signal<Vec<Msg>>,
    pub members: Signal<HashMap<String, MemberProfile>>,
    pub outbox_messages: Signal<Vec<OutboxMessageType>>,
    pub reply: Signal<Option<ReplyState>>,
}

impl Default for TextChannelState {
    fn default() -> Self {
        Self {
            metadata: Signal::new(TextChannelMetaData {
                id: String::new(),
                name: String::new(),
            }),
            description: Signal::new(None),
            messages: Signal::new(Vec::new()),
            members: Signal::new(HashMap::new()),
            outbox_messages: Signal::new(Vec::new()),
            reply: Signal::new(None),
        }
    }
}
