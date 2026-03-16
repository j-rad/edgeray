//! Models stub for WASM target
//!
//! This module provides the same types as `rustray::types` but in a platform-agnostic
//! way for Wasm builds where rustray is not available.

pub mod parser;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Protocol {
    #[default]
    Vless,
    Vmess,
    Trojan,
    Shadowsocks,
    Hysteria2,
    Flow,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarrierType {
    Reality,
    Mqtt,
    Cdn,
    Direct,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ServerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub address: String,
    pub port: u16,
    pub remarks: String,
    pub protocol: Protocol,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub flow: Option<String>,
    #[serde(default)]
    pub security: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub sni: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub pbk: Option<String>,
    #[serde(default)]
    pub sid: Option<String>,
    #[serde(default)]
    pub service_name: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub allow_insecure: Option<bool>,
}

impl ServerConfig {
    pub fn to_uri(&self) -> String {
        match self.protocol {
            Protocol::Vless => {
                let uuid = self.uuid.as_deref().unwrap_or("");
                let host = &self.address;
                let port = self.port;
                let remarks = url::form_urlencoded::byte_serialize(self.remarks.as_bytes())
                    .collect::<String>();

                let mut params = Vec::new();
                params.push(("security", self.security.as_deref().unwrap_or("none")));
                params.push(("type", self.network.as_deref().unwrap_or("tcp")));

                if let Some(sni) = &self.sni {
                    params.push(("sni", sni));
                }
                if let Some(path) = &self.path {
                    params.push(("path", path));
                }
                if let Some(flow) = &self.flow {
                    params.push(("flow", flow));
                }
                if let Some(pbk) = &self.pbk {
                    params.push(("pbk", pbk));
                }
                if let Some(sid) = &self.sid {
                    params.push(("sid", sid));
                }
                if let Some(fp) = &self.fingerprint {
                    params.push(("fp", fp));
                }

                let query = url::form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(params)
                    .finish();

                format!("vless://{}@{}:{}?{}#{}", uuid, host, port, query, remarks)
            }
            Protocol::Trojan => {
                let password = self.password.as_deref().unwrap_or("");
                let host = &self.address;
                let port = self.port;
                let remarks = url::form_urlencoded::byte_serialize(self.remarks.as_bytes())
                    .collect::<String>();

                let mut params = Vec::new();
                params.push(("security", self.security.as_deref().unwrap_or("tls")));
                params.push(("type", self.network.as_deref().unwrap_or("tcp")));

                if let Some(sni) = &self.sni {
                    params.push(("sni", sni));
                }
                if let Some(path) = &self.path {
                    params.push(("path", path));
                }
                if let Some(host_hdr) = &self.host {
                    params.push(("host", host_hdr));
                }

                let query = url::form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(params)
                    .finish();

                format!(
                    "trojan://{}@{}:{}?{}#{}",
                    password, host, port, query, remarks
                )
            }
            // Fallback/TODO for other protocols
            _ => String::from("edgeray://unsupported-protocol"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoutingMode {
    Global,
    #[default]
    BypassLan,
    BypassMainland,
    Direct,
    Rule,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PerAppMode {
    #[default]
    Global,
    Whitelist,
    Blacklist,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TunnelConfig {
    #[serde(skip)]
    pub file_descriptor: Option<i32>,
    pub active_server: ServerConfig,
    #[serde(default = "default_tun_name")]
    pub tun_name: String,
    #[serde(default = "default_tun_ip")]
    pub tun_ip: String,
    #[serde(default = "default_tun_cidr")]
    pub tun_cidr: u8,
    #[serde(default = "default_tun_mtu")]
    pub tun_mtu: u16,
    #[serde(default)]
    pub routing_mode: RoutingMode,
    #[serde(default)]
    pub geodata_dir: Option<String>,
    #[serde(default)]
    pub per_app_mode: PerAppMode,
    #[serde(default)]
    pub per_app_list: Vec<String>,
    #[serde(default)]
    pub sniffing: bool,
    #[serde(default)]
    pub dns_hijacking: bool,
    #[serde(default)]
    pub lock_vpn: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub urls: Vec<String>,
    pub update_interval: u64,
    pub last_update: Option<u64>,
    #[serde(default)]
    pub filter_tags: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub node_count: usize,
}

impl Default for Subscription {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Subscription".to_string(),
            urls: vec![],
            update_interval: 3600,
            last_update: None,
            filter_tags: vec![],
            enabled: true,
            node_count: 0,
        }
    }
}

fn default_tun_name() -> String {
    "ray0".to_string()
}

fn default_tun_ip() -> String {
    "10.0.0.1".to_string()
}

fn default_tun_cidr() -> u8 {
    24
}

fn default_tun_mtu() -> u16 {
    1500
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FakeDnsConfig {
    #[serde(default = "default_fakedns_pool")]
    pub ip_pool: String,
    #[serde(default = "default_fakedns_size")]
    pub pool_size: u32,
    #[serde(default = "default_fakedns_max")]
    pub max_entries: usize,
    #[serde(default)]
    pub persist_path: Option<String>,
    #[serde(default = "default_fakedns_save_interval")]
    pub save_interval_secs: u64,
}

impl Default for FakeDnsConfig {
    fn default() -> Self {
        Self {
            ip_pool: default_fakedns_pool(),
            pool_size: default_fakedns_size(),
            max_entries: default_fakedns_max(),
            persist_path: None,
            save_interval_secs: default_fakedns_save_interval(),
        }
    }
}

fn default_fakedns_pool() -> String {
    "198.18.0.0/16".to_string()
}

fn default_fakedns_size() -> u32 {
    65536
}

fn default_fakedns_max() -> usize {
    65535
}

fn default_fakedns_save_interval() -> u64 {
    30
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppSettings {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_ui_mode")]
    pub ui_mode: String,
    #[serde(default)]
    pub start_on_boot: bool,
    #[serde(default)]
    pub allow_insecure: bool,
    #[serde(default = "default_routing_mode_str")]
    pub routing_mode: String,
    #[serde(default = "default_true")]
    pub sniffing: bool,
    #[serde(default = "default_true")]
    pub dns_hijacking: bool,
    #[serde(default)]
    pub lock_vpn: bool,
    #[serde(default = "default_doh")]
    pub doh_url: String,
    #[serde(default)]
    pub auto_update: bool,
    #[serde(default = "default_core")]
    pub active_core: String,
    #[serde(default = "default_rustray_version")]
    pub rustray_version: String,
    #[serde(default = "default_singbox_version")]
    pub singbox_version: String,

    // Phase 9: Advanced Tuning
    #[serde(default = "default_fec_shards")]
    pub fec_data_shards: u8,
    #[serde(default = "default_fec_parities")]
    pub fec_parities: u8,
    #[serde(default = "default_mqtt_heartbeat")]
    pub mqtt_heartbeat_interval: u64,
    #[serde(default = "default_fingerprint_interval")]
    pub fingerprint_rotation_interval: u64,

    #[serde(default)]
    pub fakedns: FakeDnsConfig,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            start_on_boot: false,
            allow_insecure: false,
            routing_mode: default_routing_mode_str(),
            sniffing: true,
            dns_hijacking: true,
            lock_vpn: false,
            doh_url: default_doh(),
            auto_update: false,
            active_core: default_core(),
            rustray_version: default_rustray_version(),
            singbox_version: default_singbox_version(),
            fec_data_shards: default_fec_shards(),
            fec_parities: default_fec_parities(),
            mqtt_heartbeat_interval: default_mqtt_heartbeat(),
            fingerprint_rotation_interval: default_fingerprint_interval(),
            fakedns: FakeDnsConfig::default(),
        }
    }
}

fn default_rustray_version() -> String {
    "v0.1.0".to_string()
}

fn default_singbox_version() -> String {
    "v1.12.14".to_string()
}

fn default_core() -> String {
    "xray".to_string()
}

fn default_theme() -> String {
    "system".to_string()
}

fn default_ui_mode() -> String {
    "simple".to_string()
}

fn default_routing_mode_str() -> String {
    "rule".to_string()
}

fn default_true() -> bool {
    true
}

fn default_doh() -> String {
    "https://1.1.1.1/dns-query".to_string()
}

fn default_fec_shards() -> u8 {
    10
}

fn default_fec_parities() -> u8 {
    3
}

fn default_mqtt_heartbeat() -> u64 {
    30
}

fn default_fingerprint_interval() -> u64 {
    3600
}

// ... rest of the content

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RoutingRule {
    pub id: String,
    #[serde(default)]
    pub enabled: bool,
    pub name: String,
    pub rule_type: RuleType,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub ips: Vec<String>,
    #[serde(default)]
    pub ports: String,
    pub outbound: String,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuleType {
    #[default]
    Field,
    Balancer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatsSnapshot {
    pub bytes_uploaded: u64,
    pub bytes_downloaded: u64,
    pub active_connections: u64,
    pub total_connections: u64,
    pub last_update: u64,
    pub connection_state: u64,
    pub errors: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DpiState {
    #[default]
    Clear,
    Throttled,
    ResetDetected,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionMetrics {
    pub rtt_ms: u64,
    pub cwnd_bytes: u64,
    pub dpi_state: DpiState,
    pub timestamp: u64,
}

// ============================================================================
// Phase 12: Privacy Guard, DNS Management, and TLS Forensics Models
// ============================================================================

/// Per-app firewall rule
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerAppRule {
    pub id: String,
    pub package_id: String,
    pub action: RuleAction,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Include, // App uses VPN
    Exclude, // App bypasses VPN
    Block,   // App network access blocked
}

/// App metadata for Privacy Guard
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppMetadata {
    pub package_id: String,
    pub name: String,
    pub icon_path: Option<String>,
    pub data_usage_mb: f64,
    pub is_system: bool,
    pub uid: Option<u32>,
}

/// Blocked DNS domain log entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockedDomain {
    pub id: String,
    pub domain: String,
    pub category: ThreatCategory,
    pub blocked_at: u64,
    pub request_count: u32,
    pub source_ip: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatCategory {
    Malware,
    Phishing,
    Tracking,
    Advertising,
    Adult,
    Gambling,
    SocialMedia,
    Unknown,
}

/// TLS connection forensics data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TlsConnectionInfo {
    pub id: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub sni: String,
    pub tls_version: String,
    pub cipher_suite: String,
    pub utls_fingerprint: String,
    pub state: ConnectionState,
    pub handshake_duration_ms: u32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub established_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Established,
    Closing,
    Closed,
}

/// DNS configuration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DnsConfig {
    pub fake_dns_enabled: bool,
    pub fake_ip_range: String,
    pub sniffer_enabled: bool,
    pub sniffer_protocols: Vec<String>,
    pub force_doh: bool,
    pub doh_server: String,
    pub dnscrypt_enabled: bool,
    pub dnscrypt_server: String,
    pub block_ads: bool,
    pub block_trackers: bool,
    pub block_malware: bool,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            fake_dns_enabled: true,
            fake_ip_range: "198.18.0.0/16".to_string(),
            sniffer_enabled: true,
            sniffer_protocols: vec!["http".to_string(), "tls".to_string()],
            force_doh: true,
            doh_server: "https://1.1.1.1/dns-query".to_string(),
            dnscrypt_enabled: false,
            dnscrypt_server: "sdns://AQcAAAAAAAAADDkuOS45Ljk6ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXQuY2VydC5xdWFkOS5uZXQ".to_string(),
            block_ads: true,
            block_trackers: true,
            block_malware: true,
        }
    }
}
