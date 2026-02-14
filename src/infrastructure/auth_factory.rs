use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

use crate::application::ports::auth_ports::TokenServicePort;
use crate::application::services::auth_application_service::AuthApplicationService;
use crate::application::services::folder_service::FolderService;
use crate::common::config::AppConfig;
use crate::common::di::AuthServices;
use crate::infrastructure::repositories::{SessionPgRepository, UserPgRepository};
use crate::infrastructure::services::jwt_service::JwtTokenService;
use crate::infrastructure::services::oidc_service::OidcService;
use crate::infrastructure::services::password_hasher::Argon2PasswordHasher;

pub async fn create_auth_services(
    config: &AppConfig,
    pool: Arc<PgPool>,
    folder_service: Option<Arc<FolderService>>,
) -> Result<AuthServices> {
    // Create JWT token service (TokenServicePort implementation)
    let token_service: Arc<dyn TokenServicePort> = Arc::new(JwtTokenService::new(
        config.auth.jwt_secret.clone(),
        config.auth.access_token_expiry_secs,
        config.auth.refresh_token_expiry_secs,
    ));

    // Create password hashing service
    let password_hasher = Arc::new(Argon2PasswordHasher::new());

    // Create PostgreSQL repositories
    let user_repository = Arc::new(UserPgRepository::new(pool.clone()));
    let session_repository = Arc::new(SessionPgRepository::new(pool.clone()));

    // Create authentication application service
    let mut auth_app_service = AuthApplicationService::new(
        user_repository,
        session_repository,
        password_hasher,
        token_service.clone(),
        config.storage_path.clone(),
    );

    // Configure folder service if available
    if let Some(folder_svc) = folder_service {
        auth_app_service = auth_app_service.with_folder_service(folder_svc);
    }

    // Configure OIDC service if enabled
    if config.oidc.enabled {
        tracing::info!(
            "Initializing OIDC service (provider: {}, issuer: {})",
            config.oidc.provider_name,
            config.oidc.issuer_url
        );

        let oidc_service = Arc::new(OidcService::new(config.oidc.clone()));
        auth_app_service = auth_app_service.with_oidc(oidc_service, config.oidc.clone());

        if config.oidc.disable_password_login {
            tracing::warn!("Password login is DISABLED — only OIDC authentication is allowed");
        }
    }

    // Package service in Arc
    let auth_application_service = Arc::new(auth_app_service);

    Ok(AuthServices {
        token_service,
        auth_application_service,
    })
}
