//! Parser stub for WASM target
//!
//! Provides a no-op implementation of share link parsing for wasm builds.

use super::ServerConfig;
use anyhow::{Result, anyhow};

/// Parse a share link (vless://, vmess://, etc) into a ServerConfig.
/// On wasm, this returns an error as parsing is not supported.
pub fn parse_share_link(_link: &str) -> Result<ServerConfig> {
    Err(anyhow!(
        "Share link parsing is not supported in wasm builds"
    ))
}
