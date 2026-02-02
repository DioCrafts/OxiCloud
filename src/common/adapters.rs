//! Adaptadores para convertir entre interfaces de dominio y aplicación
//! 
//! Este módulo contiene adaptadores que permiten usar repositorios que implementan
//! `FileStoragePort` y `FolderStoragePort` donde se espera `FileRepository` y `FolderRepository`.

use std::sync::Arc;
use async_trait::async_trait;

use crate::application::ports::outbound::{FileStoragePort, FolderStoragePort};
use crate::domain::entities::file::File;
use crate::domain::entities::folder::Folder;
use crate::domain::repositories::file_repository::{FileRepository, FileRepositoryResult, FileRepositoryError};
use crate::domain::repositories::folder_repository::{FolderRepository, FolderRepositoryResult, FolderRepositoryError};
use crate::domain::services::path_service::StoragePath;

/// Adaptador que convierte FileStoragePort a FileRepository
pub struct DomainFileRepoAdapter {
    repo: Arc<dyn FileStoragePort>,
}

impl DomainFileRepoAdapter {
    pub fn new(repo: Arc<dyn FileStoragePort>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl FileRepository for DomainFileRepoAdapter {
    async fn save_file_from_bytes(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> FileRepositoryResult<File> {
        self.repo.save_file(name, folder_id, content_type, content)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn save_file_with_id(
        &self,
        _id: String,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _content: Vec<u8>,
    ) -> FileRepositoryResult<File> {
        Err(FileRepositoryError::Other("Not implemented".to_string()))
    }
    
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File> {
        self.repo.get_file(id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn list_files(&self, folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>> {
        self.repo.list_files(folder_id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn delete_file(&self, id: &str) -> FileRepositoryResult<()> {
        self.repo.delete_file(id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn delete_file_entry(&self, id: &str) -> FileRepositoryResult<()> {
        self.delete_file(id).await
    }
    
    async fn get_file_content(&self, id: &str) -> FileRepositoryResult<Vec<u8>> {
        self.repo.get_file_content(id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn get_file_stream(&self, id: &str) -> FileRepositoryResult<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>> {
        self.repo.get_file_stream(id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> FileRepositoryResult<File> {
        self.repo.move_file(id, target_folder_id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn get_file_path(&self, id: &str) -> FileRepositoryResult<StoragePath> {
        self.repo.get_file_path(id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn move_to_trash(&self, file_id: &str) -> FileRepositoryResult<()> {
        self.repo.delete_file(file_id)
            .await
            .map_err(|e| FileRepositoryError::Other(format!("{}", e)))
    }
    
    async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> FileRepositoryResult<()> {
        tracing::info!("Restoring file from trash: {} to {}", file_id, original_path);
        
        match self.repo.get_file(file_id).await {
            Ok(_) => {
                let path_components: Vec<&str> = original_path.split('/').collect();
                let parent_folder: Option<String> = if path_components.len() > 1 {
                    tracing::info!("Attempting to restore to parent folder from path: {}", original_path);
                    None
                } else {
                    None
                };
                
                match self.repo.move_file(file_id, parent_folder).await {
                    Ok(_) => {
                        tracing::info!("Successfully restored file from trash: {}", file_id);
                        Ok(())
                    },
                    Err(e) => {
                        tracing::error!("Failed to restore file from trash: {}", e);
                        Err(FileRepositoryError::Other(format!("Failed to restore file: {}", e)))
                    }
                }
            },
            Err(e) => {
                tracing::error!("File not found in trash: {}", e);
                Err(FileRepositoryError::NotFound(file_id.to_string()))
            }
        }
    }
    
    async fn delete_file_permanently(&self, file_id: &str) -> FileRepositoryResult<()> {
        tracing::info!("Permanently deleting file: {}", file_id);
        
        match self.repo.delete_file(file_id).await {
            Ok(_) => {
                tracing::info!("Successfully deleted file permanently: {}", file_id);
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to permanently delete file: {}", e);
                Err(FileRepositoryError::Other(format!("Failed to delete file permanently: {}", e)))
            }
        }
    }
    
    async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> FileRepositoryResult<()> {
        tracing::info!("Updating content for file: {}", file_id);
        
        self.repo.update_file_content(file_id, content)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update file content: {}", e);
                FileRepositoryError::Other(format!("Failed to update file content: {}", e))
            })
    }
}

/// Adaptador que convierte FolderStoragePort a FolderRepository
pub struct DomainFolderRepoAdapter {
    repo: Arc<dyn FolderStoragePort>,
}

impl DomainFolderRepoAdapter {
    pub fn new(repo: Arc<dyn FolderStoragePort>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl FolderRepository for DomainFolderRepoAdapter {
    async fn create_folder(&self, name: String, parent_id: Option<String>) -> FolderRepositoryResult<Folder> {
        self.repo.create_folder(name, parent_id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn get_folder_by_id(&self, id: &str) -> FolderRepositoryResult<Folder> {
        self.repo.get_folder(id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn get_folder_by_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<Folder> {
        self.repo.get_folder_by_path(storage_path)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn list_folders(&self, parent_id: Option<&str>) -> FolderRepositoryResult<Vec<Folder>> {
        self.repo.list_folders(parent_id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>, 
        offset: usize, 
        limit: usize,
        include_total: bool
    ) -> FolderRepositoryResult<(Vec<Folder>, Option<usize>)> {
        self.repo.list_folders_paginated(parent_id, offset, limit, include_total)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn rename_folder(&self, id: &str, new_name: String) -> FolderRepositoryResult<Folder> {
        self.repo.rename_folder(id, new_name)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> FolderRepositoryResult<Folder> {
        self.repo.move_folder(id, new_parent_id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn delete_folder(&self, id: &str) -> FolderRepositoryResult<()> {
        self.repo.delete_folder(id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> FolderRepositoryResult<bool> {
        self.repo.folder_exists(storage_path)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn get_folder_storage_path(&self, id: &str) -> FolderRepositoryResult<StoragePath> {
        self.repo.get_folder_path(id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn folder_exists(&self, _path: &std::path::PathBuf) -> FolderRepositoryResult<bool> {
        Err(FolderRepositoryError::Other("Not implemented".to_string()))
    }
    
    async fn get_folder_by_path(&self, _path: &std::path::PathBuf) -> FolderRepositoryResult<Folder> {
        Err(FolderRepositoryError::Other("Not implemented".to_string()))
    }
    
    async fn move_to_trash(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        self.repo.delete_folder(folder_id)
            .await
            .map_err(|e| FolderRepositoryError::Other(format!("{}", e)))
    }
    
    async fn restore_from_trash(&self, _folder_id: &str, _original_path: &str) -> FolderRepositoryResult<()> {
        Err(FolderRepositoryError::Other(
            "Restore from trash should be handled by TrashService, not through this adapter".to_string()))
    }
    
    async fn delete_folder_permanently(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        self.delete_folder(folder_id).await
    }
}
