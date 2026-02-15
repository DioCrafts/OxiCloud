use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use futures::Stream;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::application::services::trash_service::TrashService;
use crate::common::errors::{DomainError, Result};
use crate::domain::entities::file::File;
use crate::domain::entities::folder::Folder;
use crate::domain::entities::trashed_item::{TrashedItem, TrashedItemType};
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::repositories::trash_repository::TrashRepository;
use crate::domain::services::path_service::StoragePath;

// Mock repositories for testing
struct MockTrashRepository {
    trash_items: Mutex<HashMap<Uuid, TrashedItem>>,
}

impl MockTrashRepository {
    fn new() -> Self {
        Self {
            trash_items: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TrashRepository for MockTrashRepository {
    async fn add_to_trash(&self, item: &TrashedItem) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        items.insert(item.id(), item.clone());
        Ok(())
    }

    async fn get_trash_items(&self, user_id: &Uuid) -> Result<Vec<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let user_items = items
            .values()
            .filter(|item| item.user_id() == *user_id)
            .cloned()
            .collect();
        Ok(user_items)
    }

    async fn get_trash_item(&self, id: &Uuid, user_id: &Uuid) -> Result<Option<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let item = items
            .get(id)
            .filter(|item| item.user_id() == *user_id)
            .cloned();
        Ok(item)
    }

    async fn restore_from_trash(&self, id: &Uuid, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        if let Some(item) = items.get(id)
            && item.user_id() == *user_id
        {
            items.remove(id);
        }
        Ok(())
    }

    async fn delete_permanently(&self, id: &Uuid, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        if let Some(item) = items.get(id)
            && item.user_id() == *user_id
        {
            items.remove(id);
        }
        Ok(())
    }

    async fn clear_trash(&self, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        items.retain(|_, item| item.user_id() != *user_id);
        Ok(())
    }

    async fn get_expired_items(&self) -> Result<Vec<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let now = Utc::now();
        let expired = items
            .values()
            .filter(|item| item.deletion_date() <= now)
            .cloned()
            .collect();
        Ok(expired)
    }
}

struct MockFileRepository {
    files: Mutex<HashMap<String, File>>,
    trashed_files: Mutex<HashMap<String, File>>,
}

impl MockFileRepository {
    fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
            trashed_files: Mutex::new(HashMap::new()),
        }
    }

    fn add_test_file(&self, id: &str, name: &str, path: &str) {
        let file = File::new(
            id.to_string(),
            name.to_string(),
            StoragePath::from_string(path),
            100,
            "text/plain".to_string(),
            None,
        )
        .unwrap();

        let mut files = self.files.lock().unwrap();
        files.insert(id.to_string(), file);
    }
}

#[async_trait]
impl FileReadPort for MockFileRepository {
    async fn get_file(&self, id: &str) -> std::result::Result<File, DomainError> {
        let files = self.files.lock().unwrap();
        if let Some(file) = files.get(id) {
            Ok(file.clone())
        } else {
            Err(DomainError::not_found("File", id.to_string()))
        }
    }

    async fn list_files(
        &self,
        _folder_id: Option<&str>,
    ) -> std::result::Result<Vec<File>, DomainError> {
        Ok(vec![])
    }

    async fn get_file_content(&self, _id: &str) -> std::result::Result<Vec<u8>, DomainError> {
        Ok(vec![])
    }

    async fn get_file_stream(
        &self,
        _id: &str,
    ) -> std::result::Result<
        Box<dyn Stream<Item = std::result::Result<Bytes, std::io::Error>> + Send>,
        DomainError,
    > {
        unimplemented!()
    }

    async fn get_file_range_stream(
        &self,
        _id: &str,
        _start: u64,
        _end: Option<u64>,
    ) -> std::result::Result<
        Box<dyn Stream<Item = std::result::Result<Bytes, std::io::Error>> + Send>,
        DomainError,
    > {
        unimplemented!()
    }

    async fn get_file_mmap(&self, _id: &str) -> std::result::Result<Bytes, DomainError> {
        unimplemented!()
    }

    async fn get_file_path(&self, _id: &str) -> std::result::Result<StoragePath, DomainError> {
        unimplemented!()
    }

    async fn get_parent_folder_id(&self, _path: &str) -> std::result::Result<String, DomainError> {
        unimplemented!()
    }
}

#[async_trait]
impl FileWritePort for MockFileRepository {
    async fn save_file(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _content: Vec<u8>,
    ) -> std::result::Result<File, DomainError> {
        unimplemented!()
    }

    async fn save_file_from_temp(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _temp_path: &std::path::Path,
        _size: u64,
        _pre_computed_hash: Option<String>,
    ) -> std::result::Result<File, DomainError> {
        unimplemented!()
    }

    async fn move_file(
        &self,
        _file_id: &str,
        _target_folder_id: Option<String>,
    ) -> std::result::Result<File, DomainError> {
        unimplemented!()
    }

    async fn rename_file(
        &self,
        _file_id: &str,
        _new_name: &str,
    ) -> std::result::Result<File, DomainError> {
        unimplemented!()
    }

    async fn delete_file(&self, _id: &str) -> std::result::Result<(), DomainError> {
        Ok(())
    }

    async fn update_file_content(
        &self,
        _file_id: &str,
        _content: Vec<u8>,
    ) -> std::result::Result<(), DomainError> {
        Ok(())
    }

    async fn register_file_deferred(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _size: u64,
    ) -> std::result::Result<(File, PathBuf), DomainError> {
        unimplemented!()
    }

    async fn copy_file(
        &self,
        _file_id: &str,
        _target_folder_id: Option<String>,
    ) -> std::result::Result<File, DomainError> {
        unimplemented!()
    }

    async fn move_to_trash(&self, id: &str) -> std::result::Result<(), DomainError> {
        let mut files = self.files.lock().unwrap();
        let mut trashed = self.trashed_files.lock().unwrap();

        if let Some(file) = files.remove(id) {
            trashed.insert(id.to_string(), file);
            Ok(())
        } else {
            Err(DomainError::not_found("File", id.to_string()))
        }
    }

    async fn restore_from_trash(
        &self,
        id: &str,
        _original_path: &str,
    ) -> std::result::Result<(), DomainError> {
        let mut files = self.files.lock().unwrap();
        let mut trashed = self.trashed_files.lock().unwrap();

        if let Some(file) = trashed.remove(id) {
            files.insert(id.to_string(), file);
            Ok(())
        } else {
            Err(DomainError::not_found(
                "File",
                format!("File {} not found in trash", id),
            ))
        }
    }

    async fn delete_file_permanently(&self, id: &str) -> std::result::Result<(), DomainError> {
        let mut trashed = self.trashed_files.lock().unwrap();
        if trashed.remove(id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found(
                "File",
                format!("File {} not found in trash", id),
            ))
        }
    }
}

struct MockFolderRepository {
    folders: Mutex<HashMap<String, Folder>>,
    trashed_folders: Mutex<HashMap<String, Folder>>,
}

impl MockFolderRepository {
    fn new() -> Self {
        Self {
            folders: Mutex::new(HashMap::new()),
            trashed_folders: Mutex::new(HashMap::new()),
        }
    }

    fn add_test_folder(&self, id: &str, name: &str, path: &str) {
        let folder = Folder::new(
            id.to_string(),
            name.to_string(),
            StoragePath::from_string(path),
            None,
        )
        .unwrap();

        let mut folders = self.folders.lock().unwrap();
        folders.insert(id.to_string(), folder);
    }
}

#[async_trait]
impl FolderRepository for MockFolderRepository {
    async fn create_folder(
        &self,
        _name: String,
        _parent_id: Option<String>,
    ) -> std::result::Result<Folder, DomainError> {
        unimplemented!()
    }

    async fn get_folder(&self, id: &str) -> std::result::Result<Folder, DomainError> {
        let folders = self.folders.lock().unwrap();
        if let Some(folder) = folders.get(id) {
            Ok(folder.clone())
        } else {
            Err(DomainError::not_found("Folder", id.to_string()))
        }
    }

    async fn get_folder_by_path(
        &self,
        _storage_path: &StoragePath,
    ) -> std::result::Result<Folder, DomainError> {
        unimplemented!()
    }

    async fn list_folders(
        &self,
        _parent_id: Option<&str>,
    ) -> std::result::Result<Vec<Folder>, DomainError> {
        Ok(vec![])
    }

    async fn list_folders_by_owner(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
    ) -> std::result::Result<Vec<Folder>, DomainError> {
        Ok(vec![])
    }

    async fn list_folders_paginated(
        &self,
        _parent_id: Option<&str>,
        _offset: usize,
        _limit: usize,
        _include_total: bool,
    ) -> std::result::Result<(Vec<Folder>, Option<usize>), DomainError> {
        Ok((vec![], Some(0)))
    }

    async fn list_folders_by_owner_paginated(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
        _offset: usize,
        _limit: usize,
        _include_total: bool,
    ) -> std::result::Result<(Vec<Folder>, Option<usize>), DomainError> {
        Ok((vec![], Some(0)))
    }

    async fn rename_folder(
        &self,
        _id: &str,
        _new_name: String,
    ) -> std::result::Result<Folder, DomainError> {
        unimplemented!()
    }

    async fn move_folder(
        &self,
        _id: &str,
        _new_parent_id: Option<&str>,
    ) -> std::result::Result<Folder, DomainError> {
        unimplemented!()
    }

    async fn delete_folder(&self, _id: &str) -> std::result::Result<(), DomainError> {
        Ok(())
    }

    async fn folder_exists(
        &self,
        _storage_path: &StoragePath,
    ) -> std::result::Result<bool, DomainError> {
        Ok(false)
    }

    async fn get_folder_path(&self, _id: &str) -> std::result::Result<StoragePath, DomainError> {
        Ok(StoragePath::from_string("/"))
    }

    async fn move_to_trash(&self, id: &str) -> std::result::Result<(), DomainError> {
        let mut folders = self.folders.lock().unwrap();
        let mut trashed = self.trashed_folders.lock().unwrap();

        if let Some(folder) = folders.remove(id) {
            trashed.insert(id.to_string(), folder);
            Ok(())
        } else {
            Err(DomainError::not_found("Folder", id.to_string()))
        }
    }

    async fn restore_from_trash(
        &self,
        id: &str,
        _original_path: &str,
    ) -> std::result::Result<(), DomainError> {
        let mut folders = self.folders.lock().unwrap();
        let mut trashed = self.trashed_folders.lock().unwrap();

        if let Some(folder) = trashed.remove(id) {
            folders.insert(id.to_string(), folder);
            Ok(())
        } else {
            Err(DomainError::not_found(
                "Folder",
                format!("Folder {} not found in trash", id),
            ))
        }
    }

    async fn delete_folder_permanently(&self, id: &str) -> std::result::Result<(), DomainError> {
        let mut trashed = self.trashed_folders.lock().unwrap();
        if trashed.remove(id).is_some() {
            Ok(())
        } else {
            Err(DomainError::not_found(
                "Folder",
                format!("Folder {} not found in trash", id),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::trash_ports::TrashUseCase;

    #[tokio::test]
    async fn test_move_file_to_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());

        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone() as Arc<dyn FileReadPort>,
            file_repo.clone() as Arc<dyn FileWritePort>,
            folder_repo.clone(),
            30, // 30 days retention
        );

        let file_id = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";

        // Add a test file to the repository
        file_repo.add_test_file(file_id, "test.txt", "/test/path/test.txt");

        // Act
        let result = service.move_to_trash(file_id, "file", user_id).await;

        // Assert
        assert!(result.is_ok(), "Moving file to trash failed: {:?}", result);

        // Verify the file is in trash
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();

        assert_eq!(
            trash_items.len(),
            1,
            "Should have exactly one item in trash"
        );
        let trash_item = &trash_items[0];

        assert_eq!(
            trash_item.original_id().to_string(),
            file_id,
            "Original ID should match file ID"
        );
        assert_eq!(
            trash_item.user_id().to_string(),
            user_id,
            "User ID should match"
        );
        assert_eq!(
            *trash_item.item_type(),
            TrashedItemType::File,
            "Item type should be File"
        );
        assert_eq!(trash_item.name(), "test.txt", "File name should match");

        // Verify file is moved in file repository
        let files = file_repo.files.lock().unwrap();
        let trashed_files = file_repo.trashed_files.lock().unwrap();

        assert!(
            files.get(file_id).is_none(),
            "File should no longer be in main storage"
        );
        assert!(
            trashed_files.get(file_id).is_some(),
            "File should be in trash storage"
        );
    }

    #[tokio::test]
    async fn test_move_folder_to_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());

        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone() as Arc<dyn FileReadPort>,
            file_repo.clone() as Arc<dyn FileWritePort>,
            folder_repo.clone(),
            30, // 30 days retention
        );

        let folder_id = "550e8400-e29b-41d4-a716-446655440002";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";

        // Add a test folder to the repository
        folder_repo.add_test_folder(folder_id, "test_folder", "/test/path/test_folder");

        // Act
        let result = service.move_to_trash(folder_id, "folder", user_id).await;

        // Assert
        assert!(
            result.is_ok(),
            "Moving folder to trash failed: {:?}",
            result
        );

        // Verify the folder is in trash
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();

        assert_eq!(
            trash_items.len(),
            1,
            "Should have exactly one item in trash"
        );
        let trash_item = &trash_items[0];

        assert_eq!(
            trash_item.original_id().to_string(),
            folder_id,
            "Original ID should match folder ID"
        );
        assert_eq!(
            trash_item.user_id().to_string(),
            user_id,
            "User ID should match"
        );
        assert_eq!(
            *trash_item.item_type(),
            TrashedItemType::Folder,
            "Item type should be Folder"
        );
        assert_eq!(trash_item.name(), "test_folder", "Folder name should match");
    }

    #[tokio::test]
    async fn test_restore_file_from_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());

        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone() as Arc<dyn FileReadPort>,
            file_repo.clone() as Arc<dyn FileWritePort>,
            folder_repo.clone(),
            30, // 30 days retention
        );

        let file_id = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";
        let file_path = "/test/path/test.txt";

        // Add a test file and move it to trash
        file_repo.add_test_file(file_id, "test.txt", file_path);
        service
            .move_to_trash(file_id, "file", user_id)
            .await
            .unwrap();

        // Get the trash item ID
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        let trash_id = trash_items[0].id().to_string();

        // Act
        let result = service.restore_item(&trash_id, user_id).await;

        // Assert
        assert!(
            result.is_ok(),
            "Restoring file from trash failed: {:?}",
            result
        );

        // Verify the file is restored in file repository
        {
            let files = file_repo.files.lock().unwrap();
            let trashed_files = file_repo.trashed_files.lock().unwrap();

            assert!(
                files.get(file_id).is_some(),
                "File should be back in main storage"
            );
            assert!(
                trashed_files.get(file_id).is_none(),
                "File should no longer be in trash storage"
            );
        }

        // Verify the trash item is removed
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(
            trash_items.len(),
            0,
            "Trash should be empty after restoration"
        );
    }

    #[tokio::test]
    async fn test_delete_permanently() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());

        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone() as Arc<dyn FileReadPort>,
            file_repo.clone() as Arc<dyn FileWritePort>,
            folder_repo.clone(),
            30, // 30 days retention
        );

        let file_id = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";

        // Add a test file and move it to trash
        file_repo.add_test_file(file_id, "test.txt", "/test/path/test.txt");
        service
            .move_to_trash(file_id, "file", user_id)
            .await
            .unwrap();

        // Get the trash item ID
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        let trash_id = trash_items[0].id().to_string();

        // Act
        let result = service.delete_permanently(&trash_id, user_id).await;

        // Assert
        assert!(
            result.is_ok(),
            "Deleting file permanently failed: {:?}",
            result
        );

        // Verify the file is permanently deleted
        {
            let files = file_repo.files.lock().unwrap();
            let trashed_files = file_repo.trashed_files.lock().unwrap();

            assert!(
                files.get(file_id).is_none(),
                "File should not be in main storage"
            );
            assert!(
                trashed_files.get(file_id).is_none(),
                "File should not be in trash storage"
            );
        }

        // Verify the trash item is removed
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(
            trash_items.len(),
            0,
            "Trash should be empty after permanent deletion"
        );
    }

    #[tokio::test]
    async fn test_empty_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());

        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone() as Arc<dyn FileReadPort>,
            file_repo.clone() as Arc<dyn FileWritePort>,
            folder_repo.clone(),
            30, // 30 days retention
        );

        let user_id = "550e8400-e29b-41d4-a716-446655440001";

        // Add multiple files and folders to trash
        let file_ids = [
            "550e8400-e29b-41d4-a716-446655440010",
            "550e8400-e29b-41d4-a716-446655440011",
        ];

        let folder_ids = [
            "550e8400-e29b-41d4-a716-446655440020",
            "550e8400-e29b-41d4-a716-446655440021",
        ];

        // Add test files and folders
        for (i, file_id) in file_ids.iter().enumerate() {
            file_repo.add_test_file(
                file_id,
                &format!("test{}.txt", i),
                &format!("/test/path/test{}.txt", i),
            );
            service
                .move_to_trash(file_id, "file", user_id)
                .await
                .unwrap();
        }

        for (i, folder_id) in folder_ids.iter().enumerate() {
            folder_repo.add_test_folder(
                folder_id,
                &format!("folder{}", i),
                &format!("/test/path/folder{}", i),
            );
            service
                .move_to_trash(folder_id, "folder", user_id)
                .await
                .unwrap();
        }

        // Verify items are in trash
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(trash_items.len(), 4, "Should have 4 items in trash");

        // Act
        let result = service.empty_trash(user_id).await;

        // Assert
        assert!(result.is_ok(), "Emptying trash failed: {:?}", result);

        // Verify all items are permanently deleted
        for file_id in &file_ids {
            let files = file_repo.files.lock().unwrap();
            let trashed_files = file_repo.trashed_files.lock().unwrap();
            assert!(
                files.get(*file_id).is_none(),
                "File should not be in main storage"
            );
            assert!(
                trashed_files.get(*file_id).is_none(),
                "File should not be in trash storage"
            );
        }

        for folder_id in &folder_ids {
            let folders = folder_repo.folders.lock().unwrap();
            let trashed_folders = folder_repo.trashed_folders.lock().unwrap();
            assert!(
                folders.get(*folder_id).is_none(),
                "Folder should not be in main storage"
            );
            assert!(
                trashed_folders.get(*folder_id).is_none(),
                "Folder should not be in trash storage"
            );
        }

        // Verify the trash is empty
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(trash_items.len(), 0, "Trash should be empty after emptying");
    }
}
