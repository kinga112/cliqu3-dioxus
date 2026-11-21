#[derive(Clone, PartialEq, Debug)]
pub enum AuthState {
    CheckingAuth,
    CheckingSavedState,
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
    pub inbox_id: String,
    pub audio: bool,
    pub video: bool,
    pub silence: bool,
    pub profile: MemberProfile,
}

impl Default for User {
    fn default() -> Self {
        Self {
            inbox_id: String::new(),
            audio: true,
            video: false,
            silence: false,
            profile: MemberProfile {
                address: String::new(),
                name: String::new(),
                description: String::new(),
                avatar: String::new(),
            },
        }
    }
}
