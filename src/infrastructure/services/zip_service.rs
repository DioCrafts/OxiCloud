use crate::{
    application::dtos::file_dto::FileDto,
    application::dtos::folder_dto::FolderDto,
    application::ports::file_ports::FileRetrievalUseCase,
    application::ports::inbound::FolderUseCase,
    application::ports::zip_ports::ZipPort,
    common::errors::{DomainError, ErrorKind, Result},
};
use async_trait::async_trait;
use futures::StreamExt;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;
use thiserror::Error;
use tracing::*;
use zip::{ZipWriter, write::SimpleFileOptions};

/// Error related to ZIP file creation
#[derive(Debug, Error)]
pub enum ZipError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Error reading file: {0}")]
    FileReadError(String),

    #[error("Error getting folder contents: {0}")]
    FolderContentsError(String),

    #[error("Folder not found: {0}")]
    FolderNotFound(String),
}

// Implement From<ZipError> for DomainError to allow the use of ?
impl From<ZipError> for DomainError {
    fn from(err: ZipError) -> Self {
        DomainError::new(ErrorKind::InternalError, "zip_service", err.to_string())
    }
}

// Implement From<zip::result::ZipError> for DomainError directly
impl From<zip::result::ZipError> for DomainError {
    fn from(err: zip::result::ZipError) -> Self {
        DomainError::new(ErrorKind::InternalError, "zip_service", err.to_string())
    }
}

/// Service for creating ZIP files.
///
/// Writes the ZIP archive to a temporary file on disk so that only one file's
/// stream-chunk (~64 KB) is held in memory at a time, regardless of archive size.
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
    /// of a folder and all its subfolders. Returns the `NamedTempFile` so the
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

        // Create a temp file to back the ZIP archive (O(1) RAM)
        let temp = NamedTempFile::new().map_err(ZipError::IoError)?;
        let raw_file = temp.reopen().map_err(ZipError::IoError)?;
        let mut zip = ZipWriter::new(raw_file);

        // Set compression options
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Track processed folders to avoid cycles
        let mut processed_folders = std::collections::HashSet::new();

        // Build the ZIP iteratively
        self.process_folder_recursively(
            &mut zip,
            &folder,
            folder_name,
            &options,
            &mut processed_folders,
        )
        .await?;

        // Finalize the ZIP (flushes central directory)
        zip.finish()?;

        Ok(temp)
    }

    /// Iterative BFS over the folder tree. Writes entries directly to the
    /// file-backed `ZipWriter` so memory stays flat.
    async fn process_folder_recursively(
        &self,
        zip: &mut ZipWriter<std::fs::File>,
        folder: &FolderDto,
        path: &str,
        options: &SimpleFileOptions,
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

            // Directory entry
            let folder_path = format!("{}/", current.path);
            match zip.add_directory(&folder_path, *options) {
                Ok(_) => debug!("Folder added to ZIP: {}", folder_path),
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
                self.add_file_to_zip_streamed(zip, &file, &folder_path, options)
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

    /// Streams file content in chunks (~64 KB) into the ZIP entry, keeping
    /// peak memory independent of individual file sizes.
    async fn add_file_to_zip_streamed(
        &self,
        zip: &mut ZipWriter<std::fs::File>,
        file: &FileDto,
        folder_path: &str,
        options: &SimpleFileOptions,
    ) -> Result<()> {
        let file_path = format!("{}{}", folder_path, file.name);
        info!("Adding file to ZIP: {}", file_path);

        let file_id = file.id.to_string();

        // Start the ZIP entry
        zip.start_file_from_path(std::path::Path::new(&file_path), *options)
            .map_err(ZipError::ZipError)?;

        // Stream file contents in chunks instead of loading all into RAM
        let stream = match self.file_service.get_file_stream(&file_id).await {
            Ok(s) => s,
            Err(e) => {
                error!("Error opening file stream {}: {}", file_id, e);
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
            zip.write_all(&bytes).map_err(ZipError::IoError)?;
        }

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
