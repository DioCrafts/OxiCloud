use id3::{Tag, TagLike};
use sqlx::{FromRow, PgPool};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::common::errors::DomainError;

#[derive(Debug, FromRow)]
pub struct AudioFileRow {
    pub file_id: Uuid,
    pub blob_hash: String,
}

pub struct AudioMetadataService {
    pool: Arc<PgPool>,
    blob_root: PathBuf,
}

impl AudioMetadataService {
    pub fn new(pool: Arc<PgPool>, blob_root: PathBuf) -> Self {
        Self { pool, blob_root }
    }

    pub fn is_audio_file(mime_type: &str) -> bool {
        mime_type.starts_with("audio/")
    }

    pub fn spawn_extraction_background(service: Arc<Self>, file_id: Uuid, file_path: PathBuf) {
        tokio::spawn(async move {
            tracing::info!("🎵 Extracting audio metadata for: {}", file_id);
            if let Err(e) = service.extract_and_save(&file_id, &file_path).await {
                tracing::warn!("Failed to extract audio metadata: {}", e);
            }
        });
    }

    pub fn spawn_extraction_with_delete_background(
        service: Arc<Self>,
        file_id: Uuid,
        file_path: PathBuf,
    ) {
        tokio::spawn(async move {
            tracing::info!("🎵 Updating audio metadata for: {}", file_id);
            let _ = service.delete_metadata(&file_id).await;
            if let Err(e) = service.extract_and_save(&file_id, &file_path).await {
                tracing::warn!("Failed to update audio metadata: {}", e);
            }
        });
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        let prefix = &hash[0..2];
        self.blob_root.join(prefix).join(format!("{}.blob", hash))
    }

    fn get_duration_secs(file_path: &Path) -> i32 {
        match mp3_duration::from_path(file_path) {
            Ok(dur) => dur.as_secs_f64().round() as i32,
            Err(_) => {
                if let Ok(tag) = Tag::read_from_path(file_path) {
                    tag.duration().unwrap_or(0) as i32
                } else {
                    0
                }
            }
        }
    }

    pub async fn extract_and_save(
        &self,
        file_id: &Uuid,
        file_path: &Path,
    ) -> Result<(), DomainError> {
        info!(
            "AudioMetadataService: blob_root={:?}, file_id={}, file_path={:?}, exists={}",
            self.blob_root,
            file_id,
            file_path,
            file_path.exists()
        );

        if !file_path.exists() {
            warn!("File does not exist: {:?}", file_path);
            return Ok(());
        }

        let tag = match Tag::read_from_path(file_path) {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to read ID3 tag from {:?}: {}", file_path, e);
                return Ok(());
            }
        };

        let title = tag.title().map(|s| s.to_string());
        let artist = tag.artist().map(|s| s.to_string());
        let album = tag.album().map(|s| s.to_string());
        let genre = tag.genre().map(|s| s.to_string());
        let track_number: Option<i32> = tag.track().map(|n| n as i32);
        let disc_number: Option<i32> = tag.disc().map(|n| n as i32);
        let year: Option<i32> = tag.year();
        let duration_secs = Self::get_duration_secs(file_path);

        let album_artist =
            tag.frames()
                .find(|f| f.id() == "TPE2")
                .and_then(|f| match f.content() {
                    id3::frame::Content::Text(t) => Some(t.clone()),
                    _ => None,
                });

        info!(
            "Extracted audio metadata for file {}: title={:?}, artist={:?}, album={:?}, duration={}s",
            file_id, title, artist, album, duration_secs
        );

        info!("Saving metadata to database for file_id={}", file_id);

        sqlx::query(
            r#"
            INSERT INTO audio.file_metadata
                (file_id, title, artist, album, album_artist, genre, track_number, disc_number,
                 year, duration_secs, format)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (file_id) DO UPDATE SET
                title = EXCLUDED.title,
                artist = EXCLUDED.artist,
                album = EXCLUDED.album,
                album_artist = EXCLUDED.album_artist,
                genre = EXCLUDED.genre,
                track_number = EXCLUDED.track_number,
                disc_number = EXCLUDED.disc_number,
                year = EXCLUDED.year,
                duration_secs = EXCLUDED.duration_secs,
                format = EXCLUDED.format,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(file_id)
        .bind(&title)
        .bind(&artist)
        .bind(&album)
        .bind(&album_artist)
        .bind(&genre)
        .bind(track_number)
        .bind(disc_number)
        .bind(year)
        .bind(duration_secs)
        .bind("MPEG")
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            DomainError::database_error(format!("Failed to save audio metadata: {}", e))
        })?;

        Ok(())
    }

    pub async fn delete_metadata(&self, file_id: &Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM audio.file_metadata WHERE file_id = $1")
            .bind(file_id)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                DomainError::database_error(format!("Failed to delete audio metadata: {}", e))
            })?;
        Ok(())
    }

    pub async fn reextract_all_audio_metadata(
        &self,
    ) -> Result<MetadataExtractionResult, DomainError> {
        let audio_files = sqlx::query_as::<_, AudioFileRow>(
            r#"
            SELECT id as file_id, blob_hash
            FROM storage.files
            WHERE mime_type LIKE 'audio/%'
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::database_error(format!("Failed to fetch audio files: {}", e)))?;

        let total = audio_files.len();
        let mut processed = 0;
        let mut failed = 0;

        info!("Starting metadata extraction for {} audio files", total);

        for audio_file in audio_files {
            let file_path = self.blob_path(&audio_file.blob_hash);
            match self.extract_and_save(&audio_file.file_id, &file_path).await {
                Ok(()) => processed += 1,
                Err(e) => {
                    warn!(
                        "Failed to extract metadata for file {}: {}",
                        audio_file.file_id, e
                    );
                    failed += 1;
                }
            }
        }

        info!(
            "Metadata extraction complete: {} processed, {} failed out of {} total",
            processed, failed, total
        );

        Ok(MetadataExtractionResult {
            total,
            processed,
            failed,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct MetadataExtractionResult {
    pub total: usize,
    pub processed: usize,
    pub failed: usize,
}
