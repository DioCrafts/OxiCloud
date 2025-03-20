use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use sqlx::PgPool;

use crate::domain::services::auth_service::AuthService;
use crate::application::services::auth_application_service::AuthApplicationService;

use crate::domain::services::path_service::PathService;
use crate::infrastructure::repositories::folder_fs_repository::FolderFsRepository;
use crate::infrastructure::repositories::file_fs_repository::FileFsRepository;
use crate::infrastructure::services::file_system_i18n_service::FileSystemI18nService;
use crate::infrastructure::services::id_mapping_service::IdMappingService;
use crate::infrastructure::services::cache_manager::StorageCacheManager;
use crate::infrastructure::services::file_metadata_cache::FileMetadataCache;
use crate::application::services::folder_service::FolderService;
use crate::application::services::file_service::FileService;
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::application::services::storage_mediator::{StorageMediator, FileSystemStorageMediator};
use crate::application::services::sharing_service::SharingService;
use crate::application::services::public_link_service::PublicLinkService;
use crate::application::ports::inbound::{FileUseCase, FolderUseCase, UseCaseFactory};
use crate::application::ports::outbound::{FileStoragePort, FolderStoragePort};
use crate::application::ports::file_ports::{FileUploadUseCase, FileRetrievalUseCase, FileManagementUseCase, FileUseCaseFactory};
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort, FilePathResolutionPort};
use crate::infrastructure::repositories::{FileMetadataManager, FilePathResolver, FileFsReadRepository, FileFsWriteRepository};
use crate::application::services::{FileUploadService, FileRetrievalService, FileManagementService, AppFileUseCaseFactory};
use crate::common::errors::DomainError;
use crate::domain::services::i18n_service::I18nService;
use crate::common::config::AppConfig;
use crate::domain::repositories::folder_repository::FolderRepository;

/// Fábrica para los diferentes componentes de la aplicación
#[allow(dead_code)]
pub struct AppServiceFactory {
    storage_path: PathBuf,
    locales_path: PathBuf,
    config: AppConfig,
    db_pool: Option<Arc<PgPool>>,
}

impl AppServiceFactory {
    /// Crea una nueva fábrica de servicios
    #[allow(dead_code)]
    pub fn new(storage_path: PathBuf, locales_path: PathBuf) -> Self {
        Self {
            storage_path,
            locales_path,
            config: AppConfig::default(),
            db_pool: None,
        }
    }
    
    /// Crea una nueva fábrica de servicios con configuración personalizada
    #[allow(dead_code)]
    pub fn with_config(storage_path: PathBuf, locales_path: PathBuf, config: AppConfig) -> Self {
        Self {
            storage_path,
            locales_path,
            config,
            db_pool: None,
        }
    }
    
    /// Crea una nueva fábrica de servicios con base de datos
    #[allow(dead_code)]
    pub fn with_database(mut self, db_pool: Arc<PgPool>) -> Self {
        self.db_pool = Some(db_pool);
        self
    }
    
    /// Inicializa los servicios base del sistema
    #[allow(dead_code)]
    pub async fn create_core_services(&self) -> Result<CoreServices, DomainError> {
        // Path service
        let path_service = Arc::new(PathService::new(self.storage_path.clone()));
        
        // Cache manager
        // TTL values in milliseconds and max entries for cache
        let file_ttl_ms = 60_000; // 1 minute for files
        let dir_ttl_ms = 120_000; // 2 minutes for directories
        let max_entries = 10_000; // Maximum cache entries
        let cache_manager = Arc::new(StorageCacheManager::new(file_ttl_ms, dir_ttl_ms, max_entries));
        
        // Iniciar tarea de limpieza de caché en segundo plano
        let cache_manager_clone = cache_manager.clone();
        tokio::spawn(async move {
            StorageCacheManager::start_cleanup_task(cache_manager_clone).await;
        });
        
        // ID mapping service
        let id_mapping_path = self.storage_path.join("folder_ids.json");
        let id_mapping_service = Arc::new(
            IdMappingService::new(id_mapping_path).await?
        );
        
        Ok(CoreServices {
            path_service,
            cache_manager,
            id_mapping_service,
            config: self.config.clone(),
        })
    }
    
    /// Inicializa los servicios de repositorio utilizando el patrón Builder mejorado
    #[allow(dead_code)]
    pub fn create_repository_services(&self, core: &CoreServices) -> RepositoryServices {
        // Storage mediator - con inicialización diferida para folder repository
        let folder_repository_holder = Arc::new(RwLock::new(None));
        
        let storage_mediator = Arc::new(FileSystemStorageMediator::new_with_lazy_folder(
            folder_repository_holder.clone(),
            core.path_service.clone(),
            core.id_mapping_service.clone()
        ));
        
        // Folder repository
        let folder_repository = Arc::new(FolderFsRepository::new(
            self.storage_path.clone(),
            storage_mediator.clone(),
            core.id_mapping_service.clone(),
            core.path_service.clone(),
        ));
        
        // Actualizar el holder para el mediador una vez que el repository está creado
        if let Ok(mut holder) = folder_repository_holder.write() {
            *holder = Some(folder_repository.clone());
        }
        
        // Metadata cache
        let metadata_cache = Arc::new(
            FileMetadataCache::default_with_config(core.config.clone())
        );
        
        // Componentes refactorizados
        let metadata_manager = Arc::new(FileMetadataManager::new(
            metadata_cache.clone(),
            core.config.clone()
        ));
        
        let path_resolver = Arc::new(FilePathResolver::new(
            core.path_service.clone(),
            storage_mediator.clone(),
            core.id_mapping_service.clone()
        ));
        
        // File repositories separados para lectura y escritura
        let file_read_repository = Arc::new(FileFsReadRepository::new(
            self.storage_path.clone(),
            metadata_manager.clone(),
            path_resolver.clone(),
            core.config.clone(),
            None // processor will be added later if needed
        ));
        
        let file_write_repository = Arc::new(FileFsWriteRepository::new(
            self.storage_path.clone(),
            metadata_manager.clone(),
            path_resolver.clone(),
            storage_mediator.clone(),
            core.config.clone(),
            None // processor will be added later if needed
        ));
        
        // Legacy file repository - mantenido por compatibilidad
        let file_repository = Arc::new(FileFsRepository::new(
            self.storage_path.clone(), 
            storage_mediator.clone(),
            core.id_mapping_service.clone(),
            core.path_service.clone(),
            metadata_cache,
        ));
        
        // I18n repository
        let i18n_repository = Arc::new(FileSystemI18nService::new(
            self.locales_path.clone()
        ));
        
        RepositoryServices {
            folder_repository,
            file_repository,
            file_read_repository,
            file_write_repository,
            i18n_repository,
            storage_mediator,
            metadata_manager,
            path_resolver,
        }
    }
    
    /// Inicializa los servicios de aplicación
    #[allow(dead_code)]
    pub fn create_application_services(&self, repos: &RepositoryServices) -> ApplicationServices {
        // Servicios principales
        let folder_service = Arc::new(FolderService::new(
            repos.folder_repository.clone()
        ));
        
        // Antiguo servicio único
        let file_service = Arc::new(FileService::new(
            repos.file_repository.clone()
        ));
        
        // Nuevos servicios refactorizados
        let file_upload_service = Arc::new(FileUploadService::new(
            repos.file_write_repository.clone()
        ));
        
        let file_retrieval_service = Arc::new(FileRetrievalService::new(
            repos.file_read_repository.clone()
        ));
        
        let file_management_service = Arc::new(FileManagementService::new(
            repos.file_write_repository.clone()
        ));
        
        let file_use_case_factory = Arc::new(AppFileUseCaseFactory::new(
            repos.file_read_repository.clone(),
            repos.file_write_repository.clone()
        ));
        
        let i18n_service = Arc::new(I18nApplicationService::new(
            repos.i18n_repository.clone()
        ));
        
        // Sharing services are created only if database pool is available
        let (sharing_service, public_link_service) = if let Some(db_pool) = self.db_pool.as_ref() {
            // Create repositories for sharing features
            use crate::domain::repositories::shared_file_repository::SharedFileRepository;
            use crate::domain::repositories::public_link_repository::PublicLinkRepository;
            use crate::infrastructure::repositories::pg::shared_file_pg_repository::SharedFilePgRepository;
            use crate::infrastructure::repositories::pg::public_link_pg_repository::PublicLinkPgRepository;
            
            let shared_file_repo = Arc::new(SharedFilePgRepository::new(db_pool.clone())) as Arc<dyn SharedFileRepository>;
            let public_link_repo = Arc::new(PublicLinkPgRepository::new(db_pool.clone())) as Arc<dyn PublicLinkRepository>;
            
            // Create services
            let sharing_svc = Arc::new(SharingService::new(
                shared_file_repo,
                repos.file_repository.clone(),
            ));
            
            let public_link_svc = Arc::new(PublicLinkService::new(
                public_link_repo,
                repos.file_repository.clone(),
            ));
            
            (Some(sharing_svc), Some(public_link_svc))
        } else {
            (None, None)
        };
        
        ApplicationServices {
            folder_service,
            file_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service,
            sharing_service,
            public_link_service,
        }
    }
}

/// Contenedor para servicios base
#[allow(dead_code)]
pub struct CoreServices {
    pub path_service: Arc<PathService>,
    pub cache_manager: Arc<StorageCacheManager>,
    pub id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    pub config: AppConfig,
}

/// Contenedor para servicios de repositorio
#[allow(dead_code)]
pub struct RepositoryServices {
    pub folder_repository: Arc<dyn FolderStoragePort>,
    pub file_repository: Arc<dyn FileStoragePort>,
    pub file_read_repository: Arc<dyn FileReadPort>,
    pub file_write_repository: Arc<dyn FileWritePort>,
    pub i18n_repository: Arc<dyn I18nService>,
    pub storage_mediator: Arc<dyn StorageMediator>,
    pub metadata_manager: Arc<FileMetadataManager>,
    pub path_resolver: Arc<FilePathResolver>,
}

/// Contenedor para servicios de aplicación
#[allow(dead_code)]
pub struct ApplicationServices {
    pub folder_service: Arc<dyn FolderUseCase>,
    pub file_service: Arc<dyn FileUseCase>,
    pub file_upload_service: Arc<dyn FileUploadUseCase>,
    pub file_retrieval_service: Arc<dyn FileRetrievalUseCase>,
    pub file_management_service: Arc<dyn FileManagementUseCase>,
    pub file_use_case_factory: Arc<dyn FileUseCaseFactory>,
    pub i18n_service: Arc<I18nApplicationService>,
    pub sharing_service: Option<Arc<SharingService>>,
    pub public_link_service: Option<Arc<PublicLinkService>>,
}

/// Contenedor para servicios de autenticación
#[allow(dead_code)]
pub struct AuthServices {
    pub auth_service: Arc<AuthService>,
    pub auth_application_service: Arc<AuthApplicationService>,
}

/// Estado global de la aplicación para dependency injection
pub struct AppState {
    pub core: CoreServices,
    pub repositories: RepositoryServices,
    pub applications: ApplicationServices,
    pub db_pool: Option<Arc<PgPool>>,
    pub auth_service: Option<AuthServices>,
}

impl Default for AppState {
    fn default() -> Self {
        // This is just a minimal stub version for auth middleware
        // We'll need to create proper instance in main.rs
        
        let config = crate::common::config::AppConfig::default();
        let path_service = Arc::new(
            crate::domain::services::path_service::PathService::new(
                std::path::PathBuf::from("./storage")
            )
        );
        
        // Create stub service implementations
        struct DummyIdMappingService;
        #[async_trait::async_trait]
        impl crate::application::ports::outbound::IdMappingPort for DummyIdMappingService {
            async fn get_or_create_id(&self, _path: &crate::domain::services::path_service::StoragePath) -> Result<String, crate::common::errors::DomainError> {
                Ok("dummy-id".to_string())
            }
            
            async fn get_path_by_id(&self, _id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::common::errors::DomainError> {
                Ok(crate::domain::services::path_service::StoragePath::from_string("/"))
            }
            
            async fn update_path(&self, _id: &str, _new_path: &crate::domain::services::path_service::StoragePath) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn remove_id(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn save_changes(&self) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
        }
        
        struct DummyStorageMediator;
        #[async_trait::async_trait]
        impl crate::application::services::storage_mediator::StorageMediator for DummyStorageMediator {
            async fn get_folder_path(&self, _folder_id: &str) -> Result<std::path::PathBuf, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(std::path::PathBuf::from("/tmp"))
            }
            
            async fn get_folder_storage_path(&self, _folder_id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(crate::domain::services::path_service::StoragePath::root())
            }
            
            async fn get_folder(&self, _folder_id: &str) -> Result<crate::domain::entities::folder::Folder, crate::application::services::storage_mediator::StorageMediatorError> {
                Err(crate::application::services::storage_mediator::StorageMediatorError::NotFound("Stub not implemented".to_string()))
            }
            
            async fn file_exists_at_path(&self, _path: &std::path::Path) -> Result<bool, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(false)
            }
            
            async fn file_exists_at_storage_path(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> Result<bool, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(false)
            }
            
            async fn folder_exists_at_path(&self, _path: &std::path::Path) -> Result<bool, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(false)
            }
            
            async fn folder_exists_at_storage_path(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> Result<bool, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(false)
            }
            
            fn resolve_path(&self, _relative_path: &std::path::Path) -> std::path::PathBuf {
                std::path::PathBuf::from("/tmp")
            }
            
            fn resolve_storage_path(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> std::path::PathBuf {
                std::path::PathBuf::from("/tmp")
            }
            
            async fn ensure_directory(&self, _path: &std::path::Path) -> Result<(), crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(())
            }
            
            async fn ensure_storage_directory(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> Result<(), crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(())
            }
            
            async fn get_file_owner(&self, _file_id: &str) -> Result<Option<String>, crate::application::services::storage_mediator::StorageMediatorError> {
                Ok(None)
            }
        }
        
        struct DummyFileReadPort;
        #[async_trait::async_trait]
        impl crate::application::ports::storage_ports::FileReadPort for DummyFileReadPort {
            async fn get_file(&self, _id: &str) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<crate::domain::entities::file::File>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_content(&self, _id: &str) -> Result<Vec<u8>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, crate::common::errors::DomainError> {
                let empty_stream = futures::stream::empty::<Result<bytes::Bytes, std::io::Error>>();
                Ok(Box::new(empty_stream))
            }
        }
        
        struct DummyFileWritePort;
        #[async_trait::async_trait]
        impl crate::application::ports::storage_ports::FileWritePort for DummyFileWritePort {
            async fn save_file(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _content: Vec<u8>,
                _user_id: Option<String>,
            ) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn move_file(&self, _file_id: &str, _target_folder_id: Option<String>) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
        }
        
        struct DummyFileStoragePort;
        #[async_trait::async_trait]
        impl crate::application::ports::outbound::FileStoragePort for DummyFileStoragePort {
            async fn save_file(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _content: Vec<u8>,
                _user_id: Option<String>,
            ) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn get_file(&self, _id: &str) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<crate::domain::entities::file::File>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn get_file_content(&self, _id: &str) -> Result<Vec<u8>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, crate::common::errors::DomainError> {
                let empty_stream = futures::stream::empty::<Result<bytes::Bytes, std::io::Error>>();
                Ok(Box::new(empty_stream))
            }
            
            async fn move_file(&self, _file_id: &str, _target_folder_id: Option<String>) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn get_file_path(&self, _id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::common::errors::DomainError> {
                Ok(crate::domain::services::path_service::StoragePath::from_string("/"))
            }
        }
        
        struct DummyFolderStoragePort;
        #[async_trait::async_trait]
        impl crate::application::ports::outbound::FolderStoragePort for DummyFolderStoragePort {
            async fn create_folder(&self, _name: String, _parent_id: Option<String>, _user_id: Option<String>) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::folder::Folder::default())
            }
            
            async fn get_folder(&self, _id: &str) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::folder::Folder::default())
            }
            
            async fn get_folder_by_path(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::folder::Folder::default())
            }
            
            async fn list_folders(&self, _parent_id: Option<&str>) -> Result<Vec<crate::domain::entities::folder::Folder>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn list_folders_paginated(
                &self,
                _parent_id: Option<&str>,
                _offset: usize,
                _limit: usize,
                _include_total: bool
            ) -> Result<(Vec<crate::domain::entities::folder::Folder>, Option<usize>), crate::common::errors::DomainError> {
                Ok((Vec::new(), Some(0)))
            }
            
            async fn rename_folder(&self, _id: &str, _new_name: String) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::folder::Folder::default())
            }
            
            async fn move_folder(&self, _id: &str, _new_parent_id: Option<&str>) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::folder::Folder::default())
            }
            
            async fn delete_folder(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn folder_exists(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> Result<bool, crate::common::errors::DomainError> {
                Ok(false)
            }
            
            async fn get_folder_path(&self, _id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::common::errors::DomainError> {
                Ok(crate::domain::services::path_service::StoragePath::from_string("/"))
            }
        }
        
        struct DummyFilePathResolutionPort;
        #[async_trait::async_trait]
        impl crate::application::ports::storage_ports::FilePathResolutionPort for DummyFilePathResolutionPort {
            async fn get_file_path(&self, _id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::common::errors::DomainError> {
                Ok(crate::domain::services::path_service::StoragePath::from_string("/"))
            }
            
            fn resolve_path(&self, _storage_path: &crate::domain::services::path_service::StoragePath) -> std::path::PathBuf {
                std::path::PathBuf::from("/")
            }
        }
        
        struct DummyI18nService;
        #[async_trait::async_trait]
        impl crate::domain::services::i18n_service::I18nService for DummyI18nService {
            async fn translate(&self, _key: &str, _locale: crate::domain::services::i18n_service::Locale) -> crate::domain::services::i18n_service::I18nResult<String> {
                Ok(String::new())
            }
            
            async fn load_translations(&self, _locale: crate::domain::services::i18n_service::Locale) -> crate::domain::services::i18n_service::I18nResult<()> {
                Ok(())
            }
            
            async fn available_locales(&self) -> Vec<crate::domain::services::i18n_service::Locale> {
                vec![crate::domain::services::i18n_service::Locale::default()]
            }
            
            async fn is_supported(&self, _locale: crate::domain::services::i18n_service::Locale) -> bool {
                true
            }
        }
        
        struct DummyFolderUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::inbound::FolderUseCase for DummyFolderUseCase {
            async fn create_folder(&self, _dto: crate::application::dtos::folder_dto::CreateFolderDto) -> Result<crate::application::dtos::folder_dto::FolderDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::folder_dto::FolderDto::default())
            }
            
            async fn get_folder(&self, _id: &str) -> Result<crate::application::dtos::folder_dto::FolderDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::folder_dto::FolderDto::default())
            }
            
            async fn get_folder_by_path(&self, _path: &str) -> Result<crate::application::dtos::folder_dto::FolderDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::folder_dto::FolderDto::default())
            }
            
            async fn list_folders(&self, _parent_id: Option<&str>) -> Result<Vec<crate::application::dtos::folder_dto::FolderDto>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn list_folders_paginated(
                &self,
                _parent_id: Option<&str>,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto
            ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<crate::application::dtos::folder_dto::FolderDto>, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::pagination::PaginatedResponseDto::new(
                    Vec::new(),
                    0,
                    10,
                    0
                ))
            }
            
            async fn rename_folder(&self, _id: &str, _dto: crate::application::dtos::folder_dto::RenameFolderDto) -> Result<crate::application::dtos::folder_dto::FolderDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::folder_dto::FolderDto::default())
            }
            
            async fn move_folder(&self, _id: &str, _dto: crate::application::dtos::folder_dto::MoveFolderDto) -> Result<crate::application::dtos::folder_dto::FolderDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::folder_dto::FolderDto::default())
            }
            
            async fn delete_folder(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
        }
        
        struct DummyFileUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::inbound::FileUseCase for DummyFileUseCase {
            async fn upload_file(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _content: Vec<u8>,
                _user_id: Option<String>,
            ) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn get_file(&self, _id: &str) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<crate::application::dtos::file_dto::FileDto>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn get_file_content(&self, _id: &str) -> Result<Vec<u8>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, crate::common::errors::DomainError> {
                // Create an empty stream
                let empty_stream = futures::stream::empty::<Result<bytes::Bytes, std::io::Error>>();
                Ok(Box::new(empty_stream))
            }
            
            async fn move_file(&self, _file_id: &str, _folder_id: Option<String>) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
        }
        
        struct DummyFileUploadUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::file_ports::FileUploadUseCase for DummyFileUploadUseCase {
            async fn upload_file(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _content: Vec<u8>,
                _user_id: Option<String>,
            ) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
        }
        
        struct DummyFileRetrievalUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::file_ports::FileRetrievalUseCase for DummyFileRetrievalUseCase {
            async fn get_file(&self, _id: &str) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<crate::application::dtos::file_dto::FileDto>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_content(&self, _id: &str) -> Result<Vec<u8>, crate::common::errors::DomainError> {
                Ok(Vec::new())
            }
            
            async fn get_file_stream(&self, _id: &str) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, crate::common::errors::DomainError> {
                // Create an empty stream
                let empty_stream = futures::stream::empty::<Result<bytes::Bytes, std::io::Error>>();
                Ok(Box::new(empty_stream))
            }
        }
        
        struct DummyFileManagementUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::file_ports::FileManagementUseCase for DummyFileManagementUseCase {
            async fn move_file(&self, _file_id: &str, _folder_id: Option<String>) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
        }
        
        struct DummyFileUseCaseFactory;
        impl crate::application::ports::file_ports::FileUseCaseFactory for DummyFileUseCaseFactory {
            fn create_file_upload_use_case(&self) -> std::sync::Arc<dyn crate::application::ports::file_ports::FileUploadUseCase> {
                std::sync::Arc::new(DummyFileUploadUseCase)
            }
            
            fn create_file_retrieval_use_case(&self) -> std::sync::Arc<dyn crate::application::ports::file_ports::FileRetrievalUseCase> {
                std::sync::Arc::new(DummyFileRetrievalUseCase)
            }
            
            fn create_file_management_use_case(&self) -> std::sync::Arc<dyn crate::application::ports::file_ports::FileManagementUseCase> {
                std::sync::Arc::new(DummyFileManagementUseCase)
            }
        }
        
        struct DummyI18nApplicationService {};
        
        // Need to implement the actual service to match the type signature in DI container
        impl DummyI18nApplicationService {
            fn dummy() -> crate::application::services::i18n_application_service::I18nApplicationService {
                // We need to create an actual I18nApplicationService
                crate::application::services::i18n_application_service::I18nApplicationService::new(
                    Arc::new(DummyI18nService) as Arc<dyn crate::domain::services::i18n_service::I18nService>
                )
            }
        }
        
        // Create service instances
        let id_mapping_service = Arc::new(DummyIdMappingService) as Arc<dyn crate::application::ports::outbound::IdMappingPort>;
        let storage_mediator = Arc::new(DummyStorageMediator) as Arc<dyn crate::application::services::storage_mediator::StorageMediator>;
        let i18n_repository = Arc::new(DummyI18nService) as Arc<dyn crate::domain::services::i18n_service::I18nService>;
        let folder_service = Arc::new(DummyFolderUseCase) as Arc<dyn crate::application::ports::inbound::FolderUseCase>;
        let file_service = Arc::new(DummyFileUseCase) as Arc<dyn crate::application::ports::inbound::FileUseCase>;
        let file_upload_service = Arc::new(DummyFileUploadUseCase) as Arc<dyn crate::application::ports::file_ports::FileUploadUseCase>;
        let file_retrieval_service = Arc::new(DummyFileRetrievalUseCase) as Arc<dyn crate::application::ports::file_ports::FileRetrievalUseCase>;
        let file_management_service = Arc::new(DummyFileManagementUseCase) as Arc<dyn crate::application::ports::file_ports::FileManagementUseCase>;
        let file_use_case_factory = Arc::new(DummyFileUseCaseFactory) as Arc<dyn crate::application::ports::file_ports::FileUseCaseFactory>;
        
        // This creates the core services needed for basic functionality
        let core_services = CoreServices {
            path_service: path_service.clone(),
            cache_manager: Arc::new(crate::infrastructure::services::cache_manager::StorageCacheManager::default()),
            id_mapping_service: id_mapping_service.clone(),
            config: config.clone(),
        };
        
        // Create empty repository implementations
        let repository_services = RepositoryServices {
            folder_repository: Arc::new(DummyFolderStoragePort) as Arc<dyn crate::application::ports::outbound::FolderStoragePort>,
            file_repository: Arc::new(DummyFileStoragePort) as Arc<dyn crate::application::ports::outbound::FileStoragePort>,
            file_read_repository: Arc::new(DummyFileReadPort) as Arc<dyn crate::application::ports::storage_ports::FileReadPort>,
            file_write_repository: Arc::new(DummyFileWritePort) as Arc<dyn crate::application::ports::storage_ports::FileWritePort>,
            i18n_repository,
            storage_mediator: storage_mediator.clone(),
            metadata_manager: Arc::new(crate::infrastructure::repositories::FileMetadataManager::default()),
            path_resolver: Arc::new(crate::infrastructure::repositories::file_path_resolver::FilePathResolver::new(
                path_service.clone(),
                storage_mediator.clone(),
                id_mapping_service.clone()
            )),
        };
        
        // Create application services
        let application_services = ApplicationServices {
            folder_service,
            file_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service: Arc::new(DummyI18nApplicationService::dummy()),
            sharing_service: None,
            public_link_service: None,
        };
        
        // Return a minimal app state
        Self {
            core: core_services,
            repositories: repository_services,
            applications: ApplicationServices {
                folder_service: application_services.folder_service,
                file_service: application_services.file_service,
                file_upload_service: application_services.file_upload_service,
                file_retrieval_service: application_services.file_retrieval_service,
                file_management_service: application_services.file_management_service,
                file_use_case_factory: application_services.file_use_case_factory,
                i18n_service: application_services.i18n_service,
                sharing_service: None,
                public_link_service: None,
            },
            db_pool: None,
            auth_service: None,
        }
    }
}

impl AppState {
    pub fn new(
        core: CoreServices,
        repositories: RepositoryServices,
        applications: ApplicationServices,
    ) -> Self {
        Self {
            core,
            repositories,
            applications,
            db_pool: None,
            auth_service: None,
        }
    }
    
    pub fn with_database(mut self, db_pool: Arc<PgPool>) -> Self {
        self.db_pool = Some(db_pool);
        self
    }
    
    pub fn with_auth_services(mut self, auth_services: AuthServices) -> Self {
        self.auth_service = Some(auth_services);
        self
    }
}