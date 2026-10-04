use crate::modules::walletconnect::signer::WalletConnectXmtpSigner;
use crate::modules::walletconnect::{connection::Handler, methods};
// use crate::xmtp::XMTP;
use crate::modules::xmtp::xmtp::XMTP;
use reown_relay_client::{
    ConnectionOptions,
    websocket::{Client, PublishedMessage},
};
use reown_relay_rpc::{
    auth::{AuthToken, ed25519_dalek::SigningKey},
    domain::Topic,
};
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use url::Url;
use x25519_dalek::{EphemeralSecret, PublicKey};

pub struct WalletConnectHandler {
    pub uri: String,
    client: Client,
    relay_rx: Option<Receiver<PublishedMessage>>,
}

impl WalletConnectHandler {
    pub fn new(uri: String) -> Self {
        let (relay_tx, relay_rx) = mpsc::channel::<PublishedMessage>(100);
        let handler = Handler::new("app_client", relay_tx.clone());
        let client = Client::new(handler);

        Self {
            uri,
            client,
            relay_rx: Some(relay_rx),
        }
    }

    /*
        Completes full Wallet Connect request
        Connect ->
        Subscribe to Pairing Topic ->
        Session Proposal ->
        Subscribe to Session Topic ->
        Personal Sign Request ->
        Signature to XMTP client
    */
    pub async fn init(&mut self, xmtp_tx: oneshot::Sender<XMTP>) {
        let client_clone = self.client.clone();

        self.connect().await;

        let (pairing_topic_hex, pairing_sym_key_hex) = self.parse_uri(&self.uri);
        let pairing_topic = Topic::from(pairing_topic_hex);
        self.client
            .subscribe(pairing_topic.clone())
            .await
            .expect("failed to subscribe to topic");

        let secret = EphemeralSecret::random_from_rng(&mut rand::thread_rng());
        let public_key = PublicKey::from(&secret);
        let message = methods::build_session_propose_message(pairing_sym_key_hex, public_key);

        self.client
            .publish(
                pairing_topic,
                Arc::from(message),
                None,
                1100,
                Duration::from_secs(300),
                true,
            )
            .await
            .expect("failed to public message");
        println!("Published session proposal in Connect");

        let relay_rx = self.relay_rx.take().expect("relay rx already taken");
        self.receive_and_reply(client_clone, Some(secret), relay_rx, Some(xmtp_tx));
    }

    // Connect to WC Web Socket Relay
    async fn connect(&self) {
        let key = SigningKey::generate(&mut rand::thread_rng());
        let auth = AuthToken::new("https://cliqu3.com")
            .aud("wss://relay.walletconnect.com")
            .ttl(Duration::from_secs(60 * 60))
            .as_jwt(&key)
            .expect("Failed to create JWT");
        println!("Generated JWT for authentication: {}", auth);

        let opts = ConnectionOptions::new("418defda3aa82cefc151946c325b1bdf", auth)
            .with_address("wss://relay.walletconnect.com");

        self.client
            .connect(&opts)
            .await
            .expect("failed to connect client");
    }

    /*
        Spawns Tokio Task for Relay and Signer Receivers
        Receive Relay messages and Decrypt
        Reply to decrypted messages accordingly

        Receive Sign request to build Personal Sign message
    */
    fn receive_and_reply(
        &mut self,
        client_clone: Client,
        mut secret_opt: Option<EphemeralSecret>,
        mut relay_rx: Receiver<PublishedMessage>,
        mut xmtp_tx: Option<oneshot::Sender<XMTP>>,
    ) {
        let (signer_tx, mut signer_rx) = mpsc::channel::<(String, oneshot::Sender<Vec<u8>>)>(10);
        let (_, pairing_sym_key_hex) = self.parse_uri(&self.uri);
        let cancelation_token = CancellationToken::new();
        tokio::spawn(async move {
            let mut persistent_session_sym_key: Option<String> = None;
            let mut persistent_session_topic_opt: Option<Topic> = None;
            let mut user_address: Option<String> = None;

            // The tx sent by the signer and received by signer_rx to send the signature bytes back to signer
            let mut signature_response_tx_opt: Option<oneshot::Sender<Vec<u8>>> = None;

            loop {
                tokio::select! {

                    // Receives messages from Connection Handler (WC Relay)
                    // Tag details: https://specs.walletconnect.com/2.0/specs/clients/sign/rpc-methods
                    Some(message) = relay_rx.recv() => {
                        // println!("Received message in listener: {}", message);
                        match message.tag {
                            1101 => {
                                // wc_sessionPropose Approval from Pairing Topic
                                // Approve response: Encrypted Message with Session Topic and Session Sym Key
                                println!("Received Response: wc_sessionPropose success: Approve tag 1101");
                                if let Some(secret) = secret_opt.take() {
                                    let (session_topic, session_sym_key_hex) =
                                        methods::decrypt_session_propose_response(message, pairing_sym_key_hex.clone(), secret)
                                        .expect("failed to decrypt dession propose response");
                                    persistent_session_sym_key = Some(session_sym_key_hex);
                                    persistent_session_topic_opt = Some(session_topic.clone());
                                    client_clone
                                        .subscribe(session_topic.clone())
                                        .await
                                        .expect("Failed to subscribe to session topic");
                                    // possible ack_body here but why tag 1101 again? Idk if needed ..
                                }else {
                                    println!("Secret already consumed because Session is already active");
                                }
                            },
                            1102 => {
                                // wc_sessionSettle Request from Session Topic
                                println!("Received Response: wc_sessionSettle request: tag 1102");
                                let session_sym_key_hex = persistent_session_sym_key.clone().expect("failed to get persistent session sym key");
                                let address = methods::decrypt_session_settle_request(message, session_sym_key_hex).expect("failed to decrypt sesion settle request");
                                // Do I need to respond to the session settle request with 1103?
                                // Does not seem like it now, but will leave this comment in for now

                                user_address = Some(address.clone());
                                let signer = Arc::new(WalletConnectXmtpSigner {
                                    address: address,
                                    to_loop: signer_tx.clone(),
                                });

                                if let Some(tx) = xmtp_tx.take() {
                                    let token = cancelation_token.clone();
                                    std::thread::spawn(move || {
                                        println!("Initializing XMTP client on blocking thread...");
                                        let xmtp = XMTP::new(Some(signer.clone()), None);
                                        let _ = tx.send(xmtp);

                                        // after xmtp auth cancel wc tasks
                                        token.cancel();
                                    });
                                }
                            }
                            1103 => {
                                println!("Received Response: wc_sessionSettle success: tag 1103");
                            },
                            1109 => {
                                // wc_sessionRequest Response from Session Topic
                                // Response to 1108 Personal Sign request
                                println!("Received Response: wc_sessionRequest success: tag: 1109");
                                let session_sym_key_hex = persistent_session_sym_key.clone().expect("failed to get persistent session sym key");
                                let signature_response_tx = signature_response_tx_opt.take().expect("");
                                let signature = methods::decrypt_personal_sign_response(message, session_sym_key_hex)
                                    .expect("failed to get sig");
                                signature_response_tx.send(signature)
                                    .expect("failed to send signature response over tx to signer");
                            },
                            _ => println!("Other tag: {:?}", message.tag),
                        }
                    }

                    // Receives text to sign from and signature response TX from Signer TX
                    // then sends Personal Sign Request which will trigger 1109 Response
                    Some((text_to_sign, response_channel)) = signer_rx.recv() => {
                        if let (Some(session_sym_key_hex), Some(session_topic)) = (persistent_session_sym_key.clone(), persistent_session_topic_opt.clone()) {
                            signature_response_tx_opt = Some(response_channel);
                            let user_addr = user_address.clone().expect("should have address stored");
                            let message_64 = methods::build_personal_sign_message(user_addr, text_to_sign, session_sym_key_hex);
                            client_clone
                                .publish(
                                    session_topic.clone(),
                                    Arc::from(message_64),
                                    None,
                                    1108,
                                    std::time::Duration::from_secs(600),
                                    true
                                )
                                .await
                                .expect("Failed to send personal sign");
                            println!("Sent Personal Sign Request");
                        }
                    }

                    _ = cancelation_token.cancelled() => {
                        println!("WC Cancelation Token Called!");
                        break;
                    }
                }
            }
            // xmtp_opt
        });

        // println!("end of task getting xmtp from handle");
        // let xmtp_option = handle.await.expect("could not get xmtp option");
        // match handle.await {
        //     Ok(initialized_xmtp) => {
        //         self.xmtp_client = Some(initialized_xmtp);
        //         println!("XMTP moved to handler state!");
        //     }
        //     Err(e) => eprintln!("Task failed: {:?}", e),
        // }
        // xmtp_option
    }

    // Parse URI to get Pairing Topic and Sym key
    fn parse_uri(&self, uri: &str) -> (String, String) {
        let parsed_url = Url::parse(uri)
            .map_err(|e| format!("Invalid URL: {}", e))
            .expect("failed to parse url");

        let pairing_topic_hex = uri
            .strip_prefix("wc:")
            .and_then(|s| s.split('@').next())
            .ok_or("Invalid WC URI format")
            .expect("Failed to extract topic from URI")
            .to_string();

        let query_params: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

        let pairing_sym_key_hex = query_params
            .get("symKey")
            .cloned()
            .ok_or("Missing symKey in URI")
            .expect("sym key didnt exist");

        (pairing_topic_hex, pairing_sym_key_hex)
    }
}
