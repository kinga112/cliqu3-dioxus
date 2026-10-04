use crate::modules::walletconnect::crypto;
use alloy::hex;
use hkdf::Hkdf;
use reown_relay_client::websocket::PublishedMessage;
use reown_relay_rpc::domain::Topic;
use sha2::{Digest, Sha256};
use x25519_dalek::{EphemeralSecret, PublicKey};

//
// Specs: https://specs.walletconnect.com/2.0/specs/clients/sign/rpc-methods
//

// Build Session Proposal message (Base64): Tag 1100
pub fn build_session_propose_message(pairing_sym_key_hex: String, public_key: PublicKey) -> String {
    let public_key_hex = hex::encode(public_key.as_bytes());
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let proposal_json = serde_json::json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": "wc_sessionPropose",
        "params": {
            "relays": [{"protocol": "irn"}],
            "proposer": {
                "publicKey": public_key_hex,
                "metadata": {
                    "name": "Cliqu3",
                    "description": "Web3 Messaging",
                    "url": "https://cliqu3.com",
                    "icons": ["https://avatars.githubusercontent.com/u/37784886"]
                }
            },
            "requiredNamespaces": {
                "eip155": {
                    "methods": ["eth_sendTransaction", "personal_sign"],
                    "chains": ["eip155:1"], // Ethereum Mainnet
                    "events": ["accountsChanged", "chainChanged"]
                }
            },
            "optionalNamespaces": {
                "eip155": {
                    "methods": ["eth_sendTransaction", "personal_sign"],
                    "chains": ["eip155:1"],
                    "events": ["accountsChanged", "chainChanged"]
                }
            }
        }
    })
    .to_string();
    let message_b64 = crypto::type_0_encryption(&proposal_json, &pairing_sym_key_hex)
        .expect("failed to encrypt with type 0");

    // println!("Encrypted proposal message: {:?}", message_b64);
    message_b64
}

// Build Session Request message for personal sign: Tag 1108
pub fn build_personal_sign_message(
    user_address: String,
    text_to_sign: String,
    session_sym_key_hex: String,
) -> String {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let json_body = serde_json::json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": "wc_sessionRequest",
        "params": {
            "request": {
                "method": "personal_sign",
                "params": [
                    format!("0x{}", hex::encode(text_to_sign)), // Must be hex with 0x
                    user_address.to_lowercase()
                ],
                "expiry": now + 600 // 10 minutes
            },
            "chainId": "eip155:1"
        }
    })
    .to_string();
    let message_b64 = crypto::type_0_encryption(&json_body, &session_sym_key_hex)
        .expect("failed to encrypt with type 0");
    message_b64
}

// Decrypt Session Proposal response message: Tag 1101
pub fn decrypt_session_propose_response(
    message: PublishedMessage,
    pairing_sym_key_hex: String,
    secret: EphemeralSecret,
) -> Result<(Topic, String), String> {
    let decrypted_message = crypto::decrypt_type_0(&message.message, pairing_sym_key_hex);

    let value: serde_json::Value =
        serde_json::from_str(&decrypted_message).expect("Failed to parse message JSON");

    if !value["result"]["responderPublicKey"].is_null() {
        let public_key_hex = value["result"]["responderPublicKey"]
            .as_str()
            .expect("Could not find responderPublicKey in the response");

        let public_key_bytes =
            hex::decode(public_key_hex).expect("Failed to decode responder public key from hex");

        let wallet_public =
            PublicKey::from(<[u8; 32]>::try_from(public_key_bytes).expect("Invalid length"));

        let shared_secret = secret.diffie_hellman(&wallet_public);
        let ikm = shared_secret.as_bytes(); // "Input Keying Material"

        let hk = Hkdf::<Sha256>::new(None, ikm);
        let mut okm = [0u8; 32]; // "Output Keying Material"
        hk.expand(&[], &mut okm).expect("HKDF expansion failed");

        let sym_key_hex = hex::encode(okm);
        let mut hasher = Sha256::new();
        hasher.update(okm);

        let session_topic = Topic::from(hex::encode(hasher.finalize()));

        Ok((session_topic, sym_key_hex))
    } else {
        Err("Message did not include responderPublicKey".to_owned())
    }
}

// Decrypt Session Settle response message: Tag 1102
pub fn decrypt_session_settle_request(
    message: PublishedMessage,
    session_sym_key_hex: String,
) -> Result<String, String> {
    let decrypted_message = crypto::decrypt_type_0(&message.message, session_sym_key_hex);

    let value: serde_json::Value =
        serde_json::from_str(&decrypted_message).expect("Failed to parse message JSON");

    if !value["params"]["namespaces"]["eip155"]["accounts"].is_null() {
        let accounts = value["params"]["namespaces"]["eip155"]["accounts"]
            .as_array()
            .expect("Could not find accounts in the request");

        let full_account = accounts[0]
            .as_str()
            .expect("issue getting account 0 as str");
        let user_address = full_account
            .split(':')
            .last()
            .expect("error splitting full account into address");

        Ok(user_address.to_string())
    } else {
        Err("Message did not include accounts".to_owned())
    }
}

// Decrypt Personal Sign response message: Tag 1109
pub fn decrypt_personal_sign_response(
    message: PublishedMessage,
    session_sym_key_hex: String,
) -> Result<Vec<u8>, String> {
    let decrypted_message = crypto::decrypt_type_0(&message.message, session_sym_key_hex);
    let value: serde_json::Value =
        serde_json::from_str(&decrypted_message).expect("Failed to parse message JSON");

    if !value["result"].is_null() {
        let sig_hex = value["result"]
            .as_str()
            .expect("could not get signature as str");
        let sig_bytes =
            hex::decode(sig_hex.trim_start_matches("0x")).expect("Wallet returned invalid hex");

        Ok(sig_bytes)
    } else {
        Err("Message did not include accounts".to_owned())
    }
}
