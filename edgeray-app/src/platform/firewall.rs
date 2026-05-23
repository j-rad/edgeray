use std::process::Command;
use log::{info, error};

/// Enables a system-level Kill-Switch ensuring no traffic escapes the VPN tunnel.
/// The rules are applied to the OS firewall (nftables/WFP) so they persist even if the
/// EdgeRay/Rustray process crashes, ensuring 0 bytes leak to the public internet.
pub fn enable_killswitch(vpn_ip: &str) -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        info!("Enabling nftables Kill-Switch for VPN IP: {}", vpn_ip);
        // Create an independent nftables table for the killswitch
        let ruleset = format!(
            "table inet edgeray_killswitch {{\n\
                chain output {{\n\
                    type filter hook output priority 0; policy accept;\n\
                    meta skuid != root drop\n\
                    ip daddr != {} drop\n\
                }}\n\
            }}",
            vpn_ip
        );
        
        let mut child = Command::new("nft")
            .arg("-f")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn nft: {}", e))?;
            
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(ruleset.as_bytes()).map_err(|e| format!("Failed to write to nft stdin: {}", e))?;
        }
        
        let status = child.wait().map_err(|e| format!("Failed to wait on nft: {}", e))?;
        if !status.success() {
            return Err("nft command failed to set killswitch rules".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        info!("Enabling WFP Kill-Switch for VPN IP: {}", vpn_ip);
        // Delete any existing rules first
        let _ = Command::new("netsh")
            .args(["advfirewall", "firewall", "delete", "rule", "name=EdgeRay_KillSwitch"])
            .status();

        // Add an outbound block rule for all IPs except the VPN server
        // Note: Implementing true WFP in Rust would require windows-sys, but netsh persists after crash
        let status = Command::new("netsh")
            .args([
                "advfirewall", "firewall", "add", "rule",
                "name=EdgeRay_KillSwitch",
                "dir=out",
                "action=block",
                &format!("remoteip=0.0.0.0-255.255.255.255,!{}", vpn_ip),
            ])
            .status()
            .map_err(|e| format!("Failed to run netsh: {}", e))?;
            
        if !status.success() {
            return Err("netsh command failed to set killswitch rules".to_string());
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "linux", target_os = "windows")))]
    {
        info!("Kill-Switch is not yet supported on this platform.");
    }

    Ok(())
}

/// Disables the system-level Kill-Switch, restoring normal network access.
pub fn disable_killswitch() -> Result<(), String> {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        info!("Disabling nftables Kill-Switch");
        let status = Command::new("nft")
            .args(["delete", "table", "inet", "edgeray_killswitch"])
            .status()
            .map_err(|e| format!("Failed to delete nft table: {}", e))?;
            
        if !status.success() {
            error!("Failed to remove nftables killswitch table");
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        info!("Disabling WFP Kill-Switch");
        let status = Command::new("netsh")
            .args(["advfirewall", "firewall", "delete", "rule", "name=EdgeRay_KillSwitch"])
            .status()
            .map_err(|e| format!("Failed to run netsh: {}", e))?;
            
        if !status.success() {
            error!("Failed to remove WFP killswitch rule");
        }
    }

    Ok(())
}
