package com.jrad.edgeray_app

import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import java.lang.ref.WeakReference

/**
 * MainActivity - EdgeRay main entry point
 * 
 * Handles deep links (edgeray://, vless://, vmess://, trojan://, ss://)
 * and provides Tauri commands for Android-specific functionality.
 */
class MainActivity : TauriActivity() {
    private val scope = CoroutineScope(Dispatchers.Main + Job())
    private lateinit var appListProvider: AppListProvider
    
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)
        
        // Initialize app list provider
        appListProvider = AppListProvider(this)
        
        // Handle deep link if launched from one
        handleDeepLink(intent)
    }
    
    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        // Handle deep link when app is already running
        handleDeepLink(intent)
    }
    
    /**
     * Handle deep link URIs
     * 
     * Supported schemes:
     * - edgeray://
     * - vless://
     * - vmess://
     * - trojan://
     * - ss:// (Shadowsocks)
     */
    private fun handleDeepLink(intent: Intent) {
        val data: Uri? = intent.data
        
        if (data != null) {
            val scheme = data.scheme
            val uri = data.toString()
            
            android.util.Log.d("MainActivity", "Deep link received: $uri")
            
            when (scheme) {
                "edgeray", "vless", "vmess", "trojan", "ss" -> {
                    // Send deep link to Dioxus UI via Tauri event
                    scope.launch {
                        try {
                            // Emit event to frontend
                            val payload = JSObject().apply {
                                put("uri", uri)
                                put("scheme", scheme)
                            }
                            
                            val plugin = AndroidPlugin.instance?.get()
                            if (plugin != null) {
                                plugin.trigger("deep-link", payload)
                                android.util.Log.d("MainActivity", "Emitted deep-link event: $uri")
                            } else {
                                android.util.Log.w("MainActivity", "AndroidPlugin instance not available, cannot emit deep-link")
                            }
                        } catch (e: Exception) {
                            android.util.Log.e("MainActivity", "Error handling deep link", e)
                        }
                    }
                }
            }
        }
    }
    
    /**
     * Broadcast connection state change to Quick Settings Tile
     */
    fun broadcastConnectionState(isConnected: Boolean) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            EdgeRayQSTile.updateTileState(isConnected)
        }
    }
}

/**
 * Tauri Plugin for Android-specific commands
 */
@TauriPlugin
class AndroidPlugin(private val activity: MainActivity) : Plugin(activity) {
    companion object {
        var instance: WeakReference<AndroidPlugin>? = null
    }

    init {
        instance = WeakReference(this)
    }
// ... (skip lines)
    /**
     * Update Quick Settings Tile state
     * 
     * @param isConnected Connection state
     */
    @Command
    fun updateQSTile(invoke: app.tauri.plugin.Invoke) {
        val args = invoke.parseArgs(UpdateQSTileArgs::class.java)
        
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            EdgeRayQSTile.updateTileState(args.isConnected)
            invoke.resolve()
        } else {
            invoke.reject("Quick Settings Tiles not supported on this Android version")
        }
    }

    /**
     * Request to ignore battery optimizations
     */
    @Command
    fun requestBatteryOptimizationIgnore(invoke: app.tauri.plugin.Invoke) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            try {
                val intent = android.content.Intent().apply {
                    action = android.provider.Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS
                    data = android.net.Uri.parse("package:${activity.packageName}")
                }
                activity.startActivity(intent)
                invoke.resolve()
            } catch (e: Exception) {
                android.util.Log.e("AndroidPlugin", "Failed to request battery optimization ignore", e)
                invoke.reject(e.message)
            }
        } else {
            invoke.resolve() // Not needed on older versions
        }
    }
    
    /**
     * Arguments for getInstalledApps command
     */
    @InvokeArg
    class GetInstalledAppsArgs {
        var includeSystemApps: Boolean? = null
        var includeIcons: Boolean? = null
    }
    
    /**
     * Arguments for getAppsByPackages command
     */
    @InvokeArg
    class GetAppsByPackagesArgs {
        lateinit var packageNames: List<String>
        var includeIcons: Boolean? = null
    }
    
    /**
     * Arguments for updateQSTile command
     */
    @InvokeArg
    class UpdateQSTileArgs {
        var isConnected: Boolean = false
    }
}

