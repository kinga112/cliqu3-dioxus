use crate::config;
use crate::modules::docs::db::TextChannelMetaData;
use crate::modules::walletconnect::crypto;
use crate::modules::walletconnect::signer::WalletConnectXmtpSigner;
use crate::states::dm_states::DM_MESSAGES;
use crate::states::global_states::{CURRENT_SCREEN, Screen};
use crate::states::server_states::{MESSAGES, OUTBOX_MESSAGES};
use crate::states::user_states::{MemberProfile, USER};
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
use dioxus::signals::{ReadableExt, WritableExt, WritableVecExt};
use prost::Message as ProstMessage;
use tokio::task::JoinError;
use xmtp::{ConversationType, ListMessagesOptions, XmtpError};

// use reqwest::Url;
// use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use url::Url;
// use tauri::Emitter;
use xmtp::content::{ContentTypeId, EncodedContent, Reaction, ReactionAction, ReactionV2};
use xmtp::{
    AccountIdentifier, AlloySigner, Client, ConsentState, Conversation, CreateGroupOptions, Env,
    IdentifierKind, Message, MessageEvent, Recipient, Signer, content::Content,
};

use aes_gcm::{
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
    aead::{Aead, AeadCore, Generate, Key, KeyInit},
};

// #[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TextChannel {
    // id: String,
    // name: Option<String>,
    pub metadata: TextChannelMetaData,
    pub description: Option<String>,
    // creator: String,
    pub messages: Vec<Msg>,
    // pub messages: Vec<Arc<Msg>>,
    pub members: HashMap<String, MemberProfile>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Msg {
    pub id: String,
    // content: String,
    pub content: Content,
    // pub content_type: String,
    pub from: String,
    pub timestamp: i64,
    // pub reactions: Vec<Reaction>,
    pub reactions: HashMap<String, Vec<String>>, // pub reactions: Vec<ReactionV2>, Ready to add?
                                                 // pub reactions: Vec<Reaction>,
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

    // pub fn init_stream(&self, app: tauri::AppHandle) {
    pub fn init_stream(&self) {
        println!("initializing XMTP stream");
        let consent_states = vec![ConsentState::Allowed, ConsentState::Unknown];
        let handle =
            xmtp::stream::messages(&self.client, None, &consent_states).expect("stream failed");

        tokio::task::spawn_blocking(move || {
            while let Some(event) = handle.recv() {
                println!("event: {:?}", event);
                // let convo = self.client.conversation(&event.conversation_id).expect("failed to get convo").expect("no convo with that id");
                // let payload = serde_json::json!({
                //     "conversation_id": event.conversation_id,
                //     "message_id": event.message_id,
                // });

                // app.emit("xmtp_stream_event", payload)
                //     .expect("failed to emit iroh event");
            }
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
        println!("ANOTHER");

        // let m: Vec<Recipient> = vec![];

        let convo = self
            .client
            .group(members, &opts)
            .expect("failed to create xmtp group");
        println!("created new group: {:?}", convo.id());
        Ok(convo.id())
    }

    // pub fn get_conversation(&self, id: &str) -> Result<Conversation, String> {
    pub fn get_conversation(&self, id: &str) -> Result<TextChannel, String> {
        // change function name to get_text_channel?
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
            let profile = MemberProfile {
                address: address,
                name: "".to_string(),
                avatar: "".to_string(),
                description: "".to_string(),
            };
            member_profiles.insert(member.inbox_id, profile);
        }

        let mut messages = vec![];
        for message in convo.messages().expect("failed to get messages") {
            // let content_type = message.content_type.clone();
            // let a = message.conversation_id.clone();
            // println!("convo ID: {a}");
            // if content_type.is_some() {
            // let b = content_type.unwrap();
            // println!("content type: {b}");
            // }

            // if !message.reactions.is_empty() {
            // println!("REACTIONS: {:?}", message.reactions);
            // }

            let msg = self.get_msg(message).expect("failed to decode message");
            // messages.push(Arc::new(msg));
            messages.push(msg);
        }

        // let metadata = convo.metadata().expect("failed to get metadata");
        // println!("creator: {:?}", metadata.creator_inbox_id);
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
            // messages: convo.messages().expect("failed to get messages"),
            // creator: convo
            //     .metadata()
            //     .expect("failed to get convo metadata")
            //     .creator_inbox_id,
        };

        Ok(text_channel)
        // Ok(convo)
    }

    pub fn get_msg(&self, message: Message) -> Result<Msg, String> {
        let msg: Msg;
        let timestamp = message.sent_at_ns;
        let content = message.decode().expect("failed to decode message");
        let mut reactions: HashMap<String, Vec<String>> = HashMap::new();
        for reaction in message.reactions {
            if reactions.get(&reaction.content).is_some() {
                // let r = reactions.get(&reaction.content).unwrap();
                if reaction.action == ReactionAction::Added {
                    reactions
                        .entry(reaction.content)
                        .or_insert(Vec::new())
                        .push(reaction.sender_inbox_id);
                } else {
                    if let Some(users) = reactions.get_mut(&reaction.content) {
                        users.retain(|user| user != &reaction.sender_inbox_id);
                    }
                }
            } else {
                reactions.insert(reaction.content, vec![reaction.sender_inbox_id]);
            }
        }
        msg = Msg {
            id: message.id,
            // content: text,
            content: content,
            from: message.sender_inbox_id,
            // reactions: message.reactions,
            reactions: reactions,
            timestamp: timestamp,
        };
        Ok(msg)
    }

    pub fn get_conversations(&self) -> Result<Vec<Conversation>, String> {
        let convos = self
            .client
            .conversations()
            .expect("failed to list conversations");
        Ok(convos)
    }

    pub fn get_dms(&self) -> Result<Vec<Conversation>, String> {
        let convos = self
            .client
            .list_dms()
            .expect("failed to list dm conversations");
        // let mut dms = Vec::<Conversation>::new();
        // for convo in convos {
        //     if convo.name().is_none() {
        //         continue;
        //     }
        //     if convo.name().unwrap() == "Cliqu3 DM Group" {
        //         dms.push(convo);
        //     }
        // }

        Ok(convos)
    }

    pub async fn send_text(&self, id: &str, text: String) {
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");
        println!("before sending message");
        let text_clone = text.clone();
        let result = tokio::task::spawn_blocking(move || convo.send_text(&text)).await;
        self.update_message_states(text_clone, result);
    }

    pub fn update_message_states(
        &self,
        text: String,
        result: Result<Result<String, XmtpError>, JoinError>,
    ) {
        match result {
            Ok(Ok(id)) => {
                let message = self
                    .client
                    .message_by_id(&id)
                    .expect("failed 1")
                    .expect("failed 2");
                let msg = self.get_msg(message).expect("failed to get message");
                // MESSAGES.write().push(Arc::new(msg));
                match *CURRENT_SCREEN.read() {
                    Screen::Server => {
                        MESSAGES.write().push(msg);
                    }
                    Screen::DirectMessages => {
                        DM_MESSAGES.write().push(msg);
                    }
                    Screen::Settings => {
                        MESSAGES.write().push(msg);
                    }
                }
                // MESSAGES.write().push(msg);
                OUTBOX_MESSAGES.write().retain(|msg| msg != &text);
            }
            Ok(Err(e)) => println!("Failed to send message: {e}"),
            Err(e) => println!("spawn_blocking panicked: {e}"),
        }
    }

    pub async fn send_reaction(
        &self,
        id: &str,
        message_id: &str,
        emoji: &str,
        action: ReactionAction,
    ) {
        let user_inbox_id = USER.read().inbox_id.clone();
        self.apply_reaction(message_id, &user_inbox_id, emoji, action);
        let convo = self
            .client
            .conversation(id)
            .expect("failed to get convo")
            .expect("convo doesnt exist with id");
        println!("Sending Reaction!");
        // let options: ListMessagesOptions {

        // };
        // let a = convo.list_messages(options);
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
        let mut messages = MESSAGES.write();
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
        let text_clone = text.clone();
        let result =
            tokio::task::spawn_blocking(move || convo.send_text_reply(&reference, &text)).await;
        self.update_message_states(text_clone, result);
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

        // let convo = self
        //     .client
        //     .conversation(id)
        //     .expect("failed to get convo")
        //     .expect("convo doesnt exist with id");
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

    async fn add_member(&self, ids: Vec<String>, address: String) {
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
}

// struct InviteContentStuff {
//     ec: EncodedContent
// }

// impl  {
//     fn encode_to_vec(&self) -> Vec<u8>
//     where
//         Self: Sized,
//     {
//         let mut buf = Vec::with_capacity(self.encoded_len());

//         self.encode_raw(&mut buf);
//         buf
//     }
// }
