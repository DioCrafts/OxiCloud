//! App Password application service.
//!
//! Orchestrates creation, verification, listing, and revocation of
//! application-specific passwords for DAV clients.

use crate::application::dtos::app_password_dto::*;
use crate::application::ports::auth_ports::{
    AppPasswordStoragePort, PasswordHasherPort, UserStoragePort,
};
use crate::common::errors::DomainError;
use crate::domain::entities::app_password::AppPassword;
use chrono::{Duration, Utc};
use std::sync::Arc;

/// App password token length (32 random alphanumeric chars after prefix).
const TOKEN_LENGTH: usize = 32;
/// Prefix for all app password tokens (makes them easily identifiable).
const TOKEN_PREFIX: &str = "oxicloud-";

pub struct AppPasswordService {
    repo: Arc<dyn AppPasswordStoragePort>,
    hasher: Arc<dyn PasswordHasherPort>,
    user_repo: Arc<dyn UserStoragePort>,
    base_url: String,
}

impl AppPasswordService {
    pub fn new(
        repo: Arc<dyn AppPasswordStoragePort>,
        hasher: Arc<dyn PasswordHasherPort>,
        user_repo: Arc<dyn UserStoragePort>,
        base_url: String,
    ) -> Self {
        Self {
            repo,
            hasher,
            user_repo,
            base_url,
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
        let expires_at = request.expires_in_days.map(|days| {
            Utc::now() + Duration::days(days as i64)
        });

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

        let expires_str = saved
            .expires_at
            .map(|dt| dt.to_rfc3339());

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
    pub async fn revoke(&self, user_id: &str, id: &str) -> Result<AppPasswordRevokeResponseDto, DomainError> {
        let ap = self.repo.get_by_id(id).await?;
        if ap.user_id != user_id {
            return Err(DomainError::unauthorized(
                "You can only revoke your own app passwords",
            ));
        }
        self.repo.revoke(id).await?;
        Ok(AppPasswordRevokeResponseDto {
            status: "revoked".to_string(),
            id: id.to_string(),
        })
    }

    /// Verify username + app password for HTTP Basic Auth.
    ///
    /// Returns `(user_id, username, email, role)` on success.
    pub async fn verify_basic_auth(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(String, String, String, String), DomainError> {
        // Look up user by username
        let user = self
            .user_repo
            .get_user_by_username(username)
            .await
            .map_err(|_| DomainError::unauthorized("Invalid username or app password"))?;

        // Get all active app passwords for this user
        let app_passwords = self
            .repo
            .get_active_by_user_id(user.id())
            .await?;

        if app_passwords.is_empty() {
            return Err(DomainError::unauthorized(
                "Invalid username or app password",
            ));
        }

        // Try each app password hash
        for ap in &app_passwords {
            if let Ok(true) = self.hasher.verify_password(password, &ap.password_hash).await {
                // Update last_used_at (fire-and-forget; don't fail auth on touch error)
                let _ = self.repo.touch_last_used(&ap.id).await;

                return Ok((
                    user.id().to_string(),
                    user.username().to_string(),
                    user.email().to_string(),
                    user.role().to_string(),
                ));
            }
        }

        Err(DomainError::unauthorized(
            "Invalid username or app password",
        ))
    }
}
