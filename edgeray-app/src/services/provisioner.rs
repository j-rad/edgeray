//! Auto-Carrier Provisioner Service
//!
//! Probes paths to target nodes and selects the most optimal and stealthy carrier protocol.
//! Considers network conditions, protocol overhead, latency, and censorship resistance.

use crate::drivers::{BackendDriver, DriverError};
use crate::models::{CarrierType, ServerConfig};
use anyhow::Result;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Result of probing a specific carrier path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub carrier_type: CarrierType,
    pub latency_ms: u32,
    pub success: bool,
    pub jitter_ms: Option<u32>,
    pub packet_loss: Option<f32>,
    pub timestamp: u64,
}

/// Carrier probe configuration
#[derive(Debug, Clone)]
pub struct ProbeConfig {
    /// Number of probe packets to send
    pub probe_count: u32,
    /// Timeout for each probe
    pub probe_timeout: Duration,
    /// Interval between probes
    pub probe_interval: Duration,
    /// Whether to test bandwidth (more intrusive)
    pub test_bandwidth: bool,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            probe_count: 3,
            probe_timeout: Duration::from_secs(5),
            probe_interval: Duration::from_millis(200),
            test_bandwidth: false,
        }
    }
}

/// Carrier selection criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionPriority {
    /// Prefer lowest latency
    Speed,
    /// Prefer most stealthy carrier
    Stealth,
    /// Balance between speed and stealth
    Balanced,
    /// Prefer most reliable connection
    Reliability,
}

impl Default for SelectionPriority {
    fn default() -> Self {
        SelectionPriority::Balanced
    }
}

/// Cached probe results for a node
#[derive(Debug, Clone)]
struct NodeProbeCache {
    results: HashMap<CarrierType, ProbeResult>,
    last_updated: Instant,
    selected_carrier: Option<CarrierType>,
}

impl NodeProbeCache {
    fn new() -> Self {
        Self {
            results: HashMap::new(),
            last_updated: Instant::now(),
            selected_carrier: None,
        }
    }

    fn is_stale(&self, max_age: Duration) -> bool {
        self.last_updated.elapsed() > max_age
    }
}

/// The auto-provisioner service for carrier selection
pub struct Provisioner {
    backend: Arc<dyn BackendDriver>,
    probe_config: ProbeConfig,
    selection_priority: SelectionPriority,
    /// Cache of probe results per node
    cache: Arc<RwLock<HashMap<String, NodeProbeCache>>>,
    /// Maximum cache age before re-probing
    cache_max_age: Duration,
}

impl Provisioner {
    /// Create a new provisioner service
    pub fn new(backend: Arc<dyn BackendDriver>) -> Self {
        Self {
            backend,
            probe_config: ProbeConfig::default(),
            selection_priority: SelectionPriority::Balanced,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_max_age: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Configure probe settings
    pub fn with_probe_config(mut self, config: ProbeConfig) -> Self {
        self.probe_config = config;
        self
    }

    /// Set selection priority
    pub fn with_priority(mut self, priority: SelectionPriority) -> Self {
        self.selection_priority = priority;
        self
    }

    /// Set cache max age
    pub fn with_cache_age(mut self, age: Duration) -> Self {
        self.cache_max_age = age;
        self
    }

    /// Get cached carrier for a node, or None if not cached/stale
    pub async fn get_cached_carrier(&self, node_id: &str) -> Option<CarrierType> {
        let cache = self.cache.read().await;
        cache.get(node_id).and_then(|c| {
            if c.is_stale(self.cache_max_age) {
                None
            } else {
                c.selected_carrier
            }
        })
    }

    /// Probe all available carriers for a node and select the best one
    pub async fn select_best_carrier(&self, node: &ServerConfig) -> Result<Option<CarrierType>> {
        let node_id = node.id.clone().unwrap_or_else(|| node.address.clone());
        info!(
            "Starting carrier provisioning for node: {} ({})",
            node.remarks, node_id
        );

        // Check cache first
        if let Some(cached) = self.get_cached_carrier(&node_id).await {
            debug!("Using cached carrier for {}: {:?}", node_id, cached);
            return Ok(Some(cached));
        }

        // Get available carriers based on node configuration
        let available_carriers = self.get_available_carriers(node);
        if available_carriers.is_empty() {
            warn!("No compatible carriers found for node: {}", node.remarks);
            return Ok(None);
        }

        info!(
            "Probing {} carriers for {}: {:?}",
            available_carriers.len(),
            node.remarks,
            available_carriers
        );

        // Probe all carriers concurrently
        let probe_futures = available_carriers
            .iter()
            .map(|carrier| self.probe_carrier(node, *carrier));
        let results: Vec<ProbeResult> = join_all(probe_futures).await;

        // Filter successful probes
        let successful_probes: Vec<ProbeResult> =
            results.into_iter().filter(|r| r.success).collect();

        if successful_probes.is_empty() {
            warn!("All carrier probes failed for node: {}", node.remarks);
            return Ok(None);
        }

        // Select best carrier based on priority
        let best_carrier = self.select_by_priority(&successful_probes);

        // Update cache
        {
            let mut cache = self.cache.write().await;
            let entry = cache
                .entry(node_id.clone())
                .or_insert_with(NodeProbeCache::new);
            entry.results.clear();
            for probe in &successful_probes {
                entry.results.insert(probe.carrier_type, probe.clone());
            }
            entry.selected_carrier = best_carrier;
            entry.last_updated = Instant::now();
        }

        info!(
            "Selected best carrier for {}: {:?} (from {} successful probes)",
            node.remarks,
            best_carrier,
            successful_probes.len()
        );

        Ok(best_carrier)
    }

    /// Probe a single carrier and measure latency/quality
    async fn probe_carrier(&self, node: &ServerConfig, carrier: CarrierType) -> ProbeResult {
        let node_id = node.id.clone().unwrap_or_else(|| node.address.clone());
        debug!("Probing {:?} carrier for node {}", carrier, node_id);

        let _start = Instant::now();
        let mut latencies: Vec<u32> = Vec::with_capacity(self.probe_config.probe_count as usize);
        let mut failures = 0u32;

        for i in 0..self.probe_config.probe_count {
            let _probe_start = Instant::now();

            // Simulate carrier-specific probe
            // In production, this would send actual probe packets via the carrier
            let probe_result = self.execute_probe(node, carrier).await;

            match probe_result {
                Ok(latency) => {
                    latencies.push(latency);
                    debug!(
                        "Probe {}/{} for {:?}: {}ms",
                        i + 1,
                        self.probe_config.probe_count,
                        carrier,
                        latency
                    );
                }
                Err(e) => {
                    failures += 1;
                    debug!(
                        "Probe {}/{} for {:?} failed: {}",
                        i + 1,
                        self.probe_config.probe_count,
                        carrier,
                        e
                    );
                }
            }

            if i < self.probe_config.probe_count - 1 {
                crate::utils::sleep(self.probe_config.probe_interval).await;
            }
        }

        let success = !latencies.is_empty();
        let avg_latency = if success {
            latencies.iter().sum::<u32>() / latencies.len() as u32
        } else {
            u32::MAX
        };

        // Calculate jitter (variance in latency)
        let jitter = if latencies.len() >= 2 {
            let min = *latencies.iter().min().unwrap();
            let max = *latencies.iter().max().unwrap();
            Some(max - min)
        } else {
            None
        };

        // Calculate packet loss
        let packet_loss = Some(failures as f32 / self.probe_config.probe_count as f32 * 100.0);

        ProbeResult {
            carrier_type: carrier,
            latency_ms: avg_latency,
            success,
            jitter_ms: jitter,
            packet_loss,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Execute a single probe for a carrier type
    async fn execute_probe(
        &self,
        node: &ServerConfig,
        carrier: CarrierType,
    ) -> Result<u32, DriverError> {
        let _start = Instant::now();

        // Carrier-specific probe logic
        match carrier {
            CarrierType::Reality => {
                // REALITY probes use TLS handshake timing
                self.probe_tls_handshake(node).await
            }
            CarrierType::Mqtt => {
                // MQTT probes use CONNECT/CONNACK timing
                self.probe_mqtt_connect(node).await
            }
            CarrierType::Cdn => {
                // CDN probes use HTTP HEAD request timing
                self.probe_http_head(node).await
            }
            CarrierType::Direct => {
                // Direct probes use TCP handshake timing
                self.probe_tcp_connect(node).await
            }
        }
    }

    /// Probe TLS handshake timing (REALITY)
    async fn probe_tls_handshake(&self, node: &ServerConfig) -> Result<u32, DriverError> {
        let _start = Instant::now();

        // In production: perform actual TLS handshake with REALITY
        // For now, simulate with TCP connect + overhead estimate
        let base_latency = self.probe_tcp_connect(node).await?;

        // REALITY adds ~20-50ms overhead for the handshake
        let reality_overhead = 35;

        Ok(base_latency + reality_overhead)
    }

    /// Probe MQTT connection timing
    async fn probe_mqtt_connect(&self, node: &ServerConfig) -> Result<u32, DriverError> {
        let _start = Instant::now();

        // In production: send MQTT CONNECT and wait for CONNACK
        let base_latency = self.probe_tcp_connect(node).await?;

        // MQTT adds ~10-30ms overhead
        let mqtt_overhead = 20;

        Ok(base_latency + mqtt_overhead)
    }

    /// Probe HTTP HEAD request timing (CDN)
    async fn probe_http_head(&self, node: &ServerConfig) -> Result<u32, DriverError> {
        let _start = Instant::now();

        // In production: send HTTP HEAD request
        let base_latency = self.probe_tcp_connect(node).await?;

        // HTTP/WS adds ~15-40ms overhead
        let http_overhead = 25;

        Ok(base_latency + http_overhead)
    }

    /// Probe raw TCP connection timing
    /// Probe raw TCP connection timing
    #[cfg(not(target_arch = "wasm32"))]
    async fn probe_tcp_connect(&self, node: &ServerConfig) -> Result<u32, DriverError> {
        
        use tokio::net::TcpStream;

        let start = Instant::now();
        let addr = format!("{}:{}", node.address, node.port);

        let result =
            tokio::time::timeout(self.probe_config.probe_timeout, TcpStream::connect(&addr)).await;

        match result {
            Ok(Ok(_stream)) => {
                let latency = start.elapsed().as_millis() as u32;
                Ok(latency)
            }
            Ok(Err(e)) => Err(DriverError::Connection(e.to_string())),
            Err(_) => Err(DriverError::Timeout("TCP connect timeout".to_string())),
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn probe_tcp_connect(&self, _node: &ServerConfig) -> Result<u32, DriverError> {
        // In WASM/Web, we cannot make arbitrary TCP connections.
        // We could use fetch/websocket if supported by the endpoint, but for now we stub it.
        // Real probing should happen via a backend proxy or specialized endpoint.
        // Returning a dummy 10ms to allow flow to proceed.
        Ok(10)
    }

    /// Select best carrier based on priority setting
    fn select_by_priority(&self, probes: &[ProbeResult]) -> Option<CarrierType> {
        if probes.is_empty() {
            return None;
        }

        let mut scored: Vec<(CarrierType, f64)> = probes
            .iter()
            .map(|p| {
                let score = self.calculate_score(p);
                (p.carrier_type, score)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored.first().map(|(c, _)| *c)
    }

    /// Calculate composite score for a probe result
    fn calculate_score(&self, probe: &ProbeResult) -> f64 {
        let stealth_weight = self.carrier_stealth_score(probe.carrier_type) as f64;
        let latency_score = 1000.0 / (probe.latency_ms.max(1) as f64);
        let reliability_score = 100.0 - probe.packet_loss.unwrap_or(0.0) as f64;
        let jitter_score = 100.0 / (probe.jitter_ms.unwrap_or(1).max(1) as f64);

        match self.selection_priority {
            SelectionPriority::Speed => latency_score * 10.0 + reliability_score + stealth_weight,
            SelectionPriority::Stealth => stealth_weight * 50.0 + latency_score + reliability_score,
            SelectionPriority::Balanced => {
                latency_score * 5.0 + stealth_weight * 20.0 + reliability_score * 2.0
            }
            SelectionPriority::Reliability => {
                reliability_score * 10.0 + jitter_score * 5.0 + latency_score
            }
        }
    }

    /// Get carriers supported by the node's configuration
    fn get_available_carriers(&self, node: &ServerConfig) -> Vec<CarrierType> {
        let mut carriers = Vec::new();

        // Check if node supports REALITY (has pbk/sid)
        if node.pbk.is_some() && node.sid.is_some() {
            carriers.push(CarrierType::Reality);
        }

        // Check for WebSocket/CDN support
        if node.network.as_deref() == Some("ws")
            || node.network.as_deref() == Some("http")
            || node.path.is_some()
        {
            carriers.push(CarrierType::Cdn);
        }

        // Direct is always available as fallback
        carriers.push(CarrierType::Direct);

        // If node has specific MQTT config, add MQTT
        // (In practice, check for MQTT-specific settings)
        if node.network.as_deref() == Some("mqtt") {
            carriers.push(CarrierType::Mqtt);
        }

        // If no special carriers detected, provide defaults for testing
        if carriers.len() == 1 {
            carriers.push(CarrierType::Reality);
            carriers.push(CarrierType::Cdn);
        }

        carriers
    }

    /// Stealth score for carrier type (higher = more stealthy)
    fn carrier_stealth_score(&self, carrier: CarrierType) -> u8 {
        match carrier {
            CarrierType::Reality => 100, // Most stealthy - looks like real HTTPS
            CarrierType::Mqtt => 80,     // Good stealth - looks like IoT traffic
            CarrierType::Cdn => 60,      // Moderate - looks like CDN traffic
            CarrierType::Direct => 20,   // Low stealth - recognizable protocol
        }
    }

    /// Get human-readable carrier name
    pub fn carrier_name(carrier: CarrierType) -> &'static str {
        match carrier {
            CarrierType::Reality => "REALITY",
            CarrierType::Mqtt => "MQTT IoT",
            CarrierType::Cdn => "CDN/WebSocket",
            CarrierType::Direct => "Direct",
        }
    }

    /// Get all probe results for a node (from cache)
    pub async fn get_probe_results(
        &self,
        node_id: &str,
    ) -> Option<HashMap<CarrierType, ProbeResult>> {
        let cache = self.cache.read().await;
        cache.get(node_id).map(|c| c.results.clone())
    }

    /// Force re-probe for a specific node
    pub async fn invalidate_cache(&self, node_id: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(node_id);
    }

    /// Clear all cached results
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::MockDriver;
    use crate::models::Protocol;

    fn create_test_node() -> ServerConfig {
        ServerConfig {
            id: Some("test-node-1".to_string()),
            address: "127.0.0.1".to_string(),
            port: 443,
            remarks: "Test Node".to_string(),
            protocol: Protocol::Vless,
            uuid: Some("test-uuid".to_string()),
            password: None,
            network: Some("ws".to_string()),
            flow: None,
            security: Some("tls".to_string()),
            fingerprint: None,
            sni: Some("example.com".to_string()),
            host: None,
            path: Some("/ws".to_string()),
            method: None,
            pbk: Some("test-pbk".to_string()),
            sid: Some("test-sid".to_string()),
            service_name: None,
            group: None,
            allow_insecure: None,
        }
    }

    #[test]
    fn test_carrier_stealth_ordering() {
        let provisioner = Provisioner::new(Arc::new(MockDriver::new("test")));

        assert!(
            provisioner.carrier_stealth_score(CarrierType::Reality)
                > provisioner.carrier_stealth_score(CarrierType::Mqtt)
        );
        assert!(
            provisioner.carrier_stealth_score(CarrierType::Mqtt)
                > provisioner.carrier_stealth_score(CarrierType::Cdn)
        );
        assert!(
            provisioner.carrier_stealth_score(CarrierType::Cdn)
                > provisioner.carrier_stealth_score(CarrierType::Direct)
        );
    }

    #[test]
    fn test_get_available_carriers() {
        let provisioner = Provisioner::new(Arc::new(MockDriver::new("test")));
        let node = create_test_node();

        let carriers = provisioner.get_available_carriers(&node);

        // Should include REALITY (has pbk/sid), CDN (has ws network), and Direct
        assert!(carriers.contains(&CarrierType::Reality));
        assert!(carriers.contains(&CarrierType::Cdn));
        assert!(carriers.contains(&CarrierType::Direct));
    }

    #[test]
    fn test_score_calculation() {
        let provisioner = Provisioner::new(Arc::new(MockDriver::new("test")));

        let probe1 = ProbeResult {
            carrier_type: CarrierType::Reality,
            latency_ms: 50,
            success: true,
            jitter_ms: Some(5),
            packet_loss: Some(0.0),
            timestamp: 0,
        };

        let probe2 = ProbeResult {
            carrier_type: CarrierType::Direct,
            latency_ms: 30,
            success: true,
            jitter_ms: Some(2),
            packet_loss: Some(0.0),
            timestamp: 0,
        };

        // With balanced priority, REALITY should score higher due to stealth
        let score1 = provisioner.calculate_score(&probe1);
        let score2 = provisioner.calculate_score(&probe2);

        // REALITY has higher stealth, should win in balanced mode
        assert!(
            score1 > score2,
            "REALITY should score higher than Direct in balanced mode"
        );
    }

    #[test]
    fn test_probe_config_default() {
        let config = ProbeConfig::default();
        assert_eq!(config.probe_count, 3);
        assert_eq!(config.probe_timeout, Duration::from_secs(5));
        assert!(!config.test_bandwidth);
    }
}

/// Dioxus hook to access the backend driver
pub fn use_driver() -> Arc<dyn BackendDriver> {
    use crate::drivers::DriverFactory;
    use dioxus::prelude::*;

    // For now, we return a local driver by default.
    // In a real app, this might come from a context or global state.
    use_hook(|| DriverFactory::local()).clone()
}
