//! Backend Driver Module
//!
//! Trait-based abstraction for controlling VPN backends.
//! Supports both local (UniFFI) and remote (Actix-web API) targets.

pub mod execution_config;
#[cfg(not(target_arch = "wasm32"))]
pub mod local_uniffi;
pub mod remote_actix;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::broadcast;

pub use execution_config::ExecutionConfig;
#[cfg(not(target_arch = "wasm32"))]
pub use local_uniffi::LocalUniFFIDriver;
pub use remote_actix::RemoteActixDriver;

/// Errors that can occur during driver operations
#[derive(Error, Debug, Clone)]
pub enum DriverError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Engine not running")]
    NotRunning,

    #[error("Engine already running")]
    AlreadyRunning,

    #[error("FFI error: {0}")]
    Ffi(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for DriverError {
    fn from(e: serde_json::Error) -> Self {
        DriverError::Serialization(e.to_string())
    }
}

/// Metrics snapshot from the VPN engine
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsSnapshot {
    /// Bytes uploaded since start
    pub bytes_uploaded: u64,
    /// Bytes downloaded since start
    pub bytes_downloaded: u64,
    /// Current upload speed (bytes/sec)
    pub upload_speed: f64,
    /// Current download speed (bytes/sec)
    pub download_speed: f64,
    /// Active connections count
    pub active_connections: u32,
    /// Total connections since start
    pub total_connections: u64,
    /// Current latency to proxy in milliseconds
    pub latency_ms: Option<u32>,
    /// Connection state (0=disconnected, 1=connecting, 2=connected)
    pub connection_state: u8,
    /// Timestamp of this snapshot
    pub timestamp: u64,
}

pub use crate::models::ConnectionMetrics;

/// Remote node information for RemoteActixDriver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteNode {
    /// Node name/identifier
    pub name: String,
    /// Base URL for API (e.g., "http://192.168.1.1:8080")
    pub base_url: String,
    /// Pre-shared key for authentication
    pub psk: String,
    /// Request timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    10000
}

impl RemoteNode {
    pub fn new(
        name: impl Into<String>,
        base_url: impl Into<String>,
        psk: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into(),
            psk: psk.into(),
            timeout_ms: default_timeout(),
        }
    }
}

/// Driver type enum for factory pattern
#[derive(Debug, Clone)]
pub enum DriverType {
    /// Local device VPN via UniFFI bindings
    Local,
    /// Remote router via Actix-web API
    Remote(RemoteNode),
    /// Mock driver for testing
    Mock(String),
}

/// Backend driver trait for VPN control
///
/// This trait abstracts the differences between local UniFFI-based control
/// and remote HTTP-based control of VPN engines.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BackendDriver: Send + Sync {
    /// Start the VPN engine with the current configuration
    async fn start(&self) -> Result<(), DriverError>;

    /// Stop the VPN engine
    async fn stop(&self) -> Result<(), DriverError>;

    /// Push a new configuration to the engine
    ///
    /// For local drivers, this reconfigures the UniFFI engine.
    /// For remote drivers, this sends the config JSON to the router API.
    async fn push_config(&self, config: ExecutionConfig) -> Result<(), DriverError>;

    /// Pull current metrics from the engine
    ///
    /// Returns a snapshot of traffic stats, connection count, and latency.
    async fn pull_metrics(&self) -> Result<MetricsSnapshot, DriverError>;

    /// Check if the engine is currently running
    async fn is_running(&self) -> bool;

    /// Get the driver type
    fn driver_type(&self) -> DriverType;

    /// Get the driver name for display
    fn name(&self) -> &str;

    /// Update an external core component
    async fn update_core(&self, core_name: String) -> Result<String, DriverError>;

    /// Pull connection-level metrics
    async fn pull_connection_metrics(
        &self,
        conn_id: &str,
    ) -> Result<Vec<ConnectionMetrics>, DriverError>;
}

/// Driver factory for creating appropriate driver instances
#[cfg(not(target_arch = "wasm32"))]
pub struct DriverFactory;

#[cfg(not(target_arch = "wasm32"))]
impl DriverFactory {
    /// Create a driver based on the driver type
    pub fn create(driver_type: DriverType) -> Arc<dyn BackendDriver> {
        let driver: Arc<dyn BackendDriver> = match driver_type {
            DriverType::Local => Arc::new(LocalUniFFIDriver::new()) as Arc<dyn BackendDriver>,
            DriverType::Remote(node) => {
                Arc::new(RemoteActixDriver::new(node)) as Arc<dyn BackendDriver>
            }
            DriverType::Mock(name) => Arc::new(MockDriver::new(name)) as Arc<dyn BackendDriver>,
        };
        driver
    }

    /// Create a local UniFFI driver
    pub fn local() -> Arc<dyn BackendDriver> {
        Arc::new(LocalUniFFIDriver::new()) as Arc<dyn BackendDriver>
    }

    /// Create a remote Actix driver
    pub fn remote(node: RemoteNode) -> Arc<dyn BackendDriver> {
        Arc::new(RemoteActixDriver::new(node))
    }
}

#[cfg(target_arch = "wasm32")]
pub struct DriverFactory;

#[cfg(target_arch = "wasm32")]
impl DriverFactory {
    pub fn local() -> Arc<dyn BackendDriver> {
        // In WASM, "local" effectively means we talk to the backend serving us
        let node = RemoteNode::new("Local Node", "http://127.0.0.1:8080", "default-psk");
        Arc::new(RemoteActixDriver::new(node))
    }
}

/// Multi-driver manager for controlling multiple backends
#[cfg(not(target_arch = "wasm32"))]
pub struct DriverManager {
    drivers: Vec<Arc<dyn BackendDriver>>,
    metrics_tx: broadcast::Sender<(DriverType, MetricsSnapshot)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DriverManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            drivers: Vec::new(),
            metrics_tx: tx,
        }
    }

    /// Subscribe to metrics updates
    pub fn subscribe_metrics(&self) -> broadcast::Receiver<(DriverType, MetricsSnapshot)> {
        self.metrics_tx.subscribe()
    }

    /// Start a background polling task for all drivers at 1Hz
    pub fn start_polling(&self) {
        let drivers = self.drivers.clone();
        let tx = self.metrics_tx.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                for driver in &drivers {
                    if driver.is_running().await {
                        if let Ok(metrics) = driver.pull_metrics().await {
                            let _ = tx.send((driver.driver_type(), metrics));
                        }
                    }
                }
            }
        });
    }

    /// Add a driver to the manager
    pub fn add_driver(&mut self, driver: Arc<dyn BackendDriver>) {
        self.drivers.push(driver);
    }

    /// Get all drivers
    pub fn drivers(&self) -> &[Arc<dyn BackendDriver>] {
        &self.drivers
    }

    /// Start all drivers
    pub async fn start_all(&self) -> Vec<Result<(), DriverError>> {
        let mut results = Vec::with_capacity(self.drivers.len());
        for driver in &self.drivers {
            results.push(driver.start().await);
        }
        results
    }

    /// Stop all drivers
    pub async fn stop_all(&self) -> Vec<Result<(), DriverError>> {
        let mut results = Vec::with_capacity(self.drivers.len());
        for driver in &self.drivers {
            results.push(driver.stop().await);
        }
        results
    }

    /// Pull metrics from all drivers
    pub async fn pull_all_metrics(&self) -> Vec<Result<MetricsSnapshot, DriverError>> {
        let mut results = Vec::with_capacity(self.drivers.len());
        for driver in &self.drivers {
            results.push(driver.pull_metrics().await);
        }
        results
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock driver for testing behavior without real hardware/network
pub struct MockDriver {
    name: String,
    running: std::sync::atomic::AtomicBool,
}

impl MockDriver {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BackendDriver for MockDriver {
    async fn start(&self) -> Result<(), DriverError> {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn stop(&self) -> Result<(), DriverError> {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    async fn push_config(&self, _config: ExecutionConfig) -> Result<(), DriverError> {
        Ok(())
    }

    async fn pull_metrics(&self) -> Result<MetricsSnapshot, DriverError> {
        Ok(MetricsSnapshot {
            bytes_uploaded: 1000,
            bytes_downloaded: 2000,
            upload_speed: 100.0,
            download_speed: 200.0,
            active_connections: 5,
            latency_ms: Some(25),
            connection_state: 2,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ..Default::default()
        })
    }

    async fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn driver_type(&self) -> DriverType {
        DriverType::Mock(self.name.clone())
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn update_core(&self, core_name: String) -> Result<String, DriverError> {
        Ok(format!("{}-mock-v1.0", core_name))
    }

    async fn pull_connection_metrics(
        &self,
        _conn_id: &str,
    ) -> Result<Vec<ConnectionMetrics>, DriverError> {
        Ok(vec![ConnectionMetrics {
            rtt_ms: 45,
            cwnd_bytes: 65535,
            dpi_state: crate::models::DpiState::Clear,
            timestamp: 123456789,
        }])
    }
}
