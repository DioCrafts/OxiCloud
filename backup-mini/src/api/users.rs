//! API para gestión de usuarios

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::core::users::User;
use crate::utils::auth::hash_password;
use super::{ApiError, ApiResponse};

/// Configuración de rutas para el módulo de usuarios
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_current_user))
            .route("/me", web::put().to(update_user)),
    );
}

/// Datos para actualizar un usuario
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    email: Option<String>,
    display_name: Option<String>,
    current_password: Option<String>,
    new_password: Option<String>,
}

/// Endpoint para obtener información del usuario actual
async fn get_current_user(
    user: web::ReqData<User>,
) -> Result<HttpResponse, ApiError> {
    // Devolver usuario sin password_hash
    let mut user = user.into_inner();
    user.password_hash = "".to_string();
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

/// Endpoint para actualizar información del usuario
async fn update_user(
    db: web::Data<sqlx::PgPool>,
    user: web::ReqData<User>,
    update_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut user = user.into_inner();
    let user_id = user.id;
    
    // Iniciar transacción para mantener consistencia
    let mut tx = db.begin().await?;
    
    // Si se quiere cambiar la contraseña, verificar la actual
    if update_data.new_password.is_some() {
        // La contraseña actual es requerida
        let current_password = update_data.current_password
            .as_ref()
            .ok_or_else(|| ApiError::BadRequest("Se requiere la contraseña actual".into()))?;
        
        // Verificar contraseña actual
        if !crate::utils::auth::verify_password(current_password, &user.password_hash)? {
            return Err(ApiError::Unauthorized("Contraseña actual incorrecta".into()));
        }
        
        // Actualizar contraseña
        let new_password_hash = hash_password(update_data.new_password.as_ref().unwrap())?;
        
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            new_password_hash,
            user_id
        )
        .execute(&mut tx)
        .await?;
        
        user.password_hash = new_password_hash;
    }
    
    // Actualizar otros campos si existen
    if update_data.email.is_some() || update_data.display_name.is_some() {
        let email = update_data.email.clone().unwrap_or_else(|| user.email.clone());
        let display_name = update_data.display_name.clone().unwrap_or_else(|| user.display_name.clone());
        
        sqlx::query!(
            r#"
            UPDATE users
            SET 
                email = $1,
                display_name = $2,
                updated_at = NOW()
            WHERE id = $3
            "#,
            email,
            display_name,
            user_id
        )
        .execute(&mut tx)
        .await?;
        
        user.email = email;
        user.display_name = display_name;
    }
    
    // Commit de la transacción
    tx.commit().await?;
    
    // Devolver usuario actualizado (sin password_hash)
    user.password_hash = "".to_string();
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}