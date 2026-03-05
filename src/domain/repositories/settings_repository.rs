use crate::common::errors::DomainError;
use std::collections::HashMap;

/// Repository for platform settings stored in the database.
/// Settings are key-value pairs organized by category (e.g., "oidc", "general").
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

    /// Atomically claim system initialization.
    ///
    /// Inserts `system_initialized = "true"` **only if the key does not
    /// already exist**.  Returns `true` when this call was the one that
    /// performed the insert (i.e. the caller "won" the race), `false` if
    /// the system was already initialized.
    ///
    /// The default implementation falls back to the non-atomic
    /// get-then-set pattern for repositories that don't support a native
    /// atomic upsert.
    async fn try_claim_initialization(&self, admin_user_id: &str) -> Result<bool, DomainError> {
        // Default: non-atomic fallback (overridden by PG implementation)
        match self.get("system_initialized").await? {
            Some(v) if v == "true" => Ok(false),
            _ => {
                self.set(
                    "system_initialized",
                    "true",
                    "system",
                    false,
                    Some(admin_user_id),
                )
                .await?;
                Ok(true)
            }
        }
    }
}
