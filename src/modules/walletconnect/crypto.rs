use aes_gcm::{
    // Nonce,
    aead::{Aead, KeyInit},
};
use alloy::hex;
use anyhow::Result;
use base64::prelude::*;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::Generate};
use hkdf::Hkdf;
// use rand::RngCore;
use reown_relay_rpc::domain::Topic;
use sha2::{Digest, Sha256};
use x25519_dalek::{EphemeralSecret, PublicKey};

// Encryption specs for Wallet Connect
// https://specs.walletconnect.com/2.0/specs/clients/core/crypto/crypto-envelopes

// Encrypt Type 0 Envelope
pub fn type_0_encryption(json_payload: &str, sym_key_hex: &str) -> Result<String> {
    let key_bytes = hex::decode(sym_key_hex).expect("Invalid sym_key hex");
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(&key);

    // let mut nonce_bytes = [0u8; 12];
    // rand::thread_rng().fill_bytes(&mut nonce_bytes);
    // let nonce = Nonce::from_slice(&nonce_bytes);
    let nonce = Nonce::generate();

    // Encrypt JSON payload
    let ciphertext = cipher
        .encrypt(&nonce, json_payload.as_bytes())
        .expect("Encryption failed");
    // let ciphertext = cipher
    //     .encrypt(nonce_bytes, json_payload.as_bytes())
    //     .expect("Encryption failed");

    // Construct the Type 0 Envelope: [0] + [12 bytes nonce] + [ciphertext]
    let mut envelope = Vec::with_capacity(1 + 12 + ciphertext.len());
    envelope.push(0u8);
    // envelope.extend_from_slice(&nonce_bytes);
    envelope.extend_from_slice(&nonce);
    envelope.extend_from_slice(&ciphertext);

    // Ok(base64::encode(envelope))
    Ok(BASE64_STANDARD.encode(envelope))
}

// Decrypt Type 0 Envelopes
pub fn decrypt_type_0(message_b64: &str, sym_key: String) -> String {
    // let buffer = base64::decode(message_b64).expect("Failed to decode base64 message");
    let buffer = BASE64_STANDARD
        .decode(message_b64)
        .expect("Failed to decode base64 message");

    if buffer.is_empty() || buffer[0] != 0 {
        // panic!("Not a Type 0 envelope. Use Type 1 function instead.");
        println!("Not a Type 0 envelope. Use Type 1 function instead.");
        return String::new();
    }

    let nonce_bytes = &buffer[1..13];
    let ciphertext = &buffer[13..];
    let sym_key_bytes = hex::decode(&sym_key).expect("Invalid hex key");
    let key = Key::from_slice(&sym_key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext_bytes = cipher
        .decrypt(nonce, ciphertext)
        .expect("Failed to decrypt");

    String::from_utf8(plaintext_bytes).expect("Failed to parse decrypted bytes as UTF-8")
}

// Decrypt Type 1 Envelopes
pub fn decrypt_type_1(message_b64: &str, sym_key: String) -> (String, String) {
    let buffer = BASE64_STANDARD
        .decode(message_b64)
        .expect("Failed to decode base64 message");

    if buffer.is_empty() || buffer[0] != 1 {
        panic!("Not a Type 1 envelope. Use Type 0 function instead.");
    }

    let wallet_pub_key_bytes = &buffer[1..33];
    let iv_bytes = &buffer[33..45];
    let sealbox = &buffer[45..];
    let sym_key_bytes = hex::decode(&sym_key).expect("Invalid hex key");
    let key = Key::from_slice(&sym_key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(iv_bytes);
    let plaintext_bytes = cipher.decrypt(nonce, sealbox).expect("Failed to decrypt");
    let json_payload =
        String::from_utf8(plaintext_bytes).expect("Failed to parse decrypted bytes as UTF-8");
    let wallet_pub_key_hex = hex::encode(wallet_pub_key_bytes);
    (wallet_pub_key_hex, json_payload)
}

// derive session topic and Output Keying Material from relay response
// after session proposal using HKDF Key Derivation Function (Tag 1100)
pub fn derive_session_details(public_key_hex: &str, secret: EphemeralSecret) -> (Topic, String) {
    let public_key_bytes =
        hex::decode(public_key_hex).expect("Failed to decode responder public key from hex");
    let wallet_public =
        PublicKey::from(<[u8; 32]>::try_from(public_key_bytes).expect("Invalid length"));
    let shared_secret = secret.diffie_hellman(&wallet_public);
    let ikm = shared_secret.as_bytes(); // "Input Keying Material"

    // RUN HKDF (The Spec Requirement)
    // Salt is empty, Info is empty for this specific step in WCv2
    let hk = Hkdf::<Sha256>::new(None, ikm);
    let mut okm = [0u8; 32]; // "Output Keying Material"
    hk.expand(&[], &mut okm).expect("HKDF expansion failed");

    let sym_key_hex = hex::encode(okm);
    // let sym_key_bytes = okm;

    let mut hasher = Sha256::new();
    hasher.update(okm); // Hash the raw bytes
    // session_topic = Some(hex::encode(hasher.finalize()));
    // let topic = Topic::from(session_topic.clone().expect("Session topic should be set"));
    let session_topic = Topic::from(hex::encode(hasher.finalize()));

    (session_topic, sym_key_hex)
}
