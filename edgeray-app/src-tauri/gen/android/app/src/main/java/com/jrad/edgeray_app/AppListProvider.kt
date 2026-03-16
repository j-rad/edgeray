package com.jrad.edgeray_app

import android.content.Context
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.util.Base64
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject
import java.io.ByteArrayOutputStream

/**
 * AppListProvider - High-performance Android app list fetcher
 * 
 * Provides installed application information for per-app proxy configuration.
 * Optimized for handling 200+ apps with icon caching and virtualization support.
 */
class AppListProvider(private val context: Context) {
    
    companion object {
        private const val TAG = "AppListProvider"
        private const val ICON_SIZE = 128 // Icon size in pixels
        private const val ICON_QUALITY = 85 // JPEG quality (0-100)
    }
    
    /**
     * Data class representing an installed application
     */
    data class AppInfo(
        val packageName: String,
        val appName: String,
        val uid: Int,
        val isSystemApp: Boolean,
        val iconBase64: String?
    )
    
    /**
     * Fetch all installed applications
     * 
     * @param includeSystemApps Whether to include system applications
     * @param includeIcons Whether to include app icons (base64 encoded)
     * @return List of AppInfo objects
     */
    suspend fun getInstalledApps(
        includeSystemApps: Boolean = false,
        includeIcons: Boolean = true
    ): List<AppInfo> = withContext(Dispatchers.IO) {
        val packageManager = context.packageManager
        val apps = mutableListOf<AppInfo>()
        
        try {
            // Get all installed packages
            val packages = packageManager.getInstalledApplications(PackageManager.GET_META_DATA)
            
            for (appInfo in packages) {
                // Filter system apps if requested
                val isSystemApp = (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) != 0
                if (!includeSystemApps && isSystemApp) {
                    continue
                }
                
                try {
                    val appName = packageManager.getApplicationLabel(appInfo).toString()
                    val packageName = appInfo.packageName
                    val uid = appInfo.uid
                    
                    // Get app icon if requested
                    val iconBase64 = if (includeIcons) {
                        try {
                            val icon = packageManager.getApplicationIcon(appInfo)
                            encodeIconToBase64(icon)
                        } catch (e: Exception) {
                            android.util.Log.w(TAG, "Failed to load icon for $packageName", e)
                            null
                        }
                    } else {
                        null
                    }
                    
                    apps.add(
                        AppInfo(
                            packageName = packageName,
                            appName = appName,
                            uid = uid,
                            isSystemApp = isSystemApp,
                            iconBase64 = iconBase64
                        )
                    )
                } catch (e: Exception) {
                    android.util.Log.w(TAG, "Failed to process app: ${appInfo.packageName}", e)
                }
            }
            
            // Sort alphabetically by app name
            apps.sortBy { it.appName.lowercase() }
            
        } catch (e: Exception) {
            android.util.Log.e(TAG, "Error fetching installed apps", e)
        }
        
        apps
    }
    
    /**
     * Get app information for specific package names
     * 
     * @param packageNames List of package names to fetch
     * @param includeIcons Whether to include app icons
     * @return List of AppInfo objects
     */
    suspend fun getAppsByPackageNames(
        packageNames: List<String>,
        includeIcons: Boolean = true
    ): List<AppInfo> = withContext(Dispatchers.IO) {
        val packageManager = context.packageManager
        val apps = mutableListOf<AppInfo>()
        
        for (packageName in packageNames) {
            try {
                val appInfo = packageManager.getApplicationInfo(packageName, 0)
                val appName = packageManager.getApplicationLabel(appInfo).toString()
                val uid = appInfo.uid
                val isSystemApp = (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) != 0
                
                val iconBase64 = if (includeIcons) {
                    try {
                        val icon = packageManager.getApplicationIcon(appInfo)
                        encodeIconToBase64(icon)
                    } catch (e: Exception) {
                        null
                    }
                } else {
                    null
                }
                
                apps.add(
                    AppInfo(
                        packageName = packageName,
                        appName = appName,
                        uid = uid,
                        isSystemApp = isSystemApp,
                        iconBase64 = iconBase64
                    )
                )
            } catch (e: PackageManager.NameNotFoundException) {
                android.util.Log.w(TAG, "Package not found: $packageName")
            } catch (e: Exception) {
                android.util.Log.e(TAG, "Error fetching app info for $packageName", e)
            }
        }
        
        apps
    }
    
    /**
     * Convert app list to JSON format
     * 
     * @param apps List of AppInfo objects
     * @return JSON string representation
     */
    fun toJson(apps: List<AppInfo>): String {
        val jsonArray = JSONArray()
        
        for (app in apps) {
            val jsonObject = JSONObject().apply {
                put("package_name", app.packageName)
                put("app_name", app.appName)
                put("uid", app.uid)
                put("is_system_app", app.isSystemApp)
                if (app.iconBase64 != null) {
                    put("icon_base64", app.iconBase64)
                }
            }
            jsonArray.put(jsonObject)
        }
        
        return jsonArray.toString()
    }
    
    /**
     * Encode drawable icon to base64 string
     * 
     * @param drawable The drawable to encode
     * @return Base64 encoded string or null if encoding fails
     */
    private fun encodeIconToBase64(drawable: Drawable): String? {
        try {
            val bitmap = drawableToBitmap(drawable)
            val scaledBitmap = Bitmap.createScaledBitmap(bitmap, ICON_SIZE, ICON_SIZE, true)
            
            val outputStream = ByteArrayOutputStream()
            scaledBitmap.compress(Bitmap.CompressFormat.PNG, ICON_QUALITY, outputStream)
            val byteArray = outputStream.toByteArray()
            
            return Base64.encodeToString(byteArray, Base64.NO_WRAP)
        } catch (e: Exception) {
            android.util.Log.e(TAG, "Error encoding icon to base64", e)
            return null
        }
    }
    
    /**
     * Convert Drawable to Bitmap
     * 
     * @param drawable The drawable to convert
     * @return Bitmap representation
     */
    private fun drawableToBitmap(drawable: Drawable): Bitmap {
        if (drawable is BitmapDrawable) {
            return drawable.bitmap
        }
        
        val bitmap = Bitmap.createBitmap(
            drawable.intrinsicWidth.coerceAtLeast(1),
            drawable.intrinsicHeight.coerceAtLeast(1),
            Bitmap.Config.ARGB_8888
        )
        
        val canvas = Canvas(bitmap)
        drawable.setBounds(0, 0, canvas.width, canvas.height)
        drawable.draw(canvas)
        
        return bitmap
    }
    
    /**
     * Search apps by name or package name
     * 
     * @param query Search query
     * @param apps List of apps to search
     * @return Filtered list of apps
     */
    fun searchApps(query: String, apps: List<AppInfo>): List<AppInfo> {
        if (query.isBlank()) {
            return apps
        }
        
        val lowerQuery = query.lowercase()
        return apps.filter {
            it.appName.lowercase().contains(lowerQuery) ||
            it.packageName.lowercase().contains(lowerQuery)
        }
    }
}
