use futures::StreamExt;
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

    /// Extract ID3 tag and MP3 duration from a file.
    ///
    /// All I/O is synchronous (id3 + mp3_duration crates), so this MUST
    /// only be called inside `spawn_blocking`.
    fn extract_metadata_blocking(file_path: &Path) -> Option<AudioMetadataFields> {
        if !file_path.exists() {
            warn!("File does not exist: {:?}", file_path);
            return None;
        }

        let tag = match Tag::read_from_path(file_path) {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to read ID3 tag from {:?}: {}", file_path, e);
                return None;
            }
        };

        let duration_secs = match mp3_duration::from_path(file_path) {
            Ok(dur) => dur.as_secs_f64().round() as i32,
            Err(_) => tag.duration().unwrap_or(0) as i32,
        };

        let album_artist =
            tag.frames()
                .find(|f| f.id() == "TPE2")
                .and_then(|f| match f.content() {
                    id3::frame::Content::Text(t) => Some(t.clone()),
                    _ => None,
                });

        Some(AudioMetadataFields {
            title: tag.title().map(|s| s.to_string()),
            artist: tag.artist().map(|s| s.to_string()),
            album: tag.album().map(|s| s.to_string()),
            album_artist,
            genre: tag.genre().map(|s| s.to_string()),
            track_number: tag.track().map(|n| n as i32),
            disc_number: tag.disc().map(|n| n as i32),
            year: tag.year(),
            duration_secs,
        })
    }

    pub async fn extract_and_save(
        &self,
        file_id: &Uuid,
        file_path: &Path,
    ) -> Result<(), DomainError> {
        info!(
            "AudioMetadataService: blob_root={:?}, file_id={}, file_path={:?}",
            self.blob_root, file_id, file_path,
        );

        // ── Sync I/O on the blocking thread pool (never stalls Tokio workers) ──
        let path = file_path.to_path_buf();
        let metadata = tokio::task::spawn_blocking(move || Self::extract_metadata_blocking(&path))
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "AudioMetadataService",
                    format!("spawn_blocking join error: {e}"),
                )
            })?;

        let Some(m) = metadata else {
            return Ok(());
        };

        info!(
            "Extracted audio metadata for file {}: title={:?}, artist={:?}, album={:?}, duration={}s",
            file_id, m.title, m.artist, m.album, m.duration_secs
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
        .bind(&m.title)
        .bind(&m.artist)
        .bind(&m.album)
        .bind(&m.album_artist)
        .bind(&m.genre)
        .bind(m.track_number)
        .bind(m.disc_number)
        .bind(m.year)
        .bind(m.duration_secs)
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
        // Stream rows one-by-one instead of fetch_all to keep memory O(1).
        let mut stream = sqlx::query_as::<_, AudioFileRow>(
            r#"
            SELECT id as file_id, blob_hash
            FROM storage.files
            WHERE mime_type LIKE 'audio/%'
            "#,
        )
        .fetch(&*self.pool);

        let mut total: usize = 0;
        let mut processed: usize = 0;
        let mut failed: usize = 0;

        info!("Starting streaming metadata extraction for audio files");

        while let Some(row) = stream.next().await {
            total += 1;
            let audio_file = row.map_err(|e| {
                DomainError::database_error(format!("Failed to fetch audio file row: {}", e))
            })?;
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

/// Extracted audio metadata fields transferred from the blocking thread.
struct AudioMetadataFields {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    genre: Option<String>,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    year: Option<i32>,
    duration_secs: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct MetadataExtractionResult {
    pub total: usize,
    pub processed: usize,
    pub failed: usize,
}
