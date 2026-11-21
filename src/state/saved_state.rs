use std::{
    path::{self, Path},
    sync::{Arc, LazyLock, RwLock},
};

use dioxus::hooks::use_context;
use iroh_gossip::proto::Config;
use redb::{Database, Error, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::{
    config,
    modules::{docs::db::DB, xmtp::xmtp::XMTP_STATE},
    smart_contract::interact::fetch_profiles,
    state::app_state::{AppState, Screen},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub audio: bool,
    pub video: bool,
    pub silence: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavedAppState {
    pub user_config: UserConfig,
    pub last_screen: Screen,
    pub last_server: String,
    pub last_text_channel: String,
}

pub static SAVED_STATE_HANDLER: LazyLock<RwLock<Option<Arc<SavedStateHandler>>>> =
    LazyLock::new(|| RwLock::new(None));

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("app_state");

pub struct SavedStateHandler {
    db: Database,
}

impl SavedStateHandler {
    pub fn new(address: &str) -> Self {
        let path_str = format!("{}/saved_state.redb", config::BASE_FILE_LOCATION);
        let path = path::PathBuf::from(path_str);
        let db = Database::create(path).expect("failed to create redb");
        Self { db }
    }

    pub fn store_state(&self, state: &SavedAppState) {
        let json_string = serde_json::to_string(state).expect("failed to serialize state");

        let write_txn = self
            .db
            .begin_write()
            .expect("failed to begin write to table in redb");
        {
            let mut table = write_txn
                .open_table(TABLE)
                .expect("failed to open table in redb");
            table
                .insert("saved_state", json_string.as_str())
                .expect("failed to insert to table in redb");
        }
        write_txn
            .commit()
            .expect("failed to commit to table in redb");
    }

    pub fn get_state(&self) -> anyhow::Result<SavedAppState> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        // if let Some(guard) = table.get("saved_state").expect("failed to get saved state") {
        //     let saved_state: SavedAppState =
        //         serde_json::from_str(guard.value()).expect("failed to deserialize saved app state");
        //     Ok(saved_state)
        // } else {
        //     Err("Failed to get Saved App State".to_string())
        // }

        let guard = table
            .get("saved_state")
            .expect("failed to get saved state")
            .unwrap();
        let saved_state: SavedAppState = serde_json::from_str(guard.value())?;
        Ok(saved_state)
    }
}
