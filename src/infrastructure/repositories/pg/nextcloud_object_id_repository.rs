use sqlx::{PgPool, Row};
use std::sync::Arc;

use crate::common::errors::{DomainError, ErrorKind, Result};

pub struct NextcloudObjectIdRepository {
    pool: Arc<PgPool>,
}

impl NextcloudObjectIdRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(&self, object_type: &str, object_id: &str) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO storage.nextcloud_object_ids (object_type, object_id)
            VALUES ($1, $2::uuid)
            ON CONFLICT (object_type, object_id)
            DO UPDATE SET object_id = EXCLUDED.object_id
            RETURNING id
            "#,
        )
        .bind(object_type)
        .bind(object_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "NextcloudFileId",
                format!("Failed to get/create Nextcloud ID: {}", e),
            )
        })?;

        Ok(row.get::<i64, _>("id"))
    }

    /// Get the OxiCloud object ID from a Nextcloud numeric ID.
    pub async fn get_object_id(&self, nc_id: i64, object_type: &str) -> Result<String> {
        let row = sqlx::query(
            r#"
            SELECT object_id
            FROM storage.nextcloud_object_ids
            WHERE id = $1 AND object_type = $2
            "#,
        )
        .bind(nc_id)
        .bind(object_type)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| {
            DomainError::new(
                ErrorKind::DatabaseError,
                "NextcloudFileId",
                format!("Failed to lookup Nextcloud ID: {}", e),
            )
        })?;

        match row {
            Some(row) => {
                let uuid: sqlx::types::Uuid = row.get("object_id");
                Ok(uuid.to_string())
            }
            None => Err(DomainError::new(
                ErrorKind::NotFound,
                "NextcloudFileId",
                format!("No mapping found for Nextcloud ID: {}", nc_id),
            )),
        }
    }
}
