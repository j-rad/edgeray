use async_trait::async_trait;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use tokio::sync::Mutex;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::desktop as native_impl;
#[cfg(target_os = "android")]
use crate::mobile::android as native_impl;
#[cfg(target_os = "ios")]
use crate::mobile::ios as native_impl;

/// VPN Service Trait
/// Abstracts platform-specific VPN implementation details.
#[async_trait]
pub trait VpnService: Send + Sync {
    async fn start(&self, config: &str) -> Result<(), String>;
    async fn stop(&self) -> Result<(), String>;
    fn is_running(&self) -> bool;
    fn protect_socket(&self, fd: i32) -> bool {
        // Default implementation returns true (no-op)
        // Override for Android/iOS
        let _ = fd;
        true
    }
}

#[derive(Default)]
pub struct VpnManager {
    service: Arc<Mutex<Option<Box<dyn VpnService>>>>,
}

impl VpnManager {
    pub fn new() -> Self {
        Self {
            service: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_vpn<R: Runtime>(
        &self,
        _app: &AppHandle<R>,
        config: &str,
    ) -> Result<(), String> {
        // Initialize platform-specific service if not already exists
        let mut service_guard = self.service.lock().await;

        if service_guard.is_none() {
            let service = native_impl::create_vpn_service(_app)?;
            *service_guard = Some(service);
        }

        if let Some(service) = service_guard.as_ref() {
            if service.is_running() {
                return Err("VPN is already running".to_string());
            }
            service.start(config).await
        } else {
            Err("Failed to initialize VPN service".to_string())
        }
    }

    pub async fn stop_vpn<R: Runtime>(&self, _app: &AppHandle<R>) -> Result<(), String> {
        let mut service_guard = self.service.lock().await;
        if let Some(service) = service_guard.as_ref() {
            service.stop().await?;
        }
        *service_guard = None; // cleanup
        Ok(())
    }
}
