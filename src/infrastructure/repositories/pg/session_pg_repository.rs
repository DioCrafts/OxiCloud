use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use chrono::Utc;
use futures::future::BoxFuture;

use crate::domain::entities::session::Session;
use crate::domain::repositories::session_repository::{SessionRepository, SessionRepositoryError, SessionRepositoryResult};
use crate::application::ports::auth_ports::SessionStoragePort;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::pg::transaction_utils::with_transaction;

// Implementar From<sqlx::Error> para SessionRepositoryError para permitir conversiones automáticas
impl From<sqlx::Error> for SessionRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        SessionPgRepository::map_sqlx_error(err)
    }
}

pub struct SessionPgRepository {
    pool: Arc<PgPool>,
}

impl SessionPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
    
    // Método auxiliar para mapear errores SQL a errores de dominio
    pub fn map_sqlx_error(err: sqlx::Error) -> SessionRepositoryError {
        match err {
            sqlx::Error::RowNotFound => {
                SessionRepositoryError::NotFound("Sesión no encontrada".to_string())
            },
            _ => SessionRepositoryError::DatabaseError(
                format!("Error de base de datos: {}", err)
            ),
        }
    }
}

#[async_trait]
impl SessionRepository for SessionPgRepository {
    /// Crea una nueva sesión utilizando una transacción
    async fn create_session(&self, session: Session) -> SessionRepositoryResult<Session> {
        // Crear una copia de la sesión para el closure
        let session_clone = session.clone();
        
        with_transaction(
            &self.pool,
            "create_session",
            |tx| {
                Box::pin(async move {
                    // Insertar la sesión
                    sqlx::query(
                        r#"
                        INSERT INTO auth.sessions (
                            id, user_id, refresh_token, expires_at, 
                            ip_address, user_agent, created_at, revoked
                        ) VALUES (
                            $1, $2, $3, $4, $5, $6, $7, $8
                        )
                        "#
                    )
                    .bind(session_clone.id())
                    .bind(session_clone.user_id())
                    .bind(session_clone.refresh_token())
                    .bind(session_clone.expires_at())
                    .bind(session_clone.ip_address())
                    .bind(session_clone.user_agent())
                    .bind(session_clone.created_at())
                    .bind(session_clone.is_revoked())
                    .execute(&mut **tx)
                    .await
                    .map_err(Self::map_sqlx_error)?;
                    
                    // Opcionalmente, actualizar el último login del usuario
                    // dentro de la misma transacción
                    sqlx::query(
                        r#"
                        UPDATE auth.users
                        SET last_login_at = NOW(), updated_at = NOW()
                        WHERE id = $1
                        "#
                    )
                    .bind(session_clone.user_id())
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| {
                        // Convertimos el error pero sin interrumpir la creación
                        // de la sesión si falla la actualización
                        tracing::warn!("No se pudo actualizar last_login_at para usuario {}: {}", 
                                    session_clone.user_id(), e);
                        SessionRepositoryError::DatabaseError(format!(
                            "Sesión creada pero no se pudo actualizar last_login_at: {}", e
                        ))
                    })?;
                    
                    Ok(session_clone)
                }) as BoxFuture<'_, SessionRepositoryResult<Session>>
            }
        ).await?;
        
        Ok(session)
    }
    
    /// Obtiene una sesión por ID
    async fn get_session_by_id(&self, id: &str) -> SessionRepositoryResult<Session> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(Session::from_raw(
            row.get("id"),
            row.get("user_id"),
            row.get("refresh_token"),
            row.get("expires_at"),
            row.get("ip_address"),
            row.get("user_agent"),
            row.get("created_at"),
            row.get("revoked"),
        ))
    }
    
    /// Obtiene una sesión por token de actualización
    async fn get_session_by_refresh_token(&self, refresh_token: &str) -> SessionRepositoryResult<Session> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE refresh_token = $1
            "#
        )
        .bind(refresh_token)
        .fetch_one(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(Session::from_raw(
            row.get("id"),
            row.get("user_id"),
            row.get("refresh_token"),
            row.get("expires_at"),
            row.get("ip_address"),
            row.get("user_agent"),
            row.get("created_at"),
            row.get("revoked"),
        ))
    }
    
    /// Obtiene todas las sesiones de un usuario
    async fn get_sessions_by_user_id(&self, user_id: &str) -> SessionRepositoryResult<Vec<Session>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, refresh_token, expires_at, 
                ip_address, user_agent, created_at, revoked
            FROM auth.sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        let sessions = rows.into_iter()
            .map(|row| {
                Session::from_raw(
                    row.get("id"),
                    row.get("user_id"),
                    row.get("refresh_token"),
                    row.get("expires_at"),
                    row.get("ip_address"),
                    row.get("user_agent"),
                    row.get("created_at"),
                    row.get("revoked"),
                )
            })
            .collect();

        Ok(sessions)
    }
    
    /// Revoca una sesión específica utilizando una transacción
    async fn revoke_session(&self, session_id: &str) -> SessionRepositoryResult<()> {
        let id = session_id.to_string(); // Clone para uso en closure
        
        with_transaction(
            &self.pool,
            "revoke_session",
            |tx| {
                Box::pin(async move {
                    // Revocar la sesión
                    let result = sqlx::query(
                        r#"
                        UPDATE auth.sessions
                        SET revoked = true
                        WHERE id = $1
                        RETURNING user_id
                        "#
                    )
                    .bind(&id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(Self::map_sqlx_error)?;
                    
                    // Si encontramos la sesión, podemos registrar un evento de seguridad
                    if let Some(row) = result {
                        let user_id: String = row.try_get("user_id").unwrap_or_default();
                        
                        // Registrar evento de seguridad (en una tabla de seguridad)
                        // Esto es opcional pero muestra cómo se puede realizar operaciones
                        // adicionales en la misma transacción
                        tracing::info!("Sesión con ID {} del usuario {} revocada", id, user_id);
                    }
                    
                    Ok(())
                }) as BoxFuture<'_, SessionRepositoryResult<()>>
            }
        ).await
    }
    
    /// Revoca todas las sesiones de un usuario utilizando una transacción
    async fn revoke_all_user_sessions(&self, user_id: &str) -> SessionRepositoryResult<u64> {
        let user_id_clone = user_id.to_string(); // Clone para uso en closure
        
        with_transaction(
            &self.pool,
            "revoke_all_user_sessions",
            |tx| {
                Box::pin(async move {
                    // Revocar todas las sesiones del usuario
                    let result = sqlx::query(
                        r#"
                        UPDATE auth.sessions
                        SET revoked = true
                        WHERE user_id = $1 AND revoked = false
                        "#
                    )
                    .bind(&user_id_clone)
                    .execute(&mut **tx)
                    .await
                    .map_err(Self::map_sqlx_error)?;
                    
                    let affected = result.rows_affected();
                    
                    // Registrar evento de seguridad
                    if affected > 0 {
                        tracing::info!("Revocadas {} sesiones del usuario {}", affected, user_id_clone);
                    }
                    
                    Ok(affected)
                }) as BoxFuture<'_, SessionRepositoryResult<u64>>
            }
        ).await
    }
    
    /// Elimina sesiones expiradas
    async fn delete_expired_sessions(&self) -> SessionRepositoryResult<u64> {
        let now = Utc::now();
        
        let result = sqlx::query(
            r#"
            DELETE FROM auth.sessions
            WHERE expires_at < $1
            "#
        )
        .bind(now)
        .execute(&*self.pool)
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(result.rows_affected())
    }
}

// Implementación del puerto de almacenamiento para la capa de aplicación
#[async_trait]
impl SessionStoragePort for SessionPgRepository {
    async fn create_session(&self, session: Session) -> Result<Session, DomainError> {
        SessionRepository::create_session(self, session).await.map_err(DomainError::from)
    }
    
    async fn get_session_by_refresh_token(&self, refresh_token: &str) -> Result<Session, DomainError> {
        SessionRepository::get_session_by_refresh_token(self, refresh_token)
            .await
            .map_err(DomainError::from)
    }
    
    async fn revoke_session(&self, session_id: &str) -> Result<(), DomainError> {
        SessionRepository::revoke_session(self, session_id).await.map_err(DomainError::from)
    }
    
    async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DomainError> {
        SessionRepository::revoke_all_user_sessions(self, user_id)
            .await
            .map_err(DomainError::from)
    }
}