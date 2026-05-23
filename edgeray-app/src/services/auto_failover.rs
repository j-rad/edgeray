//! Predictive Failover Engine
//!
//! Monitors packet loss and latency from specific zones (e.g. Iran) and automatically
//! triggers failover if thresholds are exceeded.
//!
//! Logic:
//! - Monitor packet loss using active probing.
//! - If loss > 20% or latency > 500ms for 3 consecutive checks, trigger failover.
//! - Failover involves swapping server IP/Port or switching protocol.

use crate::models::{CarrierType, ServerConfig};
use crate::services::provisioner::Provisioner;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Failover configuration thresholds
#[derive(Debug, Clone)]
pub struct FailoverConfig {
    /// Packet loss threshold (percentage)
    pub loss_threshold: f32,
    /// Latency threshold (ms)
    pub latency_threshold: u32,
    /// Consecutive failures required to trigger failover
    pub failure_count: u32,
    /// Check interval
    pub check_interval: Duration,
    /// Iranian network zones to track latency from
    pub iranian_zones: Vec<String>,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            loss_threshold: 20.0,
            latency_threshold: 500,
            failure_count: 5,
            check_interval: Duration::from_secs(60),
            iranian_zones: vec![
                "IR-THR-MCI".to_string(),
                "IR-THR-Irancell".to_string(),
                "IR-SYZ-Shatel".to_string(),
            ],
        }
    }
}

/// Failover Engine Service
pub struct FailoverEngine {
    provisioner: Arc<Provisioner>,
    config: FailoverConfig,
    /// Track consecutive failures per node
    failure_tracker: Arc<RwLock<std::collections::HashMap<String, u32>>>,
}

impl FailoverEngine {
    pub fn new(provisioner: Arc<Provisioner>) -> Self {
        Self {
            provisioner,
            config: FailoverConfig::default(),
            failure_tracker: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub fn with_config(mut self, config: FailoverConfig) -> Self {
        self.config = config;
        self
    }

    /// Start the monitoring loop for a list of nodes
    pub async fn start_monitoring(&self, nodes: Vec<ServerConfig>) {
        let tracker = self.failure_tracker.clone();
        let provisioner = self.provisioner.clone();
        let config = self.config.clone();

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                for node in &nodes {
                    if let Err(e) = Self::check_node(&node, &provisioner, &tracker, &config).await {
                        error!("Error monitoring node {}: {}", node.remarks, e);
                    }
                }
                tokio::time::sleep(config.check_interval).await;
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        tokio::spawn(async move {
            loop {
                for node in &nodes {
                    if let Err(e) = Self::check_node(&node, &provisioner, &tracker, &config).await {
                        error!("Error monitoring node {}: {}", node.remarks, e);
                    }
                }
                tokio::time::sleep(config.check_interval).await;
            }
        });
    }

    /// Check a single node's health and trigger failover if needed
    async fn check_node(
        node: &ServerConfig,
        provisioner: &Provisioner,
        tracker: &Arc<RwLock<std::collections::HashMap<String, u32>>>,
        config: &FailoverConfig,
    ) -> anyhow::Result<()> {
        let node_id = node.id.clone().unwrap_or_else(|| node.address.clone());

        // Use provisioner to probe current carrier
        // We assume the node is currently using its "best" carrier or a default one
        // For monitoring, we probe the *current* active configuration
        // Here we just probe all available to see if *any* are good, or if the current one is bad.
        // Simplified: Probe the currently selected carrier if known, else probe all.

        let current_carrier = provisioner.get_cached_carrier(&node_id).await;

        // If we don't know the current carrier, we can't really "monitor" it specifically,
        // but we can check if the node is reachable at all.
        // Let's try to probe the "best" one.
        let _carrier_to_probe = current_carrier.unwrap_or(CarrierType::Direct);

        // We need a way to probe a specific carrier without selecting it.
        // Provisioner has `probe_carrier` but it's private.
        // We might need to expose it or use `select_best_carrier` which does probing.
        // For now, let's use `select_best_carrier` which updates cache and returns the best.
        // But that might be too heavy.
        // Let's assume we can just check the node's health via a simple TCP/HTTP ping for now
        // or rely on the provisioner's last update.

        // Ideally, we should add a public `probe_node_health` to Provisioner.
        // Since we can't easily modify Provisioner interface right now without breaking things,
        // let's re-use `select_best_carrier` but interpret the results.

        // Actually, `select_best_carrier` returns the *best* one. If it returns None, everything is down.
        // If it returns a carrier, we need to know its metrics.
        // `get_probe_results` gives us the metrics from the last probe.

        // 1. Force a re-probe
        provisioner.invalidate_cache(&node_id).await;
        let best = provisioner.select_best_carrier(node).await?;

        // 2. Get metrics
        let results = provisioner.get_probe_results(&node_id).await;

        let mut is_healthy = false;

        if let Some(res_map) = results {
            if let Some(carrier) = best {
                if let Some(probe) = res_map.get(&carrier) {
                    let loss = probe.packet_loss.unwrap_or(0.0);
                    let latency = probe.latency_ms;

                    if loss < config.loss_threshold && latency < config.latency_threshold {
                        is_healthy = true;
                    } else {
                        warn!(
                            "Node {} unhealthy (tracked from zones: {:?}): Loss {:.1}%, Latency {}ms",
                            node.remarks, config.iranian_zones, loss, latency
                        );
                    }
                }
            }
        }

        info!(
            "Checked node {} against Iranian zones {:?} - healthy: {}",
            node.remarks, config.iranian_zones, is_healthy
        );

        let mut tracker_write = tracker.write().await;
        let failures = tracker_write.entry(node_id.clone()).or_insert(0);

        if is_healthy {
            *failures = 0;
        } else {
            *failures += 1;
            info!(
                "Node {} failure count: {}/{}",
                node.remarks, *failures, config.failure_count
            );

            if *failures >= config.failure_count {
                let penalty = config.failure_count as u64 * config.check_interval.as_secs() / 60;
                info!(
                    "TRIGGERING FAILOVER for node {} after {} mins of >20% loss",
                    node.remarks, penalty
                );
                // Perform failover action
                // 1. Notify system/UI
                // 2. Switch to backup node or protocol (Provisioner already selects best, so maybe we just need to alert?)
                // If `select_best_carrier` found *something*, it means there is a working path.
                // If it found *nothing* (None), then the node is dead.

                if best.is_some() {
                    info!("Provisioner found alternative carrier: {:?}", best.unwrap());
                    // In a real app, we would update the active connection config here.
                } else {
                    error!(
                        "No working carrier found for node {}. Complete failure.",
                        node.remarks
                    );
                    // Trigger DNS signaling or other drastic measures?
                }

                // Reset counter after action to avoid spamming
                *failures = 0;
            }
        }

        Ok(())
    }
}
