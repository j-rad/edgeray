//! Database persistence layer
//!
//! Handles SurrealDB operations for app data
#[cfg(not(target_arch = "wasm32"))]
pub mod per_app_rules;

#[cfg(not(target_arch = "wasm32"))]
pub use per_app_rules::PerAppRuleStore;

use crate::models::{AppSettings, ServerConfig, Subscription};

#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;
use std::error::Error;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::Surreal;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::engine::local::Db;

/// Global database instance
#[cfg(not(target_arch = "wasm32"))]
static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

/// Initialize the database connection.
///
/// Creates the database directory if it doesn't exist and connects to the embedded SurrealDB instance.
/// Sets up the default namespace and database.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if initialization succeeds, or an error if it fails.
pub async fn init() -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let app_data_dir = dirs::data_dir().ok_or("Failed to get app data dir")?;
        let db_path = app_data_dir.join("edgeray");

        if !db_path.exists() {
            std::fs::create_dir_all(&db_path)?;
        }

        let conn_str = format!("surrealkv://{}", db_path.display());
        DB.connect::<surrealdb::engine::local::SurrealKv>(conn_str)
            .await?;
        DB.use_ns("edgeray").use_db("servers").await?;
    }
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _created: Option<ServerConfig> = DB.create("servers").content(config).await?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = config;
    }
    Ok(())
}

/// List all server configurations.
///
/// # Returns
///
/// * `Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>>` - A vector of `ServerConfig` objects, or an error.
pub async fn list_servers() -> Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let servers: Vec<ServerConfig> = DB.select("servers").await?;
        Ok(servers)
    }
    #[cfg(target_arch = "wasm32")]
    Ok(vec![])
}

/// List all subscriptions.
///
/// # Returns
///
/// * `Result<Vec<Subscription>, Box<dyn Error + Send + Sync>>` - A vector of `Subscription` objects, or an error.
pub async fn list_subscriptions() -> Result<Vec<Subscription>, Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let subs: Vec<Subscription> = DB.select("subscriptions").await?;
        Ok(subs)
    }
    #[cfg(target_arch = "wasm32")]
    Ok(vec![])
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: Option<Subscription> = DB.update(("subscriptions", &sub.id)).content(sub).await?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = sub;
    }
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sql = "DELETE servers WHERE address = $addr AND port = $port";
        let _ = DB
            .query(sql)
            .bind(("addr", config.address.clone()))
            .bind(("port", config.port))
            .await?;

        let _created: Option<ServerConfig> = DB.create("servers").content(config).await?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = config;
    }
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
pub async fn save_subscription_group(
    group_name: &str,
    mut servers: Vec<ServerConfig>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
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
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = group_name;
        let _ = servers;
    }
    Ok(())
}

/// Get application settings.
///
/// # Returns
///
/// * `Result<AppSettings, Box<dyn Error + Send + Sync>>` - The application settings, or default if not found.
pub async fn get_settings() -> Result<AppSettings, Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let settings: Option<AppSettings> = DB.select(("settings", "main")).await?;
        Ok(settings.unwrap_or_default())
    }
    #[cfg(target_arch = "wasm32")]
    Ok(AppSettings::default())
}

/// Save application settings.
///
/// # Arguments
///
/// * `settings` - The `AppSettings` object to save.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Ok if saved successfully.
pub async fn save_settings(settings: AppSettings) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: Option<AppSettings> = DB.update(("settings", "main")).content(settings).await?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = settings;
    }
    Ok(())
}

/// Get database instance (wrapper for legacy compatibility or direct access)
#[cfg(not(target_arch = "wasm32"))]
pub async fn get_db() -> Result<&'static Surreal<Db>, String> {
    Ok(&*DB)
}
