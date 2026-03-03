use sqlx::PgPool;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    let token_service: Arc<JwtTokenService> = Arc::new(JwtTokenService::new(
        config.auth.jwt_secret.clone(),
        config.auth.access_token_expiry_secs,
        config.auth.refresh_token_expiry_secs,
    ));

    // Create password hashing service with configured Argon2id parameters
    let password_hasher = Arc::new(Argon2PasswordHasher::new(
        config.auth.hash_memory_cost,
        config.auth.hash_time_cost,
        config.auth.hash_parallelism,
    ));

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

    // Account lockout service — in-memory brute-force protection
    let login_lockout = Arc::new(
        crate::infrastructure::services::login_lockout_service::LoginLockoutService::new(
            config.auth.rate_limit.lockout_max_failures,
            config.auth.rate_limit.lockout_duration_secs,
            100_000, // Track up to 100k accounts concurrently
        ),
    );
    tracing::info!(
        "Login lockout service initialized: max {} failures, {}s lockout",
        config.auth.rate_limit.lockout_max_failures,
        config.auth.rate_limit.lockout_duration_secs,
    );

    Ok(AuthServices {
        token_service,
        auth_application_service,
        login_lockout,
    })
}
