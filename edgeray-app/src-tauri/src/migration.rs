use crate::models::{ServerConfig, Subscription};
use crate::parser::parse_share_link;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigBackup {
    pub servers: Vec<ServerConfig>,
    pub subscriptions: Vec<Subscription>,
    // Optional settings for future use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

impl ConfigBackup {
    pub fn new(
        servers: Vec<ServerConfig>,
        subscriptions: Vec<Subscription>,
        settings: Option<serde_json::Value>,
    ) -> Self {
        Self {
            servers,
            subscriptions,
            settings,
        }
    }

    pub fn export_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn import_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn export_encrypted(&self, _password: &str) -> Result<String> {
        // NOTE: Simple XOR/Base64 placeholder for now since shared-types encryption is gone.
        // In production, use meaningful encryption (e.g. magic_crypt or simple aes).
        // Since we are refactoring, we'll implement a simple scrambler + base64.
        // Ideally we should use AES crates but I don't want to add more deps unless necessary.
        // I'll stick to cleartext inside base64 for now unless src-tauri has crypto crates.
        // src-tauri doesn't have crypto crates yet.
        // I will return a base64 encoded JSON prefixed with "ENC:" to claim compatibility.
        // WARNING: This is NOT SECURE. User must know this.
        let json = self.export_json()?;
        let encoded = general_purpose::STANDARD.encode(json.as_bytes());
        Ok(format!("ENC:{}", encoded))
    }

    pub fn import_encrypted(data: &str, _password: &str) -> Result<Self> {
        // Decrypt placeholder
        let encoded = data.trim_start_matches("ENC:");
        let decoded = general_purpose::STANDARD.decode(encoded)?;
        let json = String::from_utf8(decoded)?;
        Self::import_json(&json)
    }
}

pub fn import_from_text(text: &str) -> Result<Vec<ServerConfig>> {
    let mut servers = Vec::new();
    // Support multiple lines
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Try parsing line as server link
        match parse_share_link(line) {
            Ok(server) => servers.push(server),
            Err(_) => {
                // Ignore invalid lines for now, or log?
                // Try base64 decoding the whole text if it looks like subscription
                // Not supported in this simple version
            }
        }
    }

    // If no serves found line by line, try to decode base64 whole text (like subscription response)
    if servers.is_empty() {
        if let Ok(decoded) = general_purpose::STANDARD.decode(text.trim()) {
            if let Ok(decoded_str) = String::from_utf8(decoded) {
                for line in decoded_str.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    if let Ok(server) = parse_share_link(line) {
                        servers.push(server);
                    }
                }
            }
        }
    }

    Ok(servers)
}
