use crate::{
    application::dtos::file_dto::FileDto,
    application::dtos::folder_dto::FolderDto,
    application::ports::file_ports::FileRetrievalUseCase,
    application::ports::inbound::FolderUseCase,
    application::ports::zip_ports::ZipPort,
    common::errors::{DomainError, ErrorKind, Result},
};
use async_trait::async_trait;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;
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

/// Service for creating ZIP files
pub struct ZipService {
    file_service: Arc<dyn FileRetrievalUseCase>,
    folder_service: Arc<dyn FolderUseCase>,
}

impl ZipService {
    /// Creates a new instance of the ZIP service with a reference to the file service
    pub fn new(
        file_service: Arc<dyn FileRetrievalUseCase>,
        folder_service: Arc<dyn FolderUseCase>,
    ) -> Self {
        Self {
            file_service,
            folder_service,
        }
    }

    /// Creates a ZIP file with the contents of a folder and all its subfolders
    /// Returns the ZIP bytes
    pub async fn create_folder_zip(&self, folder_id: &str, folder_name: &str) -> Result<Vec<u8>> {
        info!(
            "Creating ZIP for folder: {} (ID: {})",
            folder_name, folder_id
        );

        // Verify if the folder exists
        let folder = match self.folder_service.get_folder(folder_id).await {
            Ok(folder) => folder,
            Err(e) => {
                error!("Error getting folder {}: {}", folder_id, e);
                return Err(ZipError::FolderNotFound(folder_id.to_string()).into());
            }
        };

        // Create an in-memory buffer for the ZIP
        let buf = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);

        // Set compression options
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Object to track processed folders and avoid cycles
        let mut processed_folders = std::collections::HashSet::new();

        // Process the root folder and build the ZIP
        self.process_folder_recursively(
            &mut zip,
            &folder,
            folder_name,
            &options,
            &mut processed_folders,
        )
        .await?;

        // Finalize the ZIP and get the bytes
        let mut zip_buf = zip.finish()?;

        let mut bytes = Vec::new();
        match zip_buf.read_to_end(&mut bytes) {
            Ok(_) => Ok(bytes),
            Err(e) => {
                error!("Error reading finalized ZIP: {}", e);
                Err(ZipError::IoError(e).into())
            }
        }
    }

    // Alternative implementation to avoid recursion in async
    async fn process_folder_recursively(
        &self,
        zip: &mut ZipWriter<Cursor<Vec<u8>>>,
        folder: &FolderDto,
        path: &str,
        options: &SimpleFileOptions,
        processed_folders: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        // Structure to represent pending work
        struct PendingFolder {
            folder: FolderDto,
            path: String,
        }

        // Work queue for iterative processing
        let mut work_queue = vec![PendingFolder {
            folder: folder.clone(),
            path: path.to_string(),
        }];

        // Process the queue while there are elements
        while let Some(current) = work_queue.pop() {
            let folder_id = current.folder.id.to_string();

            // Avoid cycles
            if processed_folders.contains(&folder_id) {
                continue;
            }

            processed_folders.insert(folder_id.clone());

            // Create the directory entry in the ZIP
            let folder_path = format!("{}/", current.path);
            match zip.add_directory(&folder_path, *options) {
                Ok(_) => debug!("Folder added to ZIP: {}", folder_path),
                Err(e) => {
                    warn!("Could not add folder to ZIP (it may already exist): {}", e);
                    // Continue even if creating the directory fails (it could be a duplicate)
                }
            }

            // Add files from the folder to the ZIP
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

            // Add each file to the ZIP
            for file in files {
                self.add_file_to_zip(zip, &file, &folder_path, options)
                    .await?;
            }

            // Process subfolders
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

            // Add subfolders to the queue
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

    // Adds a file to the ZIP
    async fn add_file_to_zip(
        &self,
        zip: &mut ZipWriter<Cursor<Vec<u8>>>,
        file: &FileDto,
        folder_path: &str,
        options: &SimpleFileOptions,
    ) -> Result<()> {
        let file_path = format!("{}{}", folder_path, file.name);
        info!("Adding file to ZIP: {}", file_path);

        // Get the file content
        let file_id = file.id.to_string();
        let content = match self.file_service.get_file_content(&file_id).await {
            Ok(content) => content,
            Err(e) => {
                error!("Error reading file content {}: {}", file_id, e);
                return Err(ZipError::FileReadError(format!(
                    "Error reading file {}: {}",
                    file_id, e
                ))
                .into());
            }
        };

        // Write file to the ZIP
        match zip.start_file_from_path(std::path::Path::new(&file_path), *options) {
            Ok(_) => match zip.write_all(&content) {
                Ok(_) => {
                    debug!("File added to ZIP: {}", file_path);
                    Ok(())
                }
                Err(e) => {
                    error!("Error writing file content {}: {}", file_path, e);
                    Err(ZipError::IoError(e).into())
                }
            },
            Err(e) => {
                error!("Error starting file in ZIP {}: {}", file_path, e);
                Err(ZipError::ZipError(e).into())
            }
        }
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

#[async_trait]
impl ZipPort for ZipService {
    async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> std::result::Result<Vec<u8>, DomainError> {
        self.create_folder_zip(folder_id, folder_name).await
    }
}
