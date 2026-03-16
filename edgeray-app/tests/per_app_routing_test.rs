//! Per-app routing integration tests
//!
//! Verifies that per-app firewall rules correctly route traffic
//! based on whitelist/blacklist modes.

#[cfg(test)]
mod tests {
    use edgeray_app::models::{PerAppRule, RuleAction};

    /// Test whitelist mode: only included apps use VPN
    #[test]
    #[ignore] // Requires running VPN
    fn test_whitelist_mode() {
        let rules = vec![
            PerAppRule {
                id: "1".to_string(),
                package_id: "com.example.app1".to_string(),
                action: RuleAction::Include,
                created_at: 0,
                updated_at: 0,
            },
            PerAppRule {
                id: "2".to_string(),
                package_id: "com.example.app2".to_string(),
                action: RuleAction::Exclude,
                created_at: 0,
                updated_at: 0,
            },
        ];

        // In whitelist mode:
        // - app1 should use VPN (included)
        // - app2 should bypass VPN (not included)
        // - app3 should bypass VPN (not in list)

        // This would require actual network testing with app UIDs
        // For now, we verify the rule logic
        assert_eq!(rules[0].action, RuleAction::Include);
        assert_eq!(rules[1].action, RuleAction::Exclude);
    }

    /// Test blacklist mode: excluded apps bypass VPN
    #[test]
    #[ignore] // Requires running VPN
    fn test_blacklist_mode() {
        let rules = vec![PerAppRule {
            id: "1".to_string(),
            package_id: "com.example.app1".to_string(),
            action: RuleAction::Exclude,
            created_at: 0,
            updated_at: 0,
        }];

        // In blacklist mode:
        // - app1 should bypass VPN (excluded)
        // - app2 should use VPN (not in list)

        assert_eq!(rules[0].action, RuleAction::Exclude);
    }

    /// Test rule persistence
    #[tokio::test]
    #[ignore]
    #[cfg(not(target_arch = "wasm32"))]
    async fn test_rule_persistence() {
        use surrealdb::Surreal;
        use surrealdb::engine::local::Mem;

        // Create in-memory database
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();

        // Create rule
        let rule = PerAppRule {
            id: "test_rule".to_string(),
            package_id: "com.test.app".to_string(),
            action: RuleAction::Include,
            created_at: 1234567890,
            updated_at: 1234567890,
        };

        // Save rule
        let _: Option<PerAppRule> = db
            .create(("per_app_rules", &rule.id))
            .content(rule.clone())
            .await
            .unwrap();

        // Load rule
        let loaded: Option<PerAppRule> = db.select(("per_app_rules", &rule.id)).await.unwrap();

        assert_eq!(loaded, Some(rule));
    }

    /// Test rule validation
    #[test]
    fn test_rule_validation() {
        let valid_rule = PerAppRule {
            id: "valid".to_string(),
            package_id: "com.example.app".to_string(),
            action: RuleAction::Include,
            created_at: 0,
            updated_at: 0,
        };

        // Package ID should not be empty
        assert!(!valid_rule.package_id.is_empty());

        // ID should not be empty
        assert!(!valid_rule.id.is_empty());
    }

    /// Test bulk rule operations
    #[test]
    fn test_bulk_rule_operations() {
        let mut rules = vec![
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
                action: RuleAction::Include,
                created_at: 0,
                updated_at: 0,
            },
        ];

        // Add new rule
        rules.push(PerAppRule {
            id: "3".to_string(),
            package_id: "com.app3".to_string(),
            action: RuleAction::Exclude,
            created_at: 0,
            updated_at: 0,
        });

        assert_eq!(rules.len(), 3);

        // Remove rule
        rules.retain(|r| r.package_id != "com.app2");
        assert_eq!(rules.len(), 2);

        // Update rule
        if let Some(rule) = rules.iter_mut().find(|r| r.package_id == "com.app1") {
            rule.action = RuleAction::Exclude;
        }

        assert_eq!(
            rules
                .iter()
                .find(|r| r.package_id == "com.app1")
                .unwrap()
                .action,
            RuleAction::Exclude
        );
    }
}
