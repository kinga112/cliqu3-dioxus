use anyhow::{Error, Result, anyhow};
use futures::StreamExt;
// use bytes::Bytes;
// use futures_lite::{Stream, StreamExt};
use hmac::{Hmac, Mac};
// use iroh::discovery::dns::DnsDiscovery;
// use iroh::discovery::mdns::MdnsDiscovery;
// use iroh::discovery::pkarr::PkarrPublisher;
// use iroh::discovery::{Discovery, IntoDiscovery};
use iroh::{Endpoint, EndpointAddr, EndpointId, protocol::Router};
// use iroh::{NodeAddr, PublicKey, SecretKey, discovery};
use iroh::endpoint::presets;
use iroh::{PublicKey, SecretKey};
use iroh_blobs::api::downloader::{
    ContentDiscovery, DownloadProgress, Downloader, SupportedRequest,
};
use iroh_blobs::api::remote::Remote;
use iroh_blobs::protocol::Request;
use iroh_blobs::store::mem::MemStore;
use iroh_blobs::{ALPN as BLOBS_ALPN, BlobsProtocol, store::fs::FsStore};
use iroh_docs::ContentStatus;
use iroh_docs::api::Doc;
use iroh_docs::api::protocol::{AddrInfoOptions, ShareMode};
use iroh_docs::engine::{self, Engine, LiveEvent};
use iroh_docs::{
    ALPN as DOCS_ALPN, AuthorId, DocTicket, NamespaceId, protocol::Docs, store::Query,
};
use iroh_gossip::{ALPN as GOSSIP_ALPN, net::Gossip};
// use iroh_roq::ALPN as ROQ_ALPN;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use std::{path, vec};
// use tauri::Emitter;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;
// use iroh_docs::sync::
// use base64::{engine::general_purpose, Engine};
use iroh_live;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Server {
    // pub id: String,
    // pub name: String,
    pub creator_hash: String,
    pub metadata: ServerMetadata,
    // pub time_created: i64,
    // pub users: Vec<String>,
    pub text_channels: Vec<TextChannelMetaData>,
    // pub voice_channels: Vec<VoiceChannel>,
    pub voice_channels: HashMap<String, VoiceChannel>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ServerMetadata {
    pub id: String,
    pub ticket: String,
    pub name: String,
    pub pic: String,
    // pub creator: String,
    pub creator_address: String,
    // pub creatorHash:
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct VoiceChannel {
    pub name: String,
    pub ticket: String,
    // pub active_users: Vec<String>,
    pub active_users: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TextChannelMetaData {
    pub id: String,
    pub name: String,
}

pub struct ServerDocs {
    docs: Docs,
    // blobs: MemStore,
    blobs: FsStore,
    author: AuthorId,
    router: Router,
    cancellation_token: Option<CancellationToken>,
    // server_docs: Vec<Doc>,
    // user_docs: vec!<Doc>,
}

impl ServerDocs {
    // Create a new ServersDb (a single document)
    // add wallet param to append to path var for multi saved user support
    pub async fn new(path: path::PathBuf) -> Result<Self> {
        // let endpoint = Endpoint::builder().discovery_n0().bind().await?;
        // let mut rng = rand::rngs::OsRng;
        // let _key = iroh_base::SecretKey::generate(&mut rng);
        // let cancellation_token = CancellationToken::new();
        // let secret_key = SecretKey::generate(&mut rng);
        let bytes = [
            41, 114, 100, 25, 163, 215, 83, 118, 170, 9, 130, 194, 128, 166, 248, 242, 146, 19, 3,
            56, 185, 96, 153, 203, 31, 235, 40, 51, 198, 193, 87, 149,
        ];
        let secret_key = SecretKey::from_bytes(&bytes);
        // println!("secret key: {:?}", secret_key.to_bytes());
        // let endpoint = Endpoint::builder().secret_key(secret_key).relay_mode(iroh::RelayMode::Default).discovery_n0().bind().await?;
        // let endpoint = Endpoint::builder()
        //     .secret_key(secret_key)
        //     .relay_mode(iroh::RelayMode::Default)
        //     .discovery(PkarrPublisher::n0_dns())
        //     .discovery(DnsDiscovery::n0_dns())
        //     .discovery(MdnsDiscovery::builder())
        //     .bind()
        //     .await?;
        // let endpoint = Endpoint::builder(presets::N0)
        //     // .relay_mode(iroh::RelayMode::Default)
        //     .secret_key(secret_key)
        //     // .alpns(vec![b"my-alpn".to_vec()])
        //     .bind()
        //     .await?;

        let endpoint = Endpoint::bind(presets::N0).await?;

        let gossip = Gossip::builder().spawn(endpoint.clone());

        // let store = MemStore::new();

        // let blobs_path = path::PathBuf::from("C:/Users/kinga/Documents/cliqu3/iroh_blobs");
        // let docs_path = path::PathBuf::from("C:/Users/kinga/Documents/cliqu3/iroh_docs");
        // tokio::fs::create_dir_all(&blobs_path).await?;
        // tokio::fs::create_dir_all(&docs_path).await?;
        // println!("blobs path: {:?}", blobs_path.clone());
        println!("path: {:?}", path.clone());

        let blobs = FsStore::load(&path).await?;
        // let blobs = MemStore::default();

        let docs = Docs::persistent(path)
            .spawn(endpoint.clone(), (*blobs).clone(), gossip.clone())
            .await?;

        let author = docs.author_default().await?;

        // let engine = Engine::spawn(endpoint, gossip, replica_store, bao_store, downloader, default_author_storage, protect_cb);

        let router = Router::builder(endpoint.clone())
            .accept(BLOBS_ALPN, BlobsProtocol::new(&blobs, None))
            .accept(GOSSIP_ALPN, gossip.clone())
            .accept(DOCS_ALPN, docs.clone())
            .spawn();

        Ok(Self {
            docs,
            blobs,
            author,
            router,
            cancellation_token: None,
        })
    }

    pub async fn create_server(
        &self,
        name: &str,
        pic: &str,
        creator_address: &str,
    ) -> Result<ServerMetadata> {
        println!("create server");
        let creator_hash = self.creator_hash().expect("creator hash failed");
        println!("hash: {:?}", creator_hash);

        // let text_channels = vec![text_channel_id.to_string()];
        let voice_channel = VoiceChannel {
            name: "voice channel 1".to_string(),
            ticket: String::new(),
            // active_users: Vec::new(),
            active_users: HashMap::new(),
        };

        let uuid = Uuid::new_v4();

        let mut voice_channels = HashMap::new();
        voice_channels.insert(uuid.to_string(), voice_channel);

        let doc = self.docs.create().await?;

        println!("server docs id: {:?}", doc.id());

        let ticket = doc
            .share(ShareMode::Write, AddrInfoOptions::RelayAndAddresses)
            .await
            .unwrap();

        let metadata = ServerMetadata {
            id: doc.id().to_string(),
            ticket: ticket.to_string(),
            name: name.to_string(),
            pic: pic.to_string(),
            creator_address: creator_address.to_string(),
        };

        let creator_hash_bytes = serde_json::to_vec(&creator_hash)?;
        let metadata_bytes = serde_json::to_vec(&metadata)?;
        // let text_channels_bytes = serde_json::to_vec(&text_channels)?;
        let voice_channels_bytes = serde_json::to_vec(&voice_channels)?;

        // let creator_hash_bytes = serde_json::to_vec(&creator_hash)?;
        // let metadata_bytes = serde_json::to_vec(&metadata)?;
        // let text_channels_bytes = serde_json::to_vec(&text_channels)?;
        // let voice_channels_bytes = serde_json::to_vec(&voice_channels)?;

        // let s = match str::from_utf8(&metadata_bytes) {
        //     Ok(v) => v,
        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        // };
        // self.add_text_channel(&doc.id().to_string(), text_channel_id).await.expect("adding text channel failed");
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "creator_hash".to_string().into_bytes(),
                creator_hash_bytes,
            )
            .await?;
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "metadata".to_string().into_bytes(),
                metadata_bytes,
            )
            .await?;
        // let _ = doc.set_bytes(self.author.clone(), "text_channels".to_string().into_bytes(), text_channels_bytes.clone()).await?;
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "voice_channels".to_string().into_bytes(),
                voice_channels_bytes,
            )
            .await?;

        // Ok(doc.id().to_string())
        Ok(metadata)
    }

    pub fn creator_hash(&self) -> Result<String> {
        println!("creator hash");
        let mut mac = HmacSha256::new_from_slice(self.author.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(b"cliqu3-auth");

        let result = mac.finalize().into_bytes().to_vec();
        let creator_hash = serde_json::to_string(&result)?;

        Ok(creator_hash)
    }

    pub async fn update_server_metadata(
        &self,
        id: &str,
        metadata: ServerMetadata,
    ) -> Result<String> {
        // update server metadata if user is the creator
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");

        let creator_hash_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"creator_hash".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("creator_hash_entry not found"))?;

        let creator_hash_bytes = self
            .blobs
            .get_bytes(creator_hash_entry.content_hash())
            .await
            .unwrap();
        let stored_creator_hash: String = serde_json::from_slice(&creator_hash_bytes)?;
        let creator_hash = self.creator_hash()?;

        if creator_hash == stored_creator_hash {
            let server_id = NamespaceId::from_str(id)?;
            let doc = self.docs.open(server_id).await?.unwrap();
            let metadata_bytes = serde_json::to_vec(&metadata)?;
            println!("server docs id: {:?}", doc.id());
            let _ = doc
                .set_bytes(
                    self.author.clone(),
                    "metadata".to_string().into_bytes(),
                    metadata_bytes.clone(),
                )
                .await?;
            Ok(doc.id().to_string())
        } else {
            Ok("User does not have permission to update server metadata".to_string())
        }
    }

    // pub async fn update_voice_channel(&self, );

    pub async fn add_text_channel(
        &self,
        id: &str,
        name: &str,
        text_channel_id: &str,
    ) -> Result<()> {
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");
        // let text_channels_entry = doc.get_one(Query::single_latest_per_key()
        //                     .key_exact(&"text_channels".as_bytes()))
        //                     .await?.ok_or_else(|| anyhow!("Entry not found"))?;

        let option = doc
            .get_one(Query::single_latest_per_key().key_exact(&"text_channels".as_bytes()))
            .await
            .expect("failed to get entry");
        match option {
            Some(entry) => {
                let bytes = self
                    .blobs
                    .get_bytes(entry.content_hash())
                    .await
                    .expect("failed to get text_channels");
                let mut text_channels: Vec<TextChannelMetaData> = serde_json::from_slice(&bytes)
                    .expect("failed to convert text_channels bytes into json");
                text_channels.push(TextChannelMetaData {
                    id: text_channel_id.to_string(),
                    name: name.to_string(),
                });
                let new_bytes = serde_json::to_vec(&text_channels)?;
                let _ = doc
                    .set_bytes(
                        self.author.clone(),
                        "text_channels".to_string().into_bytes(),
                        new_bytes.clone(),
                    )
                    .await
                    .expect("set bytes failed 1");
            }
            None => {
                let text_channels = vec![TextChannelMetaData {
                    id: text_channel_id.to_string(),
                    name: name.to_string(),
                }];
                let text_channels_bytes = serde_json::to_vec(&text_channels).expect("serde failed");
                let _ = doc
                    .set_bytes(
                        self.author.clone(),
                        "text_channels".to_string().into_bytes(),
                        text_channels_bytes.clone(),
                    )
                    .await
                    .expect("set bytes failed 2");
            }
        }

        // let result = self.blobs.get_bytes(text_channels_entry.content_hash()).await;
        // let mut text_channels: Vec<String> = [];
        // match result {
        //     Ok(text_channels) => {
        //         let mut text_channels: Vec<String> = serde_json::from_slice(&text_channels_bytes).expect("failed to convert text_channels bytes into json");
        //         text_channels.push(chat_id.to_string());
        //     }
        //     Err(e) => {
        //         println!("No text channels?? : {:?}", e);
        //     }
        // }

        Ok(())
    }

    pub fn cancel(&mut self) {
        if self.cancellation_token.is_none() {
            self.cancellation_token = Some(CancellationToken::new());
        } else {
            // self.cancellation_token.unwrap().cancel();
            let token = self.cancellation_token.clone().unwrap();
            token.cancel();
            self.cancellation_token = None;
        }
    }

    pub async fn get_server(&self, id: &str) -> Result<Server> {
        // get server by id
        println!("get server: {:?}", id);
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");
        // let doc = self.docs.import(DocTicket::from_str(id).unwrap()).await.expect("getting server doc from ticket failed");
        let peers = doc.get_sync_peers().await.unwrap();
        if peers.is_some() {
            let mut addrs: Vec<EndpointAddr> = vec![];
            for peer in peers.unwrap() {
                let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                addrs.push(addr);
            }

            doc.start_sync(addrs).await.unwrap();
        }

        let metadata_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"metadata".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("metadata_entry not found"))?;

        let text_channels_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"text_channels".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("text_channels_entry not found"))?;

        let voice_channels_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"voice_channels".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("voice_channels_entry not found"))?;

        // let voice_channels_entry_result = doc.get_one(Query::single_latest_per_key()
        //                     .key_exact(&"voice_channels".as_bytes()))
        //                     .await;

        const MAX_FETCH_ATTEMPTS: u8 = 10;
        const RETRY_DELAY_MS: u64 = 1000;

        let mut attempts = 0;

        if self
            .blobs
            .has(voice_channels_entry.content_hash())
            .await
            .unwrap()
        {
            let bytes = self
                .blobs
                .get_bytes(voice_channels_entry.content_hash())
                .await
                .unwrap();
            // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
        } else {
            println!(
                "Remote entry received. Hash: {:?} is missing. Starting fetch loop.",
                voice_channels_entry.content_hash()
            );
            while !self
                .blobs
                .has(voice_channels_entry.content_hash())
                .await
                .unwrap_or(false)
                && attempts < MAX_FETCH_ATTEMPTS
            {
                attempts += 1;
                println!(
                    "Fetch attempt {}/{}. Forcing doc sync...",
                    attempts, MAX_FETCH_ATTEMPTS
                );
                let peers = doc.get_sync_peers().await.unwrap().unwrap();
                let mut addrs: Vec<EndpointAddr> = vec![];
                for peer in peers {
                    let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                    addrs.push(addr);
                }
                // 1. Force a sync: This re-initiates the content transfer request to the connected peer(s).
                match doc.start_sync(addrs).await {
                    Ok(_) => println!("INFO: Sync requested successfully."),
                    Err(e) => eprintln!("ERROR: Sync request failed: {:?}", e),
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;

                if self
                    .blobs
                    .has(voice_channels_entry.content_hash())
                    .await
                    .unwrap_or(false)
                {
                    println!("Content successfully fetched on attempt {}!", attempts);
                    let bytes = self
                        .blobs
                        .get_bytes(voice_channels_entry.content_hash())
                        .await
                        .unwrap();
                    break; // Exit the InsertRemote handling
                }
            }

            if attempts >= MAX_FETCH_ATTEMPTS {
                eprintln!(
                    "CRITICAL: Hash {:?} failed to fetch after {} attempts despite peer being present.",
                    voice_channels_entry.content_hash(),
                    MAX_FETCH_ATTEMPTS
                );
                // You may want to log this as a hard error or try to restart the endpoint/doc.
            }
        }

        // let mut voice_channels_bytes = Bytes::from("");

        // match voice_channels_entry_result {
        //     Ok(entry) => {
        //         let e = entry.unwrap();
        //         let result = self.blobs.get_bytes(e.content_hash()).await;
        //         match result {
        //             Ok(bytes) => {
        //                 voice_channels_bytes = bytes;
        //             }
        //             Err(e) => {
        //                 println!("FAILED TO GET VOICE CHANNEL BYTES FROM LOCAL BLOB BY IN GET SERVER: {:?}", e);
        //                 let peers = doc.get_sync_peers().await.unwrap().unwrap();
        //                 let mut addrs: Vec<NodeAddr> = vec![];
        //                 for peer in peers {
        //                     let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
        //                     addrs.push(addr);
        //                 }

        //                 let a = doc.start_sync(addrs).await;
        //                 println!("after start sync: {:?}", a);
        //                 // let new_bytes = self.blobs.get_bytes(e.content_hash()).await.expect("failed to get voice channels bytes after sync");
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!("FAILED TO GET VOICE CHANNEL ENTRY IN GET SERVER: {:?}", e);
        //         let peers = doc.get_sync_peers().await.unwrap().unwrap();
        //         let mut addrs: Vec<NodeAddr> = vec![];
        //         for peer in peers {
        //             let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
        //             addrs.push(addr);
        //         }

        //         let _ = doc.start_sync(addrs).await;
        //     }
        // }

        let metadata_bytes = self
            .blobs
            .get_bytes(metadata_entry.content_hash())
            .await
            .expect("failed to get metadata bytes from blob");
        let text_channels_bytes = self
            .blobs
            .get_bytes(text_channels_entry.content_hash())
            .await
            .expect("failed to get text_channels bytes from blob");
        let voice_channels_bytes = self
            .blobs
            .get_bytes(voice_channels_entry.content_hash())
            .await
            .expect("failed to get voice_channels bytes from blob");

        let metadata = serde_json::from_slice(&metadata_bytes)
            .expect("failed to convert metadata bytes into json");
        let text_channels = serde_json::from_slice(&text_channels_bytes)
            .expect("failed to convert text_channels bytes into json");
        let voice_channels: HashMap<String, VoiceChannel> =
            serde_json::from_slice(&voice_channels_bytes)
                .expect("failed to convert voice_channel bytes into json");

        let server = Server {
            creator_hash: "".to_string(), // only needed to update metadata
            metadata,
            text_channels,
            voice_channels,
        };

        Ok(server)
    }

    pub async fn get_server_metadata(&self, id: &str) -> Result<ServerMetadata> {
        // get server by id
        println!("get server metadata");
        let namespace_id = NamespaceId::from_str(id).expect("namespace id from string failed");
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .expect("opening doc with id failed")
            .expect("could not get server doc");

        let metadata_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"metadata".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("Entry not found"))?;

        let metadata_bytes = self.blobs.get_bytes(metadata_entry.content_hash()).await?;

        let metadata: ServerMetadata =
            serde_json::from_slice(&metadata_bytes).expect("deserializing json failed");
        // let ticket = doc.share(ShareMode::Write, Default::default()).await.expect("creating doc ticket failed");
        let ticket = doc
            .share(ShareMode::Write, AddrInfoOptions::RelayAndAddresses)
            .await
            .unwrap();
        println!("doc ticket in server metadata: {:?}", metadata.ticket);
        println!("new doc ticket: {:?}", ticket.to_string());
        // docaaacaij3sihmgrcu5mgkgmn3bebjtvjqwodu7h4yxsl7n5csjwzd7z4aafrlzso4fudoxq7fliucavtwp6n62lkr2ztufpoja5dkpcjapivdyaaa
        // println!("metadata: {:?}", metadata.id);
        // let ticket = doc.share(ShareMode::Write, Default::default()).await.expect("creating doc ticket failed");
        // let md = ServerMetadata {
        // id: metadata.id,
        // ticket: ticket.to_string(),
        // name: metadata.name,
        // pic: metadata.pic,
        // creator_address: metadata.creator_address,
        // };
        Ok(metadata)
    }

    pub async fn get_all_servers(&self) -> Result<Vec<ServerMetadata>, anyhow::Error> {
        let mut servers_list = Vec::new();
        let mut list = self
            .docs
            .list()
            .await
            .expect("getting list of all docs failed");
        while let Some(result) = list.next().await {
            match result {
                Ok((namespace_id, capability)) => {
                    // println!(
                    //     "Namespace: {:?}, Capability: {:?}",
                    //     namespace_id, capability
                    // );
                    let result = self.get_server_metadata(&namespace_id.to_string()).await;
                    match result {
                        Ok(server) => {
                            servers_list.push(server);
                        }
                        Err(e) => {
                            eprint!("getting server metadata failed: {:?}", e);
                        }
                    }
                    // servers_list.push(server);
                }
                Err(e) => {
                    eprintln!("Error reading stream: {:?}", e);
                }
            }
        }
        // while let Some(result) = list.next().await {

        // }
        Ok(servers_list)
    }

    pub async fn invite(&self, id: &str) -> Result<String> {
        println!("Running invite!");
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");
        println!("invite doc id: {:?}", doc.id().to_string());
        let ticket = doc.share(ShareMode::Write, Default::default()).await?;
        println!("ticket: {:?}", ticket.to_string());
        Ok(ticket.to_string())
    }

    pub async fn join_server_with_ticket(&self, ticket: &str) -> Result<String> {
        let doc_ticket = DocTicket::from_str(ticket)?;
        let server_doc = self
            .docs
            .import(doc_ticket)
            .await
            .expect("failed to import new server doc from ticket");
        server_doc
            .share(ShareMode::Write, Default::default())
            .await?;
        println!("Joined new server: {:?}", server_doc.id().to_string());
        // let result = self.get_server_metadata(&server_doc.id().to_string()).await;
        // match result {
        // Ok(metadata) => {
        // println!("metadata name: {:?}", metadata.name);
        // }
        // Err(e) => {
        // eprint!("getting server metadata on join server failed");
        // }
        // }
        Ok(server_doc.id().to_string())
    }

    pub async fn join_server(&self, server: Server) -> Result<String> {
        // let doc_ticket = DocTicket::from_str(&server.metadata.ticket)?;
        // let server_doc = self
        //     .docs
        //     .import(doc_ticket)
        //     .await
        //     .expect("failed to import new server doc from ticket");

        // let a = self.docs.

        // server_doc
        //     .share(ShareMode::Write, Default::default())
        //     .await?;
        let doc = self.docs.create().await?;

        let creator_hash_bytes = serde_json::to_vec(&server.creator_hash)?;
        let metadata_bytes = serde_json::to_vec(&server.metadata)?;
        let text_channels_bytes = serde_json::to_vec(&server.text_channels)?;
        let voice_channels_bytes = serde_json::to_vec(&server.voice_channels)?;

        let _ = doc
            .set_bytes(
                self.author.clone(),
                "creator_hash".to_string().into_bytes(),
                creator_hash_bytes,
            )
            .await?;
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "metadata".to_string().into_bytes(),
                metadata_bytes,
            )
            .await?;
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "text_channels".to_string().into_bytes(),
                text_channels_bytes.clone(),
            )
            .await?;
        let _ = doc
            .set_bytes(
                self.author.clone(),
                "voice_channels".to_string().into_bytes(),
                voice_channels_bytes,
            )
            .await?;

        println!("Joined new server: {:?}", server.metadata.id.to_string());
        // let result = self.get_server_metadata(&server_doc.id().to_string()).await;
        // match result {
        // Ok(metadata) => {
        // println!("metadata name: {:?}", metadata.name);
        // }
        // Err(e) => {
        // eprint!("getting server metadata on join server failed");
        // }
        // }
        Ok(server.metadata.id.to_string())
    }

    // pub async fn add_user_to_call(
    //     &self,
    //     id: &str,
    //     voice_channel_id: &str,
    //     user: &str,
    // ) -> Result<Endpoint> {
    //     println!("inside add user to call: id, voice id, user");
    //     println!("{:?}", id.to_string());
    //     println!("{:?}", voice_channel_id.to_string());
    //     println!("{:?}", user.to_string());
    //     let endpoint = Endpoint::builder()
    //         .discovery_n0()
    //         .alpns(vec![ROQ_ALPN.to_vec()])
    //         .bind()
    //         .await?;
    //     let namespace_id = NamespaceId::from_str(id)?;
    //     let doc = self
    //         .docs
    //         .open(namespace_id)
    //         .await
    //         .unwrap()
    //         .expect("could not get server doc");
    //     // let doc = self.docs.import(DocTicket::from_str(id).unwrap()).await.expect("could not get doc from ticket in add user to call");

    //     let voice_channels_entry = doc
    //         .get_one(Query::single_latest_per_key().key_exact(&"voice_channels".as_bytes()))
    //         .await?
    //         .ok_or_else(|| anyhow!("Entry not found"))?;

    //     let voice_channels_bytes = self
    //         .blobs
    //         .get_bytes(voice_channels_entry.content_hash())
    //         .await
    //         .unwrap();
    //     let mut voice_channels: HashMap<String, VoiceChannel> =
    //         serde_json::from_slice(&voice_channels_bytes)?;

    //     let mut voice_channel = voice_channels.get(voice_channel_id).unwrap().clone();
    //     // voice_channel.active_users.push(user.to_string());
    //     println!("node id: {:?}", endpoint.node_id().clone().to_string());
    //     voice_channel
    //         .active_users
    //         .push(endpoint.node_id().to_string());

    //     voice_channels.insert(voice_channel_id.to_string(), voice_channel);

    //     let voice_channels_bytes = serde_json::to_vec(&voice_channels)?;

    //     let _ = doc
    //         .set_bytes(
    //             self.author.clone(),
    //             "voice_channels".to_string().into_bytes(),
    //             voice_channels_bytes.clone(),
    //         )
    //         .await?;
    //     Ok(endpoint)
    // }

    pub async fn remove_user_from_call(
        &self,
        id: &str,
        voice_channel_id: &str,
        user: &str,
    ) -> Result<()> {
        println!("inside remove user to call: id, voice id, user");
        println!("{:?}", id.to_string());
        println!("{:?}", voice_channel_id.to_string());
        println!("{:?}", user.to_string());
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");

        // let doc = self.docs.import(DocTicket::from_str(id).unwrap()).await.expect("could not get doc from ticket in add user to call");

        let voice_channels_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"voice_channels".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("Entry not found"))?;

        let voice_channels_bytes = self
            .blobs
            .get_bytes(voice_channels_entry.content_hash())
            .await
            .unwrap();
        let mut voice_channels: HashMap<String, VoiceChannel> =
            serde_json::from_slice(&voice_channels_bytes)?;

        let mut voice_channel = voice_channels.get(voice_channel_id).unwrap().clone();
        // let _ = voice_channel
        //     .active_users
        //     .retain(|x| x != &user.to_string());
        // voice_channel.active_users.remove()

        voice_channels.insert(voice_channel_id.to_string(), voice_channel);

        let voice_channels_bytes = serde_json::to_vec(&voice_channels)?;

        let _ = doc
            .set_bytes(
                self.author.clone(),
                "voice_channels".to_string().into_bytes(),
                voice_channels_bytes.clone(),
            )
            .await?;

        Ok(())
    }

    pub async fn get_caller_id(&self, id: &str, voice_channel_id: &str) -> Result<String> {
        let namespace_id = NamespaceId::from_str(id)?;
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");

        let voice_channels_entry = doc
            .get_one(Query::single_latest_per_key().key_exact(&"voice_channels".as_bytes()))
            .await?
            .ok_or_else(|| anyhow!("Entry not found"))?;

        let voice_channels_bytes = self
            .blobs
            .get_bytes(voice_channels_entry.content_hash())
            .await
            .unwrap();
        let voice_channels: HashMap<String, VoiceChannel> =
            serde_json::from_slice(&voice_channels_bytes)?;

        let voice_channel = voice_channels.get(voice_channel_id).unwrap().clone();

        // Ok(voice_channel.active_users[0].clone())
        Ok(String::new())
    }

    // pub async fn set_current_server(&self, id: &str, app: tauri::AppHandle) -> Result<()> {

    pub fn handle_subscription_task_token(&mut self) {
        if self.cancellation_token.is_none() {
            println!("no sub running, creating fresh token");
            self.cancellation_token = Some(CancellationToken::new());
        } else {
            // self.cancellation_token.unwrap().cancel();
            println!("canceling current sub task, and creating fresh token");
            let token = self.cancellation_token.clone().unwrap();
            token.cancel();
            // self.cancellation_token = None;
            self.cancellation_token = Some(CancellationToken::new());
        }
    }

    //// SUBSCRIPTION IS NOT WORKING FIX WITH TICKET!
    ///      when user1 (whoever exits app first) exits app and reopens while user2 is still connected,
    ///      server updates cannot be sent from that user1 to user2 after they rejoin,
    ///      user1 can get updates from user2
    ///
    // pub async fn set_current_server(&self, ticket_str: &str, app: tauri::AppHandle) -> Result<()> {
    // pub async fn set_current_server(&mut self, id: &str, app: tauri::AppHandle) -> Result<()> {
    pub async fn set_current_server(&mut self, id: &str) -> Result<()> {
        println!("inside set_current_server");
        self.handle_subscription_task_token();
        let namespace_id = NamespaceId::from_str(id)?;
        // let ticket = DocTicket::from_str(ticket_str).unwrap();
        // let (_doc, mut subscription) = self.docs.import_and_subscribe(ticket).await.expect("subscription failed");
        // let doc = self.docs.import(ticket).await.expect("import failed");
        let doc = self
            .docs
            .open(namespace_id)
            .await
            .unwrap()
            .expect("could not get server doc");
        let status = doc.status().await.unwrap();
        println!("doc status: {:?}", status);

        let peers = doc.get_sync_peers().await.unwrap();
        if peers.is_some() {
            let mut addrs: Vec<EndpointAddr> = vec![];
            for peer in peers.unwrap() {
                let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                addrs.push(addr);
            }
            doc.start_sync(addrs).await.unwrap();
        }

        // let subscription = doc.subscribe().await.expect("subscription failed");
        let mut subscription = doc.subscribe().await.unwrap();

        println!("after subscription in set_current_server");
        // let app_handle = app.clone();

        let blobs = self.blobs.clone();
        // let endpoint = self.router.endpoint();
        let token = self.cancellation_token.clone().unwrap();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        println!("Changed server, shutting down subscription task");
                        return;
                    }
                    Some(result) = subscription.next() => {
                        println!("running subscription!!!");
                        match result {
                            Ok(event) => {
                                match &event {
                                    LiveEvent::InsertLocal { entry } => {
                                        let bytes = blobs.get_bytes(entry.content_hash()).await.expect("failed to get entry bytes from blob");
                                        let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                        let payload = serde_json::json!({
                                            "event": format!("{:?}", event),
                                            "data": data // or parse if structured
                                        });

                                        println!("updated entry payload: {:?}", payload.to_string());

                                        // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");

                                    },
                                    LiveEvent::InsertRemote { from, entry, content_status } => {
                                        const MAX_FETCH_ATTEMPTS: u8 = 10;
                                        const RETRY_DELAY_MS: u64 = 1000;
                                        println!("Remote entry received. Hash: {:?} is missing. Starting fetch loop.", entry.content_hash());

                                        let mut attempts = 0;

                                        if *content_status == ContentStatus::Complete {
                                            let bytes = blobs.get_bytes(entry.content_hash()).await.unwrap();
                                            let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                            let payload = serde_json::json!({
                                                "event": format!("{:?}", event),
                                                "data": data // or parse if structured
                                            });

                                            println!("REMOTE: updated entry payload: {:?}", payload.to_string());

                                            // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                        } else {
                                            while !blobs.has(entry.content_hash()).await.unwrap_or(false) && attempts < MAX_FETCH_ATTEMPTS {
                                                attempts += 1;
                                                println!("Fetch attempt {}/{}. Forcing doc sync...", attempts, MAX_FETCH_ATTEMPTS);
                                                let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                                let mut addrs: Vec<EndpointAddr> = vec![];
                                                for peer in peers {
                                                    let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                                    addrs.push(addr);
                                                }
                                                // 1. Force a sync: This re-initiates the content transfer request to the connected peer(s).
                                                match doc.start_sync(addrs).await {
                                                    Ok(_) => println!("INFO: Sync requested successfully."),
                                                    Err(e) => eprintln!("ERROR: Sync request failed: {:?}", e),
                                                }

                                                // 2. Wait for the transfer attempt to happen over the network.
                                                // On a stable LAN, this is where the successful transfer should occur.
                                                tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;

                                                // 3. Check again: If successful, the loop breaks and ContentReady should fire.
                                                if blobs.has(entry.content_hash()).await.unwrap_or(false) {
                                                    println!("Content successfully fetched on attempt {}!", attempts);
                                                    let bytes = blobs.get_bytes(entry.content_hash()).await.unwrap();
                                                    let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                                    let payload = serde_json::json!({
                                                        "event": format!("{:?}", event),
                                                        "data": data // or parse if structured
                                                    });

                                                    println!("REMOTE: updated entry payload: {:?}", payload.to_string());

                                                    // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                                    break; // Exit the InsertRemote handling
                                                }
                                            }

                                            if attempts >= MAX_FETCH_ATTEMPTS {
                                                eprintln!("CRITICAL: Hash {:?} failed to fetch after {} attempts despite peer being present.", entry.content_hash(), MAX_FETCH_ATTEMPTS);
                                                // You may want to log this as a hard error or try to restart the endpoint/doc.
                                            }
                                        }

                                        // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                        // let payload = serde_json::json!({
                                        //     "event": format!("{:?}", event),
                                        //     "data": data // or parse if structured
                                        // });

                                        // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

                                        // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                    },
                                    LiveEvent::ContentReady { hash } => {
                                        println!("    [    HASH IS READY    ]    ");
                                        // let bytes = blobs.get_bytes(*hash).await.expect("failed to get bytes in Content Ready");
                                        // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                        // println!("HASH DATA: {:?}", data);
                                        // println!("DATA FROM HASH : {:?}", data);
                                        // let payload = serde_json::json!({
                                        //     "event": format!("{:?}", event),
                                        //     "data": "missing data" // or parse if structured
                                        // });

                                        // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

                                        // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                        // let payload = serde_json::json!({
                                        //     "event": format!("{:?}", event),
                                        //     "data": data // or parse if structured
                                        // });

                                        // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");

                                        match blobs.get_bytes(*hash).await {
                                            Ok(bytes) => {
                                                let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
                                                println!("DATA FROM HASH : {:?}", data);

                                                let payload = serde_json::json!({
                                                    "event": format!("{:?}", event),
                                                    "data": data
                                                });

                                                println!("CONTENT READY: updated entry payload: {:?}", payload.to_string());
                                                // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                            },
                                            Err(e) => {
                                                eprintln!("FATAL ERROR: Failed to get bytes for hash {:?} after ContentReady: {:?}", hash, e);

                                                // 💡 REPAIR STEP: Trigger a full content re-sync for all neighbors
                                                // This will attempt to re-fetch the content that the doc believes should be local.

                                                let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                                let mut addrs: Vec<EndpointAddr> = vec![];
                                                for peer in peers {
                                                    let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                                    addrs.push(addr);
                                                }

                                                match doc.start_sync(addrs).await {
                                                    Ok(_) => println!("INFO: Triggered full document sync to repair missing content."),
                                                    Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
                                                }

                                                // let payload = serde_json::json!({
                                                //     "event": format!("{:?}", event),
                                                //     "data": format!("content read failed, attempted sync: {:?}", e)
                                                // });
                                                // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
                                            }
                                        }

                                    },
                                    LiveEvent::PendingContentReady => {
                                        println!("    [    PENDING CONTENT IS READY    ]    ");
                                        // let voice_channels_entry_result = doc.get_one(Query::single_latest_per_key()
                                        //     .key_exact(&"voice_channels".as_bytes()))
                                        //     .await;
                                        // // let bytes = blobs.get_bytes(entry.content_hash()).await.expect("blob get bytes failed");
                                        // // let voice_channels: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert voice_channel bytes into json");
                                        // // println!("Got Pending Content: {:?}", voice_channels);

                                        // match voice_channels_entry_result {
                                        //     Ok(entry) => {
                                        //         let e = entry.unwrap();
                                        //         let result = blobs.get_bytes(e.content_hash()).await;
                                        //         match result {
                                        //             Ok(bytes) => {
                                        //                 println!("GOT BYTES!");
                                        //                 let voice_channels: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert voice_channel bytes into json");
                                        //                 println!("Got Pending Content: {:?}", voice_channels);
                                        //             }
                                        //             Err(e) => {
                                        //                 println!("FAILED TO GET VOICE CHANNEL BYTES FROM LOCAL BLOB: {:?}", e);
                                        //                 // let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                        //                 // let mut addrs: Vec<NodeAddr> = vec![];
                                        //                 // for peer in peers {
                                        //                 //     let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                        //                 //     addrs.push(addr);
                                        //                 // }

                                        //                 // let a = doc.start_sync(addrs).await;
                                        //                 // println!("after start sync: {:?}", a);
                                        //                 // let new_bytes = self.blobs.get_bytes(e.content_hash()).await.expect("failed to get voice channels bytes after sync");
                                        //             }
                                        //         }
                                        //     }
                                        //     Err(e) => {
                                        //         println!("FAILED TO GET VOICE CHANNEL ENTRY : {:?}", e);
                                        //         // let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                        //         // let mut addrs: Vec<NodeAddr> = vec![];
                                        //         // for peer in peers {
                                        //         //     let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                        //         //     addrs.push(addr);
                                        //         // }

                                        //         // let _ = doc.start_sync(addrs).await;
                                        //     }
                                        // }

                                    },
                                    LiveEvent::NeighborUp(node_id) => {
                                        println!("    [    LIVE EVENT: PEER UP    ]    ");
                                        let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                        let mut addrs: Vec<EndpointAddr> = vec![];
                                        for peer in peers {
                                            let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                            addrs.push(addr);
                                        }

                                        match doc.start_sync(addrs).await {
                                            Ok(_) => println!("Peer Joined: Start Sync"),
                                            Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
                                        }
                                    },
                                    LiveEvent::NeighborDown(node_id) => {
                                        println!("    [    LIVE EVENT: PEER DOWN    ]    ");
                                        let peers = doc.get_sync_peers().await.unwrap().unwrap();
                                        let mut addrs: Vec<EndpointAddr> = vec![];
                                        for peer in peers {
                                            let addr = EndpointAddr::new(PublicKey::from_bytes(&peer).unwrap());
                                            addrs.push(addr);
                                        }

                                        match doc.start_sync(addrs).await {
                                            Ok(_) => println!("Peer Left: Start Sync"),
                                            Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
                                        }
                                    },
                                    LiveEvent::SyncFinished(sync) => {
                                        println!("    [    LIVE EVENT: SYNC FINISHED   ]    ");
                                    },
                                    _ => {

                                    },
                                };
                            }
                            Err(e) => {
                                eprintln!("Error in subscription: {:?}", e);
                            }
                        }
                        // println!("");
                        // println!("   [    AFTER SUBSCRIPTION    ]   ");
                        // println!("");
                    }
                }
            }

            // while let Some(result) = subscription.next().await {
            //     match result {
            //         // Ok(event) => match event {
            //         Ok(event) => {
            //             match &event {
            //                 LiveEvent::InsertLocal { entry } => {
            //                     let bytes = blobs.get_bytes(entry.content_hash()).await.expect("failed to get entry bytes from blob");
            //                     let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                     let payload = serde_json::json!({
            //                         "event": format!("{:?}", event),
            //                         "data": data // or parse if structured
            //                     });

            //                     println!("updated entry payload: {:?}", payload.to_string());

            //                     let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                 },
            //                 LiveEvent::InsertRemote { from, entry, content_status } => {
            //                     println!("NEW REMOTE CONTENT");
            //                     println!("CONTENT STATUS: {:?} ", content_status);
            //                     println!("FROM: {:?} ", from.to_string());
            //                     // let bytes = blobs.get_bytes(entry.content_hash()).await.expect("failed to get entry bytes from blob");
            //                     if *content_status == ContentStatus::Missing {
            //                         println!("CONTENT MISSING! Not sending emit to run get server, but statying sync");
            //                         // let payload = serde_json::json!({
            //                         //     "event": format!("{:?}", event),
            //                         //     "data": "missing data" // or parse if structured
            //                         // });

            //                         // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

            //                         // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                         // 💡 REPAIR STEP: Trigger a full content re-sync for all neighbors
            //                         // This will attempt to re-fetch the content that the doc believes should be local.

            //                         let peers = doc.get_sync_peers().await.unwrap().unwrap();
            //                         let mut addrs: Vec<NodeAddr> = vec![];
            //                         for peer in peers {
            //                             let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
            //                             addrs.push(addr);
            //                         }

            //                         match doc.start_sync(addrs).await {
            //                             Ok(_) => println!("INFO: Triggered full document sync to repair missing content."),
            //                             Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
            //                         }
            //                     } else if *content_status == ContentStatus::Complete {
            //                         let result = blobs.get_bytes(entry.content_hash()).await;
            //                         match result {
            //                             Ok(bytes) => {
            //                                 let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                                 let payload = serde_json::json!({
            //                                     "event": format!("{:?}", event),
            //                                     "data": data // or parse if structured
            //                                 });

            //                                 println!("REMOTE: updated entry payload: {:?}", payload.to_string());

            //                                 let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                             }
            //                             Err(e) => {
            //                                 // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                                 println!("Blobs get bytes error! Not sending emit to run get server");
            //                                 eprintln!("❌ FATAL ERROR: Failed to get bytes for entry hash after ContentReady: {:?}", e);
            //                                 // 💡 REPAIR STEP: Trigger a full content re-sync for all neighbors
            //                                 // This will attempt to re-fetch the content that the doc believes should be local.

            //                                 let peers = doc.get_sync_peers().await.unwrap().unwrap();
            //                                 let mut addrs: Vec<NodeAddr> = vec![];
            //                                 for peer in peers {
            //                                     let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
            //                                     addrs.push(addr);
            //                                 }

            //                                 match doc.start_sync(addrs).await {
            //                                     Ok(_) => println!("INFO: Triggered full document sync to repair missing content."),
            //                                     Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
            //                                 }
            //                                 // let payload = serde_json::json!({
            //                                 //     "event": format!("{:?}", event),
            //                                 //     "data": "missing data" // or parse if structured
            //                                 // });

            //                                 // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

            //                                 // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");

            //                             }
            //                         }
            //                     } else {
            //                         print!("ELSE CONTENT STATUS: {:?}", content_status);
            //                     }
            //                     // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                     // let payload = serde_json::json!({
            //                     //     "event": format!("{:?}", event),
            //                     //     "data": data // or parse if structured
            //                     // });

            //                     // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

            //                     // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                 },
            //                 LiveEvent::ContentReady { hash } => {
            //                     println!("    [    HASH IS READY    ]    ");
            //                     // let bytes = blobs.get_bytes(*hash).await.expect("failed to get bytes in Content Ready");
            //                     // let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                     // println!("HASH DATA: {:?}", data);
            //                     // println!("DATA FROM HASH : {:?}", data);
            //                     // let payload = serde_json::json!({
            //                     //     "event": format!("{:?}", event),
            //                     //     "data": "missing data" // or parse if structured
            //                     // });

            //                     // println!("REMOTE: updated entry payload: {:?}", payload.to_string());

            //                     // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                     // let payload = serde_json::json!({
            //                     //     "event": format!("{:?}", event),
            //                     //     "data": data // or parse if structured
            //                     // });

            //                     // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");

            //                     match blobs.get_bytes(*hash).await {
            //                         Ok(bytes) => {
            //                             let data: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert bytes into json");
            //                             println!("DATA FROM HASH : {:?}", data);

            //                             let payload = serde_json::json!({
            //                                 "event": format!("{:?}", event),
            //                                 "data": data
            //                             });

            //                             println!("CONTENT READY: updated entry payload: {:?}", payload.to_string());
            //                             let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                         },
            //                         Err(e) => {
            //                             eprintln!("❌ FATAL ERROR: Failed to get bytes for hash {:?} after ContentReady: {:?}", hash, e);

            //                             // 💡 REPAIR STEP: Trigger a full content re-sync for all neighbors
            //                             // This will attempt to re-fetch the content that the doc believes should be local.

            //                             let peers = doc.get_sync_peers().await.unwrap().unwrap();
            //                             let mut addrs: Vec<NodeAddr> = vec![];
            //                             for peer in peers {
            //                                 let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
            //                                 addrs.push(addr);
            //                             }

            //                             match doc.start_sync(addrs).await {
            //                                 Ok(_) => println!("INFO: Triggered full document sync to repair missing content."),
            //                                 Err(sync_e) => eprintln!("WARNING: Failed to trigger document sync: {:?}", sync_e),
            //                             }

            //                             // let payload = serde_json::json!({
            //                             //     "event": format!("{:?}", event),
            //                             //     "data": format!("content read failed, attempted sync: {:?}", e)
            //                             // });
            //                             // let _ = app_handle.emit("iroh_event", payload).expect("failed to emit iroh event");
            //                         }
            //                     }

            //                 },
            //                 LiveEvent::PendingContentReady => {
            //                     println!("    [    PENDING CONTENT IS READY    ]    ");
            //                     let voice_channels_entry_result = doc.get_one(Query::single_latest_per_key()
            //                         .key_exact(&"voice_channels".as_bytes()))
            //                         .await;
            //                     // let bytes = blobs.get_bytes(entry.content_hash()).await.expect("blob get bytes failed");
            //                     // let voice_channels: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert voice_channel bytes into json");
            //                     // println!("Got Pending Content: {:?}", voice_channels);

            //                     match voice_channels_entry_result {
            //                         Ok(entry) => {
            //                             let e = entry.unwrap();
            //                             let result = blobs.get_bytes(e.content_hash()).await;
            //                             match result {
            //                                 Ok(bytes) => {
            //                                     println!("GOT BYTES!");
            //                                     let voice_channels: serde_json::Value = serde_json::from_slice(&bytes).expect("failed to convert voice_channel bytes into json");
            //                                     println!("Got Pending Content: {:?}", voice_channels);
            //                                 }
            //                                 Err(e) => {
            //                                     println!("FAILED TO GET VOICE CHANNEL BYTES FROM LOCAL BLOB BY IN GET SERVER: {:?}", e);
            //                                     let peers = doc.get_sync_peers().await.unwrap().unwrap();
            //                                     let mut addrs: Vec<NodeAddr> = vec![];
            //                                     for peer in peers {
            //                                         let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
            //                                         addrs.push(addr);
            //                                     }

            //                                     let a = doc.start_sync(addrs).await;
            //                                     println!("after start sync: {:?}", a);
            //                                     // let new_bytes = self.blobs.get_bytes(e.content_hash()).await.expect("failed to get voice channels bytes after sync");
            //                                 }
            //                             }
            //                         }
            //                         Err(e) => {
            //                             println!("FAILED TO GET VOICE CHANNEL ENTRY IN GET SERVER: {:?}", e);
            //                             let peers = doc.get_sync_peers().await.unwrap().unwrap();
            //                             let mut addrs: Vec<NodeAddr> = vec![];
            //                             for peer in peers {
            //                                 let addr = NodeAddr::new(PublicKey::from_bytes(&peer).unwrap());
            //                                 addrs.push(addr);
            //                             }

            //                             let _ = doc.start_sync(addrs).await;
            //                         }
            //                     }

            //                 },
            //                 LiveEvent::NeighborUp(node_id) => {
            //                     println!("    [    LIVE EVENT: PEER UP    ]    ");
            //                     let node_addr = NodeAddr::new(*node_id);
            //                     doc.start_sync(vec![node_addr]).await.unwrap();
            //                 },
            //                 LiveEvent::NeighborDown(node_id) => {
            //                     println!("    [    LIVE EVENT: PEER DOWN    ]    ");
            //                 },
            //                 _ => {},
            //             };
            //         },
            //         Err(e) => {
            //             eprintln!("Error in subscription: {:?}", e);
            //         }
            //     }
            // }
        });
        Ok(())
    }
}
