use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use chrono::Utc;

use crate::domain::entities::file::File;
use crate::domain::repositories::file_repository::{FileRepositoryError, FileRepositoryResult};
use crate::common::errors::DomainError;

/// Repositorio para gestionar las relaciones entre usuarios y archivos
pub struct UserFilesRepository {
    pool: Arc<PgPool>,
}

impl UserFilesRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
    
    // Método auxiliar para mapear errores SQL a errores de dominio
    fn map_sqlx_error(err: sqlx::Error) -> FileRepositoryError {
        match err {
            sqlx::Error::RowNotFound => {
                FileRepositoryError::NotFound("Relación usuario-archivo no encontrada".to_string())
            },
            sqlx::Error::Database(db_err) => {
                if db_err.code().map_or(false, |code| code == "23505") {
                    // Código para violación de unicidad en PostgreSQL
                    FileRepositoryError::AlreadyExists(
                        "Ya existe una relación usuario-archivo para esta ruta".to_string()
                    )
                } else {
                    FileRepositoryError::MappingError(
                        format!("Error de base de datos: {}", db_err)
                    )
                }
            },
            _ => FileRepositoryError::MappingError(
                format!("Error de base de datos: {}", err)
            ),
        }
    }

    /// Registra un archivo como propiedad de un usuario
    pub async fn register_file(&self, user_id: &str, file_id: &str, file_path: &str, size_bytes: i64) -> FileRepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO auth.user_files (
                user_id, file_id, file_path, size_bytes, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
            "#
        )
        .bind(user_id)
        .bind(file_id)
        .bind(file_path)
        .bind(size_bytes)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Actualizar el uso de almacenamiento del usuario
        self.update_user_storage_usage(user_id).await?;

        Ok(())
    }

    /// Actualiza el tamaño de un archivo de usuario
    pub async fn update_file_size(&self, user_id: &str, file_id: &str, size_bytes: i64) -> FileRepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE auth.user_files
            SET 
                size_bytes = $3,
                updated_at = $4
            WHERE user_id = $1 AND file_id = $2
            "#
        )
        .bind(user_id)
        .bind(file_id)
        .bind(size_bytes)
        .bind(Utc::now())
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Actualizar el uso de almacenamiento del usuario
        self.update_user_storage_usage(user_id).await?;

        Ok(())
    }

    /// Elimina un registro de archivo de usuario
    pub async fn unregister_file(&self, user_id: &str, file_id: &str) -> FileRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM auth.user_files
            WHERE user_id = $1 AND file_id = $2
            "#
        )
        .bind(user_id)
        .bind(file_id)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Actualizar el uso de almacenamiento del usuario
        self.update_user_storage_usage(user_id).await?;

        Ok(())
    }

    /// Verifica si un usuario tiene acceso a un archivo
    pub async fn user_has_access(&self, user_id: &str, file_id: &str) -> FileRepositoryResult<bool> {
        let result = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM auth.user_files
            WHERE user_id = $1 AND file_id = $2
            "#
        )
        .bind(user_id)
        .bind(file_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let count: i64 = result.get("count");
        Ok(count > 0)
    }

    /// Obtiene todos los archivos de un usuario
    pub async fn get_user_files(&self, user_id: &str) -> FileRepositoryResult<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT file_id
            FROM auth.user_files
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let file_ids = rows.into_iter()
            .map(|row| row.get::<String, _>("file_id"))
            .collect();

        Ok(file_ids)
    }

    /// Obtiene todos los archivos de un usuario en una carpeta específica
    pub async fn get_user_files_in_folder(&self, user_id: &str, folder_path: &str) -> FileRepositoryResult<Vec<String>> {
        let folder_path_with_slash = if folder_path.ends_with('/') {
            folder_path.to_string()
        } else {
            format!("{}/", folder_path)
        };

        let rows = sqlx::query(
            r#"
            SELECT file_id
            FROM auth.user_files
            WHERE user_id = $1 
            AND (file_path = $2 OR file_path LIKE $3)
            "#
        )
        .bind(user_id)
        .bind(folder_path)
        .bind(format!("{}%", folder_path_with_slash))
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let file_ids = rows.into_iter()
            .map(|row| row.get::<String, _>("file_id"))
            .collect();

        Ok(file_ids)
    }

    /// Cambia el propietario de un archivo
    pub async fn change_file_owner(&self, file_id: &str, old_user_id: &str, new_user_id: &str) -> FileRepositoryResult<()> {
        // Primero verificamos que el archivo pertenezca al usuario antiguo
        let exists = self.user_has_access(old_user_id, file_id).await?;
        if !exists {
            return Err(FileRepositoryError::NotFound(format!(
                "El archivo {} no pertenece al usuario {}", file_id, old_user_id
            )));
        }

        // Obtener los datos del archivo antes de moverlo
        let file_data = sqlx::query(
            r#"
            SELECT file_path, size_bytes 
            FROM auth.user_files
            WHERE user_id = $1 AND file_id = $2
            "#
        )
        .bind(old_user_id)
        .bind(file_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let file_path: String = file_data.get("file_path");
        let size_bytes: i64 = file_data.get("size_bytes");

        // Eliminar la relación anterior
        self.unregister_file(old_user_id, file_id).await?;

        // Crear la nueva relación
        self.register_file(new_user_id, file_id, &file_path, size_bytes).await?;

        Ok(())
    }

    /// Actualiza el uso de almacenamiento de un usuario
    pub async fn update_user_storage_usage(&self, user_id: &str) -> FileRepositoryResult<i64> {
        let result = sqlx::query(
            r#"
            SELECT COALESCE(SUM(size_bytes), 0) as total_size
            FROM auth.user_files
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let total_size: i64 = result.get("total_size");

        // Actualizamos la tabla de usuarios con el nuevo uso de almacenamiento
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                storage_used_bytes = $2,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(total_size)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(total_size)
    }

    /// Obtiene el propietario de un archivo
    pub async fn get_file_owner(&self, file_id: &str) -> FileRepositoryResult<Option<String>> {
        let result = sqlx::query(
            r#"
            SELECT user_id 
            FROM auth.user_files
            WHERE file_id = $1
            "#
        )
        .bind(file_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(result.map(|row| row.get::<String, _>("user_id")))
    }
}