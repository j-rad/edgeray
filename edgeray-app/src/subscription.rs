//! Subscription Parser and Updater
//!
//! Handles fetching, parsing, and converting subscription data from various formats
//! (Base64, Plain Text, Clash YAML) into `ServerConfig` objects.

#![allow(dead_code)]
use crate::models::{Protocol, ServerConfig};
use crate::parser::parse_share_link;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::collections::HashMap;

/// Subscription format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionFormat {
    /// Base64-encoded list of share links
    Base64,
    /// Plain text list of share links
    PlainText,
    /// Clash YAML format
    ClashYaml,
    /// Auto-detect format
    Auto,
}

/// User-Agent presets for different clients to bypass anti-bot checks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserAgentPreset {
    EdgeRay,
    V2rayNG,
    ClashForAndroid,
    ClashX,
    Custom,
}

impl UserAgentPreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EdgeRay => "EdgeRay/1.0",
            Self::V2rayNG => "v2rayNG/1.8.16",
            Self::ClashForAndroid => "ClashForAndroid/2.5.12",
            Self::ClashX => "ClashX/1.118.0",
            Self::Custom => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        }
    }
}

/// Subscription update options
#[derive(Debug, Clone)]
pub struct SubscriptionOptions {
    pub user_agent: UserAgentPreset,
    pub custom_user_agent: Option<String>,
    pub format: SubscriptionFormat,
    pub timeout_secs: u64,
    pub include_remarks_filter: Option<String>,
    pub exclude_remarks_filter: Option<String>,
}

impl Default for SubscriptionOptions {
    fn default() -> Self {
        Self {
            user_agent: UserAgentPreset::EdgeRay,
            custom_user_agent: None,
            format: SubscriptionFormat::Auto,
            timeout_secs: 30,
            include_remarks_filter: None,
            exclude_remarks_filter: None,
        }
    }
}

/// Clash YAML proxy structure (simplified)
#[derive(Debug, Deserialize)]
struct ClashConfig {
    proxies: Option<Vec<ClashProxy>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClashProxy {
    #[serde(rename = "vmess")]
    Vmess {
        name: String,
        server: String,
        port: u16,
        uuid: String,
        #[serde(rename = "alterId")]
        alter_id: Option<u32>,
        cipher: Option<String>,
        tls: Option<bool>,
        network: Option<String>,
        #[serde(rename = "ws-opts")]
        ws_opts: Option<ClashWsOpts>,
        #[serde(rename = "grpc-opts")]
        grpc_opts: Option<ClashGrpcOpts>,
        #[serde(rename = "skip-cert-verify")]
        skip_cert_verify: Option<bool>,
        servername: Option<String>,
    },
    #[serde(rename = "vless")]
    Vless {
        name: String,
        server: String,
        port: u16,
        uuid: String,
        flow: Option<String>,
        network: Option<String>,
        tls: Option<bool>,
        #[serde(rename = "reality-opts")]
        reality_opts: Option<ClashRealityOpts>,
        #[serde(rename = "ws-opts")]
        ws_opts: Option<ClashWsOpts>,
        #[serde(rename = "grpc-opts")]
        grpc_opts: Option<ClashGrpcOpts>,
        servername: Option<String>,
    },
    #[serde(rename = "trojan")]
    Trojan {
        name: String,
        server: String,
        port: u16,
        password: String,
        sni: Option<String>,
        #[serde(rename = "skip-cert-verify")]
        skip_cert_verify: Option<bool>,
        network: Option<String>,
        #[serde(rename = "ws-opts")]
        ws_opts: Option<ClashWsOpts>,
        #[serde(rename = "grpc-opts")]
        grpc_opts: Option<ClashGrpcOpts>,
    },
    #[serde(rename = "ss")]
    Shadowsocks {
        name: String,
        server: String,
        port: u16,
        cipher: String,
        password: String,
        plugin: Option<String>,
        #[serde(rename = "plugin-opts")]
        plugin_opts: Option<HashMap<String, serde_yaml::Value>>,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct ClashWsOpts {
    path: Option<String>,
    headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct ClashGrpcOpts {
    #[serde(rename = "grpc-service-name")]
    grpc_service_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClashRealityOpts {
    #[serde(rename = "public-key")]
    public_key: Option<String>,
    #[serde(rename = "short-id")]
    short_id: Option<String>,
}

/// Update subscription from URL using default options
pub async fn update_subscription(url: &str) -> Result<Vec<ServerConfig>> {
    update_subscription_with_options(url, SubscriptionOptions::default()).await
}

/// Update subscription with custom options
pub async fn update_subscription_with_options(
    url: &str,
    options: SubscriptionOptions,
) -> Result<Vec<ServerConfig>> {
    tracing::info!("Fetching subscription from: {}", url);

    // Build HTTP client with custom headers
    let mut headers = HeaderMap::new();
    let user_agent = if let Some(custom) = &options.custom_user_agent {
        custom.as_str()
    } else {
        options.user_agent.as_str()
    };
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);

    let client_builder = reqwest::Client::builder().default_headers(headers);

    #[cfg(not(target_arch = "wasm32"))]
    let client_builder =
        client_builder.timeout(std::time::Duration::from_secs(options.timeout_secs));

    let client = client_builder.build()?;

    let response: reqwest::Response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("HTTP error: {}", response.status()));
    }

    let content: String = response.text().await?;
    tracing::debug!("Received {} bytes", content.len());

    // Parse based on format
    let mut servers = match options.format {
        SubscriptionFormat::Base64 => parse_base64_subscription(&content)?,
        SubscriptionFormat::PlainText => parse_plain_text_subscription(&content)?,
        SubscriptionFormat::ClashYaml => parse_clash_yaml_subscription(&content)?,
        SubscriptionFormat::Auto => auto_detect_and_parse(&content)?,
    };

    // Apply filters
    if let Some(include_filter) = &options.include_remarks_filter {
        servers.retain(|s| s.remarks.contains(include_filter));
    }

    if let Some(exclude_filter) = &options.exclude_remarks_filter {
        servers.retain(|s| !s.remarks.contains(exclude_filter));
    }

    if servers.is_empty() {
        return Err(anyhow!("No valid servers found in subscription"));
    }

    tracing::info!("Successfully parsed {} servers", servers.len());
    Ok(servers)
}

/// Auto-detect subscription format and parse
fn auto_detect_and_parse(content: &str) -> Result<Vec<ServerConfig>> {
    let trimmed = content.trim();

    // Try YAML first (Clash format)
    if trimmed.starts_with("proxies:")
        || trimmed.contains("type: vmess")
        || trimmed.contains("type: vless")
    {
        tracing::debug!("Detected Clash YAML format");
        return parse_clash_yaml_subscription(content);
    }

    // Try base64 decode
    if let Ok(servers) = parse_base64_subscription(content) {
        if !servers.is_empty() {
            tracing::debug!("Detected Base64 format");
            return Ok(servers);
        }
    }

    // Fallback to plain text
    tracing::debug!("Falling back to plain text format");
    parse_plain_text_subscription(content)
}

/// Parse base64-encoded subscription
fn parse_base64_subscription(content: &str) -> Result<Vec<ServerConfig>> {
    let decoded_bytes = decode_base64_flexible(content.trim())?;
    let decoded_str = String::from_utf8(decoded_bytes)
        .map_err(|_| anyhow!("Subscription content is not valid UTF-8"))?;

    parse_plain_text_subscription(&decoded_str)
}

/// Parse plain text subscription (list of share links)
fn parse_plain_text_subscription(content: &str) -> Result<Vec<ServerConfig>> {
    let mut servers = Vec::new();
    let mut errors = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        match parse_share_link(line) {
            Ok(config) => servers.push(config),
            Err(e) => {
                errors.push((line_num + 1, line.to_string(), e.to_string()));
                tracing::warn!("Failed to parse line {}: {} - {}", line_num + 1, line, e);
            }
        }
    }

    // Log summary of errors
    if !errors.is_empty() {
        tracing::warn!(
            "Failed to parse {} out of {} lines",
            errors.len(),
            content.lines().count()
        );
    }

    Ok(servers)
}

/// Parse Clash YAML subscription
fn parse_clash_yaml_subscription(content: &str) -> Result<Vec<ServerConfig>> {
    let clash_config: ClashConfig =
        serde_yaml::from_str(content).map_err(|e| anyhow!("Failed to parse Clash YAML: {}", e))?;

    let proxies = clash_config
        .proxies
        .ok_or_else(|| anyhow!("No proxies found in Clash config"))?;

    let mut servers = Vec::new();
    for proxy in proxies {
        if let Some(server) = convert_clash_proxy_to_server(proxy) {
            servers.push(server);
        }
    }

    Ok(servers)
}

/// Convert Clash proxy to ServerConfig
fn convert_clash_proxy_to_server(proxy: ClashProxy) -> Option<ServerConfig> {
    match proxy {
        ClashProxy::Vmess {
            name,
            server,
            port,
            uuid,
            cipher,
            tls: _,
            network,
            ws_opts,
            grpc_opts,
            skip_cert_verify,
            servername,
            ..
        } => {
            let (path, host) = if let Some(ws) = ws_opts {
                let host = ws
                    .headers
                    .as_ref()
                    .and_then(|h| h.get("Host"))
                    .map(|s| s.to_string());
                (ws.path, host)
            } else {
                (None, None)
            };

            let service_name = grpc_opts.and_then(|g| g.grpc_service_name);

            Some(ServerConfig {
                id: None,
                remarks: name,
                protocol: Protocol::Vmess,
                address: server,
                port,
                uuid: Some(uuid),
                password: None,
                network,
                security: cipher,
                flow: None,
                fingerprint: None,
                sni: servername,
                host,
                path,
                pbk: None,
                sid: None,
                service_name,
                method: None,
                group: None,
                allow_insecure: skip_cert_verify,
            })
        }
        ClashProxy::Vless {
            name,
            server,
            port,
            uuid,
            flow,
            network,
            reality_opts,
            ws_opts,
            grpc_opts,
            servername,
            ..
        } => {
            let (path, host) = if let Some(ws) = ws_opts {
                let host = ws
                    .headers
                    .as_ref()
                    .and_then(|h| h.get("Host"))
                    .map(|s| s.to_string());
                (ws.path, host)
            } else {
                (None, None)
            };

            let service_name = grpc_opts.and_then(|g| g.grpc_service_name);

            let (pbk, sid) = if let Some(reality) = reality_opts {
                (reality.public_key, reality.short_id)
            } else {
                (None, None)
            };

            Some(ServerConfig {
                id: None,
                remarks: name,
                protocol: Protocol::Vless,
                address: server,
                port,
                uuid: Some(uuid),
                password: None,
                network,
                security: Some("tls".to_string()),
                flow,
                fingerprint: None,
                sni: servername,
                host,
                path,
                pbk,
                sid,
                service_name,
                method: None,
                group: None,
                allow_insecure: None,
            })
        }
        ClashProxy::Trojan {
            name,
            server,
            port,
            password,
            sni,
            skip_cert_verify,
            network,
            ws_opts,
            grpc_opts,
        } => {
            let (path, host) = if let Some(ws) = ws_opts {
                let host = ws
                    .headers
                    .as_ref()
                    .and_then(|h| h.get("Host"))
                    .map(|s| s.to_string());
                (ws.path, host)
            } else {
                (None, None)
            };

            let service_name = grpc_opts.and_then(|g| g.grpc_service_name);

            Some(ServerConfig {
                id: None,
                remarks: name,
                protocol: Protocol::Trojan,
                address: server,
                port,
                uuid: None,
                password: Some(password),
                network,
                security: Some("tls".to_string()),
                flow: None,
                fingerprint: None,
                sni,
                host,
                path,
                pbk: None,
                sid: None,
                service_name,
                method: None,
                group: None,
                allow_insecure: skip_cert_verify,
            })
        }
        ClashProxy::Shadowsocks {
            name,
            server,
            port,
            cipher,
            password,
            ..
        } => Some(ServerConfig {
            id: None,
            remarks: name,
            protocol: Protocol::Shadowsocks,
            address: server,
            port,
            uuid: None,
            password: Some(password),
            network: None,
            security: None,
            flow: None,
            fingerprint: None,
            sni: None,
            host: None,
            path: None,
            pbk: None,
            sid: None,
            service_name: None,
            method: Some(cipher),
            group: None,
            allow_insecure: None,
        }),
        ClashProxy::Other => None,
    }
}

fn decode_base64_flexible(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    // 1. Try standard
    if let Ok(d) = general_purpose::STANDARD.decode(input) {
        return Ok(d);
    }
    // 2. Try URL_SAFE
    if let Ok(d) = general_purpose::URL_SAFE.decode(input) {
        return Ok(d);
    }
    // 3. Try unpadded
    if let Ok(d) = general_purpose::STANDARD_NO_PAD.decode(input) {
        return Ok(d);
    }
    general_purpose::URL_SAFE_NO_PAD.decode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_base64_subscription() {
        // Updated with valid UUIDs AND 'v': '2' which is required
        let links = "vmess://eyJ2IjoiMiIsImFkZCI6ImV4YW1wbGUuY29tIiwicG9ydCI6NDQzLCJpZCI6IjU1MGU4NDAwLWUyOWItNDFkNC1hNzE2LTQ0NjY1NTQ0MDAwMCIsInBzIjoiVGVzdCJ9\nvless://550e8400-e29b-41d4-a716-446655440000@example.com:443?security=tls#Test";
        let encoded = general_purpose::STANDARD.encode(links);

        let result = parse_base64_subscription(&encoded);
        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 2);
    }

    #[test]
    fn test_parse_plain_text_subscription() {
        let content = r#"
# Comment line
vless://uuid@example.com:443?security=tls#Test1

vmess://eyJhZGQiOiJleGFtcGxlLmNvbSIsInBvcnQiOjQ0MywiaWQiOiJ1dWlkIiwicHMiOiJUZXN0MiJ9
"#;

        let result = parse_plain_text_subscription(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clash_yaml_parsing() {
        let yaml = r#"
proxies:
  - name: "Test VLESS"
    type: vless
    server: example.com
    port: 443
    uuid: test-uuid
    flow: xtls-rprx-vision
    network: tcp
    tls: true
    servername: example.com
"#;

        let result = parse_clash_yaml_subscription(yaml);
        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].protocol, Protocol::Vless);
    }

    #[test]
    fn test_user_agent_presets() {
        assert_eq!(UserAgentPreset::EdgeRay.as_str(), "EdgeRay/1.0");
        assert_eq!(UserAgentPreset::V2rayNG.as_str(), "v2rayNG/1.8.16");
    }
}
