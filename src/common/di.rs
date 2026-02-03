use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use sqlx::PgPool;

use crate::application::services::auth_application_service::AuthApplicationService;

use crate::infrastructure::services::path_service::PathService;
use crate::infrastructure::repositories::folder_fs_repository::FolderFsRepository;
use crate::infrastructure::repositories::file_fs_repository::FileFsRepository;
use crate::infrastructure::repositories::trash_fs_repository::TrashFsRepository;
use crate::infrastructure::repositories::share_fs_repository::ShareFsRepository;
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use crate::infrastructure::services::file_system_i18n_service::FileSystemI18nService;
use crate::infrastructure::services::id_mapping_service::IdMappingService;
use crate::infrastructure::services::id_mapping_optimizer::IdMappingOptimizer;
use crate::infrastructure::services::cache_manager::StorageCacheManager;
use crate::infrastructure::services::file_metadata_cache::FileMetadataCache;
use crate::infrastructure::services::file_content_cache::{FileContentCache, FileContentCacheConfig, SharedFileContentCache};
use crate::infrastructure::services::buffer_pool::BufferPool;
use crate::infrastructure::services::trash_cleanup_service::TrashCleanupService;
use crate::application::services::folder_service::FolderService;
use crate::application::services::file_service::FileService;
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::application::services::trash_service::TrashService;
use crate::application::services::search_service::SearchService;
use crate::application::services::share_service::ShareService;
use crate::application::services::favorites_service::FavoritesService;
use crate::application::services::recent_service::RecentService;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::services::storage_mediator::{StorageMediator, FileSystemStorageMediator};
use crate::application::ports::inbound::{FileUseCase, FolderUseCase, SearchUseCase};
use crate::application::ports::outbound::{FileStoragePort, FolderStoragePort};
use crate::application::ports::favorites_ports::FavoritesUseCase;
use crate::application::ports::recent_ports::RecentItemsUseCase;
use crate::application::ports::file_ports::{FileUploadUseCase, FileRetrievalUseCase, FileManagementUseCase, FileUseCaseFactory};
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::infrastructure::repositories::{FileMetadataManager, FilePathResolver, FileFsReadRepository, FileFsWriteRepository};
use crate::application::services::{FileUploadService, FileRetrievalService, FileManagementService, AppFileUseCaseFactory};
use crate::common::errors::DomainError;
use crate::common::adapters::{DomainFileRepoAdapter, DomainFolderRepoAdapter};
use crate::domain::services::i18n_service::I18nService;
use crate::common::config::AppConfig;

/// Fábrica para los diferentes componentes de la aplicación
/// 
/// Esta fábrica centraliza la creación de todos los servicios de la aplicación,
/// garantizando el orden correcto de inicialización y resolviendo dependencias circulares.
pub struct AppServiceFactory {
    storage_path: PathBuf,
    locales_path: PathBuf,
    config: AppConfig,
}

impl AppServiceFactory {
    /// Crea una nueva fábrica de servicios
    pub fn new(storage_path: PathBuf, locales_path: PathBuf) -> Self {
        Self {
            storage_path,
            locales_path,
            config: AppConfig::default(),
        }
    }
    
    /// Crea una nueva fábrica de servicios con configuración personalizada
    pub fn with_config(storage_path: PathBuf, locales_path: PathBuf, config: AppConfig) -> Self {
        Self {
            storage_path,
            locales_path,
            config,
        }
    }
    
    /// Obtiene la configuración
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
    
    /// Obtiene la ruta de almacenamiento
    pub fn storage_path(&self) -> &PathBuf {
        &self.storage_path
    }
    
    /// Inicializa los servicios base del sistema
    pub async fn create_core_services(&self) -> Result<CoreServices, DomainError> {
        // Path service
        let path_service = Arc::new(PathService::new(self.storage_path.clone()));
        
        // Cache manager
        let file_ttl_ms = self.config.cache.file_ttl_ms;
        let dir_ttl_ms = self.config.cache.directory_ttl_ms;
        let max_entries = self.config.cache.max_entries;
        let cache_manager = Arc::new(StorageCacheManager::new(file_ttl_ms, dir_ttl_ms, max_entries));
        
        // Iniciar tarea de limpieza de caché en segundo plano
        let cache_manager_clone = cache_manager.clone();
        tokio::spawn(async move {
            StorageCacheManager::start_cleanup_task(cache_manager_clone).await;
        });
        
        // File content cache for ultra-fast file serving (hot files in RAM)
        let file_content_cache = Arc::new(FileContentCache::new(FileContentCacheConfig {
            max_file_size: 10 * 1024 * 1024,    // 10MB max per file
            max_total_size: 512 * 1024 * 1024,  // 512MB total cache
            max_entries: 10000,                  // Up to 10k files
        }));
        tracing::info!("FileContentCache initialized: max 10MB/file, 512MB total, 10k entries");
        
        // ID mapping service para carpetas
        let folder_id_mapping_path = self.storage_path.join("folder_ids.json");
        let folder_id_mapping_service = Arc::new(
            IdMappingService::new(folder_id_mapping_path).await?
        );
        
        // ID mapping service para archivos
        let file_id_mapping_path = self.storage_path.join("file_ids.json");
        let file_id_mapping_service = Arc::new(
            IdMappingService::new(file_id_mapping_path).await?
        );
        
        // Optimizer con batch processing y caching
        let id_mapping_optimizer = Arc::new(
            IdMappingOptimizer::new(folder_id_mapping_service.clone())
        );
        
        // Iniciar tarea de limpieza del optimizer
        IdMappingOptimizer::start_cleanup_task(id_mapping_optimizer.clone());
        
        // Thumbnail service para generación de miniaturas
        let thumbnail_service = Arc::new(
            crate::infrastructure::services::thumbnail_service::ThumbnailService::new(
                &self.storage_path,
                5000,  // max 5000 thumbnails en cache
                100 * 1024 * 1024,  // max 100MB de cache
            )
        );
        // Inicializar directorios de thumbnails
        thumbnail_service.initialize().await?;
        
        // Write-behind cache para uploads instantáneos de archivos pequeños
        let write_behind_cache = crate::infrastructure::services::write_behind_cache::WriteBehindCache::new();
        
        // Chunked upload service para archivos grandes (>10MB)
        let chunked_temp_dir = std::path::PathBuf::from(&self.storage_path).join(".uploads");
        let chunked_upload_service = Arc::new(
            crate::infrastructure::services::chunked_upload_service::ChunkedUploadService::new(chunked_temp_dir)
        );
        
        // Image transcoding service para conversión automática a WebP
        let image_transcode_service = Arc::new(
            crate::infrastructure::services::image_transcode_service::ImageTranscodeService::new(
                &self.storage_path,
                2000,  // max 2000 imágenes transcodificadas en cache
                50 * 1024 * 1024,  // max 50MB de cache en memoria
            )
        );
        image_transcode_service.initialize().await?;
        
        // Deduplication service para eliminar archivos duplicados
        let dedup_service = Arc::new(
            crate::infrastructure::services::dedup_service::DedupService::new(&self.storage_path)
        );
        dedup_service.initialize().await?;
        
        tracing::info!("Core services initialized: path service, cache manager, file content cache, ID mapping, thumbnails, write-behind cache, chunked upload, image transcode, dedup");
        
        Ok(CoreServices {
            path_service,
            cache_manager,
            file_content_cache,
            id_mapping_service: folder_id_mapping_service,
            file_id_mapping_service,
            id_mapping_optimizer,
            thumbnail_service,
            write_behind_cache,
            chunked_upload_service,
            image_transcode_service,
            dedup_service,
            config: self.config.clone(),
        })
    }
    
    /// Inicializa los servicios de repositorio
    pub fn create_repository_services(&self, core: &CoreServices) -> RepositoryServices {
        // Storage mediator - con inicialización diferida para folder repository
        let folder_repository_holder = Arc::new(RwLock::new(None));
        
        let storage_mediator = Arc::new(FileSystemStorageMediator::new_with_lazy_folder(
            folder_repository_holder.clone(),
            core.path_service.clone(),
            core.id_mapping_optimizer.clone()
        ));
        
        // Folder repository
        let folder_repository = Arc::new(FolderFsRepository::new(
            self.storage_path.clone(),
            storage_mediator.clone(),
            core.id_mapping_service.clone(),
            core.path_service.clone(),
        ));
        
        // Actualizar el holder para el mediador
        if let Ok(mut holder) = folder_repository_holder.write() {
            *holder = Some(folder_repository.clone());
        }
        
        // Metadata cache
        let metadata_cache = Arc::new(
            FileMetadataCache::default_with_config(core.config.clone())
        );
        
        // Iniciar tarea de limpieza de metadata cache
        let cache_clone = metadata_cache.clone();
        tokio::spawn(async move {
            FileMetadataCache::start_cleanup_task(cache_clone).await;
        });
        
        // Buffer pool para optimización de memoria
        let buffer_pool = BufferPool::new(256 * 1024, 50, 120); // 256KB buffers, 50 max, 2 min TTL
        BufferPool::start_cleaner(buffer_pool.clone());
        
        // Parallel file processor
        let parallel_processor = Arc::new(ParallelFileProcessor::new_with_buffer_pool(
            core.config.clone(),
            buffer_pool.clone()
        ));
        
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
            Some(parallel_processor.clone())
        ));
        
        let file_write_repository = Arc::new(FileFsWriteRepository::new(
            self.storage_path.clone(),
            metadata_manager.clone(),
            path_resolver.clone(),
            storage_mediator.clone(),
            core.config.clone(),
            Some(parallel_processor.clone())
        ));
        
        // File repository con procesamiento paralelo
        let file_repository = Arc::new(FileFsRepository::new_with_processor(
            self.storage_path.clone(), 
            storage_mediator.clone(),
            core.file_id_mapping_service.clone(),
            core.path_service.clone(),
            metadata_cache.clone(),
            parallel_processor
        ));
        
        // I18n repository
        let i18n_repository = Arc::new(FileSystemI18nService::new(
            self.locales_path.clone()
        ));
        
        // Trash repository
        let trash_repository = if core.config.features.enable_trash {
            Some(Arc::new(TrashFsRepository::new(
                self.storage_path.as_path(),
                core.id_mapping_service.clone(),
            )) as Arc<dyn crate::domain::repositories::trash_repository::TrashRepository>)
        } else {
            None
        };
        
        tracing::info!("Repository services initialized with parallel processing and buffer pool");
        
        RepositoryServices {
            folder_repository,
            file_repository,
            file_read_repository,
            file_write_repository,
            i18n_repository,
            storage_mediator,
            metadata_manager,
            path_resolver,
            metadata_cache,
            trash_repository,
        }
    }
    
    /// Inicializa los servicios de aplicación
    pub fn create_application_services(&self, repos: &RepositoryServices) -> ApplicationServices {
        // Servicios principales
        let folder_service = Arc::new(FolderService::new(
            repos.folder_repository.clone()
        ));
        
        let file_service = Arc::new(FileService::new(
            repos.file_repository.clone()
        ));
        
        // Servicios refactorizados
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
        
        // Search service con caché
        let search_service: Option<Arc<dyn SearchUseCase>> = Some(Arc::new(SearchService::new(
            repos.file_repository.clone(),
            repos.folder_repository.clone(),
            300, // Cache TTL in seconds (5 minutes)
            1000, // Maximum cache entries
        )));
        
        tracing::info!("Application services initialized");
        
        ApplicationServices {
            // Tipos concretos para handlers que los necesitan
            folder_service_concrete: folder_service.clone(),
            file_service_concrete: file_service.clone(),
            // Traits para abstracción
            folder_service,
            file_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service,
            trash_service: None, // Se configura después con create_trash_service
            search_service,
            share_service: None, // Se configura después con create_share_service
            favorites_service: None, // Se configura después con create_favorites_service
            recent_service: None, // Se configura después con create_recent_service
        }
    }
    
    /// Crea el servicio de papelera
    pub async fn create_trash_service(
        &self,
        repos: &RepositoryServices,
    ) -> Option<Arc<dyn TrashUseCase>> {
        if !self.config.features.enable_trash {
            tracing::info!("Trash service is disabled in configuration");
            return None;
        }
        
        let trash_repo = repos.trash_repository.as_ref()?;
        
        // Crear adaptadores
        let file_repo_adapter = Arc::new(DomainFileRepoAdapter::new(repos.file_repository.clone()));
        let folder_repo_adapter = Arc::new(DomainFolderRepoAdapter::new(repos.folder_repository.clone()));
        
        let service = Arc::new(TrashService::new(
            trash_repo.clone(),
            file_repo_adapter,
            folder_repo_adapter,
            self.config.storage.trash_retention_days,
        ));
        
        // Inicializar servicio de limpieza
        let cleanup_service = TrashCleanupService::new(
            service.clone(),
            trash_repo.clone(),
            24, // Run cleanup every 24 hours
        );
        
        cleanup_service.start_cleanup_job().await;
        tracing::info!("Trash service initialized with daily cleanup schedule");
        
        Some(service as Arc<dyn TrashUseCase>)
    }
    
    /// Crea el servicio de compartición
    pub fn create_share_service(
        &self,
        repos: &RepositoryServices,
    ) -> Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>> {
        if !self.config.features.enable_file_sharing {
            tracing::info!("File sharing service is disabled in configuration");
            return None;
        }
        
        let share_repository = Arc::new(ShareFsRepository::new(
            Arc::new(self.config.clone())
        ));
        
        let service = Arc::new(ShareService::new(
            Arc::new(self.config.clone()),
            share_repository,
            repos.file_repository.clone(),
            repos.folder_repository.clone()
        ));
        
        tracing::info!("File sharing service initialized");
        Some(service)
    }
    
    /// Crea el servicio de favoritos (requiere base de datos)
    pub fn create_favorites_service(
        &self,
        db_pool: &Arc<PgPool>,
    ) -> Arc<dyn FavoritesUseCase> {
        let service = Arc::new(FavoritesService::new(db_pool.clone()));
        tracing::info!("Favorites service initialized");
        service
    }
    
    /// Crea el servicio de elementos recientes (requiere base de datos)
    pub fn create_recent_service(
        &self,
        db_pool: &Arc<PgPool>,
    ) -> Arc<dyn RecentItemsUseCase> {
        let service = Arc::new(RecentService::new(
            db_pool.clone(),
            50 // Maximum recent items per user
        ));
        tracing::info!("Recent items service initialized");
        service
    }
    
    /// Precarga traducciones
    pub async fn preload_translations(&self, i18n_service: &I18nApplicationService) {
        use crate::domain::services::i18n_service::Locale;
        
        if let Err(e) = i18n_service.load_translations(Locale::English).await {
            tracing::warn!("Failed to load English translations: {}", e);
        }
        if let Err(e) = i18n_service.load_translations(Locale::Spanish).await {
            tracing::warn!("Failed to load Spanish translations: {}", e);
        }
        tracing::info!("Translations preloaded");
    }
    
    /// Precarga directorios en caché
    pub async fn preload_cache(&self, metadata_cache: &FileMetadataCache) {
        tracing::info!("Preloading common directories to warm up cache...");
        if let Ok(count) = metadata_cache.preload_directory(&self.storage_path, true, 1).await {
            tracing::info!("Preloaded {} directory entries into cache", count);
        }
    }
}

/// Contenedor para servicios base
#[derive(Clone)]
pub struct CoreServices {
    pub path_service: Arc<PathService>,
    pub cache_manager: Arc<StorageCacheManager>,
    pub file_content_cache: SharedFileContentCache,
    pub id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    pub file_id_mapping_service: Arc<IdMappingService>,
    pub id_mapping_optimizer: Arc<IdMappingOptimizer>,
    pub thumbnail_service: Arc<crate::infrastructure::services::thumbnail_service::ThumbnailService>,
    pub write_behind_cache: Arc<crate::infrastructure::services::write_behind_cache::WriteBehindCache>,
    pub chunked_upload_service: Arc<crate::infrastructure::services::chunked_upload_service::ChunkedUploadService>,
    pub image_transcode_service: Arc<crate::infrastructure::services::image_transcode_service::ImageTranscodeService>,
    pub dedup_service: Arc<crate::infrastructure::services::dedup_service::DedupService>,
    pub config: AppConfig,
}

/// Contenedor para servicios de repositorio
#[derive(Clone)]
pub struct RepositoryServices {
    pub folder_repository: Arc<dyn FolderStoragePort>,
    pub file_repository: Arc<dyn FileStoragePort>,
    pub file_read_repository: Arc<dyn FileReadPort>,
    pub file_write_repository: Arc<dyn FileWritePort>,
    pub i18n_repository: Arc<dyn I18nService>,
    pub storage_mediator: Arc<dyn StorageMediator>,
    pub metadata_manager: Arc<FileMetadataManager>,
    pub path_resolver: Arc<FilePathResolver>,
    pub metadata_cache: Arc<FileMetadataCache>,
    pub trash_repository: Option<Arc<dyn crate::domain::repositories::trash_repository::TrashRepository>>,
}

/// Contenedor para servicios de aplicación
#[derive(Clone)]
pub struct ApplicationServices {
    // Tipos concretos para compatibilidad con handlers existentes
    pub folder_service_concrete: Arc<FolderService>,
    pub file_service_concrete: Arc<FileService>,
    // Traits para abstracción
    pub folder_service: Arc<dyn FolderUseCase>,
    pub file_service: Arc<dyn FileUseCase>,
    pub file_upload_service: Arc<dyn FileUploadUseCase>,
    pub file_retrieval_service: Arc<dyn FileRetrievalUseCase>,
    pub file_management_service: Arc<dyn FileManagementUseCase>,
    pub file_use_case_factory: Arc<dyn FileUseCaseFactory>,
    pub i18n_service: Arc<I18nApplicationService>,
    pub trash_service: Option<Arc<dyn TrashUseCase>>,
    pub search_service: Option<Arc<dyn SearchUseCase>>,
    pub share_service: Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>>,
    pub favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    pub recent_service: Option<Arc<dyn RecentItemsUseCase>>,
}

/// Contenedor para servicios de autenticación
#[derive(Clone)]
pub struct AuthServices {
    pub token_service: Arc<dyn crate::application::ports::auth_ports::TokenServicePort>,
    pub auth_application_service: Arc<AuthApplicationService>,
}

/// Estado global de la aplicación para dependency injection
#[derive(Clone)]
pub struct AppState {
    pub core: CoreServices,
    pub repositories: RepositoryServices,
    pub applications: ApplicationServices,
    pub db_pool: Option<Arc<PgPool>>,
    pub auth_service: Option<AuthServices>,
    pub trash_service: Option<Arc<dyn TrashUseCase>>,
    pub share_service: Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>>,
    pub favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    pub recent_service: Option<Arc<dyn RecentItemsUseCase>>,
    pub storage_usage_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>>,
    pub calendar_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUseCase>>,
    pub contact_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUseCase>>,
}

impl Default for AppState {
    fn default() -> Self {
        // This is just a minimal stub version for auth middleware
        // We'll need to create proper instance in main.rs
        
        let config = crate::common::config::AppConfig::default();
        let path_service = Arc::new(
            crate::infrastructure::services::path_service::PathService::new(
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
            ) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn move_file(&self, _file_id: &str, _target_folder_id: Option<String>) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn delete_file(&self, _id: &str) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn get_folder_details(&self, _folder_id: &str) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn get_folder_path_str(&self, _folder_id: &str) -> Result<String, crate::common::errors::DomainError> {
                Ok("/Mi Carpeta - dummy".to_string())
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
            ) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn save_file_from_stream(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>>,
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
            
            async fn get_file_range_stream(
                &self, 
                _id: &str, 
                _start: u64, 
                _end: Option<u64>
            ) -> Result<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>, crate::common::errors::DomainError> {
                let empty_stream = futures::stream::empty::<Result<bytes::Bytes, std::io::Error>>();
                Ok(Box::new(empty_stream))
            }
            
            async fn get_file_mmap(&self, _id: &str) -> Result<bytes::Bytes, crate::common::errors::DomainError> {
                Ok(bytes::Bytes::new())
            }
            
            async fn move_file(&self, _file_id: &str, _target_folder_id: Option<String>) -> Result<crate::domain::entities::file::File, crate::common::errors::DomainError> {
                Ok(crate::domain::entities::file::File::default())
            }
            
            async fn get_file_path(&self, _id: &str) -> Result<crate::domain::services::path_service::StoragePath, crate::common::errors::DomainError> {
                Ok(crate::domain::services::path_service::StoragePath::from_string("/"))
            }
            
            async fn get_parent_folder_id(&self, _path: &str) -> Result<String, crate::common::errors::DomainError> {
                Ok("root".to_string())
            }
            
            async fn update_file_content(&self, _file_id: &str, _content: Vec<u8>) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
            
            async fn register_file_deferred(
                &self,
                _name: String,
                _folder_id: Option<String>,
                _content_type: String,
                _size: u64,
            ) -> Result<(crate::domain::entities::file::File, std::path::PathBuf), crate::common::errors::DomainError> {
                Ok((crate::domain::entities::file::File::default(), std::path::PathBuf::from("/tmp/dummy")))
            }
        }
        
        struct DummyFolderStoragePort;
        #[async_trait::async_trait]
        impl crate::application::ports::outbound::FolderStoragePort for DummyFolderStoragePort {
            async fn create_folder(&self, _name: String, _parent_id: Option<String>) -> Result<crate::domain::entities::folder::Folder, crate::common::errors::DomainError> {
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
        
        // File path resolution is handled by other components
        
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
            ) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn get_file(&self, _id: &str) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn get_file_by_path(&self, _path: &str) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn create_file(&self, _parent_path: &str, _filename: &str, _content: &[u8], _content_type: &str) -> Result<crate::application::dtos::file_dto::FileDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::file_dto::FileDto::default())
            }
            
            async fn update_file(&self, _path: &str, _content: &[u8]) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
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
        
        struct DummyI18nApplicationService {}
        
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
        
        // Create dummy ID mapping service for files
        let dummy_file_id_mapping = Arc::new(IdMappingService::dummy());
        let dummy_id_optimizer = Arc::new(IdMappingOptimizer::new(dummy_file_id_mapping.clone()));
        
        // Create file content cache for stub
        let file_content_cache = Arc::new(FileContentCache::new(FileContentCacheConfig::default()));
        
        // Create dummy thumbnail service
        let dummy_thumbnail_service = Arc::new(
            crate::infrastructure::services::thumbnail_service::ThumbnailService::new(
                &std::path::PathBuf::from("./storage"),
                100,
                10 * 1024 * 1024,
            )
        );
        
        // Create dummy write-behind cache
        let dummy_write_behind_cache = crate::infrastructure::services::write_behind_cache::WriteBehindCache::new();
        
        // Create dummy chunked upload service
        let dummy_chunked_upload_service = Arc::new(
            crate::infrastructure::services::chunked_upload_service::ChunkedUploadService::new(
                std::path::PathBuf::from("./storage/.uploads")
            )
        );
        
        // Create dummy image transcode service
        let dummy_image_transcode_service = Arc::new(
            crate::infrastructure::services::image_transcode_service::ImageTranscodeService::new(
                &std::path::PathBuf::from("./storage"),
                100,
                10 * 1024 * 1024,
            )
        );
        
        // Create dummy dedup service
        let dummy_dedup_service = Arc::new(
            crate::infrastructure::services::dedup_service::DedupService::new(
                &std::path::PathBuf::from("./storage")
            )
        );
        
        // This creates the core services needed for basic functionality
        let core_services = CoreServices {
            path_service: path_service.clone(),
            cache_manager: Arc::new(crate::infrastructure::services::cache_manager::StorageCacheManager::default()),
            file_content_cache,
            id_mapping_service: id_mapping_service.clone(),
            file_id_mapping_service: dummy_file_id_mapping,
            id_mapping_optimizer: dummy_id_optimizer,
            thumbnail_service: dummy_thumbnail_service,
            write_behind_cache: dummy_write_behind_cache,
            chunked_upload_service: dummy_chunked_upload_service,
            image_transcode_service: dummy_image_transcode_service,
            dedup_service: dummy_dedup_service,
            config: config.clone(),
        };
        
        // Create dummy metadata cache
        let dummy_metadata_cache = Arc::new(FileMetadataCache::default_with_config(config.clone()));
        
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
            metadata_cache: dummy_metadata_cache,
            trash_repository: None, // No trash repository in minimal mode
        };
        
        // Create dummy search use case
        struct DummySearchUseCase;
        #[async_trait::async_trait]
        impl crate::application::ports::inbound::SearchUseCase for DummySearchUseCase {
            async fn search(
                &self, 
                _criteria: crate::application::dtos::search_dto::SearchCriteriaDto
            ) -> Result<crate::application::dtos::search_dto::SearchResultsDto, crate::common::errors::DomainError> {
                Ok(crate::application::dtos::search_dto::SearchResultsDto::empty())
            }
            
            async fn clear_search_cache(&self) -> Result<(), crate::common::errors::DomainError> {
                Ok(())
            }
        }
        
        // Create dummy concrete services for compatibility
        let dummy_folder_storage = Arc::new(DummyFolderStoragePort) as Arc<dyn crate::application::ports::outbound::FolderStoragePort>;
        let dummy_file_storage = Arc::new(DummyFileStoragePort) as Arc<dyn crate::application::ports::outbound::FileStoragePort>;
        let folder_service_concrete = Arc::new(FolderService::new(dummy_folder_storage));
        let file_service_concrete = Arc::new(FileService::new(dummy_file_storage));

        // Create application services
        let application_services = ApplicationServices {
            folder_service_concrete: folder_service_concrete.clone(),
            file_service_concrete: file_service_concrete.clone(),
            folder_service,
            file_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service: Arc::new(DummyI18nApplicationService::dummy()),
            trash_service: None, // No trash service in minimal mode
            search_service: Some(Arc::new(DummySearchUseCase) as Arc<dyn crate::application::ports::inbound::SearchUseCase>),
            share_service: None, // No share service in minimal mode
            favorites_service: None, // No favorites service in minimal mode
            recent_service: None, // No recent service in minimal mode
        };
        
        // Return a minimal app state
        Self {
            core: core_services,
            repositories: repository_services,
            applications: application_services,
            db_pool: None,
            auth_service: None,
            trash_service: None,
            share_service: None,
            favorites_service: None,
            recent_service: None,
            storage_usage_service: None,
            calendar_service: None,
            contact_service: None,
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
            trash_service: None,
            share_service: None,
            favorites_service: None,
            recent_service: None,
            storage_usage_service: None,
            calendar_service: None,
            contact_service: None,
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
    
    pub fn with_trash_service(mut self, trash_service: Arc<dyn TrashUseCase>) -> Self {
        self.trash_service = Some(trash_service);
        self
    }
    
    pub fn with_share_service(mut self, share_service: Arc<dyn crate::application::ports::share_ports::ShareUseCase>) -> Self {
        self.share_service = Some(share_service);
        self
    }
    
    pub fn with_favorites_service(mut self, favorites_service: Arc<dyn FavoritesUseCase>) -> Self {
        self.favorites_service = Some(favorites_service);
        self
    }
    
    pub fn with_recent_service(mut self, recent_service: Arc<dyn RecentItemsUseCase>) -> Self {
        self.recent_service = Some(recent_service);
        self
    }
    
    pub fn with_storage_usage_service(mut self, storage_usage_service: Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }
    
    pub fn with_calendar_service(mut self, calendar_service: Arc<dyn crate::application::ports::storage_ports::StorageUseCase>) -> Self {
        self.calendar_service = Some(calendar_service);
        self
    }
    
    pub fn with_contact_service(mut self, contact_service: Arc<dyn crate::application::ports::storage_ports::StorageUseCase>) -> Self {
        self.contact_service = Some(contact_service);
        self
    }
}