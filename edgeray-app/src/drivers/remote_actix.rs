//! Remote Actix Driver
//!
//! Driver implementation for controlling remote OpenWrt routers via HTTP API.

#[cfg(target_arch = "wasm32")]
use crate::models::ConnectionMetrics;
#[cfg(not(target_arch = "wasm32"))]
use rustray::types::ConnectionMetrics;
use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::{BackendDriver, DriverError, DriverType, ExecutionConfig, MetricsSnapshot, RemoteNode};

/// Remote driver using HTTP API to control OpenWrt router
pub struct RemoteActixDriver {
    node: RemoteNode,
    running: AtomicBool,
    client: reqwest::Client,
}

impl RemoteActixDriver {
    pub fn new(node: RemoteNode) -> Self {
        let builder = reqwest::Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        let builder = {
            let timeout = std::time::Duration::from_millis(node.timeout_ms);
            builder
                .timeout(timeout)
                .connect_timeout(std::time::Duration::from_secs(5))
        };

        let client = builder.build().unwrap_or_else(|_| reqwest::Client::new());

        Self {
            node,
            running: AtomicBool::new(false),
            client,
        }
    }

    /// Make an authenticated request to the remote API
    async fn request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<R, DriverError> {
        let url = format!("{}{}", self.node.base_url, path);

        let mut request = self
            .client
            .request(method.clone(), &url)
            .header("X-EdgeRay-PSK", &self.node.psk)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        }

        debug!("Making {} request to {}", method, url);

        let response = request.send().await.map_err(|e| {
            if e.is_timeout() {
                DriverError::Timeout(format!("Request to {} timed out", url))
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                if e.is_connect() {
                    return DriverError::Connection(format!("Failed to connect to {}: {}", url, e));
                }
                DriverError::Network(e.to_string())
            }
        })?;

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            return Err(DriverError::Auth(format!(
                "Authentication failed for node: {}",
                self.node.name
            )));
        }

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(DriverError::Network(format!(
                "Request failed with status {}: {}",
                status, text
            )));
        }

        response
            .json::<R>()
            .await
            .map_err(|e| DriverError::Serialization(format!("Failed to parse response: {}", e)))
    }

    /// Simple GET request returning JSON
    async fn get<R: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<R, DriverError> {
        self.request::<(), R>(reqwest::Method::GET, path, None)
            .await
    }

    /// POST request with JSON body
    async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R, DriverError> {
        self.request(reqwest::Method::POST, path, Some(body)).await
    }
}

/// API response wrapper
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    #[serde(default)]
    message: Option<String>,
    data: Option<T>,
}

/// Engine status response
#[derive(Debug, Deserialize)]
struct EngineStatus {
    running: bool,
    #[serde(default)]
    uptime_secs: u64,
}

/// Stats response from remote API
#[derive(Debug, Deserialize)]
struct RemoteStats {
    bytes_uploaded: u64,
    bytes_downloaded: u64,
    #[serde(default)]
    upload_speed: f64,
    #[serde(default)]
    download_speed: f64,
    #[serde(default)]
    active_connections: u32,
    #[serde(default)]
    total_connections: u64,
    #[serde(default)]
    latency_ms: Option<u32>,
    #[serde(default)]
    connection_state: u8,
    #[serde(default)]
    pub timestamp: u64,
}

/// Connection metrics from remote API
#[derive(Debug, Deserialize)]
pub struct RemoteConnectionMetric {
    pub rtt_ms: u64,
    pub cwnd_bytes: u64,
    pub dpi_state: crate::models::DpiState,
    pub timestamp: u64,
}

/// Empty API response
#[derive(Debug, Deserialize)]
struct EmptyResponse {
    success: bool,
    #[serde(default)]
    message: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BackendDriver for RemoteActixDriver {
    async fn start(&self) -> Result<(), DriverError> {
        if self.running.load(Ordering::SeqCst) {
            return Err(DriverError::AlreadyRunning);
        }

        info!("Starting remote engine on {}...", self.node.name);

        let response: EmptyResponse = self.post("/api/engine/start", &()).await?;

        if response.success {
            self.running.store(true, Ordering::SeqCst);
            info!("Remote engine {} started", self.node.name);
            Ok(())
        } else {
            Err(DriverError::Internal(
                response.message.unwrap_or_else(|| "Failed to start".into()),
            ))
        }
    }

    async fn stop(&self) -> Result<(), DriverError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(DriverError::NotRunning);
        }

        info!("Stopping remote engine on {}...", self.node.name);

        let response: EmptyResponse = self.post("/api/engine/stop", &()).await?;

        if response.success {
            self.running.store(false, Ordering::SeqCst);
            info!("Remote engine {} stopped", self.node.name);
            Ok(())
        } else {
            Err(DriverError::Internal(
                response.message.unwrap_or_else(|| "Failed to stop".into()),
            ))
        }
    }

    async fn push_config(&self, config: ExecutionConfig) -> Result<(), DriverError> {
        info!("Pushing config to remote engine {}...", self.node.name);

        // Convert to JSON and send to the remote API
        let config_json = config.to_json()?;

        #[derive(Serialize)]
        struct ConfigPayload {
            config: String,
        }

        let payload = ConfigPayload {
            config: config_json,
        };

        let response: EmptyResponse = self.post("/api/engine/config", &payload).await?;

        if response.success {
            info!("Config pushed to {} successfully", self.node.name);

            // Start the engine with the new config
            let start_response: EmptyResponse = self.post("/api/engine/start", &()).await?;

            if start_response.success {
                self.running.store(true, Ordering::SeqCst);
                Ok(())
            } else {
                warn!(
                    "Config pushed but engine start failed: {:?}",
                    start_response.message
                );
                Err(DriverError::Internal(
                    start_response
                        .message
                        .unwrap_or_else(|| "Failed to start after config push".into()),
                ))
            }
        } else {
            Err(DriverError::Config(
                response
                    .message
                    .unwrap_or_else(|| "Failed to push config".into()),
            ))
        }
    }

    async fn pull_metrics(&self) -> Result<MetricsSnapshot, DriverError> {
        let response: ApiResponse<RemoteStats> = self.get("/api/engine/stats").await?;

        if let Some(stats) = response.data {
            Ok(MetricsSnapshot {
                bytes_uploaded: stats.bytes_uploaded,
                bytes_downloaded: stats.bytes_downloaded,
                upload_speed: stats.upload_speed,
                download_speed: stats.download_speed,
                active_connections: stats.active_connections,
                total_connections: stats.total_connections,
                latency_ms: stats.latency_ms,
                connection_state: stats.connection_state,
                timestamp: stats.timestamp,
            })
        } else {
            Err(DriverError::Internal(
                response
                    .message
                    .unwrap_or_else(|| "No stats data returned".into()),
            ))
        }
    }

    async fn is_running(&self) -> bool {
        // Try to fetch actual status from remote
        match self
            .get::<ApiResponse<EngineStatus>>("/api/engine/status")
            .await
        {
            Ok(response) => {
                if let Some(status) = response.data {
                    self.running.store(status.running, Ordering::SeqCst);
                    status.running
                } else {
                    self.running.load(Ordering::SeqCst)
                }
            }
            Err(e) => {
                debug!("Failed to fetch remote status: {:?}", e);
                self.running.load(Ordering::SeqCst)
            }
        }
    }

    fn driver_type(&self) -> DriverType {
        DriverType::Remote(self.node.clone())
    }

    fn name(&self) -> &str {
        &self.node.name
    }

    async fn update_core(&self, core_name: String) -> Result<String, DriverError> {
        info!(
            "Updating remote core {} on {}...",
            core_name, self.node.name
        );

        #[derive(Deserialize)]
        struct UpdateResponse {
            version: String,
        }

        let url = format!("/node/update_core?core_type={}", core_name);

        // Note: Using post empty body since query param carries data
        let response: ApiResponse<UpdateResponse> = self.post(&url, &()).await?;

        if response.success {
            if let Some(data) = response.data {
                info!("Remote core updated to {}", data.version);
                Ok(data.version)
            } else {
                Err(DriverError::Internal("No version returned".to_string()))
            }
        } else {
            Err(DriverError::Internal(
                response
                    .message
                    .unwrap_or_else(|| "Failed to update core".into()),
            ))
        }
    }

    async fn pull_connection_metrics(
        &self,
        conn_id: &str,
    ) -> Result<Vec<ConnectionMetrics>, DriverError> {
        let url = format!("/node/connection_metrics?conn_id={}", conn_id);
        let response: Vec<RemoteConnectionMetric> = self.get(&url).await?;

        Ok(response
            .into_iter()
            .map(|m| ConnectionMetrics {
                rtt_ms: m.rtt_ms,
                cwnd_bytes: m.cwnd_bytes,
                dpi_state: m.dpi_state.clone(),
                timestamp: m.timestamp,
            })
            .collect())
    }
}

/// Health check for remote node
pub async fn check_node_health(node: &RemoteNode) -> Result<bool, DriverError> {
    let driver = RemoteActixDriver::new(node.clone());

    match driver
        .get::<ApiResponse<serde_json::Value>>("/api/health")
        .await
    {
        Ok(response) => Ok(response.success),
        Err(e) => {
            warn!("Health check failed for {}: {:?}", node.name, e);
            Err(e)
        }
    }
}

/// Discover nodes on the local network
pub async fn discover_nodes(_subnet: &str, _port: u16) -> Vec<RemoteNode> {
    // This would scan the network for EdgeRay nodes
    // For now, return empty - actual implementation would use mDNS or broadcast
    warn!("Network discovery not yet implemented");
    Vec::new()
}
