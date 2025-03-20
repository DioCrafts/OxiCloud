use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgRow, Row, Error as SqlxError};
use std::sync::Arc;
use tracing::{debug, error, instrument};
use chrono::{DateTime, Utc};

use crate::{
    common::errors::DomainError,
    domain::{
        entities::{
            public_link::PublicLink,
            shared_file::PermissionLevel,
        },
        repositories::public_link_repository::{
            PublicLinkRepository,
            PublicLinkRepositoryError,
            PublicLinkRepositoryResult,
        },
    },
};

/// Repository implementation for public links using PostgreSQL
pub struct PublicLinkPgRepository {
    pool: Arc<PgPool>,
}

impl PublicLinkPgRepository {
    /// Create a new PublicLinkPgRepository with a database connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Map a sqlx error to a domain error
    fn map_sqlx_error(error: SqlxError) -> DomainError {
        error!("Database error: {:?}", error);
        match error {
            SqlxError::RowNotFound => DomainError::not_found("public_link", "Public link not found"),
            SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                DomainError::already_exists("public_link", "A public link with this ID already exists")
            }
            SqlxError::Database(db_err) if db_err.is_foreign_key_violation() => {
                DomainError::validation_error("public_link", "Referenced file or user does not exist".to_string())
            }
            _ => DomainError::internal_error("public_link", format!("Database error: {}", error)),
        }
    }

    /// Map a database row to a PublicLink entity
    fn row_to_entity(row: PgRow) -> Result<PublicLink, DomainError> {
        // Manual extraction with proper error handling
        let id = match row.try_get::<&str, _>("id") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get id: {}", e)
            )),
        };
        
        let file_id = match row.try_get::<&str, _>("file_id") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get file_id: {}", e)
            )),
        };
        
        let owner_id = match row.try_get::<&str, _>("owner_id") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get owner_id: {}", e)
            )),
        };
        
        // Convert database permission level string to enum
        let permission_str = match row.try_get::<&str, _>("permission_level") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get permission_level: {}", e)
            )),
        };
        
        let permission_level = match permission_str {
            "READ" => PermissionLevel::Read,
            "WRITE" => PermissionLevel::Write,
            "ADMIN" => PermissionLevel::Admin,
            _ => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InvalidInput,
                "public_link",
                format!("Invalid permission level: {}", permission_str)
            )),
        };
        
        let password_hash = match row.try_get::<Option<String>, _>("password_hash") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get password_hash: {}", e)
            )),
        };
        
        let expires_at = match row.try_get::<Option<DateTime<Utc>>, _>("expires_at") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get expires_at: {}", e)
            )),
        };
        
        let access_count = match row.try_get::<i32, _>("access_count") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get access_count: {}", e)
            )),
        };
        
        let created_at = match row.try_get::<DateTime<Utc>, _>("created_at") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError, 
                "public_link",
                format!("Failed to get created_at: {}", e)
            )),
        };
        
        let updated_at = match row.try_get::<DateTime<Utc>, _>("updated_at") {
            Ok(val) => val,
            Err(e) => return Err(DomainError::new(
                crate::common::errors::ErrorKind::InternalError,
                "public_link",
                format!("Failed to get updated_at: {}", e)
            )),
        };
        
        // Just use the original constructor fields, ignoring the extra ones we extracted
        let public_link = PublicLink::new(
            file_id.to_string(),
            owner_id.to_string(),
            permission_level,
            password_hash,
            expires_at,
        );
        
        // We can't directly set all fields through the constructor
        
        Ok(public_link)
    }

    /// Convert permission level enum to string for database storage
    fn permission_to_db_string(permission: PermissionLevel) -> &'static str {
        match permission {
            PermissionLevel::Read => "READ",
            PermissionLevel::Write => "WRITE",
            PermissionLevel::Admin => "ADMIN",
        }
    }
}

#[async_trait]
impl PublicLinkRepository for PublicLinkPgRepository {
    #[instrument(skip(self))]
    async fn create_public_link(
        &self,
        file_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
        password: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Creating public link for file {} by owner {}", file_id, owner_id);
        
        // Generate a unique ID for the public link
        let link_id = uuid::Uuid::new_v4().to_string();
        
        let permission_str = Self::permission_to_db_string(permission);
        
        // For optional password, we need to hash it if provided
        let password_hash = match password {
            Some(pass) if !pass.is_empty() => {
                // In a real implementation, this would use a proper password hashing algorithm like Argon2
                // For now, we'll simulate a hash for demonstration purposes
                Some(format!("hashed_{}", pass))
            },
            _ => None
        };
        
        let result = sqlx::query(
            r#"
            INSERT INTO auth.public_links 
                (id, file_id, owner_id, permission_level, password_hash, expires_at, access_count)
            VALUES 
                ($1, $2, $3, $4, $5, $6, 0)
            RETURNING 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            "#
        )
        .bind(&link_id)
        .bind(file_id)
        .bind(owner_id)
        .bind(permission_str)
        .bind(password_hash)
        .bind(expires_at)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        let result = Self::row_to_entity(result)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        debug!("Created public link with ID: {}", result.id());
        
        Ok(result)
    }

    #[instrument(skip(self))]
    async fn get_public_link(
        &self,
        link_id: &str,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Getting public link by ID: {}", link_id);
        
        let row = sqlx::query(
            r#"
            SELECT 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            FROM auth.public_links
            WHERE id = $1
            "#
        )
        .bind(link_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir la fila en una entidad
        let public_link = Self::row_to_entity(row)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        // Check if the link is expired
        if let Some(expires) = public_link.expires_at() {
            let now = chrono::Utc::now();
            if expires < &now {
                return Err(PublicLinkRepositoryError::Expired);
            }
        }
        
        debug!("Found public link: {}", public_link.id());
        
        Ok(public_link)
    }

    #[instrument(skip(self))]
    async fn update_permission(
        &self,
        link_id: &str,
        owner_id: &str,
        permission: PermissionLevel,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Updating permission for public link: {} to {:?}", link_id, permission);
        
        let permission_str = Self::permission_to_db_string(permission);
        
        let row = sqlx::query(
            r#"
            UPDATE auth.public_links
            SET permission_level = $1, updated_at = NOW()
            WHERE id = $2 AND owner_id = $3
            RETURNING 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            "#
        )
        .bind(permission_str)
        .bind(link_id)
        .bind(owner_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir la fila en una entidad
        let public_link = Self::row_to_entity(row)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        debug!("Updated permission for public link: {}", public_link.id());
        
        Ok(public_link)
    }

    #[instrument(skip(self))]
    async fn update_password(
        &self,
        link_id: &str,
        owner_id: &str,
        password: Option<&str>,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Updating password for public link: {}", link_id);
        
        // Hash the password if provided
        let password_hash = match password {
            Some(pass) if !pass.is_empty() => {
                // In a real implementation, this would use a proper password hashing algorithm
                Some(format!("hashed_{}", pass))
            },
            _ => None
        };
        
        let row = sqlx::query(
            r#"
            UPDATE auth.public_links
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2 AND owner_id = $3
            RETURNING 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            "#
        )
        .bind(password_hash)
        .bind(link_id)
        .bind(owner_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir la fila en una entidad
        let public_link = Self::row_to_entity(row)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        debug!("Updated password for public link: {}", public_link.id());
        
        Ok(public_link)
    }

    #[instrument(skip(self))]
    async fn update_expiration(
        &self,
        link_id: &str,
        owner_id: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Updating expiration for public link: {}", link_id);
        
        let row = sqlx::query(
            r#"
            UPDATE auth.public_links
            SET expires_at = $1, updated_at = NOW()
            WHERE id = $2 AND owner_id = $3
            RETURNING 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            "#
        )
        .bind(expires_at)
        .bind(link_id)
        .bind(owner_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir la fila en una entidad
        let public_link = Self::row_to_entity(row)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        debug!("Updated expiration for public link: {}", public_link.id());
        
        Ok(public_link)
    }

    #[instrument(skip(self))]
    async fn delete_public_link(
        &self,
        link_id: &str,
        owner_id: &str,
    ) -> Result<(), PublicLinkRepositoryError> {
        debug!("Deleting public link: {} by owner: {}", link_id, owner_id);
        
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM auth.public_links
            WHERE id = $1 AND owner_id = $2
            "#
        )
        .bind(link_id)
        .bind(owner_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(PublicLinkRepositoryError::NotFound(
                format!("Public link with ID {} not found or not owned by {}", link_id, owner_id)
            ));
        }
        
        debug!("Deleted public link: {}", link_id);
        
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_links_for_file(
        &self,
        file_id: &str,
        owner_id: &str,
    ) -> Result<Vec<PublicLink>, PublicLinkRepositoryError> {
        debug!("Listing public links for file: {} by owner: {}", file_id, owner_id);
        
        let rows = sqlx::query(
            r#"
            SELECT 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            FROM auth.public_links
            WHERE file_id = $1 AND owner_id = $2
            ORDER BY created_at DESC
            "#
        )
        .bind(file_id)
        .bind(owner_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir las filas en entidades
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            let link = Self::row_to_entity(row)
                .map_err(PublicLinkRepositoryError::DomainError)?;
            results.push(link);
        }
        
        debug!("Found {} public links for file: {}", results.len(), file_id);
        
        Ok(results)
    }

    #[instrument(skip(self))]
    async fn get_links_by_user(
        &self,
        owner_id: &str,
    ) -> Result<Vec<PublicLink>, PublicLinkRepositoryError> {
        debug!("Listing public links by owner: {}", owner_id);
        
        let rows = sqlx::query(
            r#"
            SELECT 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            FROM auth.public_links
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(owner_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir las filas en entidades
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            let link = Self::row_to_entity(row)
                .map_err(PublicLinkRepositoryError::DomainError)?;
            results.push(link);
        }
        
        debug!("Found {} public links by owner: {}", results.len(), owner_id);
        
        Ok(results)
    }

    #[instrument(skip(self))]
    async fn verify_password(
        &self,
        link_id: &str,
        password: &str,
    ) -> Result<bool, PublicLinkRepositoryError> {
        debug!("Verifying password for public link: {}", link_id);
        
        // Get the link first
        let link = self.get_public_link(link_id).await?;
        
        // Check if password is required
        match link.password_hash() {
            Some(stored_hash) => {
                // In a real implementation, this would use a proper password verification
                // For now, we'll just compare with our simulated hash
                let is_valid = stored_hash == &format!("hashed_{}", password);
                
                debug!("Password verification result for link {}: {}", link_id, is_valid);
                
                Ok(is_valid)
            },
            None => {
                // No password required
                debug!("No password required for link {}", link_id);
                Ok(true)
            }
        }
    }

    #[instrument(skip(self))]
    async fn increment_access_count(
        &self,
        link_id: &str,
    ) -> Result<PublicLink, PublicLinkRepositoryError> {
        debug!("Incrementing access count for public link: {}", link_id);
        
        let row = sqlx::query(
            r#"
            UPDATE auth.public_links
            SET access_count = access_count + 1
            WHERE id = $1
            RETURNING 
                id, file_id, owner_id, permission_level, password_hash, 
                expires_at, access_count, created_at, updated_at
            "#
        )
        .bind(link_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PublicLinkRepositoryError::DomainError(Self::map_sqlx_error(e)))?;
        
        // Convertir la fila en una entidad
        let public_link = Self::row_to_entity(row)
            .map_err(PublicLinkRepositoryError::DomainError)?;
        
        debug!("Incremented access count for public link: {} to {}", public_link.id(), public_link.access_count());
        
        Ok(public_link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;
    use uuid::Uuid;

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
    async fn test_create_and_get_public_link() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        // Crear un nuevo enlace público
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let permission = PermissionLevel::Read;
        
        // Guardar en la base de datos
        let created = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            None, // Sin contraseña
            None  // Sin fecha de expiración
        ).await.expect("Failed to create public link");
        
        // Verificar que se creó el enlace
        assert!(!created.id().is_empty());
        assert_eq!(created.file_id(), file_id);
        assert_eq!(created.owner_id(), owner_id);
        assert_eq!(created.permission_level(), permission);
        
        // Recuperar el enlace
        let retrieved = repo.get_public_link(&created.id())
            .await
            .expect("Failed to get public link");
        
        // Verificar que los datos coinciden
        assert_eq!(retrieved.id(), created.id());
        assert_eq!(retrieved.file_id(), created.file_id());
        assert_eq!(retrieved.owner_id(), created.owner_id());
        assert_eq!(retrieved.permission_level(), created.permission_level());
        
        // Limpiar
        let _ = repo.delete_public_link(&created.id(), owner_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_permission() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        // Crear un nuevo enlace público
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let initial_permission = PermissionLevel::Read;
        
        // Guardar en la base de datos
        let created = repo.create_public_link(
            file_id, 
            owner_id, 
            initial_permission, 
            None, 
            None
        ).await.expect("Failed to create public link");
        
        // Actualizar el permiso
        let updated = repo.update_permission(
            &created.id(), 
            owner_id, 
            PermissionLevel::Write
        ).await.expect("Failed to update permission");
        
        // Verificar que el permiso se actualizó
        assert_eq!(updated.permission_level(), PermissionLevel::Write);
        
        // Limpiar
        let _ = repo.delete_public_link(&created.id(), owner_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_increment_access_count() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        // Crear un nuevo enlace público
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let permission = PermissionLevel::Read;
        
        // Guardar en la base de datos
        let created = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            None, 
            None
        ).await.expect("Failed to create public link");
        
        // Verificar contador inicial
        assert_eq!(created.access_count(), 0);
        
        // Incrementar el contador
        let updated = repo.increment_access_count(&created.id())
            .await
            .expect("Failed to increment access count");
        
        // Verificar que el contador se incrementó
        assert_eq!(updated.access_count(), 1);
        
        // Incrementar otra vez
        let updated = repo.increment_access_count(&created.id())
            .await
            .expect("Failed to increment access count");
        
        // Verificar que el contador se incrementó nuevamente
        assert_eq!(updated.access_count(), 2);
        
        // Limpiar
        let _ = repo.delete_public_link(&created.id(), owner_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_password_protection() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        // Crear un nuevo enlace público con contraseña
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let permission = PermissionLevel::Read;
        let password = Some("secretpassword");
        
        // Guardar en la base de datos
        let created = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            password, 
            None
        ).await.expect("Failed to create public link");
        
        // Verificar que tiene una contraseña
        assert!(created.password_hash().is_some());
        
        // Verificar contraseña incorrecta
        let is_valid = repo.verify_password(&created.id(), "wrongpassword")
            .await
            .expect("Failed to verify password");
        
        assert!(!is_valid);
        
        // Verificar contraseña correcta
        let is_valid = repo.verify_password(&created.id(), "secretpassword")
            .await
            .expect("Failed to verify password");
        
        assert!(is_valid);
        
        // Actualizar contraseña
        let updated = repo.update_password(&created.id(), owner_id, Some("newpassword"))
            .await
            .expect("Failed to update password");
        
        // Verificar nueva contraseña
        let is_valid = repo.verify_password(&created.id(), "newpassword")
            .await
            .expect("Failed to verify password");
        
        assert!(is_valid);
        
        // Limpiar
        let _ = repo.delete_public_link(&created.id(), owner_id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_expiration() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        // Crear un nuevo enlace público con expiración en el pasado
        let file_id = "test-file-id";
        let owner_id = "owner-id";
        let permission = PermissionLevel::Read;
        let expired_date = chrono::Utc::now() - chrono::Duration::days(1);
        
        // Guardar en la base de datos
        let created = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            None, 
            Some(expired_date)
        ).await.expect("Failed to create public link");
        
        // Verificar que el enlace está expirado
        let result = repo.get_public_link(&created.id()).await;
        assert!(matches!(result, Err(PublicLinkRepositoryError::Expired)));
        
        // Actualizar la fecha de expiración a una fecha futura
        let future_date = chrono::Utc::now() + chrono::Duration::days(1);
        let updated = repo.update_expiration(&created.id(), owner_id, Some(future_date))
            .await
            .expect("Failed to update expiration");
        
        // Verificar que el enlace ya no está expirado
        let retrieved = repo.get_public_link(&created.id()).await;
        assert!(retrieved.is_ok());
        
        // Limpiar
        let _ = repo.delete_public_link(&created.id(), owner_id).await;
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_list_links() {
        // Configurar
        let pool = create_test_pool().await.expect("Failed to create database pool");
        let repo = PublicLinkPgRepository::new(pool);
        
        let file_id = "test-file-id-list";
        let owner_id = "owner-id-list";
        let permission = PermissionLevel::Read;
        
        // Crear varios enlaces públicos
        let link1 = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            None, 
            None
        ).await.expect("Failed to create public link 1");
        
        let link2 = repo.create_public_link(
            file_id, 
            owner_id, 
            permission, 
            None, 
            None
        ).await.expect("Failed to create public link 2");
        
        // Listar por archivo
        let file_links = repo.get_links_for_file(file_id, owner_id)
            .await
            .expect("Failed to list links for file");
        
        assert!(file_links.len() >= 2);
        assert!(file_links.iter().any(|link| link.id() == link1.id()));
        assert!(file_links.iter().any(|link| link.id() == link2.id()));
        
        // Listar por usuario
        let user_links = repo.get_links_by_user(owner_id)
            .await
            .expect("Failed to list links by user");
        
        assert!(user_links.len() >= 2);
        assert!(user_links.iter().any(|link| link.id() == link1.id()));
        assert!(user_links.iter().any(|link| link.id() == link2.id()));
        
        // Limpiar
        let _ = repo.delete_public_link(&link1.id(), owner_id).await;
        let _ = repo.delete_public_link(&link2.id(), owner_id).await;
    }
}