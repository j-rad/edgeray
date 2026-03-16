import NetworkExtension
import os.log

/// Logger for the Packet Tunnel Provider
private let logger = Logger(subsystem: "com.edgeray.PacketTunnel", category: "PacketTunnelProvider")

/**
 * EdgeRay Packet Tunnel Provider
 *
 * This Network Extension provides tunnel functionality on iOS by:
 * 1. Configuring network settings (tunnel IP, routes, DNS)
 * 2. Managing the packet tunnel lifecycle
 * 3. Passing packet flow to the Rust engine via UniFFI
 *
 * The Rust engine (edgeray-core) handles all actual networking:
 * - Processing IP packets through smoltcp userspace TCP stack
 * - Proxying connections through the configured tunnel server
 *
 * ## App Group Configuration
 * This extension requires an App Group to share configuration with the main app.
 * Configure in Xcode: Signing & Capabilities → App Groups → Add "group.com.edgeray.shared"
 */
class PacketTunnelProvider: NEPacketTunnelProvider {
    
    /// App Group identifier for sharing data with the main app
    private let appGroupIdentifier = "group.com.edgeray.shared"
    
    /// Configuration file name
    private let configFileName = "active_config.json"
    
    /// Rust tunnel handle (for stopping)
    private var tunnelRunning = false
    
    // MARK: - Tunnel Lifecycle
    
    override func startTunnel(options: [String : NSObject]?, completionHandler: @escaping (Error?) -> Void) {
        logger.info("Starting EdgeRay tunnel...")
        
        // 1. Load configuration
        guard let configJson = loadConfiguration() else {
            logger.error("Failed to load configuration")
            let error = NSError(domain: "com.edgeray.PacketTunnel", code: 1, userInfo: [NSLocalizedDescriptionKey: "Configuration is invalid"])
            completionHandler(error)
            return
        }
        
        logger.info("Configuration loaded successfully")
        
        // 2. Configure network settings
        let tunnelSettings = createTunnelSettings()
        
        setTunnelNetworkSettings(tunnelSettings) { [weak self] error in
            if let error = error {
                logger.error("Failed to set tunnel settings: \(error.localizedDescription)")
                completionHandler(error)
                return
            }
            
            logger.info("Tunnel settings configured successfully")
            
            // 3. Start the Rust engine
            self?.startRustEngine(configJson: configJson, completionHandler: completionHandler)
        }
    }
    
    override func stopTunnel(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        logger.info("Stopping EdgeRay tunnel, reason: \(String(describing: reason))")
        
        // Stop the Rust engine
        tunnelRunning = false
        
        do {
            let stopped = try stop_tunnel()
            logger.info("Rust tunnel stop result: \(stopped)")
        } catch {
            logger.warning("Error stopping Rust tunnel: \(error.localizedDescription)")
        }
        
        // Clear the configuration
        clearConfiguration()
        
        completionHandler()
    }
    
    // MARK: - Configuration Management
    
    /**
     * Load tunnel configuration from the shared App Group container
     */
    private func loadConfiguration() -> String? {
        // Try App Group first (preferred method)
        if let containerURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: appGroupIdentifier) {
            let configURL = containerURL.appendingPathComponent(configFileName)
            
            if FileManager.default.fileExists(atPath: configURL.path) {
                do {
                    let configJson = try String(contentsOf: configURL, encoding: .utf8)
                    logger.info("Loaded config from App Group: \(configURL.path)")
                    return configJson
                } catch {
                    logger.warning("Failed to read config from App Group: \(error.localizedDescription)")
                }
            }
        }
        
        // Try UserDefaults in App Group
        if let defaults = UserDefaults(suiteName: appGroupIdentifier) {
            if let configJson = defaults.string(forKey: "tunnel_config") {
                logger.info("Loaded config from UserDefaults")
                return configJson
            }
        }
        
        // Try options passed when starting the tunnel (for testing)
        // This can be set via NETunnelProviderProtocol.providerConfiguration
        if let protocolConfig = self.protocolConfiguration as? NETunnelProviderProtocol,
           let configData = protocolConfig.providerConfiguration?["config"] as? String {
            logger.info("Loaded config from provider configuration")
            return configData
        }
        
        logger.error("No configuration found in any location")
        return nil
    }
    
    /**
     * Clear the saved configuration after disconnect
     */
    private func clearConfiguration() {
        // Clear from App Group file
        if let containerURL = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: appGroupIdentifier) {
            let configURL = containerURL.appendingPathComponent(configFileName)
            try? FileManager.default.removeItem(at: configURL)
        }
        
        // Clear from UserDefaults
        if let defaults = UserDefaults(suiteName: appGroupIdentifier) {
            defaults.removeObject(forKey: "tunnel_config")
        }
        
        logger.info("Configuration cleared")
    }
    
    // MARK: - Network Settings
    
    /**
     * Create the tunnel network settings
     */
    private func createTunnelSettings() -> NEPacketTunnelNetworkSettings {
        // Remote address (can be any address, used for display)
        let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")
        
        // MTU size (must match edgeray-core)
        settings.mtu = 1500
        
        // IPv4 configuration
        let ipv4Settings = NEIPv4Settings(
            addresses: ["10.0.0.2"],
            subnetMasks: ["255.255.255.0"]
        )
        
        // Route all traffic through the tunnel
        ipv4Settings.includedRoutes = [NEIPv4Route.default()]
        
        // Exclude local network (optional, for split tunneling)
        // ipv4Settings.excludedRoutes = [
        //     NEIPv4Route(destinationAddress: "192.168.0.0", subnetMask: "255.255.0.0"),
        //     NEIPv4Route(destinationAddress: "10.0.0.0", subnetMask: "255.0.0.0"),
        //     NEIPv4Route(destinationAddress: "172.16.0.0", subnetMask: "255.240.0.0"),
        // ]
        
        settings.ipv4Settings = ipv4Settings
        
        // DNS configuration
        let dnsSettings = NEDNSSettings(servers: ["1.1.1.1", "8.8.8.8"])
        dnsSettings.matchDomains = [""] // Match all domains
        settings.dnsSettings = dnsSettings
        
        // Proxy settings (if needed)
        // let proxySettings = NEProxySettings()
        // settings.proxySettings = proxySettings
        
        return settings
    }
    
    // MARK: - Rust Engine Integration
    
    /**
     * Start the Rust tunnel engine
     *
     * Note: On iOS, we cannot get a raw file descriptor from NEPacketTunnelFlow.
     * Instead, we use the packetFlow property and a custom iOS-specific approach.
     *
     * Options:
     * 1. Use NEPacketTunnelFlow.readPackets/writePackets with a bridge
     * 2. Pass a placeholder FD and handle packets in Swift
     * 3. Use a shared memory/mmap approach
     *
     * For now, we'll use approach #2 with packet reading in Swift
     */
    private func startRustEngine(configJson: String, completionHandler: @escaping (Error?) -> Void) {
        tunnelRunning = true
        
        // Start packet reading loop
        startPacketLoop()
        
        // Start Rust engine in background
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            do {
                // On iOS, we pass -1 as FD to indicate packet bridging mode
                // The Rust side should detect this and use an alternative mechanism
                logger.info("Starting Rust tunnel...")
                
                try start_tunnel(fd: -1, configJson: configJson)
                
                logger.info("Rust tunnel completed")
            } catch {
                logger.error("Rust tunnel error: \(error.localizedDescription)")
                
                // Notify tunnel failure
                if self?.tunnelRunning == true {
                    self?.cancelTunnelWithError(error)
                }
            }
        }
        
        // Report success immediately (engine started asynchronously)
        completionHandler(nil)
    }
    
    /**
     * Start the packet reading loop from NEPacketTunnelFlow
     *
     * This reads packets from the system and would normally forward them to Rust.
     * For full functionality, implement a shared buffer or IPC mechanism.
     */
    private func startPacketLoop() {
        // Read packets from the packet flow
        packetFlow.readPackets { [weak self] packets, protocols in
            guard self?.tunnelRunning == true else { return }
            
            for (i, packet) in packets.enumerated() {
                let proto = protocols[i]
                logger.debug("Received packet: \(packet.count) bytes, protocol: \(proto)")
                
                // TODO: Forward packet to Rust engine
                // This would require:
                // 1. A shared buffer mechanism
                // 2. Or calling a Rust FFI function to feed packets
                // Example: try? feed_packet_to_engine(packet)
            }
            
            // Continue reading
            self?.startPacketLoop()
        }
    }
    
    // MARK: - App Messages
    
    override func handleAppMessage(_ messageData: Data, completionHandler: ((Data?) -> Void)?) {
        logger.info("Received app message: \(messageData.count) bytes")
        
        // Parse the message
        if let message = String(data: messageData, encoding: .utf8) {
            switch message {
            case "status":
                // Return current status
                let status = tunnelRunning ? "connected" : "disconnected"
                completionHandler?(status.data(using: .utf8))
                
            case "stats":
                // Return statistics (placeholder)
                let stats = """
                {"upload": 0, "download": 0, "latency": 0}
                """
                completionHandler?(stats.data(using: .utf8))
                
            default:
                // Check if it's a new configuration
                if message.hasPrefix("{") {
                    logger.info("Received new configuration via app message")
                    // Could restart tunnel with new config here
                }
                completionHandler?(nil)
            }
        } else {
            completionHandler?(nil)
        }
    }
    
    // MARK: - Sleep/Wake
    
    override func sleep(completionHandler: @escaping () -> Void) {
        logger.info("Device entering sleep mode")
        // Optionally pause network activity
        completionHandler()
    }
    
    override func wake() {
        logger.info("Device waking from sleep")
        // Resume network activity if paused
    }
}
