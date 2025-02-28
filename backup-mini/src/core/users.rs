//! Gestión de usuarios y autenticación

use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::utils::auth::{hash_password, verify_password};

/// Información de un usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Permisos de un usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_admin: bool,
    pub storage_quota: Option<i64>,
    pub can_share: bool,
    pub can_create_public_link: bool,
}

/// Datos para crear un nuevo usuario
#[derive(Debug, Deserialize)]
pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_admin: bool,
}

/// Obtiene un usuario por su ID
pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>> {
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
    .context("Error al buscar usuario por ID")?;
    
    Ok(user)
}

/// Obtiene un usuario por su nombre de usuario
pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, display_name, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Error al buscar usuario por nombre")?;
    
    Ok(user)
}

/// Verifica las credenciales de un usuario
pub async fn authenticate_user(pool: &PgPool, username: &str, password: &str) -> Result<Option<User>> {
    // Buscar usuario por nombre
    let user = match get_user_by_username(pool, username).await? {
        Some(user) => user,
        None => return Ok(None),
    };
    
    // Verificar contraseña
    if verify_password(password, &user.password_hash)? {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

/// Crea un nuevo usuario
pub async fn create_user(pool: &PgPool, user_data: CreateUserData) -> Result<User> {
    // Verificar si el usuario o email ya existen
    let existing = sqlx::query!(
        r#"
        SELECT id FROM users
        WHERE username = $1 OR email = $2
        "#,
        user_data.username,
        user_data.email
    )
    .fetch_optional(pool)
    .await?;
    
    if existing.is_some() {
        anyhow::bail!("El nombre de usuario o email ya están en uso");
    }
    
    // Hashear la contraseña
    let password_hash = hash_password(&user_data.password)?;
    
    // Usar nombre de usuario como display_name si no se proporciona
    let display_name = user_data.display_name
        .unwrap_or_else(|| user_data.username.clone());
    
    // Crear transacción
    let mut tx = pool.begin().await?;
    
    // Insertar usuario
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, display_name, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, display_name, password_hash, created_at, updated_at
        "#,
        user_data.username,
        user_data.email,
        display_name,
        password_hash
    )
    .fetch_one(&mut tx)
    .await
    .context("Error al crear usuario")?;
    
    // Crear permisos de usuario
    sqlx::query!(
        r#"
        INSERT INTO user_permissions (user_id, is_admin, can_share, can_create_public_link)
        VALUES ($1, $2, true, true)
        "#,
        user.id,
        user_data.is_admin
    )
    .execute(&mut tx)
    .await
    .context("Error al configurar permisos de usuario")?;
    
    // Confirmar transacción
    tx.commit()
        .await
        .context("Error al confirmar la transacción")?;
    
    info!("Usuario creado: {} ({})", user.username, user.id);
    Ok(user)
}

/// Elimina un usuario y todos sus datos
pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    // Verificar si el usuario existe
    let user = get_user_by_id(pool, user_id).await?
        .ok_or_else(|| anyhow::anyhow!("Usuario no encontrado"))?;
    
    // Iniciar transacción
    let mut tx = pool.begin().await?;
    
    // Buscar los archivos del usuario para eliminarlos físicamente después
    let files = sqlx::query!(
        r#"
        SELECT id, is_directory FROM files
        WHERE user_id = $1 AND is_directory = false
        "#,
        user_id
    )
    .fetch_all(&mut tx)
    .await?;
    
    // Eliminar los archivos de la base de datos
    sqlx::query!(
        r#"
        DELETE FROM files WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&mut tx)
    .await?;
    
    // Eliminar permisos de usuario
    sqlx::query!(
        r#"
        DELETE FROM user_permissions WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&mut tx)
    .await?;
    
    // Eliminar el usuario
    sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1
        "#,
        user_id
    )
    .execute(&mut tx)
    .await?;
    
    // Confirmar transacción
    tx.commit()
        .await
        .context("Error al confirmar la transacción de eliminación de usuario")?;
    
// Eliminar archivos físicos
use crate::core::config::get_config;
use tokio::fs;

let config = get_config();
let files_dir = std::path::PathBuf::from(&config.storage.data_dir)
    .join("files");

for file in files {
    if !file.is_directory {
        let file_path = files_dir.join(file.id.to_string());
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path).await {
                warn!("No se pudo eliminar el archivo físico {}: {}", file.id, e);
            }
        }
    }
}

info!("Usuario eliminado: {} ({})", user.username, user.id);
Ok(())
}

/// Actualiza los datos de un usuario
pub async fn update_user(
pool: &PgPool, 
user_id: Uuid, 
email: Option<String>, 
display_name: Option<String>
) -> Result<User> {
// Verificar si el usuario existe
let user = get_user_by_id(pool, user_id).await?
    .ok_or_else(|| anyhow::anyhow!("Usuario no encontrado"))?;

// Si se proporciona un nuevo email, verificar que no está en uso
if let Some(email) = &email {
    let existing = sqlx::query!(
        r#"
        SELECT id FROM users
        WHERE email = $1 AND id != $2
        "#,
        email,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    
    if existing.is_some() {
        anyhow::bail!("El email ya está en uso");
    }
}

// Actualizar usuario
let updated_user = sqlx::query_as!(
    User,
    r#"
    UPDATE users
    SET 
        email = COALESCE($1, email),
        display_name = COALESCE($2, display_name),
        updated_at = NOW()
    WHERE id = $3
    RETURNING id, username, email, display_name, password_hash, created_at, updated_at
    "#,
    email,
    display_name,
    user_id
)
.fetch_one(pool)
.await
.context("Error al actualizar usuario")?;

info!("Usuario actualizado: {} ({})", updated_user.username, updated_user.id);
Ok(updated_user)
}

/// Cambia la contraseña de un usuario
pub async fn change_password(
pool: &PgPool,
user_id: Uuid,
current_password: &str,
new_password: &str
) -> Result<()> {
// Verificar si el usuario existe
let user = get_user_by_id(pool, user_id).await?
    .ok_or_else(|| anyhow::anyhow!("Usuario no encontrado"))?;

// Verificar contraseña actual
if !verify_password(current_password, &user.password_hash)? {
    anyhow::bail!("Contraseña actual incorrecta");
}

// Hashear nueva contraseña
let new_password_hash = hash_password(new_password)?;

// Actualizar contraseña
sqlx::query!(
    r#"
    UPDATE users
    SET password_hash = $1, updated_at = NOW()
    WHERE id = $2
    "#,
    new_password_hash,
    user_id
)
.execute(pool)
.await
.context("Error al actualizar contraseña")?;

info!("Contraseña actualizada para usuario: {}", user.username);
Ok(())
}

/// Obtiene los permisos de un usuario
pub async fn get_user_permissions(pool: &PgPool, user_id: Uuid) -> Result<UserPermissions> {
// Intentar obtener permisos existentes
let permissions = sqlx::query_as!(
    UserPermissions,
    r#"
    SELECT id, user_id, is_admin, storage_quota, can_share, can_create_public_link
    FROM user_permissions
    WHERE user_id = $1
    "#,
    user_id
)
.fetch_optional(pool)
.await?;

// Si no existen, crear permisos por defecto
match permissions {
    Some(perms) => Ok(perms),
    None => {
        // Verificar que el usuario existe
        if get_user_by_id(pool, user_id).await?.is_none() {
            anyhow::bail!("Usuario no encontrado");
        }
        
        // Crear permisos por defecto
        let default_perms = sqlx::query_as!(
            UserPermissions,
            r#"
            INSERT INTO user_permissions (user_id, is_admin, can_share, can_create_public_link)
            VALUES ($1, false, true, true)
            RETURNING id, user_id, is_admin, storage_quota, can_share, can_create_public_link
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .context("Error al crear permisos por defecto")?;
        
        Ok(default_perms)
    }
}
}

/// Actualiza los permisos de un usuario
pub async fn update_user_permissions(
pool: &PgPool,
user_id: Uuid,
is_admin: Option<bool>,
storage_quota: Option<i64>,
can_share: Option<bool>,
can_create_public_link: Option<bool>
) -> Result<UserPermissions> {
// Verificar que el usuario existe
if get_user_by_id(pool, user_id).await?.is_none() {
    anyhow::bail!("Usuario no encontrado");
}

// Obtener permisos actuales o crear nuevos si no existen
let current_perms = get_user_permissions(pool, user_id).await?;

// Actualizar permisos
let updated_perms = sqlx::query_as!(
    UserPermissions,
    r#"
    UPDATE user_permissions
    SET 
        is_admin = COALESCE($1, is_admin),
        storage_quota = COALESCE($2, storage_quota),
        can_share = COALESCE($3, can_share),
        can_create_public_link = COALESCE($4, can_create_public_link)
    WHERE user_id = $5
    RETURNING id, user_id, is_admin, storage_quota, can_share, can_create_public_link
    "#,
    is_admin,
    storage_quota,
    can_share,
    can_create_public_link,
    user_id
)
.fetch_one(pool)
.await
.context("Error al actualizar permisos de usuario")?;

info!("Permisos actualizados para usuario ID: {}", user_id);
Ok(updated_perms)
}

/// Lista todos los usuarios (función admin)
pub async fn list_all_users(pool: &PgPool) -> Result<Vec<User>> {
let users = sqlx::query_as!(
    User,
    r#"
    SELECT id, username, email, display_name, password_hash, created_at, updated_at
    FROM users
    ORDER BY username
    "#,
)
.fetch_all(pool)
.await
.context("Error al listar usuarios")?;

// Ocultar hashes de contraseñas
let users = users.into_iter()
    .map(|mut user| {
        user.password_hash = "".to_string();
        user
    })
    .collect();

Ok(users)
}