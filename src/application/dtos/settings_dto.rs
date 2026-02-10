use serde::{Serialize, Deserialize};

// ============================================================================
// OIDC Settings DTOs (Admin Panel)
// ============================================================================

/// Current OIDC settings returned to admin UI (secrets masked)
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcSettingsDto {
    pub enabled: bool,
    pub issuer_url: String,
    pub client_id: String,
    /// True if a client secret is configured (never reveals the actual value)
    pub client_secret_set: bool,
    pub scopes: String,
    pub auto_provision: bool,
    pub admin_groups: String,
    pub disable_password_login: bool,
    pub provider_name: String,
    /// Auto-generated callback URL the admin must register in their IdP
    pub callback_url: String,
    /// Field names overridden by environment variables (read-only in UI)
    pub env_overrides: Vec<String>,
}

/// Request body for saving OIDC settings from the admin panel
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveOidcSettingsDto {
    pub enabled: bool,
    pub issuer_url: String,
    pub client_id: String,
    /// Only update if provided and non-empty (None = keep existing)
    pub client_secret: Option<String>,
    pub scopes: Option<String>,
    pub auto_provision: Option<bool>,
    pub admin_groups: Option<String>,
    pub disable_password_login: Option<bool>,
    pub provider_name: Option<String>,
}

/// Request body for testing OIDC discovery
#[derive(Debug, Serialize, Deserialize)]
pub struct TestOidcConnectionDto {
    pub issuer_url: String,
}

/// Result of OIDC connection test
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcTestResultDto {
    pub success: bool,
    pub message: String,
    pub issuer: Option<String>,
    pub authorization_endpoint: Option<String>,
    pub token_endpoint: Option<String>,
    pub userinfo_endpoint: Option<String>,
    /// Suggested provider name (derived from issuer hostname)
    pub provider_name_suggestion: Option<String>,
}
