//! SurrealDB persistence layer for per-app rules
//!
//! Handles CRUD operations for per-app firewall rules with SurrealDB
//! Note: This module is not available for WASM builds

#![cfg(not(target_arch = "wasm32"))]

use crate::models::PerAppRule;
use surrealdb::{Surreal, engine::local::Db};

pub struct PerAppRuleStore {
    db: Surreal<Db>,
}

impl PerAppRuleStore {
    /// Create a new rule store
    pub async fn new(db: Surreal<Db>) -> Result<Self, String> {
        Ok(Self { db })
    }

    /// Get all per-app rules
    pub async fn get_all(&self) -> Result<Vec<PerAppRule>, String> {
        self.db
            .select("per_app_rules")
            .await
            .map_err(|e| format!("Failed to fetch rules: {}", e))
    }

    /// Get rule by package ID
    pub async fn get_by_package(&self, package_id: &str) -> Result<Option<PerAppRule>, String> {
        let mut rules: Vec<PerAppRule> = self
            .db
            .query("SELECT * FROM per_app_rules WHERE package_id = $package_id")
            .bind(("package_id", package_id.to_string()))
            .await
            .map_err(|e| format!("Failed to query rule: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to parse result: {}", e))?;

        Ok(rules.pop())
    }

    /// Create or update a rule
    pub async fn upsert(&self, rule: &PerAppRule) -> Result<(), String> {
        let _: Option<PerAppRule> = self
            .db
            .update(("per_app_rules", &rule.id))
            .content(rule.clone())
            .await
            .map_err(|e| format!("Failed to upsert rule: {}", e))?;

        Ok(())
    }

    /// Create multiple rules
    pub async fn create_many(&self, rules: &[PerAppRule]) -> Result<(), String> {
        for rule in rules {
            self.upsert(rule).await?;
        }
        Ok(())
    }

    /// Delete a rule
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let _: Option<PerAppRule> = self
            .db
            .delete(("per_app_rules", id))
            .await
            .map_err(|e| format!("Failed to delete rule: {}", e))?;

        Ok(())
    }

    /// Delete rule by package ID
    pub async fn delete_by_package(&self, package_id: &str) -> Result<(), String> {
        self.db
            .query("DELETE FROM per_app_rules WHERE package_id = $package_id")
            .bind(("package_id", package_id.to_string()))
            .await
            .map_err(|e| format!("Failed to delete rule: {}", e))?;

        Ok(())
    }

    /// Get all included packages (for whitelist mode)
    pub async fn get_included_packages(&self) -> Result<Vec<String>, String> {
        let rules: Vec<PerAppRule> = self
            .db
            .query("SELECT * FROM per_app_rules WHERE action = 'Include'")
            .await
            .map_err(|e| format!("Failed to query included packages: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to parse result: {}", e))?;

        Ok(rules.into_iter().map(|r| r.package_id).collect())
    }

    /// Get all excluded packages (for blacklist mode)
    pub async fn get_excluded_packages(&self) -> Result<Vec<String>, String> {
        let rules: Vec<PerAppRule> = self
            .db
            .query("SELECT * FROM per_app_rules WHERE action = 'Exclude'")
            .await
            .map_err(|e| format!("Failed to query excluded packages: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to parse result: {}", e))?;

        Ok(rules.into_iter().map(|r| r.package_id).collect())
    }

    /// Clear all rules
    pub async fn clear_all(&self) -> Result<(), String> {
        self.db
            .query("DELETE FROM per_app_rules")
            .await
            .map_err(|e| format!("Failed to clear rules: {}", e))?;

        Ok(())
    }

    /// Get rule count
    pub async fn count(&self) -> Result<usize, String> {
        let result: Vec<PerAppRule> = self
            .db
            .select("per_app_rules")
            .await
            .map_err(|e| format!("Failed to count rules: {}", e))?;

        Ok(result.len())
    }

    /// Bulk update rules (replace all)
    pub async fn replace_all(&self, rules: &[PerAppRule]) -> Result<(), String> {
        // Clear existing rules
        self.clear_all().await?;

        // Insert new rules
        self.create_many(rules).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RuleAction;
    use surrealdb::engine::local::Mem;

    async fn create_test_store() -> PerAppRuleStore {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        PerAppRuleStore::new(db).await.unwrap()
    }

    #[tokio::test]
    async fn test_upsert_and_get() {
        let store = create_test_store().await;

        let rule = PerAppRule {
            id: "test_rule".to_string(),
            package_id: "com.test.app".to_string(),
            action: RuleAction::Include,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        store.upsert(&rule).await.unwrap();

        let retrieved = store.get_by_package("com.test.app").await.unwrap();
        assert_eq!(retrieved, Some(rule));
    }

    #[tokio::test]
    async fn test_delete() {
        let store = create_test_store().await;

        let rule = PerAppRule {
            id: "test_rule".to_string(),
            package_id: "com.test.app".to_string(),
            action: RuleAction::Include,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        store.upsert(&rule).await.unwrap();
        store.delete("test_rule").await.unwrap();

        let retrieved = store.get_by_package("com.test.app").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_get_included_packages() {
        let store = create_test_store().await;

        let rules = vec![
            PerAppRule {
                id: "1".to_string(),
                package_id: "com.app1".to_string(),
                action: RuleAction::Include,
                created_at: 0,
                updated_at: 0,
            },
            PerAppRule {
                id: "2".to_string(),
                package_id: "com.app2".to_string(),
                action: RuleAction::Exclude,
                created_at: 0,
                updated_at: 0,
            },
        ];

        store.create_many(&rules).await.unwrap();

        let included = store.get_included_packages().await.unwrap();
        assert_eq!(included.len(), 1);
        assert!(included.contains(&"com.app1".to_string()));
    }
}
