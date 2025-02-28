//! Gestión de archivos y directorios

use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use log::{info, error, warn};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs as tokio_fs;
use uuid::Uuid;

use crate::core::config::get_config;

/// Información de un archivo o directorio
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub mime_type: String,
    pub is_directory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Metadatos para crear o actualizar un archivo
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: Option<Uuid>,
    pub user_id: Uuid, 
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub mime_type: String,
    pub is_directory: bool,
}

/// Crea una entrada de metadatos para un archivo o directorio
pub async fn create_file_metadata(pool: &PgPool, metadata: FileMetadata) -> Result<FileInfo> {
    let id = metadata.id.unwrap_or_else(Uuid::new_v4);
    
    let file_info = sqlx::query_as!(
        FileInfo,
        r#"
        INSERT INTO files (id, user_id, filename, path, size, mime_type, is_directory)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        "#,
        id,
        metadata.user_id,
        metadata.filename,
        metadata.path,
        metadata.size,
        metadata.mime_type,
        metadata.is_directory
    )
    .fetch_one(pool)
    .await
    .context("Error al crear los metadatos del archivo")?;
    
    Ok(file_info)
}

/// Obtiene información de un archivo por ID
pub async fn get_file_by_id(pool: &PgPool, file_id: Uuid) -> Result<Option<FileInfo>> {
    let file = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE id = $1
        "#,
        file_id
    )
    .fetch_optional(pool)
    .await
    .context("Error al buscar archivo por ID")?;
    
    Ok(file)
}

/// Lista archivos y directorios en una ruta para un usuario
pub async fn list_files(pool: &PgPool, user_id: Uuid, path: &str) -> Result<Vec<FileInfo>> {
    let files = sqlx::query_as!(
        FileInfo,
        r#"
        SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
        FROM files
        WHERE user_id = $1 AND path = $2
        ORDER BY is_directory DESC, filename ASC
        "#,
        user_id,
        path
    )
    .fetch_all(pool)
    .await
    .context("Error al listar archivos")?;
    
    Ok(files)
}

/// Elimina un archivo o directorio (metadatos + contenido físico)
pub async fn delete_file(pool: &PgPool, file_id: Uuid, user_id: Uuid) -> Result<()> {
    // Obtener información del archivo
    let file = match get_file_by_id(pool, file_id).await? {
        Some(file) if file.user_id == user_id => file,
        Some(_) => anyhow::bail!("No tienes permiso para eliminar este archivo"),
        None => anyhow::bail!("Archivo no encontrado"),
    };
    
    // Si es un directorio, eliminar recursivamente los contenidos
    if file.is_directory {
        // Buscar todos los archivos y subdirectorios
        let children = sqlx::query_as!(
            FileInfo,
            r#"
            SELECT id, user_id, filename, path, size, mime_type, is_directory, created_at, updated_at
            FROM files
            WHERE user_id = $1 AND path LIKE $2
            "#,
            user_id,
            format!("{}{}/%", file.path, file.filename)
        )
        .fetch_all(pool)
        .await?;
        
        // Eliminar cada hijo recursivamente
        for child in children {
            delete_file(pool, child.id, user_id).await?;
        }
    }
    
    // Ruta del archivo físico
    let config = get_config();
    let file_path = PathBuf::from(&config.storage.data_dir)
        .join("files")
        .join(file_id.to_string());
    
    // Iniciar transacción
    let mut tx = pool.begin().await?;
    
    // Eliminar registro de la base de datos
    sqlx::query!(
        r#"
        DELETE FROM files
        WHERE id = $1 AND user_id = $2
        "#,
        file_id,
        user_id
    )
    .execute(&mut tx)
    .await
    .context("Error al eliminar metadatos del archivo")?;
    
    // Eliminar archivo físico si existe y no es un directorio
    if !file.is_directory && file_path.exists() {
        tokio_fs::remove_file(&file_path)
            .await
            .context("Error al eliminar archivo físico")?;
    }
    
    // Confirmar transacción
    tx.commit()
        .await
        .context("Error al confirmar la transacción")?;
    
    info!("Archivo eliminado: {} ({})", file.filename, file_id);
    Ok(())
}

/// Crea un directorio
pub async fn create_directory(pool: &PgPool, user_id: Uuid, parent_path: &str, dir_name: &str) -> Result<FileInfo> {
    // Validar nombre de directorio
    if dir_name.is_empty() || dir_name.contains('/') {
        anyhow::bail!("Nombre de directorio inválido");
    }
    
    // Comprobar si ya existe
    let existing = sqlx::query!(
        r#"
        SELECT id FROM files
        WHERE user_id = $1 AND path = $2 AND filename = $3
        "#,
        user_id,
        parent_path,
        dir_name
    )
    .fetch_optional(pool)
    .await?;
    
    if existing.is_some() {
        anyhow::bail!("Ya existe un archivo o directorio con ese nombre");
    }
    
    // Crear metadatos
    let dir_info = create_file_metadata(
        pool,
        FileMetadata {
            id: None,
            user_id,
            filename: dir_name.to_string(),
            path: parent_path.to_string(),
            size: 0,
            mime_type: "directory".to_string(),
            is_directory: true,
        }
    ).await?;
    
    info!("Directorio creado: {}/{}", parent_path, dir_name);
    Ok(dir_info)
}

/// Mueve un archivo o directorio
pub async fn move_file(
    pool: &PgPool,
    file_id: Uuid,
    user_id: Uuid,
    new_path: &str,
    new_filename: Option<&str>
) -> Result<FileInfo> {
    // Verificar que el archivo existe y pertenece al usuario
    let file = match get_file_by_id(pool, file_id).await? {
        Some(file) if file.user_id == user_id => file,
        Some(_) => anyhow::bail!("No tienes permiso para mover este archivo"),
        None => anyhow::bail!("Archivo no encontrado"),
    };
    
    // Usar el nombre de archivo existente si no se proporciona uno nuevo
    let new_filename = new_filename.unwrap_or(&file.filename);
    
    // Verificar que no existe un archivo con el mismo nombre en la nueva ubicación
    let existing = sqlx::query!(
        r#"
        SELECT id FROM files
        WHERE user_id = $1 AND path = $2 AND filename = $3 AND id != $4
        "#,
        user_id,
        new_path,
        new_filename,
        file_id
    )
    .fetch_optional(pool)
    .await?;
    
    if existing.is_some() {
        anyhow::bail!("Ya existe un archivo o directorio con ese nombre en la ruta de destino");
    }
    
    // Si es un directorio, actualizar también las rutas de sus descendientes
    if file.is_directory {
        // Actualizar ruta de todos los descendientes
        let old_prefix = format!("{}{}/", file.path, file.filename);
        let new_prefix = format!("{}{}/", new_path, new_filename);
        
        let mut tx = pool.begin().await?;
        
        // Actualizar ruta del directorio
        sqlx::query!(
            r#"
            UPDATE files
            SET path = $1, filename = $2, updated_at = NOW()
            WHERE id = $3
            "#,
            new_path,
            new_filename,
            file_id
        )
        .execute(&mut tx)
        .await?;
        
        // Actualizar rutas de descendientes
        sqlx::query!(
            r#"
            UPDATE files
            SET path = REPLACE(path, $1, $2), updated_at = NOW()
            WHERE user_id = $3 AND path LIKE $4
            "#,
            old_prefix,
            new_prefix,
            user_id,
            format!("{}%", old_prefix)
        )
        .execute(&mut tx)
        .await?;
        
        tx.commit().await?;
    } else {
        // Para archivos simples, solo actualizar su ruta
        sqlx::query!(
            r#"
            UPDATE files
            SET path = $1, filename = $2, updated_at = NOW()
            WHERE id = $3
            "#,
            new_path,
            new_filename,
            file_id
        )
        .execute(pool)
        .await?;
    }
    
    // Obtener y devolver el archivo actualizado
    let updated_file = get_file_by_id(pool, file_id).await?
        .ok_or_else(|| anyhow::anyhow!("No se pudo obtener el archivo actualizado"))?;
    
    info!("Archivo movido: {} -> {}/{}", 
        format!("{}/{}", file.path, file.filename),
        new_path,
        new_filename
    );
    
    Ok(updated_file)
}

/// Calcula el tamaño total de almacenamiento utilizado por un usuario
pub async fn calculate_user_storage(pool: &PgPool, user_id: Uuid) -> Result<i64> {
    let result = sqlx::query!(
        r#"
        SELECT COALESCE(SUM(size), 0) as total_size FROM files
        WHERE user_id = $1 AND is_directory = false
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(result.total_size.unwrap_or(0))
}