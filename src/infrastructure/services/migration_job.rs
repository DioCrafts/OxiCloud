//! Background migration job — copies blobs from a source backend to a target
//! backend with configurable concurrency and progress tracking.

use std::sync::Arc;

use futures::StreamExt;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::application::ports::blob_storage_ports::BlobStorageBackend;
use crate::common::errors::DomainError;
use crate::infrastructure::services::migration_blob_backend::{MigrationState, MigrationStatus};

/// Run the migration: stream all blob hashes from `storage.blobs` and copy
/// each one from `source` to `target`.
///
/// * The job respects `Paused` / `Failed` status in `state` — it will stop
///   streaming when the status is no longer `Running`.
/// * Errors on individual blobs are logged and collected in `failed_blobs`
///   but do **not** abort the full run.
/// * `concurrency` controls `buffer_unordered` parallelism (default: 4).
pub async fn run_migration(
    source: Arc<dyn BlobStorageBackend>,
    target: Arc<dyn BlobStorageBackend>,
    pool: Arc<PgPool>,
    state: Arc<RwLock<MigrationState>>,
    concurrency: usize,
) -> Result<(), DomainError> {
    // Count total blobs for progress tracking.
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM storage.blobs")
        .fetch_one(pool.as_ref())
        .await
        .unwrap_or(0);

    {
        let mut s = state.write().await;
        s.status = MigrationStatus::Running;
        s.total_blobs = total as u64;
        s.migrated_blobs = 0;
        s.migrated_bytes = 0;
        s.failed_blobs.clear();
        s.started_at = Some(chrono::Utc::now());
        s.completed_at = None;
    }

    // Stream all hashes+sizes with a cursor.
    let mut rows =
        sqlx::query_as::<_, (String, i64)>("SELECT hash, size FROM storage.blobs ORDER BY hash")
            .fetch(pool.as_ref());

    // Collect all hashes first to avoid holding the cursor across awaits.
    let mut work: Vec<(String, i64)> = Vec::with_capacity(total as usize);
    while let Some(row) = rows.next().await {
        match row {
            Ok(r) => work.push(r),
            Err(e) => {
                tracing::warn!("Error fetching blob row during migration: {}", e);
            }
        }
    }

    // Process in parallel chunks.
    let results = futures::stream::iter(work.into_iter().map(|(hash, size)| {
        let src = source.clone();
        let tgt = target.clone();
        let st = state.clone();
        async move {
            // Check if we should keep running.
            {
                let s = st.read().await;
                if s.status != MigrationStatus::Running {
                    return;
                }
            }

            // Skip if already in target.
            match tgt.blob_exists(&hash).await {
                Ok(true) => {
                    let mut s = st.write().await;
                    s.migrated_blobs += 1;
                    s.migrated_bytes += size as u64;
                    return;
                }
                Ok(false) => {}
                Err(e) => {
                    tracing::warn!("blob_exists check failed for {}: {}", hash, e);
                }
            }

            // Copy: stream from source → temp file → put into target.
            if let Err(e) = copy_blob(&src, &tgt, &hash).await {
                tracing::warn!("Failed to migrate blob {}: {}", hash, e);
                let mut s = st.write().await;
                s.failed_blobs.push(hash);
                return;
            }

            let mut s = st.write().await;
            s.migrated_blobs += 1;
            s.migrated_bytes += size as u64;
        }
    }))
    .buffer_unordered(concurrency)
    .collect::<Vec<()>>()
    .await;

    drop(results);

    // Finalize state.
    let mut s = state.write().await;
    if s.status == MigrationStatus::Running {
        if s.failed_blobs.is_empty() {
            s.status = MigrationStatus::Completed;
        } else {
            s.status = MigrationStatus::Failed;
        }
        s.completed_at = Some(chrono::Utc::now());
    }

    tracing::info!(
        "Migration finished: {}/{} blobs, {} failures",
        s.migrated_blobs,
        s.total_blobs,
        s.failed_blobs.len()
    );

    Ok(())
}

/// Copy a single blob: stream from source → spool to temp file → put_blob into target.
async fn copy_blob(
    source: &Arc<dyn BlobStorageBackend>,
    target: &Arc<dyn BlobStorageBackend>,
    hash: &str,
) -> Result<(), DomainError> {
    use tokio::io::AsyncWriteExt;

    // Create a temp file to spool content.
    let tmp_dir = std::env::temp_dir().join("oxicloud-migration");
    tokio::fs::create_dir_all(&tmp_dir).await.map_err(|e| {
        DomainError::internal_error("Migration", format!("Failed to create temp dir: {}", e))
    })?;

    let tmp_path = tmp_dir.join(format!("{}.tmp", hash));

    // Stream from source.
    let stream = source.get_blob_stream(hash).await?;

    // Write to temp file.
    let mut file = tokio::fs::File::create(&tmp_path).await.map_err(|e| {
        DomainError::internal_error("Migration", format!("Failed to create temp file: {}", e))
    })?;

    let mut stream = std::pin::pin!(stream);
    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| {
            DomainError::internal_error("Migration", format!("Stream error: {}", e))
        })?;
        file.write_all(&bytes)
            .await
            .map_err(|e| DomainError::internal_error("Migration", format!("Write error: {}", e)))?;
    }
    file.flush()
        .await
        .map_err(|e| DomainError::internal_error("Migration", format!("Flush error: {}", e)))?;
    drop(file);

    // Put into target.
    target.put_blob(hash, &tmp_path).await?;

    // Clean up temp file.
    let _ = tokio::fs::remove_file(&tmp_path).await;

    Ok(())
}

/// Verify migration integrity by comparing blob counts and sampling random hashes.
pub async fn verify_migration(
    target: Arc<dyn BlobStorageBackend>,
    pool: Arc<PgPool>,
    sample_size: usize,
) -> Result<VerificationResult, DomainError> {
    // 1. Count blobs in PG.
    let pg_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM storage.blobs")
        .fetch_one(pool.as_ref())
        .await
        .unwrap_or(0);

    // 2. Verify sample of blobs exist in target.
    let sample_rows: Vec<(String, i64)> =
        sqlx::query_as("SELECT hash, size FROM storage.blobs ORDER BY random() LIMIT $1")
            .bind(sample_size as i64)
            .fetch_all(pool.as_ref())
            .await
            .map_err(|e| {
                DomainError::internal_error("Migration", format!("Sample query failed: {}", e))
            })?;

    let mut missing = Vec::new();
    let mut size_mismatches = Vec::new();

    for (hash, expected_size) in &sample_rows {
        match target.blob_exists(hash).await {
            Ok(false) => missing.push(hash.clone()),
            Err(e) => {
                tracing::warn!("blob_exists failed for {}: {}", hash, e);
                missing.push(hash.clone());
            }
            Ok(true) => {
                // Verify size matches.
                if let Ok(actual_size) = target.blob_size(hash).await
                    && actual_size != *expected_size as u64
                {
                    size_mismatches.push(hash.clone());
                }
            }
        }
    }

    let passed = missing.is_empty() && size_mismatches.is_empty();

    Ok(VerificationResult {
        pg_blob_count: pg_count as u64,
        sample_checked: sample_rows.len() as u64,
        missing_in_target: missing,
        size_mismatches,
        passed,
    })
}

/// Result of a post-migration integrity check.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct VerificationResult {
    pub pg_blob_count: u64,
    pub sample_checked: u64,
    pub missing_in_target: Vec<String>,
    pub size_mismatches: Vec<String>,
    pub passed: bool,
}
