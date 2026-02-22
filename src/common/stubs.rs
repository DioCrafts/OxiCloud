//! Stub/Dummy implementations for dependency injection.
//!
//! These no-op implementations are used exclusively by `AppState::default()`
//! to provide a minimal, valid state for the auth middleware and route
//! construction before the real services are wired in `main.rs`.
//!
//! **None of these stubs should ever handle real user requests.**

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::dtos::pagination::{PaginatedResponseDto, PaginationRequestDto};
use crate::application::dtos::search_dto::{
    SearchCriteriaDto, SearchResultsDto, SearchSuggestionsDto,
};
use crate::application::ports::file_ports::{
    FileManagementUseCase, FileRetrievalUseCase, FileUploadUseCase, FileUseCaseFactory,
    OptimizedFileContent,
};
use crate::application::ports::inbound::{FolderUseCase, SearchUseCase};
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::application::ports::zip_ports::ZipPort;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::entities::folder::Folder;
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::services::i18n_service::{I18nResult, I18nService, Locale};
use crate::domain::services::path_service::StoragePath;

// ---------------------------------------------------------------------------
// ZipPort
// ---------------------------------------------------------------------------

/// Placeholder ZipPort that always errors. Replaced after application services
/// are fully initialised.
pub struct StubZipPort;

#[async_trait]
impl ZipPort for StubZipPort {
    async fn create_folder_zip(
        &self,
        _folder_id: &str,
        _folder_name: &str,
    ) -> Result<tempfile::NamedTempFile, DomainError> {
        Err(DomainError::internal_error(
            "ZipService",
            "ZipService not initialized",
        ))
    }
}

// ---------------------------------------------------------------------------
// FileReadPort
// ---------------------------------------------------------------------------

pub struct StubFileReadPort;

#[async_trait]
impl FileReadPort for StubFileReadPort {
    async fn get_file(&self, _id: &str) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        Ok(Vec::new())
    }

    async fn get_file_stream(
        &self,
        _id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        let empty_stream = futures::stream::empty::<Result<Bytes, std::io::Error>>();
        Ok(Box::new(empty_stream))
    }

    async fn get_file_range_stream(
        &self,
        _id: &str,
        _start: u64,
        _end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        let empty_stream = futures::stream::empty::<Result<Bytes, std::io::Error>>();
        Ok(Box::new(empty_stream))
    }

    async fn get_file_path(&self, _id: &str) -> Result<StoragePath, DomainError> {
        Ok(StoragePath::from_string("/"))
    }

    async fn get_parent_folder_id(&self, _path: &str) -> Result<String, DomainError> {
        Ok("root".to_string())
    }

    async fn get_blob_hash(&self, _file_id: &str) -> Result<String, DomainError> {
        Ok(String::new())
    }

    async fn search_files_paginated(
        &self,
        _folder_id: Option<&str>,
        _criteria: &SearchCriteriaDto,
        _user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError> {
        Ok((Vec::new(), 0))
    }

    async fn count_files(
        &self,
        _folder_id: Option<&str>,
        _criteria: &SearchCriteriaDto,
        _user_id: &str,
    ) -> Result<usize, DomainError> {
        Ok(0)
    }
}

// ---------------------------------------------------------------------------
// FileWritePort
// ---------------------------------------------------------------------------

pub struct StubFileWritePort;

#[async_trait]
impl FileWritePort for StubFileWritePort {
    async fn save_file(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _content: Vec<u8>,
    ) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn save_file_from_temp(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _temp_path: &Path,
        _size: u64,
        _pre_computed_hash: Option<String>,
    ) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn move_file(
        &self,
        _file_id: &str,
        _target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn copy_file(
        &self,
        _file_id: &str,
        _target_folder_id: Option<String>,
    ) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn rename_file(&self, _file_id: &str, _new_name: &str) -> Result<File, DomainError> {
        Ok(File::default())
    }

    async fn delete_file(&self, _id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn update_file_content(
        &self,
        _file_id: &str,
        _content: Vec<u8>,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    async fn update_file_content_from_temp(
        &self,
        _file_id: &str,
        _temp_path: &Path,
        _size: u64,
        _content_type: Option<String>,
        _pre_computed_hash: Option<String>,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    async fn register_file_deferred(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _size: u64,
    ) -> Result<(File, PathBuf), DomainError> {
        Ok((File::default(), PathBuf::from("/tmp/dummy")))
    }

    async fn move_to_trash(&self, _file_id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn restore_from_trash(
        &self,
        _file_id: &str,
        _original_path: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    async fn delete_file_permanently(&self, _file_id: &str) -> Result<(), DomainError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// FolderStoragePort
// ---------------------------------------------------------------------------

pub struct StubFolderStoragePort;

#[async_trait]
impl FolderRepository for StubFolderStoragePort {
    async fn create_folder(
        &self,
        _name: String,
        _parent_id: Option<String>,
    ) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }

    async fn get_folder(&self, _id: &str) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }

    async fn get_folder_by_path(&self, _storage_path: &StoragePath) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }

    async fn list_folders(&self, _parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError> {
        Ok(Vec::new())
    }

    async fn list_folders_by_owner(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
    ) -> Result<Vec<Folder>, DomainError> {
        Ok(Vec::new())
    }

    async fn list_folders_paginated(
        &self,
        _parent_id: Option<&str>,
        _offset: usize,
        _limit: usize,
        _include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        Ok((Vec::new(), Some(0)))
    }

    async fn list_folders_by_owner_paginated(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
        _offset: usize,
        _limit: usize,
        _include_total: bool,
    ) -> Result<(Vec<Folder>, Option<usize>), DomainError> {
        Ok((Vec::new(), Some(0)))
    }

    async fn rename_folder(&self, _id: &str, _new_name: String) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }

    async fn move_folder(
        &self,
        _id: &str,
        _new_parent_id: Option<&str>,
    ) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }

    async fn delete_folder(&self, _id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn folder_exists(&self, _storage_path: &StoragePath) -> Result<bool, DomainError> {
        Ok(false)
    }

    async fn get_folder_path(&self, _id: &str) -> Result<StoragePath, DomainError> {
        Ok(StoragePath::from_string("/"))
    }

    async fn move_to_trash(&self, _folder_id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn restore_from_trash(
        &self,
        _folder_id: &str,
        _original_path: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    async fn delete_folder_permanently(&self, _folder_id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn create_home_folder(
        &self,
        _user_id: &str,
        _name: String,
    ) -> Result<Folder, DomainError> {
        Ok(Folder::default())
    }
}

// ---------------------------------------------------------------------------
// I18nService
// ---------------------------------------------------------------------------

pub struct StubI18nService;

#[async_trait]
impl I18nService for StubI18nService {
    async fn translate(&self, _key: &str, _locale: Locale) -> I18nResult<String> {
        Ok(String::new())
    }

    async fn load_translations(&self, _locale: Locale) -> I18nResult<()> {
        Ok(())
    }

    async fn available_locales(&self) -> Vec<Locale> {
        vec![Locale::default()]
    }

    async fn is_supported(&self, _locale: Locale) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// FolderUseCase
// ---------------------------------------------------------------------------

pub struct StubFolderUseCase;

#[async_trait]
impl FolderUseCase for StubFolderUseCase {
    async fn create_folder(&self, _dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }

    async fn get_folder(&self, _id: &str) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }

    async fn get_folder_by_path(&self, _path: &str) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }

    async fn list_folders(&self, _parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError> {
        Ok(Vec::new())
    }

    async fn list_folders_for_owner(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
    ) -> Result<Vec<FolderDto>, DomainError> {
        Ok(Vec::new())
    }

    async fn list_folders_paginated(
        &self,
        _parent_id: Option<&str>,
        _pagination: &PaginationRequestDto,
    ) -> Result<PaginatedResponseDto<FolderDto>, DomainError> {
        Ok(PaginatedResponseDto::new(Vec::new(), 0, 10, 0))
    }

    async fn list_folders_for_owner_paginated(
        &self,
        _parent_id: Option<&str>,
        _owner_id: &str,
        _pagination: &PaginationRequestDto,
    ) -> Result<PaginatedResponseDto<FolderDto>, DomainError> {
        Ok(PaginatedResponseDto::new(Vec::new(), 0, 10, 0))
    }

    async fn rename_folder(
        &self,
        _id: &str,
        _dto: RenameFolderDto,
        _caller_id: &str,
    ) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }

    async fn move_folder(
        &self,
        _id: &str,
        _dto: MoveFolderDto,
        _caller_id: &str,
    ) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }

    async fn delete_folder(&self, _id: &str, _caller_id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn create_home_folder(
        &self,
        _user_id: &str,
        _name: String,
    ) -> Result<FolderDto, DomainError> {
        Ok(FolderDto::default())
    }
}

// ---------------------------------------------------------------------------
// FileUploadUseCase
// ---------------------------------------------------------------------------

pub struct StubFileUploadUseCase;

#[async_trait]
impl FileUploadUseCase for StubFileUploadUseCase {
    async fn upload_file_streaming(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _temp_path: &Path,
        _size: u64,
        _pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn upload_file(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _content: Vec<u8>,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn upload_file_from_path(
        &self,
        _name: String,
        _folder_id: Option<String>,
        _content_type: String,
        _file_path: &Path,
        _pre_computed_hash: Option<String>,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn create_file(
        &self,
        _parent_path: &str,
        _filename: &str,
        _content: &[u8],
        _content_type: &str,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn update_file(&self, _path: &str, _content: &[u8]) -> Result<(), DomainError> {
        Ok(())
    }

    async fn update_file_streaming(
        &self,
        _path: &str,
        _temp_path: &Path,
        _size: u64,
        _content_type: &str,
        _pre_computed_hash: Option<String>,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// FileRetrievalUseCase
// ---------------------------------------------------------------------------

pub struct StubFileRetrievalUseCase;

#[async_trait]
impl FileRetrievalUseCase for StubFileRetrievalUseCase {
    async fn get_file(&self, _id: &str) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<FileDto>, DomainError> {
        Ok(Vec::new())
    }

    async fn get_file_stream(
        &self,
        _id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        let empty_stream = futures::stream::empty::<Result<Bytes, std::io::Error>>();
        Ok(Box::new(empty_stream))
    }

    async fn get_file_optimized(
        &self,
        _id: &str,
        _accept_webp: bool,
        _prefer_original: bool,
    ) -> Result<(FileDto, OptimizedFileContent), DomainError> {
        Ok((
            FileDto::default(),
            OptimizedFileContent::Bytes {
                data: Bytes::new(),
                mime_type: String::new(),
                was_transcoded: false,
            },
        ))
    }

    async fn get_file_range_stream(
        &self,
        _id: &str,
        _start: u64,
        _end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        let empty_stream = futures::stream::empty::<Result<Bytes, std::io::Error>>();
        Ok(Box::new(empty_stream))
    }

    async fn get_file_by_path(&self, _path: &str) -> Result<FileDto, DomainError> {
        Err(DomainError::not_found("File", "stub"))
    }
}

// ---------------------------------------------------------------------------
// FileManagementUseCase
// ---------------------------------------------------------------------------

pub struct StubFileManagementUseCase;

#[async_trait]
impl FileManagementUseCase for StubFileManagementUseCase {
    async fn move_file(
        &self,
        _file_id: &str,
        _folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn copy_file(
        &self,
        _file_id: &str,
        _folder_id: Option<String>,
    ) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn rename_file(&self, _file_id: &str, _new_name: &str) -> Result<FileDto, DomainError> {
        Ok(FileDto::default())
    }

    async fn delete_file(&self, _id: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn delete_with_cleanup(&self, _id: &str, _user_id: &str) -> Result<bool, DomainError> {
        Ok(false)
    }
}

// ---------------------------------------------------------------------------
// FileUseCaseFactory
// ---------------------------------------------------------------------------

pub struct StubFileUseCaseFactory;

impl FileUseCaseFactory for StubFileUseCaseFactory {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase> {
        Arc::new(StubFileUploadUseCase)
    }

    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase> {
        Arc::new(StubFileRetrievalUseCase)
    }

    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase> {
        Arc::new(StubFileManagementUseCase)
    }
}

// ---------------------------------------------------------------------------
// SearchUseCase
// ---------------------------------------------------------------------------

pub struct StubSearchUseCase;

#[async_trait]
impl SearchUseCase for StubSearchUseCase {
    async fn search(&self, _criteria: SearchCriteriaDto) -> Result<SearchResultsDto, DomainError> {
        Ok(SearchResultsDto::empty())
    }

    async fn suggest(
        &self,
        _query: &str,
        _folder_id: Option<&str>,
        _limit: usize,
    ) -> Result<SearchSuggestionsDto, DomainError> {
        Ok(SearchSuggestionsDto {
            suggestions: Vec::new(),
            query_time_ms: 0,
        })
    }

    async fn clear_search_cache(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DedupPort
// ---------------------------------------------------------------------------

use crate::application::ports::dedup_ports::{
    BlobMetadataDto, DedupPort, DedupResultDto, DedupStatsDto,
};

pub struct StubDedupPort;

#[async_trait]
impl DedupPort for StubDedupPort {
    async fn store_bytes(
        &self,
        _content: &[u8],
        _content_type: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        Err(DomainError::internal_error(
            "DedupService",
            "DedupService not initialized",
        ))
    }

    async fn store_from_file(
        &self,
        _source_path: &Path,
        _content_type: Option<String>,
        _pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError> {
        Err(DomainError::internal_error(
            "DedupService",
            "DedupService not initialized",
        ))
    }

    async fn blob_exists(&self, _hash: &str) -> bool {
        false
    }

    async fn get_blob_metadata(&self, _hash: &str) -> Option<BlobMetadataDto> {
        None
    }

    async fn read_blob_stream(
        &self,
        _hash: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        Err(DomainError::internal_error(
            "DedupService",
            "DedupService not initialized",
        ))
    }

    async fn read_blob_range_stream(
        &self,
        _hash: &str,
        _start: u64,
        _end: Option<u64>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>, DomainError>
    {
        Err(DomainError::internal_error(
            "DedupService",
            "DedupService not initialized",
        ))
    }

    async fn blob_size(&self, _hash: &str) -> Result<u64, DomainError> {
        Err(DomainError::internal_error(
            "DedupService",
            "DedupService not initialized",
        ))
    }

    async fn add_reference(&self, _hash: &str) -> Result<(), DomainError> {
        Ok(())
    }

    async fn remove_reference(&self, _hash: &str) -> Result<bool, DomainError> {
        Ok(false)
    }

    async fn hash_file(&self, _path: &Path) -> Result<String, DomainError> {
        Ok(String::new())
    }

    async fn get_stats(&self) -> DedupStatsDto {
        DedupStatsDto::default()
    }

    async fn flush(&self) -> Result<(), DomainError> {
        Ok(())
    }

    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError> {
        Ok(Vec::new())
    }
}

// ---------------------------------------------------------------------------
// MetadataCachePort
// ---------------------------------------------------------------------------

use crate::application::ports::cache_ports::{
    CachedMetadataDto, ContentCachePort, MetadataCachePort,
};

pub struct StubMetadataCachePort;

#[async_trait]
impl MetadataCachePort for StubMetadataCachePort {
    async fn get_metadata(&self, _path: &Path) -> Option<CachedMetadataDto> {
        None
    }

    async fn is_file(&self, _path: &Path) -> Option<bool> {
        None
    }

    async fn refresh_metadata(&self, path: &Path) -> Result<CachedMetadataDto, DomainError> {
        Ok(CachedMetadataDto {
            path: path.to_path_buf(),
            exists: false,
            is_file: false,
            size: None,
            mime_type: None,
            created_at: None,
            modified_at: None,
        })
    }

    async fn invalidate(&self, _path: &Path) {}

    async fn invalidate_directory(&self, _dir_path: &Path) {}
}

// ---------------------------------------------------------------------------
// ContentCachePort
// ---------------------------------------------------------------------------

pub struct StubContentCachePort;

#[async_trait]
impl ContentCachePort for StubContentCachePort {
    fn should_cache(&self, _size: usize) -> bool {
        false
    }

    async fn get(&self, _file_id: &str) -> Option<(Bytes, String, String)> {
        None
    }

    async fn put(&self, _file_id: String, _content: Bytes, _etag: String, _content_type: String) {}

    async fn invalidate(&self, _file_id: &str) {}

    async fn clear(&self) {}
}
