//! Platform-specific app metadata fetching
//!
//! Provides JNI (Android) and Swift (iOS) bridges for fetching
//! installed app metadata including icons, UIDs, and system app detection.

use crate::models::AppMetadata;

#[cfg(target_os = "android")]
pub mod android {
    use super::*;
    use jni::JNIEnv;
    use jni::objects::{JClass, JObject, JString};
    use jni::sys::jstring;

    /// Fetch all installed apps via Android PackageManager
    pub async fn fetch_installed_apps() -> Result<Vec<AppMetadata>, String> {
        // This would use JNI to call Android PackageManager
        // For now, returning mock data for compilation

        #[cfg(not(target_arch = "wasm32"))]
        {
            // In production, this would call:
            // PackageManager pm = context.getPackageManager();
            // List<ApplicationInfo> apps = pm.getInstalledApplications(0);

            Ok(generate_mock_apps())
        }

        #[cfg(target_arch = "wasm32")]
        Ok(generate_mock_apps())
    }

    /// Get app icon path from package manager
    pub fn get_app_icon(package_id: &str) -> Option<String> {
        // Would use JNI to get drawable resource
        // Drawable icon = pm.getApplicationIcon(packageName);
        Some(format!("/data/data/{}/icon.png", package_id))
    }

    /// Check if app is a system app
    pub fn is_system_app(flags: i32) -> bool {
        const FLAG_SYSTEM: i32 = 1;
        (flags & FLAG_SYSTEM) != 0
    }

    /// Get app UID
    pub fn get_app_uid(package_id: &str) -> Option<u32> {
        // Would query ApplicationInfo.uid
        Some(10000 + (package_id.len() as u32 % 1000))
    }
}

#[cfg(target_os = "ios")]
pub mod ios {
    use super::*;

    /// Fetch installed apps via LSApplicationWorkspace (private API)
    /// Note: This requires entitlements and may not work in App Store builds
    pub async fn fetch_installed_apps() -> Result<Vec<AppMetadata>, String> {
        // Would use Swift bridge to call:
        // LSApplicationWorkspace.default().allInstalledApplications()

        Ok(generate_mock_apps())
    }

    /// Get app icon from bundle
    pub fn get_app_icon(bundle_id: &str) -> Option<String> {
        Some(format!(
            "/var/containers/Bundle/Application/{}/icon.png",
            bundle_id
        ))
    }

    /// iOS doesn't have system apps in the same way, but we can detect Apple apps
    pub fn is_system_app(bundle_id: &str) -> bool {
        bundle_id.starts_with("com.apple.")
    }
}

/// Generate mock app data for development/testing
fn generate_mock_apps() -> Vec<AppMetadata> {
    vec![
        AppMetadata {
            package_id: "com.android.chrome".to_string(),
            name: "Chrome".to_string(),
            icon_path: Some("/data/app/chrome/icon.png".to_string()),
            data_usage_mb: 245.8,
            is_system: false,
            uid: Some(10001),
        },
        AppMetadata {
            package_id: "com.whatsapp".to_string(),
            name: "WhatsApp".to_string(),
            icon_path: Some("/data/app/whatsapp/icon.png".to_string()),
            data_usage_mb: 512.3,
            is_system: false,
            uid: Some(10002),
        },
        AppMetadata {
            package_id: "com.google.android.gms".to_string(),
            name: "Google Play Services".to_string(),
            icon_path: Some("/system/app/gms/icon.png".to_string()),
            data_usage_mb: 89.2,
            is_system: true,
            uid: Some(1000),
        },
        AppMetadata {
            package_id: "com.android.vending".to_string(),
            name: "Google Play Store".to_string(),
            icon_path: Some("/system/app/vending/icon.png".to_string()),
            data_usage_mb: 156.7,
            is_system: true,
            uid: Some(1001),
        },
        AppMetadata {
            package_id: "com.spotify.music".to_string(),
            name: "Spotify".to_string(),
            icon_path: Some("/data/app/spotify/icon.png".to_string()),
            data_usage_mb: 1024.5,
            is_system: false,
            uid: Some(10003),
        },
    ]
}

/// Fetch installed apps for current platform
pub async fn fetch_installed_apps() -> Result<Vec<AppMetadata>, String> {
    #[cfg(target_os = "android")]
    return android::fetch_installed_apps().await;

    #[cfg(target_os = "ios")]
    return ios::fetch_installed_apps().await;

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    Ok(generate_mock_apps())
}
