use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AuthState {
    Checking,
    Unauthenticated,
    Authenticated,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MemberProfile {
    pub address: String,
    pub name: String,
    pub avatar: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    // pub authorized: AuthState,
    pub inbox_id: String,
    pub audio: bool,
    pub video: bool,
    pub silence: bool,
    pub profile: Option<MemberProfile>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            // authorized: AuthState::Checking,
            inbox_id: String::new(),
            audio: true,
            video: false,
            silence: false,
            profile: None,
        }
    }
}

// or should i just do option user and its none and then gets initialized?
// pub static USER: GlobalSignal<Option<User>> = Signal::global(|| None);
// OR ... remove authenticated from user and add global state for auth. User will always exist if auth is true..
pub static USER: GlobalSignal<User> = Signal::global(|| User::default());
pub static AUTH_STATE: GlobalSignal<AuthState> = Signal::global(|| AuthState::Checking);

// pub static AUTH_STATE: GlobalSignal<AuthState> = Signal::global(|| AuthState::Checking);
// pub static ADDRESS: GlobalSignal<Option<String>> = Signal::global(|| None);
// pub static INBOX_ID: GlobalSignal<Option<String>> = Signal::global(|| None);
// pub static PROFILE: GlobalSignal<bool> = Signal::global(|| true);
// pub static AUDIO: GlobalSignal<bool> = Signal::global(|| true);
// pub static VIDEO: GlobalSignal<bool> = Signal::global(|| false);
// pub static SILENCE: GlobalSignal<bool> = Signal::global(|| false);
