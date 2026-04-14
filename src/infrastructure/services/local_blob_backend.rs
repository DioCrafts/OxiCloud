//! Local Filesystem Blob Backend — stores blobs under `.blobs/{prefix}/{hash}.blob`.
//!
//! This is the default backend and a direct extraction of the filesystem I/O
//! that previously lived inside `DedupService`.

use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::fs::{self, File};
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;

use bytes::Bytes;

use crate::application::ports::blob_storage_ports::{
    BlobStorageBackend, BlobStream, StorageHealthStatus,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Chunk size for streaming file reads (256 KB).
const STREAM_CHUNK_SIZE: usize = 256 * 1024;

/// Compile-time lookup table for the 256 two-digit lowercase hex prefixes ("00"…"ff").
static HEX_PREFIXES: [&str; 256] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f",
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f",
    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f",
    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f",
    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f",
    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f",
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af",
    "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf",
    "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf",
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df",
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff",
];

/// Local filesystem blob backend.
///
/// Blobs are stored under `blob_root/{2-char-prefix}/{hash}.blob`.
/// Temporary upload staging uses `temp_root/`.
pub struct LocalBlobBackend {
    blob_root: PathBuf,
    temp_root: PathBuf,
}

impl LocalBlobBackend {
    /// Create a new local backend rooted at `storage_root`.
    ///
    /// Blob files go under `{storage_root}/.blobs/`, temp files under
    /// `{storage_root}/.dedup_temp/`.
    pub fn new(storage_root: &Path) -> Self {
        Self {
            blob_root: storage_root.join(".blobs"),
            temp_root: storage_root.join(".dedup_temp"),
        }
    }

    /// Compute the filesystem path for a blob hash.
    pub fn blob_path(&self, hash: &str) -> PathBuf {
        let prefix = &hash[0..2];
        self.blob_root.join(prefix).join(format!("{}.blob", hash))
    }

    /// Return a reference to the blob root directory.
    pub fn blob_root(&self) -> &Path {
        &self.blob_root
    }
}

impl BlobStorageBackend for LocalBlobBackend {
    fn initialize(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), DomainError>> + Send + '_>> {
        Box::pin(async move {
            fs::create_dir_all(&self.blob_root)
                .await
                .map_err(DomainError::from)?;
            fs::create_dir_all(&self.temp_root)
                .await
                .map_err(DomainError::from)?;

            // Create the 256 hash-prefix directories (00-ff)
            for prefix in &HEX_PREFIXES {
                fs::create_dir_all(self.blob_root.join(prefix))
                    .await
                    .map_err(DomainError::from)?;
            }
            Ok(())
        })
    }

    fn put_blob(
        &self,
        hash: &str,
        source_path: &Path,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, DomainError>> + Send + '_>> {
        let hash = hash.to_owned();
        let source_path = source_path.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);

            let file_size = fs::metadata(&source_path)
                .await
                .map_err(|e| {
                    DomainError::internal_error(
                        "Blob",
                        format!("Failed to stat source file: {}", e),
                    )
                })?
                .len();

            // Idempotent: if blob already exists, just remove the source
            if fs::try_exists(&blob_path).await.unwrap_or(false) {
                let _ = fs::remove_file(&source_path).await;
                return Ok(file_size);
            }

            // Atomic rename (same filesystem).  Falls back to copy+delete for
            // cross-device moves (EXDEV errno 18).
            if let Err(e) = fs::rename(&source_path, &blob_path).await {
                if e.raw_os_error() == Some(18) {
                    // EXDEV — cross-device link
                    fs::copy(&source_path, &blob_path).await.map_err(|ce| {
                        DomainError::internal_error(
                            "Blob",
                            format!("Failed to copy file to blob store: {}", ce),
                        )
                    })?;
                    let _ = fs::remove_file(&source_path).await;
                } else if fs::try_exists(&blob_path).await.unwrap_or(false) {
                    // Concurrent writer placed the blob — discard our copy
                    let _ = fs::remove_file(&source_path).await;
                    tracing::debug!("Blob placed by concurrent writer: {}", e);
                } else {
                    return Err(DomainError::internal_error(
                        "Blob",
                        format!("Failed to move file to blob store: {}", e),
                    ));
                }
            }

            Ok(file_size)
        })
    }

    fn put_blob_from_bytes(
        &self,
        hash: &str,
        data: Bytes,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, DomainError>> + Send + '_>> {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            let size = data.len() as u64;

            // Idempotent: if blob already exists, skip
            if fs::try_exists(&blob_path).await.unwrap_or(false) {
                return Ok(size);
            }

            // Write directly to blob path
            fs::write(&blob_path, &data).await.map_err(|e| {
                DomainError::internal_error(
                    "Blob",
                    format!("Failed to write blob from bytes: {}", e),
                )
            })?;

            Ok(size)
        })
    }

    fn get_blob_stream(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<BlobStream, DomainError>> + Send + '_>>
    {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            let file = File::open(&blob_path).await.map_err(|e| {
                DomainError::new(
                    ErrorKind::NotFound,
                    "Blob",
                    format!("Failed to open blob {}: {}", hash, e),
                )
            })?;
            Ok(Box::pin(ReaderStream::with_capacity(file, STREAM_CHUNK_SIZE)) as BlobStream)
        })
    }

    fn get_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<BlobStream, DomainError>> + Send + '_>>
    {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            let mut file = File::open(&blob_path).await.map_err(|e| {
                DomainError::new(
                    ErrorKind::NotFound,
                    "Blob",
                    format!("Failed to open blob {}: {}", hash, e),
                )
            })?;

            file.seek(std::io::SeekFrom::Start(start))
                .await
                .map_err(|e| {
                    DomainError::internal_error("Blob", format!("Failed to seek in blob: {}", e))
                })?;

            if let Some(end_pos) = end {
                use tokio::io::AsyncReadExt;
                let limit = end_pos.saturating_sub(start);
                let limited = file.take(limit);
                Ok(Box::pin(ReaderStream::with_capacity(limited, STREAM_CHUNK_SIZE)) as BlobStream)
            } else {
                Ok(Box::pin(ReaderStream::with_capacity(file, STREAM_CHUNK_SIZE)) as BlobStream)
            }
        })
    }

    fn delete_blob(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), DomainError>> + Send + '_>> {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            match fs::remove_file(&blob_path).await {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()), // idempotent
                Err(e) => Err(DomainError::internal_error(
                    "Blob",
                    format!("Failed to delete blob {}: {}", hash, e),
                )),
            }
        })
    }

    fn blob_exists(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<bool, DomainError>> + Send + '_>> {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            Ok(fs::try_exists(&blob_path).await.unwrap_or(false))
        })
    }

    fn blob_size(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, DomainError>> + Send + '_>> {
        let hash = hash.to_owned();
        Box::pin(async move {
            let blob_path = self.blob_path(&hash);
            let meta = fs::metadata(&blob_path).await.map_err(|e| {
                DomainError::new(
                    ErrorKind::NotFound,
                    "Blob",
                    format!("Failed to stat blob {}: {}", hash, e),
                )
            })?;
            Ok(meta.len())
        })
    }

    fn health_check(
        &self,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<StorageHealthStatus, DomainError>> + Send + '_>,
    > {
        Box::pin(async move {
            let writable = fs::metadata(&self.blob_root).await.is_ok();
            Ok(StorageHealthStatus {
                connected: writable,
                backend_type: "local".to_string(),
                message: if writable {
                    "Local filesystem is accessible".to_string()
                } else {
                    "Blob root directory is not accessible".to_string()
                },
                available_bytes: None,
            })
        })
    }

    fn backend_type(&self) -> &'static str {
        "local"
    }

    fn local_blob_path(&self, hash: &str) -> Option<PathBuf> {
        Some(self.blob_path(hash))
    }
}
