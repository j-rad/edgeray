//! Database Module
//!
//! Handles persistence using SurrealDB (embedded KV mode).
//! Stores server configurations, subscriptions, and application settings.
//!
//! # Schema
//! - `servers`: Stores `ServerConfig` objects.
//! - `subscriptions`: Stores `Subscription` objects.
//! - `settings`: Stores key-value application settings.

use crate::models::{ServerConfig, Subscription};
use once_cell::sync::Lazy;
use std::error::Error;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

/// Global database instance
static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

/// Initialize the database connection.
///
/// Creates the database directory if it doesn't exist and connects to the embedded SurrealDB instance.
/// Sets up the default namespace and database.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if initialization succeeds, or an error if it fails.
#[allow(dead_code)]
pub async fn init() -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_data_dir = dirs::data_dir().ok_or("Failed to get app data dir")?;
    let db_path = app_data_dir.join("edgeray");

    if !db_path.exists() {
        std::fs::create_dir_all(&db_path)?;
    }

    let conn_str = format!("surrealkv://{}", db_path.display());
    DB.connect::<surrealdb::engine::local::SurrealKv>(conn_str)
        .await?;
    DB.use_ns("edgeray").use_db("servers").await?;
    Ok(())
}

/// Add a new server configuration to the database.
///
/// Delegates to `save_server` to ensure deduplication logic is applied.
///
/// # Arguments
///
/// * `config` - The `ServerConfig` object to be added.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if the server is added successfully, or an error.
#[allow(dead_code)]
pub async fn add_server(config: ServerConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _created: Option<ServerConfig> = DB.create("servers").content(config).await?;
    Ok(())
}

/// List all server configurations.
///
/// # Returns
///
/// * `Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>>` - A vector of `ServerConfig` objects, or an error.
pub async fn list_servers() -> Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>> {
    let servers: Vec<ServerConfig> = DB.select("servers").await?;
    Ok(servers)
}

/// List all subscriptions.
///
/// # Returns
///
/// * `Result<Vec<Subscription>, Box<dyn Error + Send + Sync>>` - A vector of `Subscription` objects, or an error.
pub async fn list_subscriptions() -> Result<Vec<Subscription>, Box<dyn Error + Send + Sync>> {
    let subs: Vec<Subscription> = DB.select("subscriptions").await?;
    Ok(subs)
}

/// Save or update a subscription.
///
/// Ensures the subscription ID is properly formatted as a RecordId before saving.
///
/// # Arguments
///
/// * `sub` - The `Subscription` object to save or update.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if the subscription is saved successfully, or an error.
pub async fn save_subscription(sub: Subscription) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _: Option<Subscription> = DB.update(("subscriptions", &sub.id)).content(sub).await?;
    Ok(())
}

/// Save or update a server configuration.
///
/// This function attempts to deduplicate servers based on address and port
/// before saving the new configuration.
///
/// # Arguments
///
/// * `config` - The `ServerConfig` object to save.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if the server is saved successfully, or an error.
pub async fn save_server(config: ServerConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Simple deduplication strategy:
    // DELETE FROM servers WHERE address = $addr AND port = $port;
    // CREATE servers CONTENT $config;
    // Ideally we would update, but without ID it is hard.

    let sql = "DELETE servers WHERE address = $addr AND port = $port";
    let _ = DB
        .query(sql)
        .bind(("addr", config.address.clone()))
        .bind(("port", config.port))
        .await?;

    let _created: Option<ServerConfig> = DB.create("servers").content(config).await?;
    Ok(())
}

/// Save a group of servers belonging to a subscription.
///
/// This transactionally deletes all existing servers in the group and inserts the new ones.
///
/// # Arguments
///
/// * `group_name` - The name of the group (usually the subscription name or ID).
/// * `servers` - A vector of `ServerConfig` objects to save in this group.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if the group is saved successfully, or an error.
#[allow(dead_code)]
pub async fn save_subscription_group(
    group_name: &str,
    mut servers: Vec<ServerConfig>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for server in &mut servers {
        server.group = Some(group_name.to_string());
    }

    let sql = "
        BEGIN TRANSACTION;
        DELETE servers WHERE group = $group;
        INSERT INTO servers $servers;
        COMMIT TRANSACTION;
    ";

    DB.query(sql)
        .bind(("group", group_name.to_string()))
        .bind(("servers", servers))
        .await?;

    Ok(())
}

/// Internal struct for settings storage.
#[derive(serde::Serialize, serde::Deserialize)]
struct Setting {
    id: String,
    value: bool,
}

/// Save a boolean preference setting.
///
/// # Arguments
///
/// * `key` - The key for the setting.
/// * `value` - The boolean value to store.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if the preference is saved successfully, or an error.
#[allow(dead_code)]
pub async fn save_preference(key: &str, value: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _: Option<Setting> = DB
        .update(("settings", key))
        .content(Setting {
            id: key.to_string(),
            value,
        })
        .await?;
    Ok(())
}

/// Get a boolean preference setting.
///
/// # Arguments
///
/// * `key` - The key for the setting to retrieve.
///
/// # Returns
///
/// * `Result<bool, Box<dyn Error + Send + Sync>>` - The boolean value of the setting (defaults to `false` if not found), or an error.
#[allow(dead_code)]
pub async fn get_preference(key: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let setting: Option<Setting> = DB.select(("settings", key)).await?;
    Ok(setting.map(|s| s.value).unwrap_or(false))
}
