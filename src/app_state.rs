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
use keyring::Entry;
use std::path;
use std::sync::Arc;
use std::sync::LazyLock;
// use tauri::Emitter;
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

    pub fn login(&mut self) -> Result<()> {
        // println!("running login. address: {:?}", address);
        // self.user = Some(address.to_string());
        if self.xmtp.is_none() {
            return Ok(());
        }

        let address = self.xmtp.as_ref().unwrap().client.account_identifier()?;
        let entry = Entry::new("cliqu3", "cliqu3 user").expect("failed to create keyring entry");
        entry
            .set_secret(address.as_bytes())
            .expect("failed to set random key in keyring");

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
        // self.db = None;
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
        xmtp.init_stream();
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
        // let server_list =
        *SERVER_LIST.write() = server_list;
        // *AUTH_STATE.write() = AuthState::Authenticated;
        // *ADDRESS.write() = Some(address);
        // USER.write().authorized = AuthState::Authenticated;
        // *AUTH_STATE.write() = AuthState::Authenticated;
        // *USER.write().inbox_id = inbox_id;

        let mut profile = MemberProfile {
            address: addy.clone(),
            name: "".to_string(),
            avatar: "".to_string(),
            description: "".to_string(),
        };

        println!("ADDY: {:?}", addy.clone());
        let result = smart_contract::interact::get_profile(&addy).await;

        match result {
            Ok(user_profile) => {
                println!(
                    "GOT USER PROFILE IN CHECK SAVED USER: {:?}",
                    user_profile.clone()
                );
                profile = user_profile;
            }
            Err(e) => {
                eprint!("GETTING PROFILE FAILED WITH ERROR: {:?}", e);
            }
        }

        USER.write().profile = Some(profile);
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

    // pub fn init_user(&mut self, user: String) {
    //     self.user = Some(user);
    // }

    // pub fn create_wc_uri(&mut self) -> String {
    //     let mut raw_key = [0u8; 32];
    //     rand::thread_rng().fill_bytes(&mut raw_key);
    //     let sym_key_hex = hex::encode(raw_key);
    //     let mut hasher = Sha256::new();
    //     hasher.update(&raw_key);
    //     let pairing_topic_hex = hex::encode(hasher.finalize());
    //     let now = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //     let expiry = now + 3600;
    //     let methods = "[wc_sessionPropose]";
    //     let uri = format!(
    //         "wc:{}@2?expiryTimestamp={}&relay-protocol=irn&symKey={}&methods={}",
    //         pairing_topic_hex, expiry, sym_key_hex, methods
    //     );

    //     uri
    // }

    // pub async fn init_wallet_connect(&mut self, uri: String, app: tauri::AppHandle) {
    //     let mut wc = WalletConnectHandler::new(uri);

    //     // use to receive xmtp from spawned tasks
    //     let (xmtp_tx, xmtp_rx) = oneshot::channel();

    //     wc.init(xmtp_tx).await;

    //     let xmtp = xmtp_rx.await.expect("failed to get xmtp in receiver");
    //     let id = xmtp.client.inbox_id().expect("couldnt get xmtp inbox id");
    //     println!("GOT XMTP INBOX ID IN APP STATE: {:?}", id);
    //     self.xmtp = Some(xmtp);

    //     let _ = self.login();

    //     let payload = serde_json::json!({
    //         "data": "authenticated"
    //     });

    //     app.emit("wc_response", payload)
    //         .expect("failed to emit iroh event");
    // }
}
