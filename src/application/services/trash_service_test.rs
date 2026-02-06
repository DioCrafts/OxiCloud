use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::common::errors::{Result, DomainError};
use crate::domain::entities::file::File;
use crate::domain::entities::folder::Folder;
use crate::domain::entities::trashed_item::{TrashedItem, TrashedItemType};
use crate::domain::repositories::file_repository::{FileRepository, FileRepositoryResult, FileRepositoryError};
use crate::domain::repositories::folder_repository::{FolderRepository, FolderRepositoryResult, FolderRepositoryError};
use crate::domain::repositories::trash_repository::TrashRepository;
use crate::domain::services::path_service::StoragePath;
use crate::application::services::trash_service::TrashService;

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
        items.insert(item.id, item.clone());
        Ok(())
    }

    async fn get_trash_items(&self, user_id: &Uuid) -> Result<Vec<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let user_items = items.values()
            .filter(|item| item.user_id == *user_id)
            .cloned()
            .collect();
        Ok(user_items)
    }

    async fn get_trash_item(&self, id: &Uuid, user_id: &Uuid) -> Result<Option<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let item = items.get(id)
            .filter(|item| item.user_id == *user_id)
            .cloned();
        Ok(item)
    }

    async fn restore_from_trash(&self, id: &Uuid, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        if let Some(item) = items.get(id) {
            if item.user_id == *user_id {
                items.remove(id);
            }
        }
        Ok(())
    }

    async fn delete_permanently(&self, id: &Uuid, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        if let Some(item) = items.get(id) {
            if item.user_id == *user_id {
                items.remove(id);
            }
        }
        Ok(())
    }

    async fn clear_trash(&self, user_id: &Uuid) -> Result<()> {
        let mut items = self.trash_items.lock().unwrap();
        items.retain(|_, item| item.user_id != *user_id);
        Ok(())
    }

    async fn get_expired_items(&self) -> Result<Vec<TrashedItem>> {
        let items = self.trash_items.lock().unwrap();
        let now = Utc::now();
        let expired = items.values()
            .filter(|item| item.deletion_date <= now)
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
        ).unwrap();
        
        let mut files = self.files.lock().unwrap();
        files.insert(id.to_string(), file);
    }
}

#[async_trait]
impl FileRepository for MockFileRepository {
    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File> {
        let files = self.files.lock().unwrap();
        if let Some(file) = files.get(id) {
            Ok(file.clone())
        } else {
            Err(FileRepositoryError::NotFound(id.to_string()))
        }
    }

    async fn move_to_trash(&self, id: &str) -> FileRepositoryResult<()> {
        let mut files = self.files.lock().unwrap();
        let mut trashed = self.trashed_files.lock().unwrap();
        
        if let Some(file) = files.remove(id) {
            trashed.insert(id.to_string(), file);
            Ok(())
        } else {
            Err(FileRepositoryError::NotFound(id.to_string()))
        }
    }

    async fn restore_from_trash(&self, id: &str, original_path: &str) -> FileRepositoryResult<()> {
        let mut files = self.files.lock().unwrap();
        let mut trashed = self.trashed_files.lock().unwrap();
        
        if let Some(file) = trashed.remove(id) {
            files.insert(id.to_string(), file);
            Ok(())
        } else {
            Err(FileRepositoryError::NotFound(format!("File {} not found in trash", id)))
        }
    }

    async fn delete_file_permanently(&self, id: &str) -> FileRepositoryResult<()> {
        let mut trashed = self.trashed_files.lock().unwrap();
        if trashed.remove(id).is_some() {
            Ok(())
        } else {
            Err(FileRepositoryError::NotFound(format!("File {} not found in trash", id)))
        }
    }

    // Other methods required by the trait (not used in tests)
    async fn save_file(&self, _file: &File) -> FileRepositoryResult<()> { Ok(()) }
    async fn delete_file(&self, _id: &str) -> FileRepositoryResult<()> { Ok(()) }
    async fn get_files_in_folder(&self, _folder_id: Option<&str>) -> FileRepositoryResult<Vec<File>> { Ok(vec![]) }
    async fn move_file(&self, _id: &str, _new_folder_id: Option<&str>) -> FileRepositoryResult<()> { Ok(()) }
    async fn update_file_data(&self, _id: &str, _new_data: &[u8]) -> FileRepositoryResult<()> { Ok(()) }
    async fn get_file_data(&self, _id: &str) -> FileRepositoryResult<Vec<u8>> { Ok(vec![]) }
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
        ).unwrap();
        
        let mut folders = self.folders.lock().unwrap();
        folders.insert(id.to_string(), folder);
    }
}

#[async_trait]
impl FolderRepository for MockFolderRepository {
    async fn get_folder_by_id(&self, id: &str) -> FolderRepositoryResult<Folder> {
        let folders = self.folders.lock().unwrap();
        if let Some(folder) = folders.get(id) {
            Ok(folder.clone())
        } else {
            Err(FolderRepositoryError::NotFound(id.to_string()))
        }
    }

    async fn move_to_trash(&self, id: &str) -> FolderRepositoryResult<()> {
        let mut folders = self.folders.lock().unwrap();
        let mut trashed = self.trashed_folders.lock().unwrap();
        
        if let Some(folder) = folders.remove(id) {
            trashed.insert(id.to_string(), folder);
            Ok(())
        } else {
            Err(FolderRepositoryError::NotFound(id.to_string()))
        }
    }

    async fn restore_from_trash(&self, id: &str, original_path: &str) -> FolderRepositoryResult<()> {
        let mut folders = self.folders.lock().unwrap();
        let mut trashed = self.trashed_folders.lock().unwrap();
        
        if let Some(folder) = trashed.remove(id) {
            folders.insert(id.to_string(), folder);
            Ok(())
        } else {
            Err(FolderRepositoryError::NotFound(format!("Folder {} not found in trash", id)))
        }
    }

    async fn delete_folder_permanently(&self, id: &str) -> FolderRepositoryResult<()> {
        let mut trashed = self.trashed_folders.lock().unwrap();
        if trashed.remove(id).is_some() {
            Ok(())
        } else {
            Err(FolderRepositoryError::NotFound(format!("Folder {} not found in trash", id)))
        }
    }

    // Other methods required by the trait (not used in tests)
    async fn save_folder(&self, _folder: &Folder) -> FolderRepositoryResult<()> { Ok(()) }
    async fn delete_folder(&self, _id: &str) -> FolderRepositoryResult<()> { Ok(()) }
    async fn get_folders_in_folder(&self, _parent_id: Option<&str>) -> FolderRepositoryResult<Vec<Folder>> { Ok(vec![]) }
    async fn move_folder(&self, _id: &str, _new_parent_id: Option<&str>) -> FolderRepositoryResult<()> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_move_file_to_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());
        
        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone(),
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
        
        assert_eq!(trash_items.len(), 1, "Should have exactly one item in trash");
        let trash_item = &trash_items[0];
        
        assert_eq!(trash_item.original_id.to_string(), file_id, "Original ID should match file ID");
        assert_eq!(trash_item.user_id.to_string(), user_id, "User ID should match");
        assert_eq!(trash_item.item_type, TrashedItemType::File, "Item type should be File");
        assert_eq!(trash_item.name, "test.txt", "File name should match");
        
        // Verify file is moved in file repository
        let files = file_repo.files.lock().unwrap();
        let trashed_files = file_repo.trashed_files.lock().unwrap();
        
        assert!(files.get(file_id).is_none(), "File should no longer be in main storage");
        assert!(trashed_files.get(file_id).is_some(), "File should be in trash storage");
    }

    #[tokio::test]
    async fn test_move_folder_to_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());
        
        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone(),
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
        assert!(result.is_ok(), "Moving folder to trash failed: {:?}", result);
        
        // Verify the folder is in trash
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        
        assert_eq!(trash_items.len(), 1, "Should have exactly one item in trash");
        let trash_item = &trash_items[0];
        
        assert_eq!(trash_item.original_id.to_string(), folder_id, "Original ID should match folder ID");
        assert_eq!(trash_item.user_id.to_string(), user_id, "User ID should match");
        assert_eq!(trash_item.item_type, TrashedItemType::Folder, "Item type should be Folder");
        assert_eq!(trash_item.name, "test_folder", "Folder name should match");
    }

    #[tokio::test]
    async fn test_restore_file_from_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());
        
        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone(),
            folder_repo.clone(),
            30, // 30 days retention
        );
        
        let file_id = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";
        let file_path = "/test/path/test.txt";
        
        // Add a test file and move it to trash
        file_repo.add_test_file(file_id, "test.txt", file_path);
        service.move_to_trash(file_id, "file", user_id).await.unwrap();
        
        // Get the trash item ID
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        let trash_id = trash_items[0].id.to_string();
        
        // Act
        let result = service.restore_item(&trash_id, user_id).await;
        
        // Assert
        assert!(result.is_ok(), "Restoring file from trash failed: {:?}", result);
        
        // Verify the file is restored in file repository
        let files = file_repo.files.lock().unwrap();
        let trashed_files = file_repo.trashed_files.lock().unwrap();
        
        assert!(files.get(file_id).is_some(), "File should be back in main storage");
        assert!(trashed_files.get(file_id).is_none(), "File should no longer be in trash storage");
        
        // Verify the trash item is removed
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(trash_items.len(), 0, "Trash should be empty after restoration");
    }

    #[tokio::test]
    async fn test_delete_permanently() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());
        
        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone(),
            folder_repo.clone(),
            30, // 30 days retention
        );
        
        let file_id = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = "550e8400-e29b-41d4-a716-446655440001";
        
        // Add a test file and move it to trash
        file_repo.add_test_file(file_id, "test.txt", "/test/path/test.txt");
        service.move_to_trash(file_id, "file", user_id).await.unwrap();
        
        // Get the trash item ID
        let user_uuid = Uuid::parse_str(user_id).unwrap();
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        let trash_id = trash_items[0].id.to_string();
        
        // Act
        let result = service.delete_permanently(&trash_id, user_id).await;
        
        // Assert
        assert!(result.is_ok(), "Deleting file permanently failed: {:?}", result);
        
        // Verify the file is permanently deleted
        let files = file_repo.files.lock().unwrap();
        let trashed_files = file_repo.trashed_files.lock().unwrap();
        
        assert!(files.get(file_id).is_none(), "File should not be in main storage");
        assert!(trashed_files.get(file_id).is_none(), "File should not be in trash storage");
        
        // Verify the trash item is removed
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(trash_items.len(), 0, "Trash should be empty after permanent deletion");
    }

    #[tokio::test]
    async fn test_empty_trash() {
        // Arrange
        let trash_repo = Arc::new(MockTrashRepository::new());
        let file_repo = Arc::new(MockFileRepository::new());
        let folder_repo = Arc::new(MockFolderRepository::new());
        
        let service = TrashService::new(
            trash_repo.clone(),
            file_repo.clone(),
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
            file_repo.add_test_file(file_id, &format!("test{}.txt", i), &format!("/test/path/test{}.txt", i));
            service.move_to_trash(file_id, "file", user_id).await.unwrap();
        }
        
        for (i, folder_id) in folder_ids.iter().enumerate() {
            folder_repo.add_test_folder(folder_id, &format!("folder{}", i), &format!("/test/path/folder{}", i));
            service.move_to_trash(folder_id, "folder", user_id).await.unwrap();
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
            assert!(files.get(*file_id).is_none(), "File should not be in main storage");
            assert!(trashed_files.get(*file_id).is_none(), "File should not be in trash storage");
        }
        
        for folder_id in &folder_ids {
            let folders = folder_repo.folders.lock().unwrap();
            let trashed_folders = folder_repo.trashed_folders.lock().unwrap();
            assert!(folders.get(*folder_id).is_none(), "Folder should not be in main storage");
            assert!(trashed_folders.get(*folder_id).is_none(), "Folder should not be in trash storage");
        }
        
        // Verify the trash is empty
        let trash_items = trash_repo.get_trash_items(&user_uuid).await.unwrap();
        assert_eq!(trash_items.len(), 0, "Trash should be empty after emptying");
    }
}