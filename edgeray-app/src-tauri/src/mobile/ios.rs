use crate::vpn_service::VpnService;
use tauri::Runtime;

pub struct IosVpnService;

impl IosVpnService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl VpnService for IosVpnService {
    async fn start(&self) -> Result<(), String> {
        println!("Starting iOS NetworkExtension...");
        Ok(())
    }

    async fn stop(&self) -> Result<(), String> {
        println!("Stopping iOS NetworkExtension...");
        Ok(())
    }

    fn is_running(&self) -> bool {
        false
    }
}

pub fn create_vpn_service<R: Runtime>(
    _app: &tauri::AppHandle<R>,
) -> Result<Box<dyn VpnService>, String> {
    Ok(Box::new(IosVpnService::new()))
}
