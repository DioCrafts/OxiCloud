use crate::common::errors::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;

/// Repository for platform settings stored in the database.
/// Settings are key-value pairs organized by category (e.g., "oidc", "general").
#[async_trait]
pub trait SettingsRepository: Send + Sync + 'static {
    /// Get a single setting value by key
    async fn get(&self, key: &str) -> Result<Option<String>, DomainError>;

    /// Get all settings for a given category
    async fn get_by_category(&self, category: &str)
    -> Result<HashMap<String, String>, DomainError>;

    /// Set a setting value (upsert)
    async fn set(
        &self,
        key: &str,
        value: &str,
        category: &str,
        is_secret: bool,
        updated_by: Option<&str>,
    ) -> Result<(), DomainError>;

    /// Delete a setting by key
    async fn delete(&self, key: &str) -> Result<(), DomainError>;
}
