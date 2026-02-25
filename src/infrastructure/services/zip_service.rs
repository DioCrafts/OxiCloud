use crate::{
    application::dtos::file_dto::FileDto,
    application::ports::file_ports::FileRetrievalUseCase,
    application::ports::inbound::FolderUseCase,
    application::ports::zip_ports::ZipPort,
    common::errors::{DomainError, ErrorKind, Result},
};
use async_trait::async_trait;
use async_zip::base::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use futures::StreamExt;
use futures::io::AsyncWriteExt as FuturesWriteExt;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::io::BufWriter;
use tokio_util::compat::Compat;
use tracing::*;

/// Error related to ZIP file creation
#[derive(Debug, Error)]
pub enum ZipError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    AsyncZipError(#[from] async_zip::error::ZipError),

    #[error("Error reading file: {0}")]
    FileReadError(String),

    #[error("Error getting folder contents: {0}")]
    FolderContentsError(String),

    #[error("Folder not found: {0}")]
    FolderNotFound(String),
}

impl From<ZipError> for DomainError {
    fn from(err: ZipError) -> Self {
        DomainError::new(ErrorKind::InternalError, "zip_service", err.to_string())
    }
}

/// Type alias for the fully-async ZIP writer backed by a buffered tokio file.
type AsyncZipWriter = ZipFileWriter<Compat<BufWriter<tokio::fs::File>>>;

/// Service for creating ZIP files.
///
/// Uses `async_zip` for fully-async archive creation.  Every write (headers,
/// compressed chunk data, central directory) goes through
/// `tokio::io::BufWriter` → `tokio::fs::File`, so **no Tokio worker is ever
/// blocked** by disk I/O or compression.
pub struct ZipService {
    file_service: Arc<dyn FileRetrievalUseCase>,
    folder_service: Arc<dyn FolderUseCase>,
}

impl ZipService {
    /// Creates a new instance of the ZIP service
    pub fn new(
        file_service: Arc<dyn FileRetrievalUseCase>,
        folder_service: Arc<dyn FolderUseCase>,
    ) -> Self {
        Self {
            file_service,
            folder_service,
        }
    }

    /// Creates a ZIP file backed by a temporary file, containing the contents
    /// of a folder and all its subfolders.  Returns the `NamedTempFile` so the
    /// caller can stream it and let the OS clean up on drop.
    ///
    /// Uses **2 SQL queries** (ltree `<@`) to fetch the entire subtree instead
    /// of the previous N+1 BFS traversal.
    pub async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> Result<NamedTempFile> {
        info!(
            "Creating ZIP for folder: {} (ID: {})",
            folder_name, folder_id
        );

        // Verify the folder exists and get its path for prefix stripping
        let root_folder = match self.folder_service.get_folder(folder_id).await {
            Ok(f) => f,
            Err(e) => {
                error!("Error getting folder {}: {}", folder_id, e);
                return Err(ZipError::FolderNotFound(folder_id.to_string()).into());
            }
        };

        // ── 1. Bulk-fetch folder tree (small — one entry per folder) ────
        let all_folders = self
            .folder_service
            .list_subtree_folders(folder_id)
            .await
            .map_err(|e| ZipError::FolderContentsError(format!("subtree folders: {}", e)))?;

        // ── 2. Stream files from DB cursor — O(1) per row ───────────────
        let mut file_stream = self
            .file_service
            .stream_files_in_subtree(folder_id)
            .await
            .map_err(|e| ZipError::FolderContentsError(format!("subtree files: {}", e)))?;

        // Group files by folder_id incrementally from the stream
        let mut files_by_folder: HashMap<String, Vec<FileDto>> =
            HashMap::with_capacity(all_folders.len());
        while let Some(file) = file_stream.next().await {
            let file =
                file.map_err(|e| ZipError::FolderContentsError(format!("subtree file: {}", e)))?;
            let fid = file.folder_id.clone().unwrap_or_default();
            files_by_folder.entry(fid).or_default().push(file);
        }

        info!(
            "ZIP subtree: {} folders, {} files",
            all_folders.len(),
            files_by_folder.values().map(|v| v.len()).sum::<usize>()
        );

        // ── 3. Build a mapping: folder_id → ZIP-relative path ────────────
        //
        // The root folder's DB path is e.g. "/users/alice/Documents".
        // We want ZIP entries relative to `folder_name`, so we strip the
        // root prefix and prepend `folder_name`.
        let root_path = root_folder.path.trim_end_matches('/');
        let folder_zip_path = |db_path: &str| -> String {
            let db_path = db_path.trim_end_matches('/');
            if db_path == root_path {
                folder_name.to_string()
            } else {
                let suffix = db_path
                    .strip_prefix(root_path)
                    .unwrap_or(db_path)
                    .trim_start_matches('/');
                format!("{}/{}", folder_name, suffix)
            }
        };

        // ── 4. Open the temp file + ZIP writer ───────────────────────────
        let temp = NamedTempFile::new().map_err(ZipError::IoError)?;
        let tokio_file = tokio::fs::File::create(temp.path())
            .await
            .map_err(ZipError::IoError)?;
        let buf_writer = BufWriter::with_capacity(256 * 1024, tokio_file);
        let mut zip = ZipFileWriter::with_tokio(buf_writer);

        // ── 5. Write entries (folders are already sorted by path) ─────────
        for folder in &all_folders {
            let zip_dir = format!("{}/", folder_zip_path(&folder.path));

            // Directory entry (Stored, zero-length body)
            let dir_entry = ZipEntryBuilder::new(zip_dir.clone().into(), Compression::Stored);
            match zip.write_entry_whole(dir_entry, &[]).await {
                Ok(()) => debug!("Folder added to ZIP: {}", zip_dir),
                Err(e) => {
                    warn!("Could not add folder entry (may already exist): {}", e);
                }
            }

            // Files belonging to this folder
            if let Some(files) = files_by_folder.get(&folder.id) {
                for file in files {
                    self.add_file_to_zip_streamed(&mut zip, file, &zip_dir)
                        .await?;
                }
            }
        }

        // ── 6. Finalize ──────────────────────────────────────────────────
        let mut compat_writer = zip.close().await.map_err(ZipError::AsyncZipError)?;
        compat_writer.close().await.map_err(ZipError::IoError)?;

        Ok(temp)
    }

    /// Streams file content in chunks (~64 KB) into an async ZIP entry,
    /// keeping peak memory independent of individual file sizes.
    async fn add_file_to_zip_streamed(
        &self,
        zip: &mut AsyncZipWriter,
        file: &FileDto,
        folder_path: &str,
    ) -> Result<()> {
        let file_path = format!("{}{}", folder_path, file.name);
        info!("Adding file to ZIP: {}", file_path);

        let file_id = file.id.to_string();

        // Open a streaming entry with Deflate compression
        let entry = ZipEntryBuilder::new(file_path.clone().into(), Compression::Deflate);
        let mut entry_writer = zip
            .write_entry_stream(entry)
            .await
            .map_err(ZipError::AsyncZipError)?;

        // Stream file contents in chunks instead of loading all into RAM
        let stream = match self.file_service.get_file_stream(&file_id).await {
            Ok(s) => s,
            Err(e) => {
                error!("Error opening file stream {}: {}", file_id, e);
                // Close the partially-opened entry before returning
                let _ = entry_writer.close().await;
                return Err(ZipError::FileReadError(format!(
                    "Error streaming file {}: {}",
                    file_id, e
                ))
                .into());
            }
        };

        // Pin the stream so StreamExt::next() can be called
        let mut stream = std::pin::Pin::from(stream);

        while let Some(chunk_result) = stream.next().await {
            let bytes = chunk_result.map_err(ZipError::IoError)?;
            entry_writer
                .write_all(&bytes)
                .await
                .map_err(ZipError::IoError)?;
        }

        // Finalize the entry (writes data descriptor with CRC + sizes)
        entry_writer
            .close()
            .await
            .map_err(ZipError::AsyncZipError)?;

        debug!("File added to ZIP: {}", file_path);
        Ok(())
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

#[async_trait]
impl ZipPort for ZipService {
    async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> std::result::Result<NamedTempFile, DomainError> {
        self.create_folder_zip(folder_id, folder_name).await
    }
}
