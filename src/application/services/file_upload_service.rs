use std::sync::Arc;
use async_trait::async_trait;

use crate::application::dtos::file_dto::FileDto;
use crate::application::ports::file_ports::FileUploadUseCase;
use crate::application::ports::storage_ports::FileWritePort;
use crate::common::errors::DomainError;
use crate::application::ports::storage_ports::StorageUsagePort;
use tracing::{debug, warn};

/// Helper function to extract username from folder path string
fn extract_username_from_path(path: &str) -> Option<String> {
    // Check if path contains the folder pattern
    if !path.contains("Mi Carpeta - ") {
        return None;
    }
    
    // Split by the pattern and get the second part
    let parts: Vec<&str> = path.split("Mi Carpeta - ").collect();
    if parts.len() <= 1 {
        return None;
    }
    
    // Trim and return as owned String
    Some(parts[1].trim().to_string())
}

/// Servicio para operaciones de subida de archivos
pub struct FileUploadService {
    file_repository: Arc<dyn FileWritePort>,
    storage_usage_service: Option<Arc<dyn StorageUsagePort>>,
}

impl FileUploadService {
    /// Crea un nuevo servicio de subida de archivos
    pub fn new(file_repository: Arc<dyn FileWritePort>) -> Self {
        Self { 
            file_repository,
            storage_usage_service: None,
        }
    }
    
    /// Configura el servicio de uso de almacenamiento
    pub fn with_storage_usage_service(
        mut self, 
        storage_usage_service: Arc<dyn StorageUsagePort>
    ) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }
}

#[async_trait]
impl FileUploadUseCase for FileUploadService {
    async fn upload_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<FileDto, DomainError> {
        // Upload the file
        let file = self.file_repository.save_file(name, folder_id, content_type, content).await?;
        
        // Extract the owner's user ID if available
        // We could make this more explicit by adding a user_id parameter
        if let Some(storage_service) = &self.storage_usage_service {
            // Extract user ID from folder pattern 'Mi Carpeta - {username}'
            if let Some(folder_id) = file.folder_id() {
                // Since we don't have direct access to folder details, 
                // we'll use pattern matching on the folder ID
                // In a more complete implementation, we would use a folder repository
                let folder_id_str = folder_id;
                
                // Check if we can extract a username from context
                if let Ok(folder_path) = self.file_repository.get_folder_path_str(folder_id_str).await {
                    // Process the string to extract username without creating borrowing issues
                    if let Some(username) = extract_username_from_path(&folder_path) {
                        // Find user by username and update their storage usage
                        // We do this asynchronously to avoid blocking the upload response
                        let service_clone = Arc::clone(storage_service);
                        tokio::spawn(async move {
                            match service_clone.update_user_storage_usage(&username).await {
                                Ok(usage) => {
                                    debug!("Updated storage usage for user {} to {} bytes", username, usage);
                                },
                                Err(e) => {
                                    warn!("Failed to update storage usage for {}: {}", username, e);
                                }
                            }
                        });
                    }
                } else {
                    warn!("Could not get folder path for ID: {}", folder_id_str);
                }
            }
        }
        
        Ok(FileDto::from(file))
    }
}