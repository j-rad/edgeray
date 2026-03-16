//! Internationalization (i18n) support for EdgeRay
//!
//! Provides translation functionality with fallback to English.

#![allow(dead_code)]
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::RwLock;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    // Add more languages as needed
    // Persian,
    // Chinese,
    // Russian,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Language::English),
            _ => None,
        }
    }
}

/// Translation data structure
#[derive(Debug, Deserialize, Clone)]
pub struct Translations {
    pub app: AppTranslations,
    pub screens: ScreenTranslations,
    pub connection: ConnectionTranslations,
    pub server: ServerTranslations,
    pub subscription: SubscriptionTranslations,
    pub settings: SettingsTranslations,
    pub routing: RoutingTranslations,
    pub assets: AssetsTranslations,
    pub logs: LogsTranslations,
    pub common: CommonTranslations,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppTranslations {
    pub name: String,
    pub tagline: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScreenTranslations {
    pub dashboard: String,
    pub servers: String,
    pub groups: String,
    pub settings: String,
    pub routing: String,
    pub assets: String,
    pub logs: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConnectionTranslations {
    pub connect: String,
    pub disconnect: String,
    pub connecting: String,
    pub connected: String,
    pub disconnected: String,
    pub failed: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerTranslations {
    pub add: String,
    pub edit: String,
    pub delete: String,
    pub share: String,
    pub test: String,
    pub ping: String,
    pub no_servers: String,
    pub add_to_start: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubscriptionTranslations {
    pub title: String,
    pub add: String,
    pub update: String,
    pub update_all: String,
    pub auto_update: String,
    pub update_interval: String,
    pub last_updated: String,
    pub updating: String,
    pub no_subscriptions: String,
    pub add_subscription_hint: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SettingsTranslations {
    pub general: String,
    pub connection: String,
    pub about: String,
    pub theme: String,
    pub system: String,
    pub dark: String,
    pub light: String,
    pub start_on_boot: String,
    pub allow_insecure: String,
    pub connection_mode: String,
    pub rule: String,
    pub global: String,
    pub direct: String,
    pub routing_rules: String,
    pub per_app_proxy: String,
    pub sniffing: String,
    pub sniffing_hint: String,
    pub version: String,
    pub github: String,
    pub privacy: String,
    pub license: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoutingTranslations {
    pub title: String,
    pub mode: String,
    pub bypass_lan: String,
    pub bypass_mainland: String,
    pub global_proxy: String,
    pub custom_rules: String,
    pub add_rule: String,
    pub domain: String,
    pub ip: String,
    pub geosite: String,
    pub geoip: String,
    pub action: String,
    pub proxy: String,
    pub direct: String,
    pub block: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetsTranslations {
    pub title: String,
    pub geoip: String,
    pub geosite: String,
    pub download: String,
    pub update: String,
    pub delete: String,
    pub size: String,
    pub last_updated: String,
    pub downloading: String,
    pub no_assets: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogsTranslations {
    pub title: String,
    pub clear: String,
    pub export: String,
    pub level: String,
    pub debug: String,
    pub info: String,
    pub warn: String,
    pub error: String,
    pub no_logs: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommonTranslations {
    pub save: String,
    pub cancel: String,
    pub delete: String,
    pub edit: String,
    pub done: String,
    pub loading: String,
    pub error: String,
    pub success: String,
    pub confirm: String,
    pub yes: String,
    pub no: String,
}

/// Global translation manager
pub struct I18n {
    current_language: RwLock<Language>,
    translations: HashMap<Language, Translations>,
}

static I18N: Lazy<I18n> = Lazy::new(|| {
    let mut translations = HashMap::new();

    // Load English translations (embedded)
    let en_json = include_str!("../locales/en.json");
    if let Ok(en_trans) = serde_json::from_str::<Translations>(en_json) {
        translations.insert(Language::English, en_trans);
    }

    I18n {
        current_language: RwLock::new(Language::English),
        translations,
    }
});

impl I18n {
    /// Get the global i18n instance
    pub fn global() -> &'static I18n {
        &I18N
    }

    /// Get current language
    pub fn current_language(&self) -> Language {
        *self.current_language.read().unwrap()
    }

    /// Set current language
    pub fn set_language(&self, lang: Language) {
        *self.current_language.write().unwrap() = lang;
    }

    /// Get translations for current language
    pub fn t(&self) -> Translations {
        let lang = self.current_language();
        self.translations.get(&lang).cloned().unwrap_or_else(|| {
            // Fallback to English
            self.translations
                .get(&Language::English)
                .cloned()
                .expect("English translations must be available")
        })
    }
}

/// Helper function to get translations
pub fn t() -> Translations {
    I18n::global().t()
}

/// Helper function to set language
pub fn set_language(lang: Language) {
    I18n::global().set_language(lang);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::from_code("en"), Some(Language::English));
    }

    #[test]
    fn test_translations_loaded() {
        let trans = t();
        assert_eq!(trans.app.name, "EdgeRay");
        assert!(!trans.connection.connect.is_empty());
    }

    #[test]
    fn test_set_language() {
        set_language(Language::English);
        assert_eq!(I18n::global().current_language(), Language::English);
    }
}
