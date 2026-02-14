use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;

/// Composite that wraps `Arc<dyn FileReadPort>` + `Arc<dyn FileWritePort>`
/// and delegates each method to the corresponding port.
///
/// Thanks to the blanket impl `impl<T: FileReadPort + FileWritePort> FileStoragePort for T {}`
/// this type automatically gets `FileStoragePort`.
pub struct CompositeFileRepository {
    read: Arc<dyn FileReadPort>,
    write: Arc<dyn FileWritePort>,
}

impl CompositeFileRepository {
    pub fn new(read: Arc<dyn FileReadPort>, write: Arc<dyn FileWritePort>) -> Self {
        Self { read, write }
    }
}

// ─────────────────────────────────────────────────────
// FileReadPort — delegate to self.read
// ─────────────────────────────────────────────────────

#[async_trait]
impl FileReadPort for CompositeFileRepository {
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
        self.read.get_file(id).await
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        self.read.list_files(folder_id).await
    }

    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        self.read.get_file_content(id).await
    }

    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        self.read.get_file_stream(id).await
    }

    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        self.read.get_file_range_stream(id, start, end).await
    }

    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError> {
        self.read.get_file_mmap(id).await
    }

    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        self.read.get_file_path(id).await
    }

    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError> {
        self.read.get_parent_folder_id(path).await
    }
}

// ─────────────────────────────────────────────────────
// FileWritePort — delegate to self.write
// ─────────────────────────────────────────────────────

#[async_trait]
impl FileWritePort for CompositeFileRepository {
    async fn save_file(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<File, DomainError> {
        self.write
            .save_file(name, folder_id, content_type, content)
            .await
    }

    async fn save_file_from_stream(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        stream: std::pin::Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>,
    ) -> Result<File, DomainError> {
        self.write
            .save_file_from_stream(name, folder_id, content_type, stream)
            .await
    }

    async fn move_file(
        &self,
        file_id: &str,
        target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        self.write.move_file(file_id, target_folder_id).await
    }

    async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<File, DomainError> {
        self.write.rename_file(file_id, new_name).await
    }

    async fn delete_file(&self, id: &str) -> Result<(), DomainError> {
        self.write.delete_file(id).await
    }

    async fn update_file_content(
        &self,
        file_id: &str,
        content: Vec<u8>,
    ) -> Result<(), DomainError> {
        self.write.update_file_content(file_id, content).await
    }

    async fn register_file_deferred(
        &self,
        name: String,
        folder_id: Option<String>,
        content_type: String,
        size: u64,
    ) -> Result<(File, PathBuf), DomainError> {
        self.write
            .register_file_deferred(name, folder_id, content_type, size)
            .await
    }

    async fn move_to_trash(&self, file_id: &str) -> Result<(), DomainError> {
        self.write.move_to_trash(file_id).await
    }

    async fn restore_from_trash(
        &self,
        file_id: &str,
        original_path: &str,
    ) -> Result<(), DomainError> {
        self.write.restore_from_trash(file_id, original_path).await
    }

    async fn delete_file_permanently(&self, file_id: &str) -> Result<(), DomainError> {
        self.write.delete_file_permanently(file_id).await
    }
}
