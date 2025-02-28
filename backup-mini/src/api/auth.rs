//! API para autenticación de usuarios

use actix_web::{web, HttpResponse, Scope};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{core::users::User, utils::auth::{generate_token, verify_password}};
use super::{ApiError, ApiResponse};

/// Configuración de rutas para el módulo de autenticación
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/check", web::get().to(check_auth)),
    );
}

/// Datos para inicio de sesión
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Datos para registro de usuarios
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub display_name: Option<String>,
}

/// Respuesta de autenticación
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

/// Endpoint para iniciar sesión
async fn login(
    db: web::Data<sqlx::PgPool>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, display_name, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_optional(db.get_ref())
    .await?
    .ok_or_else(|| ApiError::Unauthorized("Credenciales inválidas".into()))?;

    // Verificar contraseña
    if !verify_password(&credentials.password, &user.password_hash)? {
        return Err(ApiError::Unauthorized("Credenciales inválidas".into()));
    }

    // Generar token JWT
    let token = generate_token(user.id)?;

    // Usuario para devolver (sin password_hash)
    let user_response = User {
        password_hash: "".to_string(),
        ..user
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(AuthResponse {
        token,
        user: user_response,
    })))
}

/// Endpoint para registrar nuevos usuarios
async fn register(
    db: web::Data<sqlx::PgPool>,
    user_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    // Comprobar si el usuario ya existe
    let existing_user = sqlx::query!(
        r#"
        SELECT id FROM users
        WHERE username = $1 OR email = $2
        "#,
        user_data.username,
        user_data.email
    )
    .fetch_optional(db.get_ref())
    .await?;

    if existing_user.is_some() {
        return Err(ApiError::BadRequest("El usuario o email ya existe".into()));
    }

    // Hashear la contraseña
    let password_hash = crate::utils::auth::hash_password(&user_data.password)?;

    // Insertar nuevo usuario
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, display_name, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, display_name, password_hash, created_at, updated_at
        "#,
        user_data.username,
        user_data.email,
        user_data.display_name.clone().unwrap_or_else(|| user_data.username.clone()),
        password_hash
    )
    .fetch_one(db.get_ref())
    .await?;

    // Generar token JWT
    let token = generate_token(user.id)?;

    // Usuario para devolver (sin password_hash)
    let user_response = User {
        password_hash: "".to_string(),
        ..user
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(AuthResponse {
        token,
        user: user_response,
    })))
}

/// Endpoint para verificar si un token es válido
async fn check_auth(
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, ApiError> {
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user.into_inner()))),
        None => Err(ApiError::Unauthorized("Token inválido o expirado".into())),
    }
}