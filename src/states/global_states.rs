use dioxus::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Screen {
    Server,
    DirectMessages,
    Settings,
}

pub static CURRENT_SCREEN: GlobalSignal<Screen> = Signal::global(|| Screen::DirectMessages);
pub static OPEN_CREATE_SERVER_MODAL: GlobalSignal<bool> = Signal::global(|| false);
