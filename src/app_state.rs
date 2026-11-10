use crate::modules::docs::db::ServerMetadata;
use crate::states::server_states::SERVER_LIST;
use crate::states::user_states::AUTH_STATE;
// use crate::states::user_states::ADDRESS;
use crate::smart_contract;
use crate::states::user_states::AuthState;
use crate::states::user_states::MemberProfile;
use crate::states::user_states::USER;
// use crate::{call::handler::CallHandler, iroh::{call::Call, cliqu3db::{Cliqu3Db, ServerDocs}}};
// use crate::walletconnect::crypto;
use crate::{config, modules::xmtp::xmtp::XMTP};
// use crate modules::docs::db::ServerDocs;
// use crate call::handler::CallHandler;
// use create cache::Moka;
use crate::modules::docs::db::ServerDocs;
use anyhow::{Error, Result};
use dioxus::core::spawn;
use keyring::Entry;
use std::path;
use std::sync::Arc;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub static APP_STATE: LazyLock<Mutex<AppState>> = LazyLock::new(|| Mutex::new(AppState::default()));

pub struct AppState {
    pub user: Option<String>,
    pub db: Option<Arc<Mutex<ServerDocs>>>,
    // pub db: Option<ServerDocs>,
    // pub call: CallHandler,
    // pub cache: Moka,
    pub xmtp: Option<XMTP>,
}

impl AppState {
    pub fn default() -> Self {
        // let call = CallHandler::new();
        // let cache = Moka::default();

        Self {
            user: None,
            db: None,
            // call,
            // cache,
            xmtp: None,
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        if self.xmtp.is_none() {
            return Ok(());
        }

        let address = self.xmtp.as_ref().unwrap().client.account_identifier()?;
        let entry = Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
        entry
            .set_secret(address.as_bytes())
            .expect("failed to set random key in keyring");

        let server_list = self
            .init_db()
            .await
            .expect("Failed to init docs db or fetch servers");
        *SERVER_LIST.write() = server_list;

        Ok(())
    }

    pub fn logout(&mut self) -> Result<()> {
        if let Some(user) = &self.user {
            let entry =
                Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
            entry
                .delete_credential()
                .expect("failed to delete password from keyring");
        }
        self.user = None;
        self.xmtp = None;
        self.db = None;
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
        let xmtp = XMTP::new(
            None,
            Some("0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e".to_string()),
        );
        let inbox_id = xmtp
            .client
            .inbox_id()
            .expect("failed to get inbox id for client");
        let addy = xmtp
            .client
            .account_identifier()
            .expect("No account identifier");
        self.xmtp = Some(xmtp);
        let server_list = self
            .init_db()
            .await
            .expect("Failed to init docs db or fetch servers");
        *SERVER_LIST.write() = server_list;
        USER.write().inbox_id = inbox_id;
        self.init_user_profile(addy);
        *AUTH_STATE.write() = AuthState::Authenticated;
    }

    // pub fn init_db(&mut self, db: ServerDocs) -> Result<()> {
    pub async fn init_db(&mut self) -> Result<Vec<ServerMetadata>> {
        let path_str = format!(
            "{}/{}",
            config::BASE_FILE_LOCATION,
            self.user.clone().unwrap_or("default".to_string())
        );
        let path = path::PathBuf::from(path_str);
        let db = ServerDocs::new(path)
            .await
            .map_err(|e| format!("couldn't init cliqu3 db: {e}"))
            .expect("failed");
        let server_list = db.get_all_servers().await?;
        self.db = Some(Arc::new(Mutex::new(db)));
        // self.db = Some(db);
        Ok(server_list)
    }

    pub fn init_user_profile(&self, address: String) {
        println!("ADDY: {:?}", address.clone());
        let profile = MemberProfile {
            address: address.clone(),
            name: "".to_string(),
            avatar: "".to_string(),
            description: "".to_string(),
        };

        USER.write().profile = profile;
        let address_clone = address.clone();
        spawn(async move {
            if let Ok(user_profile) = smart_contract::interact::get_profile(&address_clone).await {
                println!(
                    "GOT USER PROFILE IN CHECK SAVED USER: {:?}",
                    user_profile.clone()
                );
                USER.write().profile = user_profile;
            }
        });
    }
}
