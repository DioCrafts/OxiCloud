//! App Password application service.
//!
//! Orchestrates creation, verification, listing, and revocation of
//! application-specific passwords for DAV clients.

use crate::application::dtos::app_password_dto::*;
use crate::application::ports::auth_ports::{
    AppPasswordStoragePort, PasswordHasherPort, UserStoragePort,
};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::app_password::AppPassword;
use crate::infrastructure::repositories::pg::AppPasswordPgRepository;
use crate::infrastructure::repositories::pg::UserPgRepository;
use crate::infrastructure::services::password_hasher::Argon2PasswordHasher;
use chrono::{Duration, Utc};
use moka::future::Cache;
use rand_core::RngCore;
use std::sync::Arc;
use std::time::Duration as StdDuration;

/// App password token length (32 random alphanumeric chars after prefix).
const TOKEN_LENGTH: usize = 32;
/// Prefix for all app password tokens (makes them easily identifiable).
const TOKEN_PREFIX: &str = "oxicloud-";

// ── Nextcloud-format app password constants ──
const NC_APP_PASSWORD_GROUPS: usize = 5;
const NC_APP_PASSWORD_GROUP_LEN: usize = 5;
const NC_PREFIX_LEN: usize = 8;

/// TTL for cached Basic Auth verification results.
/// Balances performance (avoids repeated Argon2id + DB queries) with security
/// (limits the window during which a revoked app password remains usable).
const BASIC_AUTH_CACHE_TTL_SECS: u64 = 30;

/// Maximum number of cached Basic Auth verifications.
/// Each entry is ~160 bytes (32-byte key + 4 small strings), so 10 000
/// entries ≈ 1.6 MB — negligible compared to other in-memory caches.
const BASIC_AUTH_CACHE_MAX_ENTRIES: u64 = 10_000;

/// Cached identity returned after a successful Basic Auth verification.
#[derive(Clone)]
struct CachedBasicAuthResult {
    user_id: String,
    username: String,
    email: String,
    role: String,
}

pub struct AppPasswordService {
    repo: Arc<AppPasswordPgRepository>,
    hasher: Arc<Argon2PasswordHasher>,
    user_repo: Arc<UserPgRepository>,
    base_url: String,

    /// In-memory cache of successful Basic Auth verifications.
    ///
    /// **Key**: `blake3(username + ":" + password)` — the plain-text password
    /// is never stored; only a cryptographic hash is kept as lookup key.
    ///
    /// **Value**: the authenticated identity (user_id, username, email, role).
    ///
    /// **Eviction**: TTL-based (30 s) + capacity-based (10 000 entries).
    /// Failed verifications are *never* cached, so brute-force attackers
    /// always pay the full Argon2id cost.
    auth_cache: Cache<[u8; 32], CachedBasicAuthResult>,
}

impl AppPasswordService {
    pub fn new(
        repo: Arc<AppPasswordPgRepository>,
        hasher: Arc<Argon2PasswordHasher>,
        user_repo: Arc<UserPgRepository>,
        base_url: String,
    ) -> Self {
        let auth_cache = Cache::builder()
            .max_capacity(BASIC_AUTH_CACHE_MAX_ENTRIES)
            .time_to_live(StdDuration::from_secs(BASIC_AUTH_CACHE_TTL_SECS))
            .build();

        tracing::info!(
            "AppPasswordService Basic Auth cache initialized: TTL={}s, max={} entries",
            BASIC_AUTH_CACHE_TTL_SECS,
            BASIC_AUTH_CACHE_MAX_ENTRIES,
        );

        Self {
            repo,
            hasher,
            user_repo,
            base_url,
            auth_cache,
        }
    }

    /// Generate a random app password token using cryptographic RNG.
    fn generate_token() -> String {
        use rand_core::{OsRng, RngCore};

        let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                                ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                0123456789";
        let mut rng_bytes = [0u8; TOKEN_LENGTH];
        OsRng.fill_bytes(&mut rng_bytes);

        let random_part: String = rng_bytes
            .iter()
            .map(|&b| {
                let idx = (b as usize) % charset.len();
                charset[idx] as char
            })
            .collect();
        format!("{}{}", TOKEN_PREFIX, random_part)
    }

    /// Create a new app password for the given user.
    ///
    /// Returns the response DTO that includes the plain-text password (shown only once).
    pub async fn create(
        &self,
        user_id: &str,
        request: CreateAppPasswordRequestDto,
    ) -> Result<AppPasswordCreatedResponseDto, DomainError> {
        // Validate label
        let label = request.label.trim().to_string();
        if label.is_empty() || label.len() > 255 {
            return Err(DomainError::validation_error(
                "Label must be 1-255 characters",
            ));
        }

        // Fetch user for the username (needed for Basic Auth instructions)
        let user = self.user_repo.get_user_by_id(user_id).await?;
        let username = user.username().to_string();

        // Generate the plain-text token
        let plain_token = Self::generate_token();
        let prefix = plain_token[..TOKEN_PREFIX.len() + 8].to_string();

        // Hash the token for storage
        let password_hash = self.hasher.hash_password(&plain_token).await?;

        // Calculate expiration
        let expires_at = request
            .expires_in_days
            .map(|days| Utc::now() + Duration::days(days as i64));

        // Create entity
        let app_password = AppPassword::new(
            user_id.to_string(),
            label.clone(),
            password_hash,
            prefix.clone(),
            request.scopes.clone(),
            expires_at,
        );

        let saved = self.repo.create(app_password).await?;

        let expires_str = saved.expires_at.map(|dt| dt.to_rfc3339());

        let curl_example = format!(
            "curl -u '{}:{}' -X PROPFIND {}/webdav/",
            username, plain_token, self.base_url
        );

        Ok(AppPasswordCreatedResponseDto {
            id: saved.id,
            label,
            password: plain_token,
            username: username.clone(),
            scopes: request.scopes,
            expires_at: expires_str,
            instructions: AppPasswordInstructions {
                davx5: format!(
                    "In DAVx⁵, add account with base URL: {}/webdav/\n\
                     Username: {}\n\
                     Password: (the token shown above)",
                    self.base_url, username
                ),
                thunderbird: format!(
                    "In Thunderbird CalDAV/CardDAV:\n\
                     URL: {}/caldav/ or {}/carddav/\n\
                     Username: {}\n\
                     Password: (the token shown above)",
                    self.base_url, self.base_url, username
                ),
                rclone: format!(
                    "rclone config:\n\
                     type = webdav\n\
                     url = {}/webdav/\n\
                     vendor = other\n\
                     user = {}\n\
                     pass = (the token shown above, use 'rclone obscure' to encode)",
                    self.base_url, username
                ),
                curl_example,
            },
        })
    }

    /// List all app passwords for a user (excludes plain-text passwords).
    pub async fn list(&self, user_id: &str) -> Result<AppPasswordListResponseDto, DomainError> {
        let passwords = self.repo.list_by_user(user_id).await?;
        let total = passwords.len();

        let app_passwords = passwords
            .into_iter()
            .map(|ap| {
                let is_active = ap.active && !ap.is_expired();
                AppPasswordSummaryDto {
                    id: ap.id,
                    label: ap.label,
                    prefix: format!("{}...", ap.prefix),
                    scopes: ap.scopes,
                    created_at: ap.created_at.to_rfc3339(),
                    last_used_at: ap.last_used_at.map(|dt| dt.to_rfc3339()),
                    expires_at: ap.expires_at.map(|dt| dt.to_rfc3339()),
                    active: is_active,
                }
            })
            .collect();

        Ok(AppPasswordListResponseDto {
            app_passwords,
            total,
        })
    }

    /// Revoke (soft-delete) an app password. Verifies ownership.
    ///
    /// Also invalidates **all** cached Basic Auth entries for the owning user
    /// so that the revocation takes effect immediately (instead of waiting
    /// up to `BASIC_AUTH_CACHE_TTL_SECS`).
    pub async fn revoke(
        &self,
        user_id: &str,
        id: &str,
    ) -> Result<AppPasswordRevokeResponseDto, DomainError> {
        // Ownership enforced at SQL level (WHERE user_id = $2).
        // The get_by_id pre-check gives a clear error message when
        // the password doesn't belong to the caller.
        let ap = self.repo.get_by_id(id).await?;
        if ap.user_id != user_id {
            return Err(DomainError::unauthorized(
                "You can only revoke your own app passwords",
            ));
        }
        self.repo.revoke(id, user_id).await?;

        // Invalidate all cached auth entries for this user so the
        // revocation is effective immediately.
        let uid = user_id.to_string();
        self.auth_cache
            .invalidate_entries_if(move |_key, val| val.user_id == uid)
            .ok();

        tracing::debug!(
            "Revoked app password {} — auth cache entries for user {} invalidated",
            id,
            user_id
        );

        Ok(AppPasswordRevokeResponseDto {
            status: "revoked".to_string(),
            id: id.to_string(),
        })
    }

    /// Verify username + app password for HTTP Basic Auth.
    ///
    /// Returns `(user_id, username, email, role)` on success.
    ///
    /// Handles both `oxicloud-` format and Nextcloud format (`XXXXX-XXXXX-...`)
    /// passwords. Uses prefix-based DB lookup to minimize Argon2id attempts.
    ///
    /// Successful verifications are cached for `BASIC_AUTH_CACHE_TTL_SECS`
    /// keyed by `blake3(username:password)`.  Failed verifications are
    /// **never** cached, preserving the full Argon2id cost as a brute-force
    /// deterrent.
    pub async fn verify_basic_auth(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(String, String, String, String), DomainError> {
        // ── 1. Compute cache key = blake3("username:password") ────────
        let cache_key: [u8; 32] =
            blake3::hash(format!("{}:{}", username, password).as_bytes()).into();

        // ── 2. Cache hit → return immediately ────────────────────────
        if let Some(cached) = self.auth_cache.get(&cache_key).await {
            return Ok((cached.user_id, cached.username, cached.email, cached.role));
        }

        // ── 3. Cache miss → full verification ────────────────────────
        let user = self
            .user_repo
            .get_user_by_username(username)
            .await
            .map_err(|_| DomainError::unauthorized("Invalid username or app password"))?;

        if !user.is_active() {
            return Err(DomainError::unauthorized(
                "Invalid username or app password",
            ));
        }

        // Determine the password form and prefix for DB lookup.
        // oxicloud- format: use raw password, prefix = first 17 chars
        // NC format: normalize (strip dashes/whitespace, uppercase), prefix = first 8 chars
        let (verify_password, prefix) = if password.starts_with(TOKEN_PREFIX) {
            let pfx = password
                .get(..TOKEN_PREFIX.len() + 8)
                .unwrap_or(password)
                .to_string();
            (password.to_string(), pfx)
        } else {
            let norm = nc_normalize_password(password);
            match nc_token_prefix(&norm) {
                Ok(pfx) => (norm, pfx),
                Err(_) => {
                    return Err(DomainError::unauthorized(
                        "Invalid username or app password",
                    ));
                }
            }
        };

        // Use prefix-based lookup for efficiency (fewer Argon2id attempts)
        let candidates = self
            .repo
            .get_active_by_user_prefix(user.id(), &prefix)
            .await?;

        if candidates.is_empty() {
            return Err(DomainError::unauthorized(
                "Invalid username or app password",
            ));
        }

        for ap in &candidates {
            if let Ok(true) = self
                .hasher
                .verify_password(&verify_password, &ap.password_hash)
                .await
            {
                let _ = self.repo.touch_last_used(&ap.id).await;

                let result = CachedBasicAuthResult {
                    user_id: user.id().to_string(),
                    username: user.username().to_string(),
                    email: user.email().to_string(),
                    role: user.role().to_string(),
                };

                self.auth_cache.insert(cache_key, result.clone()).await;
                return Ok((result.user_id, result.username, result.email, result.role));
            }
        }

        Err(DomainError::unauthorized(
            "Invalid username or app password",
        ))
    }

    // ========================================================================
    // Nextcloud-format app password methods
    // ========================================================================

    /// Create a Nextcloud-format app password (`XXXXX-XXXXX-XXXXX-XXXXX-XXXXX`).
    ///
    /// Returns `(id, plain_password)`.
    pub async fn create_nc(
        &self,
        user_id: &str,
        label: &str,
    ) -> Result<(String, String), DomainError> {
        let password = generate_nc_app_password();
        let normalized = nc_normalize_password(&password);
        let prefix = nc_token_prefix(&normalized)?;
        let hash = self.hasher.hash_password(&normalized).await?;

        let ap = AppPassword::new(
            user_id.to_string(),
            label.to_string(),
            hash,
            prefix,
            "all".to_string(),
            None,
        );

        let saved = self.repo.create(ap).await?;
        Ok((saved.id, password))
    }

    /// Revoke an app password by matching the raw password value.
    /// Scoped to the authenticated user (fixes I3 — no global prefix search).
    pub async fn revoke_by_password(
        &self,
        user_id: &str,
        password: &str,
    ) -> Result<(), DomainError> {
        let normalized = nc_normalize_password(password);
        let prefix = match nc_token_prefix(&normalized) {
            Ok(pfx) => pfx,
            Err(_) => return Ok(()),
        };

        let candidates = self
            .repo
            .get_active_by_user_prefix(user_id, &prefix)
            .await?;

        for ap in candidates {
            if let Ok(true) = self
                .hasher
                .verify_password(&normalized, &ap.password_hash)
                .await
            {
                self.repo.revoke(&ap.id, user_id).await?;

                // Invalidate cache for this user
                let uid = user_id.to_string();
                self.auth_cache
                    .invalidate_entries_if(move |_key, val| val.user_id == uid)
                    .ok();
                break;
            }
        }

        Ok(())
    }

    /// List app passwords for a user (simple summary for NC UI).
    pub async fn list_nc(&self, user_id: &str) -> Result<Vec<AppPassword>, DomainError> {
        self.repo.list_by_user(user_id).await
    }

    /// Delete an app password by ID, scoped to the owning user.
    pub async fn delete_by_user(&self, id: &str, user_id: &str) -> Result<(), DomainError> {
        let deleted = self.repo.delete_by_user_and_id(id, user_id).await?;
        if !deleted {
            return Err(DomainError::new(
                ErrorKind::NotFound,
                "AppPassword",
                "App password not found",
            ));
        }
        Ok(())
    }
}

// ============================================================================
// Nextcloud app password helpers (module-private)
// ============================================================================

/// Generate a Nextcloud-format app password: `XXXXX-XXXXX-XXXXX-XXXXX-XXXXX`
/// using rejection sampling to avoid modulo bias.
fn generate_nc_app_password() -> String {
    let mut rng = rand_core::OsRng;
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let len = chars.len() as u32; // 36
    let mut groups = Vec::with_capacity(NC_APP_PASSWORD_GROUPS);

    for _ in 0..NC_APP_PASSWORD_GROUPS {
        let mut group = String::with_capacity(NC_APP_PASSWORD_GROUP_LEN);
        for _ in 0..NC_APP_PASSWORD_GROUP_LEN {
            let threshold = u32::MAX - (u32::MAX % len);
            let idx = loop {
                let val = rng.next_u32();
                if val < threshold {
                    break (val % len) as usize;
                }
            };
            group.push(chars[idx] as char);
        }
        groups.push(group);
    }

    groups.join("-")
}

/// Normalize a Nextcloud-format password: strip dashes/whitespace, uppercase.
fn nc_normalize_password(password: &str) -> String {
    password
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Extract the first 8 characters as the token prefix for DB lookup.
fn nc_token_prefix(normalized: &str) -> Result<String, DomainError> {
    if normalized.len() < NC_PREFIX_LEN {
        return Err(DomainError::new(
            ErrorKind::InvalidInput,
            "AppPassword",
            "App password too short",
        ));
    }
    Ok(normalized[..NC_PREFIX_LEN].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nc_app_password_format() {
        let password = generate_nc_app_password();
        let groups: Vec<&str> = password.split('-').collect();
        assert_eq!(groups.len(), NC_APP_PASSWORD_GROUPS);
        for group in &groups {
            assert_eq!(group.len(), NC_APP_PASSWORD_GROUP_LEN);
            assert!(group.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn test_nc_normalize_password_strips_dashes_and_whitespace() {
        assert_eq!(
            nc_normalize_password("AB12C-DE34F-GH56I"),
            "AB12CDE34FGH56I"
        );
    }

    #[test]
    fn test_nc_normalize_password_uppercases() {
        assert_eq!(nc_normalize_password("abc-def"), "ABCDEF");
    }

    #[test]
    fn test_nc_token_prefix_extracts_first_8_chars() {
        assert_eq!(nc_token_prefix("ABCDEFGHIJKLMNOP").unwrap(), "ABCDEFGH");
    }

    #[test]
    fn test_nc_token_prefix_too_short() {
        assert!(nc_token_prefix("SHORT").is_err());
    }

    #[test]
    fn test_generated_nc_password_produces_valid_prefix() {
        let password = generate_nc_app_password();
        let normalized = nc_normalize_password(&password);
        let prefix = nc_token_prefix(&normalized);
        assert!(prefix.is_ok());
        assert_eq!(prefix.unwrap().len(), NC_PREFIX_LEN);
    }
}
