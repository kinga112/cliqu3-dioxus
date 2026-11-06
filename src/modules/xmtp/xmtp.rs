use crate::modules::docs::db::TextChannelMetaData;
use crate::modules::walletconnect::signer::WalletConnectXmtpSigner;
use crate::modules::xmtp::utilities;
use crate::states::dm_states::{CURRENT_DIRECT_MESSAGE, DM_MESSAGES};
use crate::states::global_states::{CURRENT_SCREEN, Screen};
use crate::states::server_states::{MESSAGES, OUTBOX_MESSAGES};
use crate::states::user_states::{MemberProfile, USER};
use crate::{config, smart_contract};
use aes_gcm::Aes128Gcm;
use alloy::hex;
use base64::prelude::*;
// use alloy::ens;
use alloy::providers::Provider;
use alloy::signers::local::LocalSigner;
use alloy::{
    // ens::EnsResolver, ens::ProviderEnsExt, primitives::address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use dioxus::signals::ReadableExt;
use prost::Message as ProstMessage;
use rand::RngCore;
use rand::rngs::OsRng;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use xmtp::{
    ConversationOrderBy, ConversationType, ListConversationsOptions, ListMessagesOptions, XmtpError,
};

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use url::Url;
// use tauri::Emitter;
use xmtp::content::{ContentTypeId, EncodedContent, Reaction, ReactionAction};
use xmtp::{
    AccountIdentifier, AlloySigner, Client, ConsentState, Conversation, CreateGroupOptions, Env,
    IdentifierKind, Message, MessageEvent, Recipient, Signer, content::Content,
};

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, Generate, Key, KeyInit},
};

// #[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TextChannel {
    pub metadata: TextChannelMetaData,
    pub description: Option<String>,
    // creator: String,
    pub messages: Vec<Msg>,
    pub members: HashMap<String, MemberProfile>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Msg {
    pub id: String,
    pub content: Content,
    pub from: String,
    pub timestamp: i64,
    pub reactions: HashMap<String, Vec<String>>,
}

impl PartialEq for Msg {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.timestamp == other.timestamp
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct MemberProfile {
//     pub address: String,
//     pub name: String,
//     pub avatar: String,
//     pub description: String,
// }

pub struct XMTP {
    pub client: Client,
}

impl XMTP {
    pub fn new(
        signer_opt: Option<Arc<WalletConnectXmtpSigner>>,
        address_opt: Option<String>,
    ) -> Self {
        // let db_path = format!("{}/xmtp_{}.db3", config::BASE_FILE_LOCATION, signer.address);

        // println!("Initializing XMTP client with db_path: {}", db_path);
        if signer_opt.is_none() && address_opt.is_none() {
            panic!("failed to init xmtp");
        }

        // MessageEvent { message_id: "1bd3c6d6531e668b6d11b1304c96844edb777d50c33ad2f98e6e2203493ec558", conversation_id: "392152a8c721f0dfdf0f4c7aa9f8d713" }

        // Metamask: 0xcfa614caccb5b1b6f89a762191cde13360ad3c09
        // let client: Client;
        // if let Some(signer) = signer_opt {
        //     let db_path = format!("{}/xmtp_{}.db3", config::BASE_FILE_LOCATION, signer.address);
        //     client = Client::builder()
        //         .env(Env::Dev)
        //         // .encryption_key(encryption_key)
        //         .db_path(&db_path)
        //         .build(&*signer)
        //         .expect("client failed");
        //     // let a = client
        //     //     .account_identifier()
        //     //     .expect("failed to get identifier");
        //     // println!("WHAT IS THIS?? : {:?}", a);
        // } else {
        //     // if let Some(address) = address_opt {
        //     let address = address_opt.unwrap();
        //     let db_path = format!("{}/xmtp_{}.db3", config::BASE_FILE_LOCATION, address);
        //     client = Client::builder()
        //         .env(Env::Dev)
        //         // .encryption_key(encryption_key)
        //         .db_path(&db_path)
        //         .build_existing(&address, IdentifierKind::Ethereum)
        //         .expect("client failed");
        // }

        // TEMP SIGNER WITH PRIVATE KEY FOR CLIQU3 CONTRACT TESTING
        // 0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e
        let private_key_signer: PrivateKeySigner = config::PRIVATE_KEY
            .parse()
            .expect("failed to create signer");

        let xmtp_signer = AlloySigner::from(private_key_signer);

        let db_path = format!(
            "{}/xmtp_{}.db3",
            config::BASE_FILE_LOCATION,
            &xmtp_signer.address()
        );
        let client = Client::builder()
            .env(Env::Dev)
            .db_path(db_path)
            .build(&xmtp_signer)
            .expect("failed to init xmtp client");

        client.sync_welcomes().expect("failed to sync on start");
        let consent_states = [
            ConsentState::Allowed,
            ConsentState::Denied,
            ConsentState::Unknown,
        ];
        let result = client.sync_all(&consent_states).expect("");
        println!("Sync all result: {:?}", result);
        let a = client.account_identifier().expect("No account identifier");
        println!("ACCOUNT IDEN: {:?}", a);
        // let c = Client::builder().build_existing(address, kind);
        Self { client }
    }

    pub fn init_stream(
        &self,
        stream_tx: UnboundedSender<MessageEvent>,
        cancelation_token: CancellationToken,
    ) {
        println!(
            "*
            *
            *
            *
            initializing XMTP stream
            *
            *
            *
            *
            *"
        );
        let consent_states = vec![ConsentState::Allowed, ConsentState::Unknown];
        let stream =
            xmtp::stream::messages(&self.client, None, &consent_states).expect("stream failed");

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let blocking_handle = tokio::task::spawn_blocking(move || {
            while let Some(event) = stream.recv() {
                // Forward events to our async receiver.
                // If the async receiver drops this fails and exits.
                if tx.blocking_send(event).is_err() {
                    stream.close();
                    break;
                }
            }
        });

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Instantly aborts even if rx.recv() is waiting indefinitely
                    _ = cancelation_token.cancelled() => {
                        println!("Task cancelled gracefully!");
                        break;
                    }
                    // Non-blocking wait handled by the Tokio executor
                    Some(event) = rx.recv() => {
                        if stream_tx.send(event).is_err() {
                            println!("Strem TX send error");
                        }
                    }
                    else => break,
                }
            }
            println!("Worker task finished.");
        });
    }

    pub fn create_group(
        &self,
        members: &[Recipient],
        name: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
    ) -> Result<String, String> {
        println!("creating new group: {:?}", name);
        let opts = CreateGroupOptions {
            permissions: None,
            name: name,
            description: description,
            image_url: image_url,
            app_data: None,
            disappearing: None,
        };
        let convo = self
            .client
            .group(members, &opts)
            .expect("failed to create xmtp group");
        println!("created new group: {:?}", convo.id());
        Ok(convo.id())
    }

    // pub fn get_conversation(&self, id: &str) -> Result<TextChannel, String> {
    pub async fn get_conversation(&self, id: &str) -> Result<TextChannel, String> {
        // change function name to get_text_channel?
        // self.create_bytes_archive();
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");

        // let a = convo.metadata();
        // println!(
        //     "members: {:?}",
        //     convo.members().expect("failed to get members")
        // );
        // let mut members: Vec<String> = vec![];
        // for member in convo.members().expect("failed to get members") {
        //     let address = member.account_identifiers[0];
        //     members.push(address);
        // }
        //

        // let members: Vec<String> = convo
        //     .members()
        //     .expect("failed to get members")
        //     .into_iter()
        //     .filter_map(|m| m.account_identifiers.get(0).map(|id| id.to_string()))
        //     .collect();

        let mut member_profiles: HashMap<String, MemberProfile> = HashMap::new();
        let members = convo.members().expect("failed to get members");
        for member in members {
            let address = member
                .account_identifiers
                .get(0)
                .map(|id| id.to_string())
                .expect("failed to get address");
            println!("HERE IS THE ADDRESS: {:?}", address.clone());

            let mut profile = MemberProfile {
                address: address.clone(),
                name: "".to_string(),
                avatar: "".to_string(),
                description: "".to_string(),
            };

            // let result = smart_contract::interact::get_profile(&address).await;
            // let handle = Handle::current();
            // Block the current thread until the async function completes
            // let result =
            // handle.block_on(async { smart_contract::interact::get_profile(&address).await });

            let result = smart_contract::interact::get_profile(&address).await;

            match result {
                Ok(user_profile) => {
                    profile = user_profile;
                }
                Err(e) => {
                    eprint!("GETTING PROFILE FAILED WITH ERROR: {:?}", e);
                }
            }

            member_profiles.insert(member.inbox_id, profile);
        }

        let mut messages = vec![];
        for message in convo.messages().expect("failed to get messages") {
            let msg = utilities::get_msg(message).expect("failed to convert message to msg");
            messages.push(msg);
        }

        let metadata = TextChannelMetaData {
            id: convo.id(),
            name: convo.name().unwrap_or_else(|| "*No Name*".to_string()),
        };
        let text_channel = TextChannel {
            metadata: metadata,
            description: convo.description(),
            // creator: creator,
            messages: messages,
            members: member_profiles,
            // creator: convo
            //     .metadata()
            //     .expect("failed to get convo metadata")
            //     .creator_inbox_id,
        };

        Ok(text_channel)
        // Ok(convo)
    }

    pub fn get_conversations(&self) -> Result<Vec<Conversation>, String> {
        let convos = self
            .client
            .conversations()
            .expect("failed to list conversations");
        Ok(convos)
    }

    pub fn get_dms(&self, consent_state: ConsentState) -> Result<Vec<Conversation>, String> {
        let options = ListConversationsOptions {
            conversation_type: Some(ConversationType::Dm),
            limit: 10,
            created_after_ns: 0,
            created_before_ns: 0,
            last_activity_after_ns: 0,
            last_activity_before_ns: 0,
            consent_states: vec![consent_state],
            order_by: ConversationOrderBy::LastActivity,
            include_duplicate_dms: false,
        };

        let convos = self
            .client
            .list_conversations(&options)
            .expect("failed to get dms");

        Ok(convos)
    }

    pub fn send_text(&self, id: &str, text: String) -> Result<String, String> {
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");
        println!("before sending message");

        let message_id = convo
            .send_text_optimistic(&text)
            .expect("failed to send???");
        convo.publish_messages().expect("failed to publish message");
        println!(
            "MESSAGE ID OF SENT MESSAGE? *
            *
            *
            {:?}
            *
            *
            *",
            message_id
        );
        Ok(message_id)
    }

    pub async fn send_reaction(
        &self,
        id: &str,
        message_id: &str,
        emoji: &str,
        action: ReactionAction,
    ) {
        let user_inbox_id = USER.peek().inbox_id.clone();
        self.apply_reaction(message_id, &user_inbox_id, emoji, action);
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");
        println!("Sending Reaction!");
        let e = emoji.to_string();
        let msg_id = message_id.to_string();
        let result =
            tokio::task::spawn_blocking(move || convo.send_reaction(&msg_id, &e, action)).await;
        let a = result.expect("failed 1").expect("failed 2");
        println!("Sent Reaction!: {:?}", a);
    }

    fn apply_reaction(
        &self,
        message_id: &str,
        sender_inbox_id: &str,
        emoji: &str,
        action: ReactionAction,
    ) {
        // let mut messages = MESSAGES.write();
        let mut messages;
        match *CURRENT_SCREEN.peek() {
            Screen::Server => {
                messages = MESSAGES.write();
            }
            Screen::DirectMessages => {
                messages = DM_MESSAGES.write();
            }
            Screen::Settings => {
                messages = MESSAGES.write();
            }
        }
        if let Some(msg) = messages.iter_mut().find(|m| m.id == message_id) {
            let mut updated = (*msg).clone();
            let users = updated
                .reactions
                .entry(emoji.to_string())
                .or_insert_with(Vec::new);
            match action {
                ReactionAction::Added => {
                    if !users.contains(&sender_inbox_id.to_string()) {
                        users.push(sender_inbox_id.to_string());
                    }
                }
                ReactionAction::Removed => {
                    users.retain(|u| u != sender_inbox_id);
                }
                ReactionAction::Unspecified => {}
            }
            if users.is_empty() {
                updated.reactions.remove(emoji);
            }
            println!(
                "Writing reaction new message to Messages signal: {:?}",
                updated.reactions
            );
            *msg = updated;
        }
    }

    pub async fn send_text_reply(&self, id: &str, text: String, reference: String) {
        println!("SENDNING A REPLY 2");
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");
        let result =
            tokio::task::spawn_blocking(move || convo.send_text_reply(&reference, &text)).await;
    }

    // pub async fn send_invite(&self, id: &str, metadata_json: String) {
    pub async fn send_invite(&self, addresses: Vec<String>, metadata_json: String) {
        let content_type_id = ContentTypeId {
            authority_id: "cliqu3.com".to_string(),
            type_id: "invite".to_string(),
            version_major: 1,
            version_minor: 0,
        };

        // This is not a secret, if other XMTP clients want to read Cliqu3 invites
        let key_bytes =
            hex::decode("FC52BD39629CC1030A3C08974403A350F79A337C6C99F72EE3B7EADD55551B15")
                .expect("Invalid hex key");
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(&key);

        let nonce = Nonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, metadata_json.as_bytes().as_ref())
            .expect("failed to ciphertext for invite");
        let mut payload = Vec::with_capacity(nonce.len() + ciphertext.len());
        payload.extend_from_slice(&nonce);
        payload.extend_from_slice(&ciphertext);
        let ec = EncodedContent {
            r#type: Some(content_type_id),
            parameters: BTreeMap::from([("encoding".into(), "UTF-8".into())]),
            fallback: None,
            content: payload,
            compression: None,
        }
        .encode_to_vec();

        for address in addresses {
            let recipient = Recipient::Address(address);
            let convo = self
                .client
                .dm(&recipient)
                .expect("failed to get convo with recipient");
            let result = convo.send(&ec);
            let id = result.expect("failed to get result from invite send");
            println!("MESSAGE ID of INVITE: {:?}", id);
        }
    }

    pub fn verify_can_message(&self, address: String) -> Result<bool, String> {
        let identifier = AccountIdentifier {
            address,
            kind: IdentifierKind::Ethereum,
        };
        let can_message = self
            .client
            .can_message(&[identifier])
            .expect("failed to verify 'can_message'");
        Ok(can_message[0])
    }

    pub fn get_ens_name(&self, address: &str) {
        // println!("address: {}", address);
        // self.client.reverse_resolve(address)
        // let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
        // let rpc_url = "https://eth.llamarpc.com";
        let rpc_url = Url::parse("https://eth.drpc.org").expect("failed to parse rpc url");
        // .parse()
        // .expect("failed to parse rpc url");
        // let provider = ProviderBuilder::new().connect(s);
        // let provider = ProviderBuilder::new().connect_http(rpc_url);

        // Vitalik's Ethereum address.
        // let vitalik_address = address!("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");

        // let ens_name = ens::reverse_address(&vitalik_address);

        // Perform reverse ENS lookup to get the ENS name for the address.
        // let name = provider
        //
        //

        // tokio::spawn(async move {
        //     let ens_name = provider
        //         .lookup_address(&vitalik_address)
        //         .await
        //         .expect("failed to lookup address");
        //     println!("Address {vitalik_address} resolves to: {ens_name:?}");
        //     // let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;

        //     let avatar = provider
        //         .lookup_txt(&ens_name, "avatar")
        //         .await
        //         .expect("no avatar");

        //     let header = provider
        //         .lookup_txt(&ens_name, "header")
        //         .await
        //         .expect("no header");

        //     let description = provider
        //         .lookup_txt(&ens_name, "description")
        //         .await
        //         .expect("no description");
        //     println!("avatar: {:?}", avatar);
        //     println!("header: {:?}", header);
        //     println!("description: {:?}", description);
        //     // Resolve ENS name to Address
        //     // let name = "vitalik.eth";
        //     // let address = provider.resolve_name(name).await?;

        //     // Lookup ENS name given Address
        //     // let resolved_name = provider.lookup_address(address).await?;
        //     // println!("NAME: {:?}", resolved_name);
        //     // let a = provider.get_resolver(node, error_name);
        //     // let b =
        // });

        // let ens_name = provider
        //     .lookup_address(&vitalik_address)
        //     .await
        //     .expect("failed to loopup address");

        // println!("Address {vitalik_address} resolves to: {ens_name:?}");
        // Some(ens_name)
        // Ok(ens_name)
    }

    pub fn get_message(&self, id: &str) -> Message {
        let message = self
            .client
            .message_by_id(id)
            .expect("failed to get message")
            .expect("no message exists for that id");
        message
    }

    pub async fn add_member(&self, ids: Vec<String>, address: String) {
        // add member to all allowed channels
        for id in ids {
            let convo = self
                .client
                .conversation(&id)
                .expect("failed to get convo")
                .expect("convo doesnt exist with id");
            let kind = IdentifierKind::Ethereum;
            let account = AccountIdentifier {
                address: address.clone(),
                kind,
            };
            let result = convo.add_members_by_identity(&[account]);
        }
    }

    pub fn create_file_archive(&self) -> Result<(), XmtpError> {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let result = self
            .client
            .create_file_archive("C:/Users/kinga/Documents/cliqu3/archive.xmtp", &key);

        // match result {
        //     Ok(()) => {
        //         println!("NO ERRORS DURING ARCHIVE");
        //     }
        //     Err(e) => {
        //         println!("ERROR DURING ARCHIVE: {:?}", e);
        //     }
        // }
        result
    }

    pub fn create_bytes_archive(&self) -> (Result<Vec<u8>, XmtpError>, [u8; 32]) {
        // fn create_bytes_archive(&self) {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        println!("THIS IS THE KEY: {:?}", key);
        let result = self.client.create_bytes_archive(&key);

        // match result {
        //     Ok(bytes) => {
        //         println!("GOT BYTES FROM ARCHIVE: {:?}", bytes);
        //     }
        //     Err(e) => {
        //         println!("ERROR DURING ARCHIVE: {:?}", e);
        //     }
        // }
        (result, key)
    }

    // fn import_file_archive(&self, key: &[u8; 32]) {
    //     let result = self.client.import_file_archive("", key);
    // }

    pub fn import_bytes_archive(&self, data: &[u8], key: &[u8; 32]) -> Result<(), XmtpError> {
        let result = self.client.import_bytes_archive(data, key);
        result
    }
}
