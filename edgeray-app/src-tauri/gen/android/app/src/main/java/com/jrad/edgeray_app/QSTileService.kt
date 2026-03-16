package com.jrad.edgeray_app

import android.content.Intent
import android.graphics.drawable.Icon
import android.os.Build
import android.service.quicksettings.Tile
import android.service.quicksettings.TileService
import android.util.Log
import androidx.annotation.RequiresApi
import org.json.JSONObject
import java.io.File
import java.text.DecimalFormat

/**
 * Quick Settings Tile for EdgeRay VPN
 * 
 * Provides one-tap VPN toggle directly from the notification shade.
 * Displays real-time bandwidth metrics from the rustray engine via JNI.
 */
@RequiresApi(Build.VERSION_CODES.N)
class EdgeRayQSTile : TileService() {
    
    companion object {
        private const val TAG = "EdgeRayQSTile"
        internal var instance: EdgeRayQSTile? = null
        
        /**
         * Update the tile state from external components (e.g., VPN service)
         */
        fun updateTileState(isConnected: Boolean) {
            instance?.refreshTile()
        }
    }
    
    // Native method for getting metrics - implemented in Rust via JNI
    private external fun nativeGetMetricsJson(): String
    
    init {
        try {
            System.loadLibrary("rustray")
        } catch (e: UnsatisfiedLinkError) {
            Log.w(TAG, "Native library not available for QS Tile metrics")
        }
    }
    
    override fun onCreate() {
        super.onCreate()
        instance = this
    }
    
    override fun onDestroy() {
        super.onDestroy()
        instance = null
    }
    
    override fun onStartListening() {
        super.onStartListening()
        refreshTile()
    }
    
    override fun onClick() {
        super.onClick()
        
        try {
            val isConnected = EdgeRayVpnService.isServiceRunning()
            
            if (isConnected) {
                // Disconnect - send intent to VPN service
                val disconnectIntent = Intent(this, EdgeRayVpnService::class.java).apply {
                    action = EdgeRayVpnService.ACTION_DISCONNECT
                }
                startService(disconnectIntent)
                updateTileUi(false, "Disconnecting...")
            } else {
                // Connect - need VPN permission first
                val lastConfig = loadLastActiveConfig()
                if (lastConfig != null) {
                    val connectIntent = Intent(this, EdgeRayVpnService::class.java).apply {
                        action = EdgeRayVpnService.ACTION_CONNECT
                        putExtra(EdgeRayVpnService.EXTRA_CONFIG_JSON, lastConfig)
                    }
                    startService(connectIntent)
                    updateTileUi(true, "Connecting...")
                } else {
                    // No config available, open the app
                    updateTileUi(false, "Tap to configure")
                    unlockAndRun {
                        val intent = packageManager.getLaunchIntentForPackage(packageName)
                        intent?.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                        startActivity(intent)
                    }
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error toggling VPN", e)
            updateTileUi(false, "Error")
        }
    }
    
    private fun refreshTile() {
        try {
            val isConnected = EdgeRayVpnService.isServiceRunning()
            
            if (isConnected) {
                // Fetch real metrics from Rust
                val subtitle = try {
                    val metricsJson = nativeGetMetricsJson()
                    val metrics = JSONObject(metricsJson)
                    val bytesUp = metrics.optLong("bytes_uploaded", 0)
                    val bytesDown = metrics.optLong("bytes_downloaded", 0)
                    formatBandwidth(bytesUp, bytesDown)
                } catch (e: UnsatisfiedLinkError) {
                    // Fallback to DirectBuffer stats if JNI not available
                    val stats = EdgeRayVpnService.getStats()
                    formatBandwidth(stats.bytesUploaded, stats.bytesDownloaded)
                } catch (e: Exception) {
                    Log.w(TAG, "Failed to get metrics: ${e.message}")
                    "Connected"
                }
                updateTileUi(true, subtitle)
            } else {
                updateTileUi(false, "Tap to connect")
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error refreshing tile", e)
            updateTileUi(false, "Error")
        }
    }
    
    private fun updateTileUi(isConnected: Boolean, subtitle: String) {
        qsTile?.apply {
            state = if (isConnected) Tile.STATE_ACTIVE else Tile.STATE_INACTIVE
            label = "EdgeRay"
            
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                this.subtitle = subtitle
            }
            
            // Update icon based on state
            icon = Icon.createWithResource(
                applicationContext,
                if (isConnected) {
                    android.R.drawable.presence_online
                } else {
                    android.R.drawable.presence_invisible
                }
            )
            
            updateTile()
        }
    }
    
    private fun formatBandwidth(bytesUp: Long, bytesDown: Long): String {
        val formatter = DecimalFormat("#.##")
        val upStr = formatBytes(bytesUp, formatter)
        val downStr = formatBytes(bytesDown, formatter)
        return "↑$upStr ↓$downStr"
    }
    
    private fun formatBytes(bytes: Long, formatter: DecimalFormat): String {
        return when {
            bytes >= 1_073_741_824 -> "${formatter.format(bytes / 1_073_741_824.0)}GB"
            bytes >= 1_048_576 -> "${formatter.format(bytes / 1_048_576.0)}MB"
            bytes >= 1024 -> "${formatter.format(bytes / 1024.0)}KB"
            else -> "${bytes}B"
        }
    }
    
    private fun loadLastActiveConfig(): String? {
        return try {
            val dataDir = applicationContext.getExternalFilesDir(null) 
                ?: applicationContext.filesDir
            val configFile = File(dataDir, "active_config.json")
            if (configFile.exists()) configFile.readText() else null
        } catch (e: Exception) {
            Log.w(TAG, "Failed to load config: ${e.message}")
            null
        }
    }
}
