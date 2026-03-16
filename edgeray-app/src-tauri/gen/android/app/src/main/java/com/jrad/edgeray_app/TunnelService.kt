package com.jrad.edgeray_app

import android.content.Intent
import android.net.VpnService
import android.os.ParcelFileDescriptor
import android.util.Log

class TunnelService : VpnService() {
    companion object {
        const val TAG = "TunnelService"
        const val ACTION_CONNECT = "com.jrad.edgeray_app.CONNECT"
        const val ACTION_DISCONNECT = "com.jrad.edgeray_app.DISCONNECT"
    }

    private var vpnInterface: ParcelFileDescriptor? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_CONNECT -> {
                val configJson = intent.getStringExtra("config")
                connect(configJson)
            }
            ACTION_DISCONNECT -> {
                disconnect()
            }
        }
        return START_NOT_STICKY
    }

    private fun connect(configJson: String?) {
        Log.i(TAG, "Connecting VPN...")
        // 1. Configure VPN interface
        val builder = Builder()
        builder.addAddress("10.0.0.2", 24)
        builder.addRoute("0.0.0.0", 0)
        builder.setSession("EdgeRay")
        builder.setMtu(1500)
        
        // 2. Establish interface
        vpnInterface = builder.establish()
        
        if (vpnInterface != null) {
            val fd = vpnInterface!!.fd
            Log.i(TAG, "VPN Interface established. FD: $fd")
            
            // 3. Pass FD to Rust core via JNI
            // In a real implementation: RustRayCore.start(fd, configJson)
            // For now, we mock it or assume the lib.rs mobile implementation picks it up if we pass it back?
            // Usually, we start a thread here that calls into Rust.
        } else {
            Log.e(TAG, "Failed to establish VPN interface")
        }
    }

    private fun disconnect() {
        Log.i(TAG, "Disconnecting VPN...")
        vpnInterface?.close()
        vpnInterface = null
        stopSelf()
    }

    override fun onDestroy() {
        super.onDestroy()
        disconnect()
    }
}
