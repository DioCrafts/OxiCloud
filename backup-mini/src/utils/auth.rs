//! Autenticación y autorización

use actix_web::{
    web, HttpRequest, FromRequest, dev::Payload, error::ErrorUnauthorized,
};
use anyhow::{Result, anyhow, Context};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use chrono::{Duration, Utc};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use actix_web::HttpMessage;
use futures::future::BoxFuture;


use crate::core::users::User;
use crate::core::config::get_config;

/// Datos contenidos en un token JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Identificador de usuario
    pub sub: String,
    /// Momento de expiración del token (timestamp Unix)
    pub exp: i64,
    /// Momento de emisión del token (timestamp Unix)
    pub iat: i64,
}

/// Genera un hash para una contraseña
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hashear la contraseña
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Error al hashear contraseña: {}", e))?
        .to_string();

    Ok(password_hash)
}

/// Verifica una contraseña contra su hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow!("Error al parsear hash: {}", e))?;

    // Verificar contraseña
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Genera un token JWT para un usuario
pub fn generate_token(user_id: Uuid) -> Result<String> {
    let config = get_config();
    let expiry_minutes = config.security.jwt_expiry_minutes;

    // Calcular fechas de emisión y expiración
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(expiry_minutes)).timestamp();

    // Crear claims
    let claims = Claims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    // Firmar token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.security.jwt_secret.as_bytes()),
    )
    .map_err(|e| anyhow!("Error al generar token: {}", e))?;

    Ok(token)
}

/// Valida un token JWT y extrae su contenido
pub fn validate_token(token: &str) -> Result<Claims> {
    let config = get_config();

    // Decodificar y validar token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.security.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| anyhow!("Error al validar token: {}", e))?;

    Ok(token_data.claims)
}

/// Extrae el token de sesión de una petición
pub fn extract_token_from_request(req: &HttpRequest) -> Option<String> {
    // Primero buscar en la cookie de sesión
    if let Some(cookie) = req.cookie("session") {
        return Some(cookie.value().to_string());
    }

    // Luego buscar en header Authorization
    if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_str = auth_header.to_str().ok()?;
        if auth_str.starts_with("Bearer ") {
            return Some(auth_str[7..].to_string());
        }
    }

    None
}

/// Extrae el usuario de una petición
pub async fn extract_user_from_session(pool: &PgPool, req: &HttpRequest) -> Result<User> {
    // Obtener token
    let token = extract_token_from_request(req)
        .ok_or_else(|| anyhow!("No se encontró token de sesión"))?;

    // Validar token
    let claims = validate_token(&token)?;

    // Convertir ID a UUID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| anyhow!("ID de usuario inválido en token"))?;

    // Buscar usuario en la base de datos
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, display_name, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .context("Error al buscar usuario")?
    .ok_or_else(|| anyhow!("Usuario no encontrado"))?;

    Ok(user)
}

/// Extractor para obtener el usuario de la petición en controladores Actix
impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Obtener el pool de la base de datos del estado de la aplicación
        let _ = match req.app_data::<web::Data<PgPool>>() {
            Some(pool) => pool.clone(),
            None => {
                error!("No se encontró pool de base de datos en el estado de la aplicación");
                return ready(Err(ErrorUnauthorized("Error de configuración del servidor")));
            }
        };

        // Obtener usuario del middleware de autenticación
        let user = match req.extensions().get::<User>() {
            Some(user) => user.clone(),
            None => {
                warn!("No se encontró usuario en las extensiones de la petición");
                return ready(Err(ErrorUnauthorized("No autenticado")));
            }
        };

        ready(Ok(user))
    }
}

/// Middleware de autenticación
pub struct AuthenticationMiddleware {
    pool: web::Data<PgPool>,
}

impl AuthenticationMiddleware {
    pub fn new(pool: web::Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for AuthenticationMiddleware
where
    S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static + std::clone::Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthenticationMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService {
            service,
            pool: self.pool.clone(),
        }))
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: S,
    pool: web::Data<PgPool>,
}

impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest, 
        Response = actix_web::dev::ServiceResponse<B>, 
        Error = actix_web::Error
    > + std::clone::Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    
    actix_web::dev::forward_ready!(service);
    
    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let pool = self.pool.clone();
        let mut service = self.service.clone();
        
        Box::pin(async move {
            // Ignorar rutas públicas
            let path = req.path();
            if path.starts_with("/static") || path == "/login" || path == "/register" {
                return service.call(req).await;
            }
            
            // Extraer token
            let token = match extract_token_from_request(req.request()) {
                Some(token) => token,
                None => {
                    // Si es una ruta API, devolver error JSON
                    if path.starts_with("/api") {
                        return Err(ErrorUnauthorized("No autenticado"));
                    }
                    
                    // Para rutas de UI, redirigir a login
                    // En Actix 4.x, necesitamos usar un enfoque diferente para la redirección
                    // Creamos una respuesta de redirección
                    let res = actix_web::HttpResponse::Found()
                        .insert_header(("Location", "/login"))
                        .finish();
                    
                    // Terminamos la petición actual y devolvemos un error interno
                    // que será manejado por el framework
                    return Err(actix_web::error::InternalError::from_response(
                        "No autenticado", res
                    ).into());
                }
            };
            
            // Validar token
            let claims = match validate_token(&token) {
                Ok(claims) => claims,
                Err(e) => {
                    warn!("Token inválido: {}", e);
                    
                    // Si es una ruta API, devolver error JSON
                    if path.starts_with("/api") {
                        return Err(ErrorUnauthorized("Token inválido"));
                    }
                    
                    // Para rutas de UI, redirigir a login
                    // En Actix 4.x, necesitamos usar un enfoque diferente para la redirección
                    let res = actix_web::HttpResponse::Found()
                        .insert_header(("Location", "/login"))
                        .finish();
                    
                    // Terminamos la petición actual y devolvemos un error interno
                    // que será manejado por el framework
                    return Err(actix_web::error::InternalError::from_response(
                        "No autenticado", res
                    ).into());
                }
            };
            
            // Convertir ID a UUID
            let user_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(_) => {
                    warn!("ID de usuario inválido en token");
                    return Err(ErrorUnauthorized("Token inválido"));
                }
            };
            
            // Buscar usuario en la base de datos
            let user = match sqlx::query_as!(
                User,
                r#"
                SELECT id, username, email, display_name, password_hash, created_at, updated_at
                FROM users
                WHERE id = $1
                "#,
                user_id
            )
            .fetch_optional(&**pool)
            .await
            {
                Ok(Some(user)) => user,
                Ok(None) => {
                    warn!("Usuario no encontrado: {}", user_id);
                    return Err(ErrorUnauthorized("Usuario no encontrado"));
                }
                Err(e) => {
                    error!("Error al buscar usuario: {}", e);
                    return Err(ErrorUnauthorized("Error interno"));
                }
            };
            
            // Añadir usuario a las extensiones de la petición
            req.extensions_mut().insert(user);
            
            // Continuar con la petición
            service.call(req).await
        })
    }
}