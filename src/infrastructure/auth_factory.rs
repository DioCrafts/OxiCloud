use std::sync::Arc;
use anyhow::Result;
use sqlx::PgPool;

use crate::application::ports::auth_ports::TokenServicePort;
use crate::application::services::auth_application_service::AuthApplicationService;
use crate::application::services::folder_service::FolderService;
use crate::infrastructure::repositories::{UserPgRepository, SessionPgRepository};
use crate::infrastructure::services::password_hasher::Argon2PasswordHasher;
use crate::infrastructure::services::jwt_service::JwtTokenService;
use crate::common::config::AppConfig;
use crate::common::di::AuthServices;

pub async fn create_auth_services(
    config: &AppConfig, 
    pool: Arc<PgPool>,
    folder_service: Option<Arc<FolderService>>
) -> Result<AuthServices> {
    // Crear servicio de tokens JWT (implementación de TokenServicePort)
    let token_service: Arc<dyn TokenServicePort> = Arc::new(JwtTokenService::new(
        config.auth.jwt_secret.clone(),
        config.auth.access_token_expiry_secs,
        config.auth.refresh_token_expiry_secs,
    ));
    
    // Crear servicio de hashing de contraseñas
    let password_hasher = Arc::new(Argon2PasswordHasher::new());
    
    // Crear repositorios PostgreSQL
    let user_repository = Arc::new(UserPgRepository::new(pool.clone()));
    let session_repository = Arc::new(SessionPgRepository::new(pool.clone()));
    
    // Crear servicio de aplicación de autenticación
    let mut auth_app_service = AuthApplicationService::new(
        user_repository,
        session_repository,
        password_hasher,
        token_service.clone(),
    );
    
    // Configurar servicio de carpetas si está disponible
    if let Some(folder_svc) = folder_service {
        auth_app_service = auth_app_service.with_folder_service(folder_svc);
    }
    
    // Empaquetar servicio en Arc
    let auth_application_service = Arc::new(auth_app_service);
    
    Ok(AuthServices {
        token_service,
        auth_application_service,
    })
}