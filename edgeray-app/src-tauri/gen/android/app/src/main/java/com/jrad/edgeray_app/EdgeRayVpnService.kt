package com.jrad.edgeray_app

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.VpnService
import android.os.Build
import android.os.ParcelFileDescriptor
import android.util.Log
import androidx.core.app.NotificationCompat
import org.json.JSONObject
import java.io.File
import java.nio.ByteBuffer
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicReference

/**
 * EdgeRay VPN Service
 * 
 * Full system VPN service that integrates with the rustray engine.
 * Passes TUN file descriptor directly to Rust via JNI for zero-copy packet processing.
 * Uses DirectBuffers for traffic stats to minimize GC pressure.
 */
class EdgeRayVpnService : VpnService() {
    
    companion object {
        private const val TAG = "EdgeRayVpnService"
        private const val NOTIFICATION_CHANNEL_ID = "edgeray_vpn_channel"
        private const val NOTIFICATION_ID = 1001
        
        const val ACTION_CONNECT = "com.jrad.edgeray_app.ACTION_CONNECT"
        const val ACTION_DISCONNECT = "com.jrad.edgeray_app.ACTION_DISCONNECT"
        const val ACTION_STATUS_UPDATE = "com.jrad.edgeray_app.ACTION_STATUS_UPDATE"
        
        const val EXTRA_CONFIG_JSON = "config_json"
        const val EXTRA_IS_CONNECTED = "is_connected"
        const val EXTRA_BYTES_UP = "bytes_up"
        const val EXTRA_BYTES_DOWN = "bytes_down"
        
        private val isRunning = AtomicBoolean(false)
        private val currentInstance = AtomicReference<EdgeRayVpnService?>(null)
        
        /**
         * DirectBuffer for stats exchange with Rust - avoids GC
         * Layout: [8 bytes upload] [8 bytes download] [8 bytes active_conns] [8 bytes state]
         */
        private val statsBuffer: ByteBuffer = ByteBuffer.allocateDirect(32)
        
        fun isServiceRunning(): Boolean = isRunning.get()
        
        fun getInstance(): EdgeRayVpnService? = currentInstance.get()
        
        /**
         * Get current stats from the DirectBuffer (called from UI thread)
         */
        fun getStats(): VpnStats {
            synchronized(statsBuffer) {
                statsBuffer.rewind()
                return VpnStats(
                    bytesUploaded = statsBuffer.getLong(),
                    bytesDownloaded = statsBuffer.getLong(),
                    activeConnections = statsBuffer.getLong().toInt(),
                    connectionState = statsBuffer.getLong().toInt()
                )
            }
        }
    }
    
    // Native methods - implemented in Rust via JNI
    private external fun nativeStartVpnWithFd(fd: Int, configJson: String): Int
    private external fun nativeStopVpn(): Int
    private external fun nativeGetMetricsJson(): String
    private external fun nativeProtectSocket(fd: Int): Boolean
    
    init {
        try {
            System.loadLibrary("rustray")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Failed to load rustray native library", e)
        }
    }
    
    private var vpnInterface: ParcelFileDescriptor? = null
    private var statsUpdateThread: Thread? = null
    private val statsThreadRunning = AtomicBoolean(false)
    
    private val commandReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            when (intent?.action) {
                ACTION_CONNECT -> {
                    val configJson = intent.getStringExtra(EXTRA_CONFIG_JSON) ?: "{}"
                    startVpnConnection(configJson)
                }
                ACTION_DISCONNECT -> {
                    stopVpnConnection()
                }
            }
        }
    }
    
    override fun onCreate() {
        super.onCreate()
        currentInstance.set(this)
        createNotificationChannel()
        
        // Register broadcast receiver for commands
        val filter = IntentFilter().apply {
            addAction(ACTION_CONNECT)
            addAction(ACTION_DISCONNECT)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            registerReceiver(commandReceiver, filter, RECEIVER_NOT_EXPORTED)
        } else {
            @Suppress("UnspecifiedRegisterReceiverFlag")
            registerReceiver(commandReceiver, filter)
        }
        
        Log.i(TAG, "EdgeRayVpnService created")
    }
    
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_CONNECT -> {
                val configJson = intent.getStringExtra(EXTRA_CONFIG_JSON) ?: "{}"
                startVpnConnection(configJson)
            }
            ACTION_DISCONNECT -> {
                stopVpnConnection()
            }
            else -> {
                // Service started without explicit action, try to restore last session
                val lastConfig = loadLastConfig()
                if (lastConfig != null && !isRunning.get()) {
                    startVpnConnection(lastConfig)
                }
            }
        }
        return START_STICKY
    }
    
    private fun startVpnConnection(configJson: String) {
        if (isRunning.get()) {
            Log.w(TAG, "VPN already running, ignoring connect request")
            return
        }
        
        Log.i(TAG, "Starting VPN connection...")
        
        try {
            // 1. Build and establish VPN interface
            val builder = Builder()
                .setSession("EdgeRay VPN")
                .setMtu(1500)
                .addAddress("10.0.0.2", 24)
                .addRoute("0.0.0.0", 0)
                .addDnsServer("1.1.1.1")
                .addDnsServer("8.8.8.8")
            
            // Parse config for per-app settings if present
            try {
                val config = JSONObject(configJson)
                val excludedApps = config.optJSONArray("excluded_apps")
                excludedApps?.let { apps ->
                    for (i in 0 until apps.length()) {
                        val pkg = apps.optString(i)
                        if (pkg.isNotEmpty()) {
                            try {
                                builder.addDisallowedApplication(pkg)
                                Log.d(TAG, "Excluded app: $pkg")
                            } catch (e: Exception) {
                                Log.w(TAG, "Failed to exclude app $pkg: ${e.message}")
                            }
                        }
                    }
                }
                
                val includedApps = config.optJSONArray("included_apps")
                includedApps?.let { apps ->
                    for (i in 0 until apps.length()) {
                        val pkg = apps.optString(i)
                        if (pkg.isNotEmpty()) {
                            try {
                                builder.addAllowedApplication(pkg)
                                Log.d(TAG, "Included app: $pkg")
                            } catch (e: Exception) {
                                Log.w(TAG, "Failed to include app $pkg: ${e.message}")
                            }
                        }
                    }
                }
            } catch (e: Exception) {
                Log.w(TAG, "Failed to parse config JSON for per-app settings: ${e.message}")
            }
            
            // 2. Establish the VPN interface and get FD
            vpnInterface = builder.establish()
                ?: throw SecurityException("VPN permission not granted or interface unavailable")
            
            val fd = vpnInterface!!.fd
            Log.i(TAG, "VPN interface established. FD: $fd")
            
            // 3. Pass FD to Rust engine
            val result = nativeStartVpnWithFd(fd, configJson)
            if (result != 0) {
                throw RuntimeException("Native engine failed to start: error code $result")
            }
            
            isRunning.set(true)
            saveLastConfig(configJson)
            
            // 4. Start foreground service with notification
            startForeground(NOTIFICATION_ID, createNotification("Connected"))
            
            // 5. Start stats polling thread
            startStatsPolling()
            
            // 6. Broadcast status update
            sendStatusBroadcast(true)
            
            // 7. Update QS Tile
            EdgeRayQSTile.updateTileState(true)
            
            Log.i(TAG, "VPN connection established successfully")
            
        } catch (e: SecurityException) {
            Log.e(TAG, "VPN permission error: ${e.message}", e)
            handleError("Permission denied: ${e.message}")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to start VPN: ${e.message}", e)
            handleError("Connection failed: ${e.message}")
        }
    }
    
    private fun stopVpnConnection() {
        if (!isRunning.get()) {
            Log.w(TAG, "VPN not running, ignoring disconnect request")
            return
        }
        
        Log.i(TAG, "Stopping VPN connection...")
        
        try {
            // 1. Stop stats polling
            stopStatsPolling()
            
            // 2. Stop native engine
            val result = nativeStopVpn()
            if (result != 0) {
                Log.w(TAG, "Native engine stop returned non-zero: $result")
            }
            
            // 3. Close VPN interface
            vpnInterface?.close()
            vpnInterface = null
            
            isRunning.set(false)
            
            // 4. Update notification
            stopForeground(STOP_FOREGROUND_REMOVE)
            
            // 5. Broadcast status update
            sendStatusBroadcast(false)
            
            // 6. Update QS Tile
            EdgeRayQSTile.updateTileState(false)
            
            Log.i(TAG, "VPN connection stopped successfully")
            
        } catch (e: Exception) {
            Log.e(TAG, "Error stopping VPN: ${e.message}", e)
        }
        
        stopSelf()
    }
    
    private fun startStatsPolling() {
        statsThreadRunning.set(true)
        statsUpdateThread = Thread {
            while (statsThreadRunning.get()) {
                try {
                    // Fetch metrics from Rust
                    val metricsJson = nativeGetMetricsJson()
                    val metrics = JSONObject(metricsJson)
                    
                    val bytesUp = metrics.optLong("bytes_uploaded", 0)
                    val bytesDown = metrics.optLong("bytes_downloaded", 0)
                    val activeConns = metrics.optLong("active_connections", 0)
                    val state = metrics.optLong("connection_state", 0)
                    
                    // Update DirectBuffer
                    synchronized(statsBuffer) {
                        statsBuffer.rewind()
                        statsBuffer.putLong(bytesUp)
                        statsBuffer.putLong(bytesDown)
                        statsBuffer.putLong(activeConns)
                        statsBuffer.putLong(state)
                    }
                    
                    // Sleep for 1 second
                    Thread.sleep(1000)
                } catch (e: InterruptedException) {
                    break
                } catch (e: Exception) {
                    Log.w(TAG, "Stats polling error: ${e.message}")
                }
            }
        }.apply {
            isDaemon = true
            name = "EdgeRayStatsPoller"
            start()
        }
    }
    
    private fun stopStatsPolling() {
        statsThreadRunning.set(false)
        statsUpdateThread?.interrupt()
        statsUpdateThread = null
    }
    
    /**
     * Called by the Rust engine when a socket needs protection (bypass VPN)
     */
    fun protectSocket(fd: Int): Boolean {
        return try {
            protect(fd)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to protect socket $fd: ${e.message}")
            false
        }
    }
    
    private fun sendStatusBroadcast(isConnected: Boolean) {
        val stats = getStats()
        val intent = Intent(ACTION_STATUS_UPDATE).apply {
            putExtra(EXTRA_IS_CONNECTED, isConnected)
            putExtra(EXTRA_BYTES_UP, stats.bytesUploaded)
            putExtra(EXTRA_BYTES_DOWN, stats.bytesDownloaded)
            setPackage(packageName)
        }
        sendBroadcast(intent)
        
        // Also save state to file for QS Tile access
        saveConnectionState(isConnected)
    }
    
    private fun handleError(message: String) {
        Log.e(TAG, "VPN Error: $message")
        vpnInterface?.close()
        vpnInterface = null
        isRunning.set(false)
        sendStatusBroadcast(false)
        EdgeRayQSTile.updateTileState(false)
    }
    
    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                NOTIFICATION_CHANNEL_ID,
                "EdgeRay VPN",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "VPN connection status"
                setShowBadge(false)
            }
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(channel)
        }
    }
    
    private fun createNotification(status: String): Notification {
        val disconnectIntent = Intent(this, EdgeRayVpnService::class.java).apply {
            action = ACTION_DISCONNECT
        }
        val disconnectPendingIntent = PendingIntent.getService(
            this, 0, disconnectIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        
        val openAppIntent = packageManager.getLaunchIntentForPackage(packageName)
        val openAppPendingIntent = openAppIntent?.let {
            PendingIntent.getActivity(
                this, 0, it,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            )
        }
        
        return NotificationCompat.Builder(this, NOTIFICATION_CHANNEL_ID)
            .setSmallIcon(android.R.drawable.presence_online)
            .setContentTitle("EdgeRay VPN")
            .setContentText(status)
            .setOngoing(true)
            .setContentIntent(openAppPendingIntent)
            .addAction(android.R.drawable.ic_menu_close_clear_cancel, "Disconnect", disconnectPendingIntent)
            .setPriority(NotificationCompat.PRIORITY_LOW)
            .build()
    }
    
    private fun saveLastConfig(configJson: String) {
        try {
            val dataDir = getExternalFilesDir(null) ?: filesDir
            File(dataDir, "active_config.json").writeText(configJson)
        } catch (e: Exception) {
            Log.w(TAG, "Failed to save last config: ${e.message}")
        }
    }
    
    private fun loadLastConfig(): String? {
        return try {
            val dataDir = getExternalFilesDir(null) ?: filesDir
            val file = File(dataDir, "active_config.json")
            if (file.exists()) file.readText() else null
        } catch (e: Exception) {
            Log.w(TAG, "Failed to load last config: ${e.message}")
            null
        }
    }
    
    private fun saveConnectionState(isConnected: Boolean) {
        try {
            val dataDir = getExternalFilesDir(null) ?: filesDir
            val stateFile = File(dataDir, "connection_state.json")
            val json = JSONObject().apply {
                put("is_connected", isConnected)
                put("timestamp", System.currentTimeMillis())
            }
            stateFile.writeText(json.toString())
        } catch (e: Exception) {
            Log.w(TAG, "Failed to save connection state: ${e.message}")
        }
    }
    
    override fun onRevoke() {
        Log.i(TAG, "VPN permission revoked by system")
        stopVpnConnection()
        super.onRevoke()
    }
    
    override fun onDestroy() {
        Log.i(TAG, "EdgeRayVpnService destroyed")
        unregisterReceiver(commandReceiver)
        stopVpnConnection()
        currentInstance.set(null)
        super.onDestroy()
    }
}

/**
 * Data class for VPN statistics
 */
data class VpnStats(
    val bytesUploaded: Long,
    val bytesDownloaded: Long,
    val activeConnections: Int,
    val connectionState: Int
)
