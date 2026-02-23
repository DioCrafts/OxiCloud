use crate::{
    application::dtos::file_dto::FileDto,
    application::dtos::folder_dto::FolderDto,
    application::ports::file_ports::FileRetrievalUseCase,
    application::ports::inbound::FolderUseCase,
    application::ports::zip_ports::ZipPort,
    common::errors::{DomainError, ErrorKind, Result},
};
use async_trait::async_trait;
use async_zip::base::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use futures::io::AsyncWriteExt as FuturesWriteExt;
use futures::StreamExt;
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
    pub async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> Result<NamedTempFile> {
        info!(
            "Creating ZIP for folder: {} (ID: {})",
            folder_name, folder_id
        );

        // Verify the folder exists
        let folder = match self.folder_service.get_folder(folder_id).await {
            Ok(folder) => folder,
            Err(e) => {
                error!("Error getting folder {}: {}", folder_id, e);
                return Err(ZipError::FolderNotFound(folder_id.to_string()).into());
            }
        };

        // Create a temp file; open a second async handle for writing.
        let temp = NamedTempFile::new().map_err(ZipError::IoError)?;
        let tokio_file = tokio::fs::File::create(temp.path())
            .await
            .map_err(ZipError::IoError)?;

        // 256 KB buffer keeps syscall count low.
        let buf_writer = BufWriter::with_capacity(256 * 1024, tokio_file);
        let mut zip = ZipFileWriter::with_tokio(buf_writer);

        // Track processed folders to avoid cycles
        let mut processed_folders = std::collections::HashSet::new();

        // Build the ZIP iteratively
        self.process_folder_recursively(
            &mut zip,
            &folder,
            folder_name,
            &mut processed_folders,
        )
        .await?;

        // Finalize: writes central directory, then flush buffered data to disk.
        let mut compat_writer = zip.close().await.map_err(ZipError::AsyncZipError)?;
        compat_writer.close().await.map_err(ZipError::IoError)?;

        Ok(temp)
    }

    /// Iterative BFS over the folder tree.  Writes entries directly to the
    /// async `ZipFileWriter` so memory stays flat.
    async fn process_folder_recursively(
        &self,
        zip: &mut AsyncZipWriter,
        folder: &FolderDto,
        path: &str,
        processed_folders: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        struct PendingFolder {
            folder: FolderDto,
            path: String,
        }

        let mut work_queue = vec![PendingFolder {
            folder: folder.clone(),
            path: path.to_string(),
        }];

        while let Some(current) = work_queue.pop() {
            let folder_id = current.folder.id.to_string();

            if processed_folders.contains(&folder_id) {
                continue;
            }
            processed_folders.insert(folder_id.clone());

            // Directory entry (Stored, zero-length body)
            let folder_path = format!("{}/", current.path);
            let dir_entry =
                ZipEntryBuilder::new(folder_path.clone().into(), Compression::Stored);
            match zip.write_entry_whole(dir_entry, &[]).await {
                Ok(()) => debug!("Folder added to ZIP: {}", folder_path),
                Err(e) => {
                    warn!("Could not add folder to ZIP (may already exist): {}", e);
                }
            }

            // Files in this folder
            let files = match self.file_service.list_files(Some(&folder_id)).await {
                Ok(files) => files,
                Err(e) => {
                    error!("Error listing files in folder {}: {}", folder_id, e);
                    return Err(ZipError::FolderContentsError(format!(
                        "Error listing files: {}",
                        e
                    ))
                    .into());
                }
            };

            for file in files {
                self.add_file_to_zip_streamed(zip, &file, &folder_path)
                    .await?;
            }

            // Subfolders
            let subfolders = match self.folder_service.list_folders(Some(&folder_id)).await {
                Ok(folders) => folders,
                Err(e) => {
                    error!("Error listing subfolders in {}: {}", folder_id, e);
                    return Err(ZipError::FolderContentsError(format!(
                        "Error listing subfolders: {}",
                        e
                    ))
                    .into());
                }
            };

            for subfolder in subfolders {
                let subfolder_path = format!("{}/{}", current.path, subfolder.name);
                work_queue.push(PendingFolder {
                    folder: subfolder,
                    path: subfolder_path,
                });
            }
        }

        Ok(())
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
