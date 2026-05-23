// src/services/beacon_manager.rs
//! Phase 6 — Invisible Signaling (DNS Beacon) Manager
//!
//! Handles encrypting and optionally publishing the Bridge IP updates
//! to DNS TXT records.

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngCore;

pub struct BeaconManager {
    key: [u8; 32],
}

impl BeaconManager {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// Encrypt a new Bridge IP address into a base64 string suitable for a DNS TXT record.
    pub fn create_beacon_payload(&self, ip_address: &str) -> String {
        let cipher = Aes256Gcm::new(self.key.as_ref().into());

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, ip_address.as_bytes())
            .expect("encryption failure");

        let mut payload = Vec::with_capacity(12 + ciphertext.len());
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);

        STANDARD.encode(payload)
    }

    // Note: Actual publishing to Cloudflare/Route53 would be integrated here
    // using reqwest and the provider's API.
}
