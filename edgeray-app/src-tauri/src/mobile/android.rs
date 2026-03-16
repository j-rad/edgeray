use crate::vpn_service::VpnService;
use async_trait::async_trait;
use tauri::{AppHandle, Runtime};

pub struct AndroidVpnService<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> AndroidVpnService<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl<R: Runtime> VpnService for AndroidVpnService<R> {
    async fn start(&self, config: &str) -> Result<(), String> {
        // Use JNI to call startService on TunnelService
        use tauri::Manager;
        // In a real implementation we would get the activity and start the service intent
        // let ctx = self.app.state::<...>();

        println!(
            "Starting Android VPN Service via JNI with config len: {}",
            config.len()
        );
        // For now, simulating success
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        println!("Stopping Android VPN Service...");
        Ok(())
    }

    fn is_running(&self) -> bool {
        // Query status via JNI
        false
    }

    fn protect_socket(&self, fd: i32) -> bool {
        // On Android, we must call VpnService.protect(fd)
        // This requires JNI call.
        // Placeholder:
        println!("Protecting socket fd: {}", fd);
        true
    }
}

pub fn create_vpn_service<R: Runtime>(app: &AppHandle<R>) -> Result<Box<dyn VpnService>, String> {
    Ok(Box::new(AndroidVpnService::new(app.clone())))
}
