//! Platform-specific integrations
//!
//! Provides JNI/Swift bridges and platform-specific functionality
//! including haptic feedback and app metadata.

pub mod app_metadata;
pub mod haptic_manager;

pub use app_metadata::fetch_installed_apps;
pub use haptic_manager::create_haptic_engine;
