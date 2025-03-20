use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgRow, Row, Error as SqlxError};
use std::sync::Arc;
use tracing::{debug, error, instrument};

use crate::{
    common::errors::DomainError,
    domain::{
        entities::shared_file::{SharedFile, PermissionLevel},
        repositories::shared_file_repository::{
            SharedFileRepository, 
            SharedFileRepositoryError,
            SharedFileRepositoryResult,
        },
    },
};

/// Repository implementation for shared files using PostgreSQL
pub struct SharedFilePgRepository {
    pool: Arc<PgPool>,
}

impl SharedFilePgRepository {
    /// Create a new SharedFilePgRepository with a database connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Map a sqlx error to a domain error
    fn map_sqlx_error(error: SqlxError) -> DomainError {
        error!("Database error: {:?}", error);
        match error {
            SqlxError::RowNotFound => DomainError::not_found("shared_file", "Shared file not found"),
            SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                DomainError::already_exists("shared_file", "File is already shared with this user")
            }
            SqlxError::Database(db_err) if db_err.is_foreign_key_violation() => {
                DomainError::validation_error("shared_file", "Referenced file or user does not exist".to_string())
            }
            _ => DomainError::internal_error("shared_file", format!("Database error: {}", error)),
        }
    }

    /// Map a database row to a SharedFile entity
    fn row_to_entity(row: PgRow) -> Result<SharedFile, DomainError> {
        // Manual extraction with proper error handling
        let id = match row.try_get::<i32, _>("id") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get id: {}", e)
            )),
        };
        
        let file_id = match row.try_get::<&str, _>("file_id") {
            Ok(val) => val.to_string(),
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get file_id: {}", e)
            )),
        };
        
        let owner_id = match row.try_get::<&str, _>("owner_id") {
            Ok(val) => val.to_string(),
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get owner_id: {}", e)
            )),
        };
        
        let user_id = match row.try_get::<&str, _>("user_id") {
            Ok(val) => val.to_string(),
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get user_id: {}", e)
            )),
        };
        
        // Convert database permission level string to enum
        let permission_str = match row.try_get::<&str, _>("permission_level") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get permission_level: {}", e)
            )),
        };
        
        let permission_level = match permission_str {
            "READ" => PermissionLevel::Read,
            "WRITE" => PermissionLevel::Write,
            "ADMIN" => PermissionLevel::Admin,
            _ => return Err(DomainError::validation_error(
                "shared_file",
                format!("Invalid permission level: {}", permission_str)
            )),
        };
        
        let created_at = match row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get created_at: {}", e)
            )),
        };
        
        let updated_at = match row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::internal_error(
                "shared_file",
                format!("Failed to get updated_at: {}", e)
            )),
        };
        
        // Usar el constructor with_id que acepta un ID existente y timestamps
        Ok(SharedFile::with_id(
            id,
            file_id,
            owner_id,
            user_id,
            permission_level,
            created_at,
            updated_at,
        ))
    }

    /// Map a SharedFile entity to database parameters for queries
    fn entity_to_params(shared_file: &SharedFile) -> (String, String, String, String, String) {
        let permission_str = match shared_file.permission_level() {
            PermissionLevel::Read => "READ",
            PermissionLevel::Write => "WRITE",
            PermissionLevel::Admin => "ADMIN",
        };
        
        (
            shared_file.file_id().to_string(),
            shared_file.owner_id().to_string(),
            shared_file.user_id().to_string(),
            permission_str.to_string(),
            shared_file.id().map(|id| id.to_string()).unwrap_or_default(),
        )
    }
}

#[async_trait]
impl SharedFileRepository for SharedFilePgRepository {
    #[instrument(skip(self))]
    async fn share_file_with_user(
        &self,
        file_id: &str,
        owner_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> Result<SharedFile, SharedFileRepositoryError> {
        debug!("Sharing file {} from owner {} with user {} (permission: {:?})", 
               file_id, owner_id, user_id, permission);
        
        let permission_str = match permission {
            PermissionLevel::Read => "READ",
            PermissionLevel::Write => "WRITE",
            PermissionLevel::Admin => "ADMIN",
        };
        
        let row = sqlx::query(
            r#"
            INSERT INTO auth.shared_files (file_id, owner_id, user_id, permission_level)
            VALUES ($1, $2, $3, $4)
            RETURNING id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            "#
        )
        .bind(file_id)
        .bind(owner_id)
        .bind(user_id)
        .bind(permission_str)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        let shared_file = Self::row_to_entity(row)
            .map_err(SharedFileRepositoryError::DomainError)?;
        
        debug!("Created shared file with ID: {:?}", shared_file.id());
        
        Ok(shared_file)
    }

    #[instrument(skip(self))]
    async fn update_permission(
        &self,
        file_id: &str,
        user_id: &str,
        permission: PermissionLevel,
    ) -> Result<SharedFile, SharedFileRepositoryError> {
        debug!("Updating permission for file {} and user {} to {:?}", 
               file_id, user_id, permission);
        
        let permission_str = match permission {
            PermissionLevel::Read => "READ",
            PermissionLevel::Write => "WRITE",
            PermissionLevel::Admin => "ADMIN",
        };
        
        let row = sqlx::query(
            r#"
            UPDATE auth.shared_files
            SET permission_level = $1, updated_at = NOW()
            WHERE file_id = $2 AND user_id = $3
            RETURNING id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            "#
        )
        .bind(permission_str)
        .bind(file_id)
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        let shared_file = Self::row_to_entity(row)
            .map_err(SharedFileRepositoryError::DomainError)?;
            
        debug!("Updated shared file: {:?}", shared_file);
        
        Ok(shared_file)
    }

    #[instrument(skip(self))]
    async fn unshare_file(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> Result<(), SharedFileRepositoryError> {
        debug!("Unsharing file {} from user {}", file_id, user_id);
        
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM auth.shared_files
            WHERE file_id = $1 AND user_id = $2
            "#
        )
        .bind(file_id)
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(SharedFileRepositoryError::NotFound(format!(
                "Shared file with file_id {} and user_id {} not found", file_id, user_id
            )));
        }
        
        debug!("Deleted shared file with file_id: {} and user_id: {}", file_id, user_id);
        
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_shared_file(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> Result<SharedFile, SharedFileRepositoryError> {
        debug!("Getting shared file by file_id: {} and user_id: {}", file_id, user_id);
        
        let row = sqlx::query(
            r#"
            SELECT id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            FROM auth.shared_files
            WHERE file_id = $1 AND user_id = $2
            "#
        )
        .bind(file_id)
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        let shared_file = Self::row_to_entity(row)
            .map_err(SharedFileRepositoryError::DomainError)?;
        
        debug!("Found shared file: {:?}", shared_file);
        
        Ok(shared_file)
    }

    #[instrument(skip(self))]
    async fn get_files_shared_with_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<SharedFile>, SharedFileRepositoryError> {
        debug!("Listing files shared with user: {}", user_id);
        
        let rows = sqlx::query(
            r#"
            SELECT id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            FROM auth.shared_files
            WHERE user_id = $1
            ORDER BY updated_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Procesar cada fila para convertirla en una entidad SharedFile
        let mut shared_files = Vec::with_capacity(rows.len());
        for row in rows {
            let shared_file = Self::row_to_entity(row)
                .map_err(SharedFileRepositoryError::DomainError)?;
            shared_files.push(shared_file);
        }
        
        debug!("Found {} files shared with user {}", shared_files.len(), user_id);
        
        Ok(shared_files)
    }

    #[instrument(skip(self))]
    async fn get_users_with_access(
        &self,
        file_id: &str,
    ) -> Result<Vec<SharedFile>, SharedFileRepositoryError> {
        debug!("Listing users with access to file: {}", file_id);
        
        let rows = sqlx::query(
            r#"
            SELECT id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            FROM auth.shared_files
            WHERE file_id = $1
            ORDER BY user_id
            "#
        )
        .bind(file_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Process each row to a SharedFile entity
        let mut shared_files = Vec::with_capacity(rows.len());
        for row in rows {
            let shared_file = Self::row_to_entity(row)
                .map_err(SharedFileRepositoryError::DomainError)?;
            shared_files.push(shared_file);
        }
        
        debug!("Found {} users with access to file {}", shared_files.len(), file_id);
        
        Ok(shared_files)
    }

    #[instrument(skip(self))]
    async fn get_files_shared_by_user(
        &self,
        owner_id: &str,
    ) -> Result<Vec<SharedFile>, SharedFileRepositoryError> {
        debug!("Listing files shared by user: {}", owner_id);
        
        let rows = sqlx::query(
            r#"
            SELECT id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            FROM auth.shared_files
            WHERE owner_id = $1
            ORDER BY updated_at DESC
            "#
        )
        .bind(owner_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Process each row to a SharedFile entity
        let mut shared_files = Vec::with_capacity(rows.len());
        for row in rows {
            let shared_file = Self::row_to_entity(row)
                .map_err(SharedFileRepositoryError::DomainError)?;
            shared_files.push(shared_file);
        }
        
        debug!("Found {} files shared by user {}", shared_files.len(), owner_id);
        
        Ok(shared_files)
    }

    #[instrument(skip(self))]
    async fn check_user_has_access(
        &self,
        file_id: &str,
        user_id: &str,
    ) -> Result<Option<PermissionLevel>, SharedFileRepositoryError> {
        debug!("Checking if user {} has access to file {}", user_id, file_id);
        
        let result_row = sqlx::query(
            r#"
            SELECT id, file_id, owner_id, user_id, permission_level, created_at, updated_at
            FROM auth.shared_files
            WHERE file_id = $1 AND user_id = $2
            "#
        )
        .bind(file_id)
        .bind(user_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| SharedFileRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        let shared_file_opt = match result_row {
            Some(row) => {
                let shared_file = Self::row_to_entity(row)
                    .map_err(SharedFileRepositoryError::DomainError)?;
                Some(shared_file)
            },
            None => None,
        };
        
        let permission = shared_file_opt.map(|shared_file| shared_file.permission_level().clone());
        
        match &permission {
            Some(level) => debug!("User {} has access to file {} with permission: {:?}", 
                                  user_id, file_id, level),
            None => debug!("User {} does not have access to file {}", user_id, file_id),
        }
        
        Ok(permission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;
    use crate::domain::entities::shared_file::PermissionLevel;

    // Función auxiliar para crear un pool de conexiones para pruebas
    async fn create_test_pool() -> Result<Arc<PgPool>, SqlxError> {
        // Esta URL debería apuntar a tu base de datos de pruebas
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for testing");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        
        Ok(Arc::new(pool))
    }

    #[tokio::test]
    #[ignore] // Ignorar por defecto para no depender de una base de datos externa
    async fn test_share_and_get_file() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = SharedFilePgRepository::new(pool);
        
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let user_id = "user-id";
        let permission = PermissionLevel::Read;
        
        // Compartir archivo
        let shared_file = repo.share_file_with_user(file_id, owner_id, user_id, permission)
            .await
            .expect("Failed to share file");
        
        // Verificar que se asignó un ID
        assert!(shared_file.id().is_some());
        
        // Recuperar el archivo compartido
        let retrieved = repo.get_shared_file(file_id, user_id)
            .await
            .expect("Failed to get shared file");
        
        // Verificar que los datos coinciden
        assert_eq!(retrieved.file_id(), shared_file.file_id());
        assert_eq!(retrieved.owner_id(), shared_file.owner_id());
        assert_eq!(retrieved.user_id(), shared_file.user_id());
        assert_eq!(retrieved.permission_level(), shared_file.permission_level());
        
        // Limpiar
        let _ = repo.unshare_file(file_id, user_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_permission_level() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = SharedFilePgRepository::new(pool);
        
        let file_id = "test-file-id-2";
        let owner_id = "owner-id";
        let user_id = "user-id";
        let initial_permission = PermissionLevel::Read;
        
        // Compartir archivo con permiso inicial
        let shared_file = repo.share_file_with_user(file_id, owner_id, user_id, initial_permission)
            .await
            .expect("Failed to share file");
        
        // Modificar el permiso a Write
        let updated = repo.update_permission(file_id, user_id, PermissionLevel::Write)
            .await
            .expect("Failed to update permission");
        
        // Verificar que el permiso se actualizó
        assert_eq!(updated.permission_level(), PermissionLevel::Write);
        
        // Limpiar
        let _ = repo.unshare_file(file_id, user_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_unshare_file() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = SharedFilePgRepository::new(pool);
        
        let file_id = "test-file-id-3";
        let owner_id = "owner-id";
        let user_id = "user-id";
        let permission = PermissionLevel::Read;
        
        // Compartir archivo
        repo.share_file_with_user(file_id, owner_id, user_id, permission)
            .await
            .expect("Failed to share file");
        
        // Dejar de compartir el archivo
        repo.unshare_file(file_id, user_id)
            .await
            .expect("Failed to unshare file");
        
        // Verificar que se eliminó (debería devolver un error NotFound)
        let result = repo.get_shared_file(file_id, user_id).await;
        assert!(result.is_err());
        
        // Verificar que el error es del tipo correcto
        match result {
            Err(SharedFileRepositoryError::DomainError(DomainError::NotFound(_))) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_check_user_has_access() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = SharedFilePgRepository::new(pool);
        
        let file_id = "test-file-id-4";
        let owner_id = "owner-id";
        let user_id = "user-id";
        let permission = PermissionLevel::Write;
        
        // Compartir archivo
        repo.share_file_with_user(file_id, owner_id, user_id, permission)
            .await
            .expect("Failed to share file");
        
        // Verificar acceso
        let access = repo.check_user_has_access(file_id, user_id)
            .await
            .expect("Failed to check access");
        
        // El usuario debe tener acceso con nivel Write
        assert!(access.is_some());
        assert_eq!(access.unwrap(), PermissionLevel::Write);
        
        // Verificar que otro usuario no tiene acceso
        let no_access = repo.check_user_has_access(file_id, "other-user")
            .await
            .expect("Failed to check access");
        
        assert!(no_access.is_none());
        
        // Limpiar
        let _ = repo.unshare_file(file_id, user_id).await;
    }
}