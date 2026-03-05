//! PostgreSQL repository for image/video EXIF metadata.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

use crate::common::errors::DomainError;
use crate::infrastructure::services::exif_service::ExifMetadata;

/// Metadata as stored/retrieved from the database.
#[derive(Debug, Clone, Serialize)]
pub struct StoredMetadata {
    pub file_id: String,
    pub captured_at: Option<DateTime<Utc>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub orientation: Option<i16>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Repository for `storage.file_metadata` table operations.
pub struct FileMetadataRepository {
    pool: Arc<PgPool>,
}

impl FileMetadataRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Insert or update EXIF metadata for a file.
    pub async fn upsert(&self, file_id: &str, meta: &ExifMetadata) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO storage.file_metadata
                (file_id, captured_at, latitude, longitude, camera_make, camera_model, orientation, width, height)
            VALUES ($1::uuid, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (file_id) DO UPDATE SET
                captured_at  = EXCLUDED.captured_at,
                latitude     = EXCLUDED.latitude,
                longitude    = EXCLUDED.longitude,
                camera_make  = EXCLUDED.camera_make,
                camera_model = EXCLUDED.camera_model,
                orientation  = EXCLUDED.orientation,
                width        = EXCLUDED.width,
                height       = EXCLUDED.height
            "#,
        )
        .bind(file_id)
        .bind(meta.captured_at)
        .bind(meta.latitude)
        .bind(meta.longitude)
        .bind(&meta.camera_make)
        .bind(&meta.camera_model)
        .bind(meta.orientation.map(|o| o as i16))
        .bind(meta.width.map(|w| w as i32))
        .bind(meta.height.map(|h| h as i32))
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to upsert file metadata: {}", e);
            DomainError::internal_error("FileMetadata", format!("upsert: {e}"))
        })?;

        Ok(())
    }

    /// Get metadata for a single file.
    pub async fn get(&self, file_id: &str) -> Result<Option<StoredMetadata>, DomainError> {
        let row: Option<(
            String,
            Option<DateTime<Utc>>,
            Option<f64>,
            Option<f64>,
            Option<String>,
            Option<String>,
            Option<i16>,
            Option<i32>,
            Option<i32>,
        )> = sqlx::query_as(
            r#"
            SELECT file_id::text, captured_at, latitude, longitude,
                   camera_make, camera_model, orientation, width, height
              FROM storage.file_metadata
             WHERE file_id = $1::uuid
            "#,
        )
        .bind(file_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get file metadata: {}", e);
            DomainError::internal_error("FileMetadata", format!("get: {e}"))
        })?;

        Ok(row.map(
            |(file_id, captured_at, latitude, longitude, camera_make, camera_model, orientation, width, height)| {
                StoredMetadata {
                    file_id,
                    captured_at,
                    latitude,
                    longitude,
                    camera_make,
                    camera_model,
                    orientation,
                    width,
                    height,
                }
            },
        ))
    }

    /// Get metadata for multiple files in a single query.
    pub async fn get_batch(
        &self,
        file_ids: &[String],
    ) -> Result<HashMap<String, StoredMetadata>, DomainError> {
        if file_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let rows: Vec<(
            String,
            Option<DateTime<Utc>>,
            Option<f64>,
            Option<f64>,
            Option<String>,
            Option<String>,
            Option<i16>,
            Option<i32>,
            Option<i32>,
        )> = sqlx::query_as(
            r#"
            SELECT file_id::text, captured_at, latitude, longitude,
                   camera_make, camera_model, orientation, width, height
              FROM storage.file_metadata
             WHERE file_id = ANY($1::uuid[])
            "#,
        )
        .bind(file_ids)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to batch get file metadata: {}", e);
            DomainError::internal_error("FileMetadata", format!("get_batch: {e}"))
        })?;

        let mut map = HashMap::with_capacity(rows.len());
        for (file_id, captured_at, latitude, longitude, camera_make, camera_model, orientation, width, height) in rows {
            map.insert(
                file_id.clone(),
                StoredMetadata {
                    file_id,
                    captured_at,
                    latitude,
                    longitude,
                    camera_make,
                    camera_model,
                    orientation,
                    width,
                    height,
                },
            );
        }

        Ok(map)
    }
}
