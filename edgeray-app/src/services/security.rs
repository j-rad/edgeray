// src/services/security.rs
//! Phase 9 — Ephemeral Security
//!
//! Provides defense against forensic analysis.
//! - Integrates SQLCipher for SQLite database encryption.
//! - Implements `mlock` to pin sensitive keys in RAM, preventing them
//!   from being swapped to disk.
//! - Zeroize memory wipe logic for teardown.

use rusqlite::Connection;
use tracing::{info, warn};
use zeroize::Zeroize;

/// Pins a sensitive byte slice in RAM using mlock.
#[cfg(target_os = "linux")]
pub fn secure_mlock(data: &mut [u8]) -> Result<(), std::io::Error> {
    let ptr = data.as_mut_ptr() as *mut libc::c_void;
    let len = data.len();
    unsafe {
        if libc::mlock(ptr, len) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn secure_mlock(_data: &mut [u8]) -> Result<(), std::io::Error> {
    // Stub for non-Linux platforms
    Ok(())
}

/// Unpins a sensitive byte slice in RAM using munlock.
#[cfg(target_os = "linux")]
pub fn secure_munlock(data: &mut [u8]) -> Result<(), std::io::Error> {
    let ptr = data.as_mut_ptr() as *mut libc::c_void;
    let len = data.len();
    unsafe {
        if libc::munlock(ptr, len) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn secure_munlock(_data: &mut [u8]) -> Result<(), std::io::Error> {
    Ok(())
}

/// Secure container for a sensitive key (e.g. database password).
/// Automatically zeroes memory when dropped.
pub struct SecureKey {
    key: Vec<u8>,
}

impl SecureKey {
    pub fn new(key: Vec<u8>) -> Self {
        let mut s = Self { key };
        if let Err(e) = secure_mlock(&mut s.key) {
            warn!("Failed to mlock secure key: {}", e);
        }
        s
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.key).unwrap_or("")
    }
}

impl Drop for SecureKey {
    fn drop(&mut self) {
        let _ = secure_munlock(&mut self.key);
        self.key.zeroize();
    }
}

/// Initializes an encrypted SQLite connection using SQLCipher.
pub fn open_encrypted_db(db_path: &str, key: &SecureKey) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Assuming rusqlite is built with bundled SQLCipher via features
    let pragma_query = format!("PRAGMA key = '{}';", key.as_str());
    conn.execute(&pragma_query, [])?;

    // Verify encryption worked
    {
        let mut stmt = conn.prepare("SELECT count(*) FROM sqlite_master;")?;
        let _ = stmt.query_row([], |row| row.get::<_, i32>(0))?;
    }

    info!("Encrypted DB opened successfully at {}", db_path);
    Ok(conn)
}
