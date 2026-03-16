//! Local UniFFI Driver
//!
//! Driver implementation for controlling the local VPN engine via UniFFI bindings.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use rustray::types::ConnectionMetrics;
use tracing::{debug, error, info};

use super::{BackendDriver, DriverError, DriverType, ExecutionConfig, MetricsSnapshot};

/// Local driver using UniFFI bindings to rustray core
pub struct LocalUniFFIDriver {
    running: AtomicBool,
    name: String,
}

impl LocalUniFFIDriver {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(false),
            name: "Local Device".to_string(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Build the config JSON for the UniFFI engine
    fn build_connect_config(config: &ExecutionConfig) -> Result<String, DriverError> {
        use serde_json::json;

        let server = &config.server;

        let mut connect_config = json!({
            "address": server.address,
            "port": server.port,
            "uuid": server.uuid.clone().unwrap_or_default(),
            "protocol": format!("{:?}", server.protocol).to_lowercase(),
            "flow": server.flow,
            "network": server.network.clone().unwrap_or_else(|| "tcp".to_string()),
            "security": server.security.clone().unwrap_or_else(|| "tls".to_string()),
            "local_address": config.local_proxy.socks_addr,
            "local_port": config.local_proxy.socks_port,
            "enable_udp": config.local_proxy.enable_udp,
            "routing_mode": format!("{:?}", config.routing_mode).to_lowercase(),
        });

        // Add REALITY settings if present
        if let Some(pbk) = &server.pbk {
            connect_config["reality_settings"] = json!({
                "public_key": pbk,
                "short_id": server.sid.clone().unwrap_or_default(),
                "server_name": server.sni.clone().unwrap_or_else(|| server.address.clone()),
                "fingerprint": server.fingerprint.clone().unwrap_or_else(|| "chrome".to_string()),
            });
        }

        // Add uTLS fingerprint
        if let Some(fp) = &server.fingerprint {
            connect_config["utls_fingerprint"] = json!(fp);
        }

        // Add fragment settings
        if let Some(frag) = &config.fragment {
            connect_config["fragment_settings"] = json!({
                "length": frag.length,
                "interval": frag.interval,
            });
        }

        // Add TUN FD for mobile
        if let Some(tun) = &config.tun {
            if let Some(fd) = tun.fd {
                connect_config["tun_fd"] = json!(fd);
            }
        }

        // Add Flow-J settings
        if let Some(flowj) = &config.flow_j {
            let mut flowj_config = json!({
                "mode": flowj.mode,
            });

            if let Some(reality) = &flowj.reality {
                flowj_config["reality"] = json!({
                    "dest": reality.dest,
                    "server_names": reality.server_names,
                    "private_key": reality.private_key,
                    "short_ids": reality.short_ids,
                });
            }

            if let Some(cdn) = &flowj.cdn {
                flowj_config["cdn"] = json!({
                    "path": cdn.path,
                    "host": cdn.host,
                    "use_xhttp": cdn.use_xhttp,
                });
            }

            if let Some(mqtt) = &flowj.mqtt {
                flowj_config["mqtt"] = json!({
                    "broker": mqtt.broker,
                    "upload_topic": mqtt.upload_topic,
                    "download_topic": mqtt.download_topic,
                    "username": mqtt.username,
                    "password": mqtt.password,
                });
            }

            if let Some(fec) = &flowj.fec {
                flowj_config["fec"] = json!({
                    "enabled": fec.enabled,
                    "data_shards": fec.data_shards,
                    "parity_shards": fec.parity_shards,
                });
            }

            connect_config["flow_j_settings"] = flowj_config;
        }

        serde_json::to_string(&connect_config).map_err(DriverError::from)
    }
}

impl Default for LocalUniFFIDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BackendDriver for LocalUniFFIDriver {
    async fn start(&self) -> Result<(), DriverError> {
        if self.running.load(Ordering::SeqCst) {
            return Err(DriverError::AlreadyRunning);
        }

        info!("Starting local UniFFI engine...");

        // Get the engine manager singleton
        let _engine = rustray::ffi::EngineManager::new();

        // We need a config to start - this will be called after push_config
        // For now, just mark as running (actual start happens in push_config)
        self.running.store(true, Ordering::SeqCst);

        debug!("Local engine marked as running");
        Ok(())
    }

    async fn stop(&self) -> Result<(), DriverError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(DriverError::NotRunning);
        }

        info!("Stopping local UniFFI engine...");

        let engine = rustray::ffi::EngineManager::new();
        let result = engine.stop_engine();

        match result {
            rustray::ffi::RayResult::Ok => {
                self.running.store(false, Ordering::SeqCst);
                info!("Local engine stopped successfully");
                Ok(())
            }
            rustray::ffi::RayResult::NotRunning => {
                self.running.store(false, Ordering::SeqCst);
                Ok(())
            }
            err => {
                error!("Failed to stop engine: {:?}", err);
                Err(DriverError::Ffi(format!("{:?}", err)))
            }
        }
    }

    async fn push_config(&self, config: ExecutionConfig) -> Result<(), DriverError> {
        info!("Pushing config to local UniFFI engine...");

        let config_json = Self::build_connect_config(&config)?;
        debug!("Config JSON: {}", config_json);

        let engine = rustray::ffi::EngineManager::new();

        // If already running, stop first
        if self.running.load(Ordering::SeqCst) {
            let _ = engine.stop_engine();
        }

        let result = engine.start_engine(config_json, None);

        match result {
            rustray::ffi::RayResult::Ok => {
                self.running.store(true, Ordering::SeqCst);
                info!("Config pushed and engine started");
                Ok(())
            }
            rustray::ffi::RayResult::AlreadyRunning => {
                self.running.store(true, Ordering::SeqCst);
                Ok(())
            }
            err => {
                error!("Failed to start engine with config: {:?}", err);
                Err(DriverError::Ffi(format!("{:?}", err)))
            }
        }
    }

    async fn pull_metrics(&self) -> Result<MetricsSnapshot, DriverError> {
        let engine = rustray::ffi::EngineManager::new();
        let stats_json = engine.get_stats_json();

        let stats: serde_json::Value =
            serde_json::from_str(&stats_json).map_err(DriverError::from)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Ok(MetricsSnapshot {
            bytes_uploaded: stats["bytes_uploaded"].as_u64().unwrap_or(0),
            bytes_downloaded: stats["bytes_downloaded"].as_u64().unwrap_or(0),
            upload_speed: 0.0,   // Calculate from delta
            download_speed: 0.0, // Calculate from delta
            active_connections: stats["active_connections"].as_u64().unwrap_or(0) as u32,
            total_connections: stats["total_connections"].as_u64().unwrap_or(0),
            latency_ms: None, // Not provided by basic stats
            connection_state: stats["connection_state"].as_u64().unwrap_or(0) as u8,
            timestamp,
        })
    }

    async fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    fn driver_type(&self) -> DriverType {
        DriverType::Local
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn update_core(&self, core_name: String) -> Result<String, DriverError> {
        info!("Updating core: {}", core_name);

        let engine = rustray::ffi::EngineManager::new();
        let result = engine.update_core(core_name);

        match result {
            Ok(version) => {
                info!("Core updated to version: {}", version);
                Ok(version)
            }
            Err(e) => {
                error!("Failed to update core: {}", e);
                Err(DriverError::Ffi(e.to_string()))
            }
        }
    }

    async fn pull_connection_metrics(
        &self,
        conn_id: &str,
    ) -> Result<Vec<ConnectionMetrics>, DriverError> {
        use rustray::app::stats::StatsManager;

        if let Some(stats_manager) = StatsManager::global() {
            let metrics = stats_manager.get_connection_metrics(conn_id);
            Ok(metrics
                .into_iter()
                .map(|m| ConnectionMetrics {
                    rtt_ms: m.rtt_ms,
                    cwnd_bytes: m.cwnd_bytes,
                    dpi_state: m.dpi_state.clone(),
                    timestamp: m.timestamp,
                })
                .collect())
        } else {
            Err(DriverError::NotRunning)
        }
    }
}
