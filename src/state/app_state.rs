use crate::modules::docs::db::DB;
use crate::modules::docs::db::ServerMetadata;
use crate::modules::xmtp::xmtp::XmtpHandler;
use crate::smart_contract;
use crate::state::data_types::user::AuthState;
use crate::state::data_types::user::MemberProfile;
use crate::state::data_types::user::User;
use crate::state::saved_state::SAVED_STATE_HANDLER;
use crate::state::saved_state::SavedAppState;
use crate::state::saved_state::SavedStateHandler;
use crate::{config, modules::xmtp::xmtp::XMTP_STATE};
// use crate modules::docs::db::ServerDocs;
// use crate call::handler::CallHandler;
// use create cache::Moka;
use crate::modules::docs::db::ServerDocs;
use anyhow::{Error, Result};
use dioxus::core::spawn;
use dioxus::signals::ReadableExt;
use dioxus::signals::Signal;
use dioxus::signals::WritableExt;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::path;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    Server,
    DirectMessages,
    Settings,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub auth_state: Signal<AuthState>,
    pub user: Signal<User>,
    pub servers: Signal<Vec<ServerMetadata>>,
    pub current_screen: Signal<Screen>,
}

impl AppState {
    pub fn default() -> Self {
        Self {
            auth_state: Signal::new(AuthState::CheckingAuth),
            user: Signal::new(User::default()),
            servers: Signal::new(Vec::new()),
            current_screen: Signal::new(Screen::DirectMessages),
        }
    }

    pub async fn login(&mut self, address: String) -> Result<()> {
        // if self.xmtp.is_none() {
        //     return Ok(());
        // }

        // let address = self.xmtp.as_ref().unwrap().client.account_identifier()?;
        // let entry = Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
        // entry
        //     .set_secret(address.as_bytes())
        //     .expect("failed to set random key in keyring");
        let servers = self
            .init_db()
            .await
            .expect("Failed to init docs db or fetch servers");

        let saved_state_handler = SavedStateHandler::new(&address);
        *SAVED_STATE_HANDLER.write().expect("failed to write to DB") =
            Some(Arc::new(saved_state_handler));
        self.init_user_profile(address);
        *self.servers.write() = servers;
        *self.auth_state.write() = AuthState::CheckingSavedState;
        Ok(())
    }

    pub async fn logout(&mut self) -> Result<()> {
        // if self.user.peek().inbox_id != "" {
        //     let entry =
        //         Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
        //     entry
        //         .delete_credential()
        //         .expect("failed to delete password from keyring");
        // }
        // self.xmtp = None;
        // self.db = None;
        XMTP_STATE.lock().await.take();
        *self.user.write() = User::default();
        *self.auth_state.write() = AuthState::Unauthenticated;
        Ok(())
    }

    // pub fn check_saved_user(&mut self) -> Result<(String, String)> {
    pub async fn check_saved_user(&mut self) {
        println!("check_saved_user");
        // let entry = Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
        // // let key = entry
        // //     .get_secret()
        // //     .expect("failed to get password from keyring");
        // //
        // let result = entry.get_secret();
        // match result {
        //     Ok(key) => {
        //         let address = String::from_utf8(key).expect("failed to parse address");
        //         println!("address of saved user: {:?}", address);
        //         let xmtp = XMTP::new(None, Some(address.clone()));
        //         xmtp.init_stream();
        //         let inbox_id = xmtp
        //             .client
        //             .inbox_id()
        //             .expect("failed to get inbox id for client");
        //         let addy = xmtp
        //             .client
        //             .account_identifier()
        //             .expect("No account identifier");
        //         self.xmtp = Some(xmtp);
        //         let server_list = self
        //             .init_db()
        //             .await
        //             .expect("Failed to init docs db or fetch servers");
        //         // let server_list =
        //         *SERVER_LIST.write() = server_list;
        //         // *AUTH_STATE.write() = AuthState::Authenticated;
        //         // *ADDRESS.write() = Some(address);
        //         // USER.write().authorized = AuthState::Authenticated;
        //         *AUTH_STATE.write() = AuthState::Authenticated;
        //         USER.write().inbox_id = inbox_id;
        //         USER.write().profile = Some(MemberProfile {
        //             // address: address,
        //             address: addy,
        //             name: "".to_string(),
        //             avatar: "".to_string(),
        //             description: "".to_string(),
        //         });
        //     }
        //     Err(e) => {
        //         // *AUTH_STATE.write() = AuthState::Unauthenticated;
        //         *AUTH_STATE.write() = AuthState::Unauthenticated;
        //         println!("no saved user: {e}");
        //     }
        // }

        // auto login for testing
        let xmtp_new = XmtpHandler::new(
            None,
            Some("0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e".to_string()),
        );
        // let xmtp = XMTP_LOCK.lock(); // no lock held during setup
        *XMTP_STATE.lock().await = Some(xmtp_new);
        let xmtp_option = XMTP_STATE.lock().await;
        // let xmtp = XMTP_LOCK.lock().await;
        // let a = xmtp_option.as_ref();
        if let Some(xmtp) = xmtp_option.as_ref() {
            let inbox_id = xmtp
                .client
                .inbox_id()
                .expect("failed to get inbox id for client");
            self.user.write().inbox_id = inbox_id;
            let address = xmtp
                .client
                .account_identifier()
                .expect("No account identifier");
            let login_result = self.login(address).await;
        }
    }

    // pub fn init_db(&mut self, db: ServerDocs) -> Result<()> {
    pub async fn init_db(&mut self) -> Result<Vec<ServerMetadata>> {
        // let address = self.user.peek().profile.address;
        // let path_str = format!(
        //     "{}/{}",
        //     config::BASE_FILE_LOCATION,
        //     self.user.peek().profile.address
        // );
        let path_str = format!("{}/default", config::BASE_FILE_LOCATION);
        let path = path::PathBuf::from(path_str);
        let db = ServerDocs::new(path)
            .await
            .map_err(|e| format!("couldn't init cliqu3 db: {e}"))
            .expect("failed");
        let server_list = db.get_all_servers().await?;
        *DB.write().expect("failed to write to DB") = Some(Arc::new(db));
        // self.db = Some(Arc::new(Mutex::new(db)));
        // self.db = Some(db);
        Ok(server_list)
    }

    pub fn init_user_profile(mut self, address: String) {
        println!("ADDY: {:?}", address.clone());
        let profile = MemberProfile {
            address: address.clone(),
            name: "".to_string(),
            avatar: "".to_string(),
            description: "".to_string(),
        };

        self.user.write().profile = profile;
        let address_clone = address.clone();
        spawn(async move {
            if let Ok(user_profile) = smart_contract::interact::get_profile(&address_clone).await {
                println!(
                    "GOT USER PROFILE IN CHECK SAVED USER: {:?}",
                    user_profile.clone()
                );
                self.user.write().profile = user_profile;
            }
        });
    }
}
