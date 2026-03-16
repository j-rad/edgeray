use super::vpn_service::VpnService;
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Runtime};

pub struct DesktopVpnService {
    running: AtomicBool,
}

impl DesktopVpnService {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl VpnService for DesktopVpnService {
    async fn start(&self, config: &str) -> Result<(), String> {
        // Logic to start rustray engine on desktop
        // Check for root/admin privileges
        #[cfg(unix)]
        {
            if unsafe { libc::geteuid() } != 0 {
                return Err("Root privileges required. Please run as sudo/admin.".to_string());
            }
        }

        // Call rustray::ffi::EngineManager::start_engine
        let config_str = config.to_string();
        let _result = tauri::async_runtime::spawn_blocking(move || {
            let engine = rustray::ffi::EngineManager::new();
            let res = engine.start_engine(config_str, None);
            let s = format!("{:?}", res);
            if s.to_lowercase().contains("ok") || s.to_lowercase().contains("success") {
                Ok(())
            } else {
                Err(s)
            }
        })
        .await
        .map_err(|e| e.to_string())??;

        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        let _result = tauri::async_runtime::spawn_blocking(|| {
            let engine = rustray::ffi::EngineManager::new();
            let res = engine.stop_engine();
            let s = format!("{:?}", res);
            if !s.to_lowercase().contains("ok") && !s.to_lowercase().contains("success") {
                return Err(s);
            }
            Ok(())
        })
        .await
        .map_err(|e| e.to_string())??;

        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

pub fn create_vpn_service<R: Runtime>(_app: &AppHandle<R>) -> Result<Box<dyn VpnService>, String> {
    Ok(Box::new(DesktopVpnService::new()))
}
