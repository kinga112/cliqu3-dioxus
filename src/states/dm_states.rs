use dioxus::prelude::*;
use xmtp::Conversation;

use crate::modules::xmtp::xmtp::{Msg, TextChannel};

// pub static CURRENT_DIRECT_MESSAGE: GlobalSignal<Option<Conversation>> = Signal::global(|| None);
pub static CURRENT_DIRECT_MESSAGE: GlobalSignal<Option<TextChannel>> = Signal::global(|| None);
pub static DM_MESSAGES: GlobalSignal<Vec<Msg>> = Signal::global(Vec::new);
