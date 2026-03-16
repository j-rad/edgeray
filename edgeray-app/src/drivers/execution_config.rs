//! Execution Configuration
//!
//! Client-side configuration processing and JSON generation.
//! This implements the "heavy client" pattern where GeoIP/GeoSite parsing
//! happens on the client, sending finalized JSON to the dumb router core.

use crate::models::{RoutingMode, ServerConfig};
use serde::{Deserialize, Serialize};

use super::DriverError;

/// Finalized execution configuration ready to be sent to the core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// The outbound server configuration
    pub server: ServerConfig,
    /// Routing mode
    pub routing_mode: RoutingMode,
    /// Pre-parsed routing rules (domain/IP lists)
    pub routing_rules: Vec<RoutingRule>,
    /// DNS configuration
    pub dns: DnsConfig,
    /// Local proxy settings
    pub local_proxy: LocalProxyConfig,
    /// TUN device settings (for mobile/system VPN)
    pub tun: Option<TunConfig>,
    /// Mux settings
    pub mux: Option<MuxConfig>,
    /// TLS fragmenting settings for anti-censorship
    pub fragment: Option<FragmentConfig>,
    /// Flow-J specific settings
    pub flow_j: Option<FlowJConfig>,
}

impl ExecutionConfig {
    /// Create a new execution config with defaults
    pub fn new(server: ServerConfig) -> Self {
        Self {
            server,
            routing_mode: RoutingMode::default(),
            routing_rules: Vec::new(),
            dns: DnsConfig::default(),
            local_proxy: LocalProxyConfig::default(),
            tun: None,
            mux: None,
            fragment: None,
            flow_j: None,
        }
    }

    /// Create a minimal config for quick connect
    pub fn quick_connect(server: ServerConfig) -> Self {
        Self::new(server)
    }

    /// Build with routing mode
    pub fn with_routing_mode(mut self, mode: RoutingMode) -> Self {
        self.routing_mode = mode;
        self
    }

    /// Build with TUN config
    pub fn with_tun(mut self, tun: TunConfig) -> Self {
        self.tun = Some(tun);
        self
    }

    /// Build with fragment settings
    pub fn with_fragment(mut self, fragment: FragmentConfig) -> Self {
        self.fragment = Some(fragment);
        self
    }

    /// Convert to JSON string for sending to core
    pub fn to_json(&self) -> Result<String, DriverError> {
        serde_json::to_string(self).map_err(|e| DriverError::Serialization(e.to_string()))
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, DriverError> {
        serde_json::from_str(json).map_err(|e| DriverError::Serialization(e.to_string()))
    }
}

/// Routing rule for domain/IP matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule type: "field", "balancer"
    pub rule_type: String,
    /// Domain patterns (optional)
    #[serde(default)]
    pub domains: Vec<String>,
    /// IP patterns (optional)
    #[serde(default)]
    pub ips: Vec<String>,
    /// Port ranges (optional)
    #[serde(default)]
    pub ports: Vec<String>,
    /// Target outbound tag
    pub outbound: String,
}

impl RoutingRule {
    /// Create a bypass rule for specified domains
    pub fn bypass_domains(domains: Vec<String>) -> Self {
        Self {
            rule_type: "field".to_string(),
            domains,
            ips: Vec::new(),
            ports: Vec::new(),
            outbound: "direct".to_string(),
        }
    }

    /// Create a bypass rule for specified IPs
    pub fn bypass_ips(ips: Vec<String>) -> Self {
        Self {
            rule_type: "field".to_string(),
            domains: Vec::new(),
            ips,
            ports: Vec::new(),
            outbound: "direct".to_string(),
        }
    }

    /// Create a block rule
    pub fn block_domains(domains: Vec<String>) -> Self {
        Self {
            rule_type: "field".to_string(),
            domains,
            ips: Vec::new(),
            ports: Vec::new(),
            outbound: "blackhole".to_string(),
        }
    }
}

/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    /// Primary DNS servers
    pub servers: Vec<String>,
    /// Use system DNS for direct connections
    #[serde(default)]
    pub use_system_dns: bool,
    /// Enable fake DNS for domain routing
    #[serde(default)]
    pub fake_dns: bool,
    /// Fake DNS IP pool
    #[serde(default = "default_fake_dns_pool")]
    pub fake_dns_pool: String,
}

fn default_fake_dns_pool() -> String {
    "198.18.0.0/16".to_string()
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
            use_system_dns: false,
            fake_dns: false,
            fake_dns_pool: default_fake_dns_pool(),
        }
    }
}

/// Local proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalProxyConfig {
    /// SOCKS5 listen address
    #[serde(default = "default_socks_addr")]
    pub socks_addr: String,
    /// SOCKS5 listen port
    #[serde(default = "default_socks_port")]
    pub socks_port: u16,
    /// HTTP proxy listen address (optional)
    pub http_addr: Option<String>,
    /// HTTP proxy listen port
    pub http_port: Option<u16>,
    /// Enable UDP relay
    #[serde(default = "default_true")]
    pub enable_udp: bool,
}

fn default_socks_addr() -> String {
    "127.0.0.1".to_string()
}

fn default_socks_port() -> u16 {
    1080
}

fn default_true() -> bool {
    true
}

impl Default for LocalProxyConfig {
    fn default() -> Self {
        Self {
            socks_addr: default_socks_addr(),
            socks_port: default_socks_port(),
            http_addr: None,
            http_port: None,
            enable_udp: true,
        }
    }
}

/// TUN device configuration for system VPN
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunConfig {
    /// TUN interface name
    #[serde(default = "default_tun_name")]
    pub name: String,
    /// TUN IP address
    #[serde(default = "default_tun_ip")]
    pub ip: String,
    /// TUN subnet CIDR
    #[serde(default = "default_tun_cidr")]
    pub cidr: u8,
    /// MTU size
    #[serde(default = "default_mtu")]
    pub mtu: u16,
    /// File descriptor (for Android/iOS)
    pub fd: Option<i32>,
    /// Auto-configure routing
    #[serde(default = "default_true")]
    pub auto_route: bool,
}

fn default_tun_name() -> String {
    "edgeray0".to_string()
}

fn default_tun_ip() -> String {
    "10.0.0.1".to_string()
}

fn default_tun_cidr() -> u8 {
    24
}

fn default_mtu() -> u16 {
    1500
}

impl Default for TunConfig {
    fn default() -> Self {
        Self {
            name: default_tun_name(),
            ip: default_tun_ip(),
            cidr: default_tun_cidr(),
            mtu: default_mtu(),
            fd: None,
            auto_route: true,
        }
    }
}

/// Mux configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuxConfig {
    /// Enable mux
    pub enabled: bool,
    /// Maximum concurrent connections per mux session
    #[serde(default = "default_concurrency")]
    pub concurrency: i16,
    /// Maximum concurrent XUDP connections
    #[serde(default = "default_xudp")]
    pub xudp_concurrency: i16,
}

fn default_concurrency() -> i16 {
    8
}

fn default_xudp() -> i16 {
    16
}

impl Default for MuxConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            concurrency: default_concurrency(),
            xudp_concurrency: default_xudp(),
        }
    }
}

/// TLS fragment settings for anti-censorship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentConfig {
    /// Fragment length range (e.g., "10-50")
    #[serde(default = "default_frag_length")]
    pub length: String,
    /// Interval between fragments in ms (e.g., "20-50")
    #[serde(default = "default_frag_interval")]
    pub interval: String,
}

fn default_frag_length() -> String {
    "10-50".to_string()
}

fn default_frag_interval() -> String {
    "20-50".to_string()
}

impl Default for FragmentConfig {
    fn default() -> Self {
        Self {
            length: default_frag_length(),
            interval: default_frag_interval(),
        }
    }
}

/// Flow-J protocol specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowJConfig {
    /// Flow-J mode: "auto", "reality", "cdn", "mqtt"
    #[serde(default = "default_flowj_mode")]
    pub mode: String,
    /// REALITY settings (for stealth mode)
    pub reality: Option<FlowJRealityConfig>,
    /// CDN settings (for CDN camouflage)
    pub cdn: Option<FlowJCdnConfig>,
    /// MQTT settings (for IoT camouflage)
    pub mqtt: Option<FlowJMqttConfig>,
    /// FEC settings
    pub fec: Option<FlowJFecConfig>,
}

fn default_flowj_mode() -> String {
    "auto".to_string()
}

impl Default for FlowJConfig {
    fn default() -> Self {
        Self {
            mode: default_flowj_mode(),
            reality: None,
            cdn: None,
            mqtt: None,
            fec: None,
        }
    }
}

/// Flow-J REALITY settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowJRealityConfig {
    /// REALITY destination (e.g., "www.google.com:443")
    pub dest: String,
    /// Server names for SNI
    #[serde(default)]
    pub server_names: Vec<String>,
    /// Private key (base64)
    pub private_key: Option<String>,
    /// Short IDs for client authentication
    #[serde(default)]
    pub short_ids: Vec<String>,
}

/// Flow-J CDN settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowJCdnConfig {
    /// CDN path
    #[serde(default = "default_path")]
    pub path: String,
    /// CDN host header
    pub host: Option<String>,
    /// Use xHTTP transport
    #[serde(default)]
    pub use_xhttp: bool,
}

fn default_path() -> String {
    "/".to_string()
}

/// Flow-J MQTT settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowJMqttConfig {
    /// MQTT broker address
    pub broker: String,
    /// Upload topic
    #[serde(default = "default_topic")]
    pub upload_topic: String,
    /// Download topic
    #[serde(default = "default_topic")]
    pub download_topic: String,
    /// MQTT username
    pub username: Option<String>,
    /// MQTT password
    pub password: Option<String>,
}

fn default_topic() -> String {
    "sensor/data".to_string()
}

/// Flow-J FEC settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowJFecConfig {
    /// Enable FEC
    pub enabled: bool,
    /// Data shards
    #[serde(default = "default_data_shards")]
    pub data_shards: usize,
    /// Parity shards
    #[serde(default = "default_parity_shards")]
    pub parity_shards: usize,
}

fn default_data_shards() -> usize {
    10
}

fn default_parity_shards() -> usize {
    3
}

/// GeoIP/GeoSite data manager for client-side rule parsing
pub struct GeoDataManager {
    geoip_path: Option<String>,
    geosite_path: Option<String>,
}

impl GeoDataManager {
    pub fn new() -> Self {
        Self {
            geoip_path: None,
            geosite_path: None,
        }
    }

    /// Set the GeoIP database path
    pub fn with_geoip(mut self, path: impl Into<String>) -> Self {
        self.geoip_path = Some(path.into());
        self
    }

    /// Set the GeoSite database path
    pub fn with_geosite(mut self, path: impl Into<String>) -> Self {
        self.geosite_path = Some(path.into());
        self
    }

    /// Generate routing rules for the specified routing mode
    pub fn generate_rules(&self, mode: RoutingMode) -> Vec<RoutingRule> {
        match mode {
            RoutingMode::Global => {
                // Route all traffic through proxy
                vec![]
            }
            RoutingMode::BypassLan => {
                // Bypass private IP ranges
                vec![RoutingRule::bypass_ips(vec![
                    "geoip:private".to_string(),
                    "127.0.0.0/8".to_string(),
                    "10.0.0.0/8".to_string(),
                    "172.16.0.0/12".to_string(),
                    "192.168.0.0/16".to_string(),
                ])]
            }
            RoutingMode::BypassMainland => {
                // Bypass China IPs and domains
                vec![
                    RoutingRule::bypass_domains(vec!["geosite:cn".to_string()]),
                    RoutingRule::bypass_ips(vec![
                        "geoip:cn".to_string(),
                        "geoip:private".to_string(),
                    ]),
                ]
            }
            RoutingMode::Direct => {
                vec![RoutingRule::bypass_ips(vec![
                    "0.0.0.0/0".to_string(),
                    "::/0".to_string(),
                ])]
            }
            RoutingMode::Rule => {
                // Default rule behavior (same as BypassMainland for now)
                vec![
                    RoutingRule::bypass_domains(vec!["geosite:cn".to_string()]),
                    RoutingRule::bypass_ips(vec![
                        "geoip:cn".to_string(),
                        "geoip:private".to_string(),
                    ]),
                ]
            }
        }
    }
}

impl Default for GeoDataManager {
    fn default() -> Self {
        Self::new()
    }
}
