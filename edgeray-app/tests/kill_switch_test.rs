//! Kill-switch integration tests
//!
//! Verifies that the AtomicBool kill-switch prevents network leaks
//! when the VPN connection drops.

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    /// Test that kill-switch prevents leaks on VPN disconnect
    #[test]
    #[ignore] // Requires running VPN and root privileges
    fn test_killswitch_prevents_leaks() {
        // Get baseline connections
        let baseline = get_connection_count();
        println!("Baseline connections: {}", baseline);

        // Verify VPN is running
        assert!(is_vpn_running(), "VPN must be running for this test");

        // Attempt connection through VPN
        let vpn_result = test_connection("https://www.google.com");
        assert!(vpn_result.is_ok(), "Connection through VPN should succeed");

        // Kill VPN process
        kill_vpn();
        thread::sleep(Duration::from_secs(2));

        // Attempt connection without VPN (should fail due to kill-switch)
        let no_vpn_result = test_connection("https://www.google.com");
        assert!(
            no_vpn_result.is_err(),
            "Kill-switch should block connection when VPN is down"
        );

        // Verify no new connections leaked
        let final_count = get_connection_count();
        let leaked = final_count.saturating_sub(baseline);
        assert!(
            leaked <= 2,
            "Kill-switch failed: {} connections leaked",
            leaked
        );

        println!(
            "✓ Kill-switch test passed: {} connections (Δ {})",
            final_count, leaked
        );
    }

    /// Test VPN reconnection after kill-switch activation
    #[test]
    #[ignore] // Requires running VPN and root privileges
    fn test_killswitch_recovery() {
        // Kill VPN
        kill_vpn();
        thread::sleep(Duration::from_secs(2));

        // Verify kill-switch is active
        let blocked_result = test_connection("https://www.google.com");
        assert!(blocked_result.is_err(), "Kill-switch should be active");

        // Restart VPN
        start_vpn();
        thread::sleep(Duration::from_secs(5));

        // Verify connection works again
        let recovered_result = test_connection("https://www.google.com");
        assert!(
            recovered_result.is_ok(),
            "Connection should work after VPN restart"
        );

        println!("✓ Kill-switch recovery test passed");
    }

    /// Test firewall rule cleanup on VPN stop
    #[test]
    #[ignore] // Requires root privileges
    fn test_firewall_cleanup() {
        // Get baseline firewall rules
        let baseline_rules = get_firewall_rule_count();
        println!("Baseline firewall rules: {}", baseline_rules);

        // Start VPN (should add rules)
        start_vpn();
        thread::sleep(Duration::from_secs(3));

        let active_rules = get_firewall_rule_count();
        assert!(
            active_rules > baseline_rules,
            "VPN should add firewall rules"
        );

        // Stop VPN (should clean up rules)
        kill_vpn();
        thread::sleep(Duration::from_secs(3));

        let final_rules = get_firewall_rule_count();
        assert_eq!(
            final_rules, baseline_rules,
            "Firewall rules should be cleaned up on VPN stop"
        );

        println!("✓ Firewall cleanup test passed");
    }

    // Helper functions

    fn get_connection_count() -> usize {
        let output = Command::new("netstat")
            .args(["-tn"])
            .output()
            .expect("Failed to run netstat");

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| line.contains("ESTABLISHED"))
            .count()
    }

    fn get_firewall_rule_count() -> usize {
        let output = Command::new("iptables")
            .args(["-L", "-n"])
            .output()
            .expect("Failed to run iptables");

        String::from_utf8_lossy(&output.stdout).lines().count()
    }

    fn is_vpn_running() -> bool {
        Command::new("pgrep")
            .arg("rustray")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn kill_vpn() {
        let _ = Command::new("pkill").args(["-TERM", "rustray"]).status();
    }

    fn start_vpn() {
        // This should be customized based on your setup
        let _ = Command::new("rustray")
            .arg("--config")
            .arg("/etc/edgeray/config.json")
            .spawn();
    }

    fn test_connection(url: &str) -> Result<(), String> {
        let output = Command::new("curl")
            .args([
                "-s",
                "-o",
                "/dev/null",
                "-w",
                "%{http_code}",
                "--max-time",
                "5",
                url,
            ])
            .output()
            .map_err(|e| e.to_string())?;

        let status_code = String::from_utf8_lossy(&output.stdout);
        if status_code.contains("200") {
            Ok(())
        } else {
            Err(format!("Connection failed with status: {}", status_code))
        }
    }
}
