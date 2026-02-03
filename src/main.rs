use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// OxiCloud - Cloud Storage Platform
///
/// OxiCloud is a NextCloud-like file storage system built in Rust with a focus on 
/// performance, security, and clean architecture. The system provides:
///
/// - File and folder management with rich metadata
/// - User authentication and authorization
/// - File trash system with automatic cleanup
/// - Efficient handling of large files through parallel processing
/// - Compression capabilities for bandwidth optimization
/// - RESTful API and web interface
///
/// The architecture follows the Clean/Hexagonal Architecture pattern with:
///
/// - Domain Layer: Core business entities and repository interfaces (domain/*)
/// - Application Layer: Use cases and service orchestration (application/*)
/// - Infrastructure Layer: Technical implementations of repositories (infrastructure/*)
/// - Interface Layer: API endpoints and web controllers (interfaces/*)
///
/// Dependencies are managed through dependency inversion, with high-level modules
/// defining interfaces (ports) that low-level modules implement (adapters).
///
/// @author OxiCloud Development Team

// Use the library crate instead of redefining modules
use oxicloud::application;
use oxicloud::common;
use oxicloud::domain;
use oxicloud::infrastructure;
use oxicloud::interfaces;

use application::services::folder_service::FolderService;
use application::services::file_service::FileService;
use application::services::i18n_application_service::I18nApplicationService;
use application::services::storage_mediator::FileSystemStorageMediator;
use application::services::share_service::ShareService;
use application::services::favorites_service::FavoritesService;
use infrastructure::services::path_service::PathService;
use infrastructure::repositories::folder_fs_repository::FolderFsRepository;
use infrastructure::repositories::file_fs_repository::FileFsRepository;
use infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use infrastructure::repositories::share_fs_repository::ShareFsRepository;
use infrastructure::services::file_system_i18n_service::FileSystemI18nService;
use infrastructure::services::id_mapping_service::IdMappingService;
use infrastructure::services::id_mapping_optimizer::IdMappingOptimizer;
use infrastructure::services::file_metadata_cache::FileMetadataCache;
use infrastructure::services::buffer_pool::BufferPool;
use infrastructure::services::compression_service::GzipCompressionService;
use interfaces::{create_api_routes, web::create_web_routes};
use application::services::trash_service::TrashService;
use infrastructure::repositories::trash_fs_repository::TrashFsRepository;
use infrastructure::services::trash_cleanup_service::TrashCleanupService;
use common::db::create_database_pool;
use common::auth_factory::create_auth_services;
use common::di::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let config = common::config::AppConfig::from_env();
    
    // Set up storage directory
    let storage_path = config.storage_path.clone();
    if !storage_path.exists() {
        std::fs::create_dir_all(&storage_path).expect("Failed to create storage directory");
    }

    // Set up locales directory
    let locales_path = PathBuf::from("./static/locales");
    if !locales_path.exists() {
        std::fs::create_dir_all(&locales_path).expect("Failed to create locales directory");
    }
    
    // Initialize database if auth is enabled
    let db_pool = if config.features.enable_auth {
        match create_database_pool(&config).await {
            Ok(pool) => {
                tracing::info!("PostgreSQL database pool initialized successfully");
                Some(Arc::new(pool))
            },
            Err(e) => {
                tracing::error!("Failed to initialize database pool: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Create a reference to db_pool for use throughout the code
    let db_pool_ref = db_pool.as_ref();

    // Initialize path service
    let path_service = Arc::new(PathService::new(storage_path.clone()));
    
    // Initialize ID mapping service for folders
    let folder_id_mapping_path = storage_path.join("folder_ids.json");
    let folder_id_mapping_service = Arc::new(
        IdMappingService::new(folder_id_mapping_path).await
            .expect("Failed to initialize folder ID mapping service")
    );
    
    // Initialize ID mapping service for files
    let file_id_mapping_path = storage_path.join("file_ids.json");
    let file_id_mapping_service = Arc::new(
        IdMappingService::new(file_id_mapping_path).await
            .expect("Failed to initialize file ID mapping service")
    );
    
    // For backward compatibility, use folder ID service as the base ID mapping service
    let base_id_mapping_service = folder_id_mapping_service.clone();
    
    // Create optimized ID mapping service with batch processing and caching
    let id_mapping_optimizer = Arc::new(
        IdMappingOptimizer::new(base_id_mapping_service.clone())
    );
    
    // Initialize folder repository with all required components
    let folder_repository = Arc::new(FolderFsRepository::new(
        storage_path.clone(),
        Arc::new(FileSystemStorageMediator::new_stub()), // Temporary stub (will be replaced)
        base_id_mapping_service.clone(),
        path_service.clone()
    ));
    
    // Initialize storage mediator
    let storage_mediator = Arc::new(FileSystemStorageMediator::new(
        folder_repository.clone(),
        path_service.clone(),
        id_mapping_optimizer.clone()
    ));
    
    // Update folder repository with proper storage mediator
    // This replaces the stub we initialized it with
    let folder_repository = Arc::new(FolderFsRepository::new(
        storage_path.clone(),
        storage_mediator.clone(),
        base_id_mapping_service.clone(),
        path_service.clone()
    ));
    
    // Start cleanup task for ID mapping optimizer
    IdMappingOptimizer::start_cleanup_task(id_mapping_optimizer.clone());
    
    tracing::info!("ID mapping optimizer initialized with batch processing and caching");
    
    // Initialize the metadata cache 
    let config = common::config::AppConfig::default();
    let metadata_cache = Arc::new(FileMetadataCache::default_with_config(config.clone()));
    
    // Start the periodic cleanup task for cache maintenance
    let cache_clone = metadata_cache.clone();
    tokio::spawn(async move {
        FileMetadataCache::start_cleanup_task(cache_clone).await;
    });
    
    // Initialize the buffer pool for memory optimization
    // Use larger buffer size for better performance with large files
    let buffer_pool = BufferPool::new(256 * 1024, 50, 120); // 256KB buffers, 50 max, 2 min TTL
    
    // Start the buffer pool cleanup task
    BufferPool::start_cleaner(buffer_pool.clone());
    
    tracing::info!("Buffer pool initialized with 50 buffers of 256KB each");
    
    // Initialize parallel file processor with buffer pool
    let parallel_processor = Arc::new(ParallelFileProcessor::new_with_buffer_pool(
        config.clone(),
        buffer_pool.clone()
    ));
    
    // Initialize compression service with buffer pool
    let _compression_service = Arc::new(GzipCompressionService::new_with_buffer_pool(
        buffer_pool.clone()
    ));
    
    // Initialize file repository with mediator, ID mapping service, metadata cache, and parallel processor
    let file_repository = Arc::new(FileFsRepository::new_with_processor(
        storage_path.clone(), 
        storage_mediator,
        file_id_mapping_service.clone(), // Use the file-specific ID mapping service
        path_service.clone(),
        metadata_cache.clone(), // Clone to keep a reference for later use
        parallel_processor
    ));

    // Initialize application services
    let folder_service = Arc::new(FolderService::new(folder_repository.clone()));
    let file_service = Arc::new(FileService::new(file_repository.clone()));
    
    // Initialize trash service if enabled
    let trash_repository = if config.features.enable_trash {
        Some(Arc::new(TrashFsRepository::new(
            storage_path.as_path(),
            base_id_mapping_service.clone(),
        )))
    } else {
        None
    };
    
    // Create adapters for repositories (using domain interfaces instead of ports)
    struct DomainFileRepoAdapter {
        repo: Arc<dyn application::ports::outbound::FileStoragePort>
    }
    
    impl DomainFileRepoAdapter {
        fn new(repo: Arc<dyn application::ports::outbound::FileStoragePort>) -> Self {
            Self { repo }
        }
    }
    
    #[async_trait::async_trait]
    impl domain::repositories::file_repository::FileRepository for DomainFileRepoAdapter {
        async fn save_file_from_bytes(
            &self,
            name: String,
            folder_id: Option<String>,
            content_type: String,
            content: Vec<u8>,
        ) -> domain::repositories::file_repository::FileRepositoryResult<domain::entities::file::File> {
            self.repo.save_file(name, folder_id, content_type, content)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn save_file_from_stream(
            &self,
            name: String,
            folder_id: Option<String>,
            content_type: String,
            stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>>,
        ) -> domain::repositories::file_repository::FileRepositoryResult<domain::entities::file::File> {
            self.repo.save_file_from_stream(name, folder_id, content_type, stream)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn save_file_with_id(
            &self,
            _id: String,
            _name: String,
            _folder_id: Option<String>,
            _content_type: String,
            _content: Vec<u8>,
        ) -> domain::repositories::file_repository::FileRepositoryResult<domain::entities::file::File> {
            Err(domain::repositories::file_repository::FileRepositoryError::Other("Not implemented".to_string()))
        }
        
        async fn get_file_by_id(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<domain::entities::file::File> {
            self.repo.get_file(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn list_files(&self, folder_id: Option<&str>) -> domain::repositories::file_repository::FileRepositoryResult<Vec<domain::entities::file::File>> {
            self.repo.list_files(folder_id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn delete_file(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            self.repo.delete_file(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn delete_file_entry(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            self.delete_file(id).await
        }
        
        async fn get_file_content(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<Vec<u8>> {
            self.repo.get_file_content(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_file_stream(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>> {
            self.repo.get_file_stream(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_file_range_stream(
            &self,
            id: &str,
            start: u64,
            end: Option<u64>,
        ) -> domain::repositories::file_repository::FileRepositoryResult<Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>> {
            self.repo.get_file_range_stream(id, start, end)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_file_mmap(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<bytes::Bytes> {
            self.repo.get_file_mmap(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> domain::repositories::file_repository::FileRepositoryResult<domain::entities::file::File> {
            self.repo.move_file(id, target_folder_id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_file_path(&self, id: &str) -> domain::repositories::file_repository::FileRepositoryResult<domain::services::path_service::StoragePath> {
            self.repo.get_file_path(id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn move_to_trash(&self, file_id: &str) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            // Since we're using TrashService to handle trashing, this method is not directly used
            // but we'll implement it by delegating to the repository's delete_file method
            self.repo.delete_file(file_id)
                .await
                .map_err(|e| domain::repositories::file_repository::FileRepositoryError::Other(format!("{}", e)))
        }
        
        async fn restore_from_trash(&self, file_id: &str, original_path: &str) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            
            
            tracing::info!("Restoring file from trash: {} to {}", file_id, original_path);
            
            // We need to get the file from trash first to ensure it exists
            match self.repo.get_file(file_id).await {
                Ok(_) => {
                    // Extract the parent folder ID from the original path if available
                    let path_components: Vec<&str> = original_path.split('/').collect();
                    let parent_folder: Option<String> = if path_components.len() > 1 {
                        // Try to extract folder ID from path, but this is just a simplified approach
                        // In a real implementation, we would need to find or create the folder
                        tracing::info!("Attempting to restore to parent folder from path: {}", original_path);
                        None // No folder ID for now, will go to root
                    } else {
                        None // No parent folder, go to root
                    };
                    
                    // Use move_file to attempt to restore the file to its original location or root
                    match self.repo.move_file(file_id, parent_folder).await {
                        Ok(_) => {
                            tracing::info!("Successfully restored file from trash: {}", file_id);
                            Ok(())
                        },
                        Err(e) => {
                            tracing::error!("Failed to restore file from trash: {}", e);
                            Err(domain::repositories::file_repository::FileRepositoryError::Other(format!("Failed to restore file: {}", e)))
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("File not found in trash: {}", e);
                    Err(domain::repositories::file_repository::FileRepositoryError::NotFound(file_id.to_string()))
                }
            }
        }
        
        async fn delete_file_permanently(&self, file_id: &str) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            tracing::info!("Permanently deleting file: {}", file_id);
            
            // Directly attempt to delete the file using the file service
            match self.repo.delete_file(file_id).await {
                Ok(_) => {
                    tracing::info!("Successfully deleted file permanently: {}", file_id);
                    Ok(())
                },
                Err(e) => {
                    tracing::error!("Failed to permanently delete file: {}", e);
                    Err(domain::repositories::file_repository::FileRepositoryError::Other(format!("Failed to delete file permanently: {}", e)))
                }
            }
        }
        
        async fn update_file_content(&self, file_id: &str, content: Vec<u8>) -> domain::repositories::file_repository::FileRepositoryResult<()> {
            tracing::info!("Updating content for file: {}", file_id);
            
            self.repo.update_file_content(file_id, content)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update file content: {}", e);
                    domain::repositories::file_repository::FileRepositoryError::Other(format!("Failed to update file content: {}", e))
                })
        }
    }
    
    struct DomainFolderRepoAdapter {
        repo: Arc<dyn application::ports::outbound::FolderStoragePort>
    }
    
    impl DomainFolderRepoAdapter {
        fn new(repo: Arc<dyn application::ports::outbound::FolderStoragePort>) -> Self {
            Self { repo }
        }
    }
    
    #[async_trait::async_trait]
    impl domain::repositories::folder_repository::FolderRepository for DomainFolderRepoAdapter {
        async fn create_folder(&self, name: String, parent_id: Option<String>) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            self.repo.create_folder(name, parent_id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_folder_by_id(&self, id: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            self.repo.get_folder(id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_folder_by_storage_path(&self, storage_path: &domain::services::path_service::StoragePath) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            self.repo.get_folder_by_path(storage_path)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn list_folders(&self, parent_id: Option<&str>) -> domain::repositories::folder_repository::FolderRepositoryResult<Vec<domain::entities::folder::Folder>> {
            self.repo.list_folders(parent_id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn list_folders_paginated(
            &self, 
            parent_id: Option<&str>, 
            offset: usize, 
            limit: usize,
            include_total: bool
        ) -> domain::repositories::folder_repository::FolderRepositoryResult<(Vec<domain::entities::folder::Folder>, Option<usize>)> {
            self.repo.list_folders_paginated(parent_id, offset, limit, include_total)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn rename_folder(&self, id: &str, new_name: String) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            self.repo.rename_folder(id, new_name)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn move_folder(&self, id: &str, new_parent_id: Option<&str>) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            self.repo.move_folder(id, new_parent_id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn delete_folder(&self, id: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<()> {
            self.repo.delete_folder(id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn folder_exists_at_storage_path(&self, storage_path: &domain::services::path_service::StoragePath) -> domain::repositories::folder_repository::FolderRepositoryResult<bool> {
            self.repo.folder_exists(storage_path)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn get_folder_storage_path(&self, id: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::services::path_service::StoragePath> {
            self.repo.get_folder_path(id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn folder_exists(&self, _path: &std::path::PathBuf) -> domain::repositories::folder_repository::FolderRepositoryResult<bool> {
            Err(domain::repositories::folder_repository::FolderRepositoryError::Other("Not implemented".to_string()))
        }
        
        async fn get_folder_by_path(&self, _path: &std::path::PathBuf) -> domain::repositories::folder_repository::FolderRepositoryResult<domain::entities::folder::Folder> {
            Err(domain::repositories::folder_repository::FolderRepositoryError::Other("Not implemented".to_string()))
        }
        
        async fn move_to_trash(&self, folder_id: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<()> {
            // Since we're using TrashService to handle trashing, this method is not directly used
            // but we'll still use delete_folder since the underlying repository has proper trash support
            self.repo.delete_folder(folder_id)
                .await
                .map_err(|e| domain::repositories::folder_repository::FolderRepositoryError::Other(format!("{}", e)))
        }
        
        async fn restore_from_trash(&self, _folder_id: &str, original_path: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<()> {
            // Convert the original_path to a StoragePath for the repository
            use crate::domain::services::path_service::StoragePath;
            let storage_path = StoragePath::from_string(original_path);
            let _ = storage_path; // Prevent unused variable warning
            
            // The underlying repo doesn't have a direct API for this, but the implementation exists
            // in the folder repository through TrashService
            // This should be coordinated through TrashService instead
            Err(domain::repositories::folder_repository::FolderRepositoryError::Other(
                "Restore from trash should be handled by TrashService, not through this adapter".to_string()))
        }
        
        async fn delete_folder_permanently(&self, folder_id: &str) -> domain::repositories::folder_repository::FolderRepositoryResult<()> {
            // The repository now has proper implementation for permanent deletion
            // But we still use delete_folder since that's the method available on FolderStoragePort
            self.delete_folder(folder_id).await
        }
    }
    
    // Create repository adapters
    let file_repo_adapter = Arc::new(DomainFileRepoAdapter::new(file_repository.clone()));
    let folder_repo_adapter = Arc::new(DomainFolderRepoAdapter::new(folder_repository.clone()));
    
    // Create the trash service with properly typed adapters
    let trash_service = if let Some(ref trash_repo) = trash_repository {
        let service = Arc::new(TrashService::new(
            trash_repo.clone(),
            file_repo_adapter,
            folder_repo_adapter,
            config.storage.trash_retention_days,
        ));
        
        // Initialize trash cleanup service
        let cleanup_service = TrashCleanupService::new(
            service.clone(),
            trash_repo.clone(),
            24, // Run cleanup every 24 hours
        );
        
        // Start cleanup job if trash is enabled
        if config.features.enable_trash {
            cleanup_service.start_cleanup_job().await;
            tracing::info!("Trash cleanup service started with daily schedule");
        }
        
        Some(service as Arc<dyn application::ports::trash_ports::TrashUseCase>)
    } else {
        None
    };
    
    // Initialize i18n service
    let i18n_repository = Arc::new(FileSystemI18nService::new(locales_path.clone()));
    let i18n_service = Arc::new(I18nApplicationService::new(i18n_repository.clone()));
    
    // Preload translations
    if let Err(e) = i18n_service.load_translations(domain::services::i18n_service::Locale::English).await {
        tracing::warn!("Failed to load English translations: {}", e);
    }
    if let Err(e) = i18n_service.load_translations(domain::services::i18n_service::Locale::Spanish).await {
        tracing::warn!("Failed to load Spanish translations: {}", e);
    }
    
    tracing::info!("Compression service initialized with buffer pool support");
    
    // Initialize auth services if enabled and database connection is available
    let auth_services = if config.features.enable_auth && db_pool_ref.is_some() {
        match create_auth_services(
            &config, 
            db_pool_ref.unwrap().clone(),
            Some(folder_service.clone())  // Pasar el servicio de carpetas para creación automática de carpetas de usuario
        ).await {
            Ok(services) => {
                tracing::info!("Authentication services initialized successfully with folder service");
                Some(services)
            },
            Err(e) => {
                tracing::error!("Failed to initialize authentication services: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Create thumbnail service
    let thumbnail_service = Arc::new(
        infrastructure::services::thumbnail_service::ThumbnailService::new(
            &storage_path,
            5000,  // max 5000 thumbnails en cache
            100 * 1024 * 1024,  // max 100MB de cache
        )
    );
    // Note: thumbnail directories will be initialized when first used
    
    // Create write-behind cache for zero-latency small file uploads
    let write_behind_cache = infrastructure::services::write_behind_cache::WriteBehindCache::new();
    
    // Create chunked upload service for large file uploads (>10MB)
    let chunked_upload_service = Arc::new(
        infrastructure::services::chunked_upload_service::ChunkedUploadService::new(
            storage_path.join(".uploads")
        )
    );
    
    // Create image transcode service for automatic WebP conversion
    let image_transcode_service = Arc::new(
        infrastructure::services::image_transcode_service::ImageTranscodeService::new(
            &storage_path,
            2000,  // max 2000 transcoded images in cache
            50 * 1024 * 1024,  // max 50MB memory cache
        )
    );
    // Initialize transcode service directories
    if let Err(e) = image_transcode_service.initialize().await {
        tracing::warn!("Failed to initialize image transcode service: {}", e);
    }
    
    // Create deduplication service for content-addressable storage
    let dedup_service = Arc::new(
        infrastructure::services::dedup_service::DedupService::new(&storage_path)
    );
    if let Err(e) = dedup_service.initialize().await {
        tracing::warn!("Failed to initialize dedup service: {}", e);
    }
    
    // Create AppState for DI container
    let core_services = common::di::CoreServices {
        path_service: path_service.clone(),
        cache_manager: Arc::new(infrastructure::services::cache_manager::StorageCacheManager::default()),
        file_content_cache: Arc::new(infrastructure::services::file_content_cache::FileContentCache::default()),
        id_mapping_service: base_id_mapping_service.clone(),
        file_id_mapping_service: file_id_mapping_service.clone(),
        id_mapping_optimizer: id_mapping_optimizer.clone(),
        thumbnail_service: thumbnail_service.clone(),
        write_behind_cache: write_behind_cache.clone(),
        chunked_upload_service: chunked_upload_service.clone(),
        image_transcode_service: image_transcode_service.clone(),
        dedup_service: dedup_service.clone(),
        config: config.clone(),
    };
    
    // Crear stubs para los repositorios
    let file_read_stub = Arc::new(infrastructure::repositories::FileFsReadRepository::default_stub());
    let file_write_stub = Arc::new(infrastructure::repositories::FileFsWriteRepository::default_stub());
    let storage_mediator_stub = Arc::new(application::services::storage_mediator::FileSystemStorageMediator::new_stub());
    let metadata_manager = Arc::new(infrastructure::repositories::FileMetadataManager::default());
    let path_resolver_stub = Arc::new(infrastructure::repositories::FilePathResolver::default_stub());
    
    let repository_services = common::di::RepositoryServices {
        folder_repository: Arc::new(FolderFsRepository::new(
            storage_path.clone(),
            storage_mediator_stub.clone(),
            folder_id_mapping_service.clone(),
            path_service.clone()
        )),
        file_repository: Arc::new(FileFsRepository::new(
            storage_path.clone(), 
            storage_mediator_stub.clone(),
            file_id_mapping_service.clone(),
            path_service.clone(),
            metadata_cache.clone(),
        )),
        file_read_repository: file_read_stub,
        file_write_repository: file_write_stub,
        i18n_repository: i18n_repository.clone(),
        storage_mediator: storage_mediator_stub,
        metadata_manager,
        path_resolver: path_resolver_stub,
        metadata_cache: metadata_cache.clone(),
        trash_repository: trash_repository.clone().map(|repo| {
            // Convert Arc<TrashFsRepository> to Arc<dyn TrashRepository>
            let repo: Arc<dyn crate::domain::repositories::trash_repository::TrashRepository> = repo;
            repo
        }),
    };
    
    // Create the search service
    let search_service: Option<Arc<dyn application::ports::inbound::SearchUseCase>> = {
        // Create the search service with caching
        let search_service = Arc::new(application::services::search_service::SearchService::new(
            file_repository.clone(),
            folder_repository.clone(),
            300, // Cache TTL in seconds (5 minutes)
            1000, // Maximum cache entries
        ));
        
        tracing::info!("Search service initialized with caching (TTL: 300s, max entries: 1000)");
        Some(search_service)
    };
    
    // Initialize share repository and service if enabled
    let share_service: Option<Arc<dyn application::ports::share_ports::ShareUseCase>> = if config.features.enable_file_sharing {
        let share_repository = Arc::new(ShareFsRepository::new(
            Arc::new(config.clone())
        ));
        
        let share_service = Arc::new(ShareService::new(
            Arc::new(config.clone()),
            share_repository,
            file_repository.clone(),
            folder_repository.clone()
        ));
        
        tracing::info!("File sharing service initialized successfully");
        Some(share_service)
    } else {
        tracing::info!("File sharing service is disabled in configuration");
        None
    };

    // Initialize favorites service if database is available
    let favorites_service: Option<Arc<dyn application::ports::favorites_ports::FavoritesUseCase>> = 
    if let Some(pool) = db_pool_ref {
        // Create a new favorites service with the database pool
        let favorites_service = Arc::new(FavoritesService::new(
            pool.clone()
        ));
        
        tracing::info!("Favorites service initialized successfully");
        Some(favorites_service)
    } else {
        tracing::info!("Favorites service is disabled (requires database connection)");
        None
    };
    
    // Initialize recent items service if database is available
    let recent_service: Option<Arc<dyn application::ports::recent_ports::RecentItemsUseCase>> = 
    if let Some(pool) = db_pool_ref {
        // Create a new service with the database pool
        let service = Arc::new(application::services::recent_service::RecentService::new(
            pool.clone(),
            50 // Maximum recent items per user
        ));
        
        tracing::info!("Recent items service initialized successfully");
        Some(service)
    } else {
        tracing::info!("Recent items service is disabled (requires database connection)");
        None
    };
    
    // For now, we'll use a placeholder for the contact service
    // Instead of using the real PostgreSQL repositories, we'll create a dummy implementation
    // This makes the code compile, and we can replace it with the real implementation later
    let contact_service: Option<Arc<dyn application::ports::storage_ports::StorageUseCase>> = None;

    let application_services = common::di::ApplicationServices {
        // Concrete types for backward compatibility with handlers
        folder_service_concrete: folder_service.clone(),
        file_service_concrete: file_service.clone(),
        // Trait objects for abstraction
        folder_service: folder_service.clone(),
        file_service: file_service.clone(),
        file_upload_service: Arc::new(application::services::file_upload_service::FileUploadService::new(
            Arc::new(infrastructure::repositories::FileFsWriteRepository::default_stub())
        )),
        file_retrieval_service: Arc::new(application::services::file_retrieval_service::FileRetrievalService::new(
            Arc::new(infrastructure::repositories::FileFsReadRepository::default_stub())
        )),
        file_management_service: Arc::new(application::services::file_management_service::FileManagementService::new(
            Arc::new(infrastructure::repositories::FileFsWriteRepository::default_stub())
        )),
        file_use_case_factory: Arc::new(application::services::file_use_case_factory::AppFileUseCaseFactory::new(
            Arc::new(infrastructure::repositories::FileFsReadRepository::default_stub()),
            Arc::new(infrastructure::repositories::FileFsWriteRepository::default_stub())
        )),
        i18n_service: i18n_service.clone(),
        trash_service: trash_service.clone(),
        search_service: search_service.clone(),
        share_service: share_service.clone(),
        favorites_service: favorites_service.clone(),
        recent_service: recent_service.clone()
    };
    
    // Create the AppState without Arc first
    let calendar_service_option = None;
    
    let mut app_state = AppState {
        core: core_services,
        repositories: repository_services,
        applications: application_services,
        db_pool: db_pool.clone(),
        auth_service: auth_services.clone(),
        trash_service: trash_service.clone(),
        share_service: share_service.clone(),
        favorites_service: favorites_service.clone(),
        recent_service: recent_service.clone(),
        storage_usage_service: None,
        calendar_service: calendar_service_option,
        contact_service: contact_service.clone(),
    };
    
    // Initialize storage usage service
    let _storage_usage_service = if let Some(pool) = db_pool_ref {
        // Create a user repository that implements UserStoragePort
        let user_repository = Arc::new(
            infrastructure::repositories::pg::UserPgRepository::new(pool.clone())
        );
        
        // Create storage usage service that uses database for user information
        // and file repository for storage calculation
        let service = Arc::new(application::services::storage_usage_service::StorageUsageService::new(
            file_repository.clone(),
            user_repository,
        ));
        
        tracing::info!("Storage usage service initialized successfully");
        
        // Add the service to the app state
        app_state = app_state.with_storage_usage_service(service.clone());
        
        Some(service)
    } else {
        tracing::info!("Storage usage service is disabled (requires database connection)");
        None
    };
    
    // Wrap in Arc after all modifications
    let app_state = Arc::new(app_state);

    // Build application router
    let api_routes = create_api_routes(folder_service, file_service, Some(i18n_service), trash_service, search_service, share_service, favorites_service, recent_service);
    let web_routes = create_web_routes();
    
    // Build the app router
    // Import auth handler
    use interfaces::api::handlers::auth_handler::auth_routes;
    
    // Create basic app router
    let mut app = Router::new()
        .nest("/api", api_routes)
        .merge(web_routes)
        .layer(TraceLayer::new_for_http());
    
    // Add auth routes if auth is enabled
    if config.features.enable_auth && auth_services.is_some() {
        // Create auth routes with app state (no middleware needed - handlers extract token directly)
        let auth_router = auth_routes().with_state(app_state.clone());
        
        // Add auth routes at /api/auth
        app = app.nest("/api/auth", auth_router);
    }

    // Preload common directories to warm the cache
    tracing::info!("Preloading common directories to warm up cache...");
    if let Ok(count) = metadata_cache.preload_directory(&storage_path, true, 1).await {
        tracing::info!("Preloaded {} directory entries into cache", count);
    }
    
    // Start server with clear message
    let addr = SocketAddr::from(([0, 0, 0, 0], 8086));
    tracing::info!("Starting OxiCloud server on http://{}", addr);
    
    // Start the server
    tracing::info!("Authentication system initialized successfully");
    
    // Import the redirect middleware
    use crate::interfaces::middleware::redirect::redirect_middleware;
    
    // Apply the redirect middleware to handle legacy routes
    app = app.layer(axum::middleware::from_fn(redirect_middleware));
    
    // Create a standard TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Server binding to http://{}", addr);
    tracing::info!("Starting server with Axum routes...");
    
    // Axum 0.8 requires the state to match the expected type
    // Extract the state from Arc so we can pass it to the router
    let app_state_inner = Arc::try_unwrap(app_state)
        .unwrap_or_else(|arc| (*arc).clone());
    
    // Add global state to the router
    let app = app.with_state(app_state_inner);
    
    // Use axum's serve function with the router with state
    axum::serve(listener, app).await?;
    
    tracing::info!("Server shutdown completed");
    
    Ok(())
}

