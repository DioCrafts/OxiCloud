use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use futures::future::BoxFuture;

use crate::domain::entities::user::{User, UserRole};
use crate::domain::repositories::user_repository::{UserRepository, UserRepositoryError, UserRepositoryResult, StorageStats};
use crate::application::ports::auth_ports::UserStoragePort;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::pg::transaction_utils::with_transaction;

// Implement From<sqlx::Error> for UserRepositoryError to allow automatic conversions
impl From<sqlx::Error> for UserRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        UserPgRepository::map_sqlx_error(err)
    }
}

pub struct UserPgRepository {
    pool: Arc<PgPool>,
}

impl UserPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
    
    // Helper method to map SQL errors to domain errors
    pub fn map_sqlx_error(err: sqlx::Error) -> UserRepositoryError {
        match err {
            sqlx::Error::RowNotFound => {
                UserRepositoryError::NotFound("User not found".to_string())
            },
            sqlx::Error::Database(db_err) => {
                if db_err.code().map_or(false, |code| code == "23505") {
                    // PostgreSQL uniqueness violation code
                    UserRepositoryError::AlreadyExists(
                        "User or email already exists".to_string()
                    )
                } else {
                    UserRepositoryError::DatabaseError(
                        format!("Database error: {}", db_err)
                    )
                }
            },
            _ => UserRepositoryError::DatabaseError(
                format!("Database error: {}", err)
            ),
        }
    }
}

#[async_trait]
impl UserRepository for UserPgRepository {
    /// Creates a new user using a transaction
    async fn create_user(&self, user: User) -> UserRepositoryResult<User> {
        // Create a copy of the user for the closure
        let user_clone = user.clone();
        
        with_transaction(
            &self.pool,
            "create_user",
            |tx| {
                // We need to move the closure into a BoxFuture to return inside
                // the with_transaction call
                Box::pin(async move {
                    // Use getters to extract the values
                    // Convert user.role() to string to pass it as plain text
                    let role_str = user_clone.role().to_string();
                    
                    // Modify the SQL to do an explicit cast to the auth.userrole type
                    let _result = sqlx::query(
                        r#"
                        INSERT INTO auth.users (
                            id, username, email, password_hash, role, 
                            storage_quota_bytes, storage_used_bytes, 
                            created_at, updated_at, last_login_at, active,
                            oidc_provider, oidc_subject
                        ) VALUES (
                            $1, $2, $3, $4, $5::auth.userrole, $6, $7, $8, $9, $10, $11,
                            $12, $13
                        )
                        RETURNING *
                        "#
                    )
                    .bind(user_clone.id())
                    .bind(user_clone.username())
                    .bind(user_clone.email())
                    .bind(user_clone.password_hash())
                    .bind(&role_str) // Convert to string but with explicit cast in SQL
                    .bind(user_clone.storage_quota_bytes())
                    .bind(user_clone.storage_used_bytes())
                    .bind(user_clone.created_at())
                    .bind(user_clone.updated_at())
                    .bind(user_clone.last_login_at())
                    .bind(user_clone.is_active())
                    .bind(user_clone.oidc_provider())
                    .bind(user_clone.oidc_subject())
                    .execute(&mut **tx)
                    .await
                    .map_err(Self::map_sqlx_error)?;
                    
                    // We could perform additional operations here,
                    // such as configuring permissions, roles, etc.
                    
                    Ok(user_clone)
                }) as BoxFuture<'_, UserRepositoryResult<User>>
            }
        ).await?;
        
        Ok(user) // Return the original user for simplicity
    }
    
    /// Gets a user by ID
    async fn get_user_by_id(&self, id: &str) -> UserRepositoryResult<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Convert role string to UserRole enum
        let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
        let role = match role_str.as_deref() {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        };
        
        Ok(User::from_data_full(
            row.get("id"),
            row.get("username"),
            row.get("email"),
            row.get("password_hash"),
            role,
            row.get("storage_quota_bytes"),
            row.get("storage_used_bytes"),
            row.get("created_at"),
            row.get("updated_at"),
            row.get("last_login_at"),
            row.get("active"),
            row.get("oidc_provider"),
            row.get("oidc_subject"),
        ))
    }
    
    /// Gets a user by username
    async fn get_user_by_username(&self, username: &str) -> UserRepositoryResult<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            WHERE username = $1
            "#
        )
        .bind(username)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Convert role string to UserRole enum
        let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
        let role = match role_str.as_deref() {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        };
        
        Ok(User::from_data_full(
            row.get("id"),
            row.get("username"),
            row.get("email"),
            row.get("password_hash"),
            role,
            row.get("storage_quota_bytes"),
            row.get("storage_used_bytes"),
            row.get("created_at"),
            row.get("updated_at"),
            row.get("last_login_at"),
            row.get("active"),
            row.get("oidc_provider"),
            row.get("oidc_subject"),
        ))
    }
    
    /// Gets a user by email
    async fn get_user_by_email(&self, email: &str) -> UserRepositoryResult<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        // Convert role string to UserRole enum
        let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
        let role = match role_str.as_deref() {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        };
        
        Ok(User::from_data_full(
            row.get("id"),
            row.get("username"),
            row.get("email"),
            row.get("password_hash"),
            role,
            row.get("storage_quota_bytes"),
            row.get("storage_used_bytes"),
            row.get("created_at"),
            row.get("updated_at"),
            row.get("last_login_at"),
            row.get("active"),
            row.get("oidc_provider"),
            row.get("oidc_subject"),
        ))
    }
    
    /// Updates an existing user using a transaction
    async fn update_user(&self, user: User) -> UserRepositoryResult<User> {
        // Create a copy of the user for the closure
        let user_clone = user.clone();
        
        with_transaction(
            &self.pool,
            "update_user",
            |tx| {
                Box::pin(async move {
                    // Update the user
                    sqlx::query(
                        r#"
                        UPDATE auth.users
                        SET 
                            username = $2,
                            email = $3,
                            password_hash = $4,
                            role = $5::auth.userrole,
                            storage_quota_bytes = $6,
                            storage_used_bytes = $7,
                            updated_at = $8,
                            last_login_at = $9,
                            active = $10
                        WHERE id = $1
                        "#
                    )
                    .bind(user_clone.id())
                    .bind(user_clone.username())
                    .bind(user_clone.email())
                    .bind(user_clone.password_hash())
                    .bind(&user_clone.role().to_string())
                    .bind(user_clone.storage_quota_bytes())
                    .bind(user_clone.storage_used_bytes())
                    .bind(user_clone.updated_at())
                    .bind(user_clone.last_login_at())
                    .bind(user_clone.is_active())
                    .execute(&mut **tx)
                    .await
                    .map_err(Self::map_sqlx_error)?;
                    
                    // We could perform additional operations here inside
                    // the same transaction, such as updating permissions, etc.
                    
                    Ok(user_clone)
                }) as BoxFuture<'_, UserRepositoryResult<User>>
            }
        ).await?;
        
        Ok(user)
    }
    
    /// Updates only the storage usage of a user
    async fn update_storage_usage(&self, user_id: &str, usage_bytes: i64) -> UserRepositoryResult<()> {
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
        .bind(usage_bytes)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
    
    /// Updates the last login date
    async fn update_last_login(&self, user_id: &str) -> UserRepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                last_login_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
    
    /// Lists users with pagination
    async fn list_users(&self, limit: i64, offset: i64) -> UserRepositoryResult<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let users = rows.into_iter()
            .map(|row| {
                // Convert role string to UserRole enum for each row
                let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
                let role = match role_str.as_deref() {
                    Some("admin") => UserRole::Admin,
                    _ => UserRole::User,
                };
                
                User::from_data_full(
                    row.get("id"),
                    row.get("username"),
                    row.get("email"),
                    row.get("password_hash"),
                    role,
                    row.get("storage_quota_bytes"),
                    row.get("storage_used_bytes"),
                    row.get("created_at"),
                    row.get("updated_at"),
                    row.get("last_login_at"),
                    row.get("active"),
                    row.get("oidc_provider"),
                    row.get("oidc_subject"),
                )
            })
            .collect();

        Ok(users)
    }
    
    /// Activates or deactivates a user
    async fn set_user_active_status(&self, user_id: &str, active: bool) -> UserRepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                active = $2,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(active)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
    
    /// Changes a user's password
    async fn change_password(&self, user_id: &str, password_hash: &str) -> UserRepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                password_hash = $2,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(password_hash)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
    
    /// Changes a user's role
    async fn change_role(&self, user_id: &str, role: UserRole) -> UserRepositoryResult<()> {
        // Convert the role to string for the binding
        let role_str = role.to_string();
        
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                role = $2::auth.userrole,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(&role_str)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
    
    /// Lists users by role
    async fn list_users_by_role(&self, role: &str) -> UserRepositoryResult<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            WHERE role::text = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(role)
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let users = rows.into_iter()
            .map(|row| {
                // Convert role string to UserRole enum for each row
                let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
                let role = match role_str.as_deref() {
                    Some("admin") => UserRole::Admin,
                    _ => UserRole::User,
                };
                
                User::from_data_full(
                    row.get("id"),
                    row.get("username"),
                    row.get("email"),
                    row.get("password_hash"),
                    role,
                    row.get("storage_quota_bytes"),
                    row.get("storage_used_bytes"),
                    row.get("created_at"),
                    row.get("updated_at"),
                    row.get("last_login_at"),
                    row.get("active"),
                    row.get("oidc_provider"),
                    row.get("oidc_subject"),
                )
            })
            .collect();

        Ok(users)
    }
    
    /// Deletes a user
    async fn delete_user(&self, user_id: &str) -> UserRepositoryResult<()> {
        sqlx::query(
            r#"
            DELETE FROM auth.users
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }

    /// Finds a user by OIDC provider + subject pair
    async fn get_user_by_oidc_subject(&self, provider: &str, subject: &str) -> UserRepositoryResult<User> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, role::text as role_text, 
                storage_quota_bytes, storage_used_bytes, 
                created_at, updated_at, last_login_at, active,
                oidc_provider, oidc_subject
            FROM auth.users
            WHERE oidc_provider = $1 AND oidc_subject = $2
            "#
        )
        .bind(provider)
        .bind(subject)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let role_str: Option<String> = row.try_get("role_text").unwrap_or(None);
        let role = match role_str.as_deref() {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        };

        Ok(User::from_data_full(
            row.get("id"),
            row.get("username"),
            row.get("email"),
            row.get("password_hash"),
            role,
            row.get("storage_quota_bytes"),
            row.get("storage_used_bytes"),
            row.get("created_at"),
            row.get("updated_at"),
            row.get("last_login_at"),
            row.get("active"),
            row.get("oidc_provider"),
            row.get("oidc_subject"),
        ))
    }

    /// Updates a user's storage quota
    async fn update_storage_quota(&self, user_id: &str, quota_bytes: i64) -> UserRepositoryResult<()> {
        sqlx::query(
            r#"
            UPDATE auth.users
            SET 
                storage_quota_bytes = $2,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(user_id)
        .bind(quota_bytes)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }

    /// Counts the total number of users
    async fn count_users(&self) -> UserRepositoryResult<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM auth.users"
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let count: i64 = row.get("count");
        Ok(count)
    }

    /// Gets aggregated storage statistics
    async fn get_storage_stats(&self) -> UserRepositoryResult<StorageStats> {
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(*) as total_users,
                COUNT(*) FILTER (WHERE active = true) as active_users,
                COALESCE(SUM(storage_quota_bytes), 0) as total_quota_bytes,
                COALESCE(SUM(storage_used_bytes), 0) as total_used_bytes,
                COUNT(*) FILTER (WHERE storage_quota_bytes > 0 AND storage_used_bytes > storage_quota_bytes * 0.8) as users_over_80_percent,
                COUNT(*) FILTER (WHERE storage_quota_bytes > 0 AND storage_used_bytes > storage_quota_bytes) as users_over_quota
            FROM auth.users
            "#
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(StorageStats {
            total_users: row.get("total_users"),
            active_users: row.get("active_users"),
            total_quota_bytes: row.get("total_quota_bytes"),
            total_used_bytes: row.get("total_used_bytes"),
            users_over_80_percent: row.get("users_over_80_percent"),
            users_over_quota: row.get("users_over_quota"),
        })
    }
}

// Storage port implementation for the application layer
#[async_trait]
impl UserStoragePort for UserPgRepository {
    async fn create_user(&self, user: User) -> Result<User, DomainError> {
        UserRepository::create_user(self, user).await.map_err(DomainError::from)
    }
    
    async fn get_user_by_id(&self, id: &str) -> Result<User, DomainError> {
        UserRepository::get_user_by_id(self, id).await.map_err(DomainError::from)
    }
    
    async fn get_user_by_username(&self, username: &str) -> Result<User, DomainError> {
        UserRepository::get_user_by_username(self, username).await.map_err(DomainError::from)
    }
    
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        UserRepository::get_user_by_email(self, email).await.map_err(DomainError::from)
    }
    
    async fn update_user(&self, user: User) -> Result<User, DomainError> {
        UserRepository::update_user(self, user).await.map_err(DomainError::from)
    }
    
    async fn update_storage_usage(&self, user_id: &str, usage_bytes: i64) -> Result<(), DomainError> {
        UserRepository::update_storage_usage(self, user_id, usage_bytes)
            .await
            .map_err(DomainError::from)
    }
    
    async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError> {
        UserRepository::list_users(self, limit, offset).await.map_err(DomainError::from)
    }
    
    async fn list_users_by_role(&self, role: &str) -> Result<Vec<User>, DomainError> {
        UserRepository::list_users_by_role(self, role).await.map_err(DomainError::from)
    }
    
    async fn delete_user(&self, user_id: &str) -> Result<(), DomainError> {
        UserRepository::delete_user(self, user_id)
            .await
            .map_err(DomainError::from)
    }
    
    async fn change_password(&self, user_id: &str, password_hash: &str) -> Result<(), DomainError> {
        UserRepository::change_password(self, user_id, password_hash)
            .await
            .map_err(DomainError::from)
    }

    async fn get_user_by_oidc_subject(&self, provider: &str, subject: &str) -> Result<User, DomainError> {
        UserRepository::get_user_by_oidc_subject(self, provider, subject)
            .await
            .map_err(DomainError::from)
    }

    async fn set_user_active_status(&self, user_id: &str, active: bool) -> Result<(), DomainError> {
        UserRepository::set_user_active_status(self, user_id, active)
            .await
            .map_err(DomainError::from)
    }

    async fn change_role(&self, user_id: &str, role: &str) -> Result<(), DomainError> {
        let user_role = match role {
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        };
        UserRepository::change_role(self, user_id, user_role)
            .await
            .map_err(DomainError::from)
    }

    async fn update_storage_quota(&self, user_id: &str, quota_bytes: i64) -> Result<(), DomainError> {
        UserRepository::update_storage_quota(self, user_id, quota_bytes)
            .await
            .map_err(DomainError::from)
    }

    async fn count_users(&self) -> Result<i64, DomainError> {
        UserRepository::count_users(self)
            .await
            .map_err(DomainError::from)
    }
}