use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;

use crate::application::services::admin_settings_service::AdminSettingsService;
use crate::application::services::auth_application_service::AuthApplicationService;

use crate::application::ports::cache_ports::ContentCachePort;
use crate::application::ports::chunked_upload_ports::ChunkedUploadPort;

use crate::application::ports::dedup_ports::DedupPort;
use crate::application::ports::favorites_ports::FavoritesUseCase;
use crate::application::ports::file_ports::{
    FileManagementUseCase, FileRetrievalUseCase, FileUploadUseCase, FileUseCaseFactory,
};
use crate::application::ports::inbound::{FolderUseCase, SearchUseCase};
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::ports::recent_ports::RecentItemsUseCase;
use crate::application::ports::storage_ports::{FileReadPort, FileWritePort};
use crate::application::ports::thumbnail_ports::ThumbnailPort;
use crate::application::ports::transcode_ports::ImageTranscodePort;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::application::ports::zip_ports::ZipPort;
use crate::application::services::favorites_service::FavoritesService;
use crate::application::services::folder_service::FolderService;
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::application::services::recent_service::RecentService;
use crate::application::services::search_service::SearchService;
use crate::application::services::share_service::ShareService;
use crate::application::services::trash_service::TrashService;
use crate::application::services::{
    AppFileUseCaseFactory, FileManagementService, FileRetrievalService, FileUploadService,
};
use crate::common::config::AppConfig;
use crate::common::errors::DomainError;
use crate::domain::services::i18n_service::I18nService;
use crate::infrastructure::repositories::pg::{
    FileBlobReadRepository, FileBlobWriteRepository, FolderDbRepository, TrashDbRepository,
};
use crate::infrastructure::repositories::share_fs_repository::ShareFsRepository;
use crate::infrastructure::services::file_content_cache::{
    FileContentCache, FileContentCacheConfig,
};
use crate::infrastructure::services::file_system_i18n_service::FileSystemI18nService;
use crate::infrastructure::services::path_service::PathService;
use crate::infrastructure::services::trash_cleanup_service::TrashCleanupService;

use crate::common::stubs::{
    StubDedupPort, StubFileManagementUseCase, StubFileReadPort,
    StubFileRetrievalUseCase, StubFileUploadUseCase, StubFileUseCaseFactory, StubFileWritePort,
    StubFolderStoragePort, StubFolderUseCase, StubI18nService, StubSearchUseCase, StubZipPort,
};

/// Factory for the different application components
///
/// This factory centralizes the creation of all application services,
/// ensuring the correct initialization order and resolving circular dependencies.
pub struct AppServiceFactory {
    storage_path: PathBuf,
    locales_path: PathBuf,
    config: AppConfig,
}

impl AppServiceFactory {
    /// Creates a new service factory
    pub fn new(storage_path: PathBuf, locales_path: PathBuf) -> Self {
        Self {
            storage_path,
            locales_path,
            config: AppConfig::default(),
        }
    }

    /// Creates a new service factory with custom configuration
    pub fn with_config(storage_path: PathBuf, locales_path: PathBuf, config: AppConfig) -> Self {
        Self {
            storage_path,
            locales_path,
            config,
        }
    }

    /// Gets the configuration
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Gets the storage path
    pub fn storage_path(&self) -> &PathBuf {
        &self.storage_path
    }

    /// Initializes the core system services.
    ///
    /// Requires a `PgPool` because `DedupService` stores its index in PostgreSQL.
    pub async fn create_core_services(
        &self,
        db_pool: &Arc<PgPool>,
    ) -> Result<CoreServices, DomainError> {
        // Path service (still needed for blob storage root + thumbnails)
        let path_service = Arc::new(PathService::new(self.storage_path.clone()));

        // File content cache for ultra-fast file serving (hot files in RAM)
        let file_content_cache = Arc::new(FileContentCache::new(FileContentCacheConfig {
            max_file_size: 10 * 1024 * 1024,   // 10MB max per file
            max_total_size: 512 * 1024 * 1024, // 512MB total cache
            max_entries: 10000,                // Up to 10k files
        }));
        tracing::info!("FileContentCache initialized: max 10MB/file, 512MB total, 10k entries");

        // Thumbnail service for thumbnail generation
        let thumbnail_service = Arc::new(
            crate::infrastructure::services::thumbnail_service::ThumbnailService::new(
                &self.storage_path,
                5000,              // max 5000 thumbnails in cache
                100 * 1024 * 1024, // max 100MB cache
            ),
        );
        // Initialize thumbnail directories
        thumbnail_service.initialize().await?;

        // Chunked upload service for large files (>10MB)
        let chunked_temp_dir = std::path::PathBuf::from(&self.storage_path).join(".uploads");
        let chunked_upload_service = Arc::new(
            crate::infrastructure::services::chunked_upload_service::ChunkedUploadService::new(
                chunked_temp_dir,
            )
            .await,
        );

        // Image transcoding service for automatic WebP conversion
        let image_transcode_service = Arc::new(
            crate::infrastructure::services::image_transcode_service::ImageTranscodeService::new(
                &self.storage_path,
                2000,             // max 2000 transcoded images in cache
                50 * 1024 * 1024, // max 50MB in-memory cache
            ),
        );
        image_transcode_service.initialize().await?;

        // Deduplication service — PRIMARY blob storage engine (PostgreSQL-backed index)
        let dedup_service = Arc::new(
            crate::infrastructure::services::dedup_service::DedupService::new(
                &self.storage_path,
                db_pool.clone(),
            ),
        );
        dedup_service.initialize().await?;

        tracing::info!(
            "Core services initialized: path service, file content cache, thumbnails, chunked upload, image transcode, dedup (PRIMARY blob storage)"
        );

        Ok(CoreServices {
            path_service,
            file_content_cache,
            thumbnail_service,
            chunked_upload_service,
            image_transcode_service,
            dedup_service,
            zip_service: Arc::new(StubZipPort), // Placeholder - replaced after app services init
            config: self.config.clone(),
        })
    }

    /// Initializes the repository services (blob-storage model).
    ///
    /// Requires a PgPool since all metadata lives in PostgreSQL.
    pub fn create_repository_services(
        &self,
        core: &CoreServices,
        db_pool: &Arc<PgPool>,
    ) -> RepositoryServices {
        // Folder repository — PostgreSQL-backed virtual folders
        let folder_repo_concrete = Arc::new(FolderDbRepository::new(db_pool.clone()));
        let folder_repository: Arc<dyn FolderStoragePort> = folder_repo_concrete.clone();

        // File repositories — PostgreSQL metadata + blob content via DedupService
        let file_read_repository: Arc<dyn FileReadPort> = Arc::new(FileBlobReadRepository::new(
            db_pool.clone(),
            core.dedup_service.clone(),
            folder_repo_concrete.clone(),
        ));

        let file_write_repository: Arc<dyn FileWritePort> = Arc::new(FileBlobWriteRepository::new(
            db_pool.clone(),
            core.dedup_service.clone(),
            folder_repo_concrete.clone(),
        ));

        // I18n repository
        let i18n_repository = Arc::new(FileSystemI18nService::new(self.locales_path.clone()));

        // Trash repository — reads soft-delete flags from storage.files/folders
        let trash_repository = if core.config.features.enable_trash {
            Some(Arc::new(TrashDbRepository::new(
                db_pool.clone(),
                core.config.storage.trash_retention_days,
            ))
                as Arc<
                    dyn crate::domain::repositories::trash_repository::TrashRepository,
                >)
        } else {
            None
        };

        tracing::info!(
            "Repository services initialized with 100% blob storage model (PG metadata + DedupService blobs)"
        );

        RepositoryServices {
            folder_repository,
            folder_repo_concrete,
            file_read_repository,
            file_write_repository,
            i18n_repository,
            trash_repository,
        }
    }

    /// Initializes the application services
    pub fn create_application_services(
        &self,
        core: &CoreServices,
        repos: &RepositoryServices,
        trash_service: Option<Arc<dyn TrashUseCase>>,
    ) -> ApplicationServices {
        // Main services
        let folder_service = Arc::new(FolderService::new(repos.folder_repository.clone()));

        // Refactored services with all infrastructure ports
        // In blob model, dedup is handled by the repository — no separate write-behind needed
        let file_upload_service = Arc::new(FileUploadService::new_with_read(
            repos.file_write_repository.clone(),
            repos.file_read_repository.clone(),
        ));

        let file_retrieval_service = Arc::new(FileRetrievalService::new_with_cache(
            repos.file_read_repository.clone(),
            core.file_content_cache.clone(),
            core.image_transcode_service.clone(),
        ));

        // FileManagementService with dedup and trash
        let file_management_service = Arc::new(FileManagementService::new_full(
            repos.file_write_repository.clone(),
            repos.file_read_repository.clone(),
            trash_service.clone(),
            core.dedup_service.clone(),
        ));

        let file_use_case_factory = Arc::new(AppFileUseCaseFactory::new(
            repos.file_read_repository.clone(),
            repos.file_write_repository.clone(),
        ));

        let i18n_service = Arc::new(I18nApplicationService::new(repos.i18n_repository.clone()));

        // Search service with cache
        let search_service: Option<Arc<dyn SearchUseCase>> = Some(Arc::new(SearchService::new(
            repos.file_read_repository.clone(),
            repos.folder_repository.clone(),
            300,  // Cache TTL in seconds (5 minutes)
            1000, // Maximum cache entries
        )));

        tracing::info!("Application services initialized");

        ApplicationServices {
            // Concrete types for handlers that need them
            folder_service_concrete: folder_service.clone(),
            // Traits for abstraction
            folder_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service,
            trash_service, // Already set via parameter
            search_service,
            share_service: None,     // Configured later with create_share_service
            favorites_service: None, // Configured later with create_favorites_service
            recent_service: None,    // Configured later with create_recent_service
        }
    }

    /// Creates the trash service
    pub async fn create_trash_service(
        &self,
        repos: &RepositoryServices,
    ) -> Option<Arc<dyn TrashUseCase>> {
        if !self.config.features.enable_trash {
            tracing::info!("Trash service is disabled in configuration");
            return None;
        }

        let trash_repo = repos.trash_repository.as_ref()?;

        // Wire ports directly to TrashService — no adapter layer needed
        let service = Arc::new(TrashService::new(
            trash_repo.clone(),
            repos.file_read_repository.clone(),
            repos.file_write_repository.clone(),
            repos.folder_repository.clone(),
            self.config.storage.trash_retention_days,
        ));

        // Initialize cleanup service
        let cleanup_service = TrashCleanupService::new(
            service.clone(),
            trash_repo.clone(),
            24, // Run cleanup every 24 hours
        );

        cleanup_service.start_cleanup_job().await;
        tracing::info!("Trash service initialized with daily cleanup schedule");

        Some(service as Arc<dyn TrashUseCase>)
    }

    /// Creates the sharing service
    pub fn create_share_service(
        &self,
        repos: &RepositoryServices,
    ) -> Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>> {
        if !self.config.features.enable_file_sharing {
            tracing::info!("File sharing service is disabled in configuration");
            return None;
        }

        let share_repository = Arc::new(ShareFsRepository::new(Arc::new(self.config.clone())));

        // Build a password hasher for share password verification
        let password_hasher: Arc<dyn crate::application::ports::auth_ports::PasswordHasherPort> =
            Arc::new(crate::infrastructure::services::password_hasher::Argon2PasswordHasher::new());

        let service = Arc::new(ShareService::new(
            Arc::new(self.config.clone()),
            share_repository,
            repos.file_read_repository.clone(),
            repos.folder_repository.clone(),
            password_hasher,
        ));

        tracing::info!("File sharing service initialized");
        Some(service)
    }

    /// Creates the favorites service (requires database)
    pub fn create_favorites_service(&self, db_pool: &Arc<PgPool>) -> Arc<dyn FavoritesUseCase> {
        let repo = Arc::new(
            crate::infrastructure::repositories::pg::FavoritesPgRepository::new(db_pool.clone()),
        );
        let service = Arc::new(FavoritesService::new(repo));
        tracing::info!("Favorites service initialized");
        service
    }

    /// Creates the recent items service (requires database)
    pub fn create_recent_service(&self, db_pool: &Arc<PgPool>) -> Arc<dyn RecentItemsUseCase> {
        let repo = Arc::new(
            crate::infrastructure::repositories::pg::RecentItemsPgRepository::new(db_pool.clone()),
        );
        let service = Arc::new(RecentService::new(
            repo, 50, // Maximum recent items per user
        ));
        tracing::info!("Recent items service initialized");
        service
    }

    /// Preloads translations
    pub async fn preload_translations(&self, i18n_service: &I18nApplicationService) {
        use crate::domain::services::i18n_service::Locale;

        if let Err(e) = i18n_service.load_translations(Locale::English).await {
            tracing::warn!("Failed to load English translations: {}", e);
        }
        if let Err(e) = i18n_service.load_translations(Locale::Spanish).await {
            tracing::warn!("Failed to load Spanish translations: {}", e);
        }
        if let Err(e) = i18n_service.load_translations(Locale::French).await {
            tracing::warn!("Failed to load French translations: {}", e);
        }
        if let Err(e) = i18n_service.load_translations(Locale::German).await {
            tracing::warn!("Failed to load German translations: {}", e);
        }
        if let Err(e) = i18n_service.load_translations(Locale::Portuguese).await {
            tracing::warn!("Failed to load Portuguese translations: {}", e);
        }
        tracing::info!("Translations preloaded");
    }

    /// Creates the storage usage service (requires database)
    pub fn create_storage_usage_service(
        &self,
        _repos: &RepositoryServices,
        db_pool: &Arc<PgPool>,
    ) -> Arc<dyn crate::application::ports::storage_ports::StorageUsagePort> {
        let user_repository = Arc::new(
            crate::infrastructure::repositories::pg::UserPgRepository::new(db_pool.clone()),
        );
        let service = Arc::new(
            crate::application::services::storage_usage_service::StorageUsageService::new(
                db_pool.clone(),
                user_repository,
            ),
        );
        tracing::info!("Storage usage service initialized");
        service
    }

    /// Builds the complete AppState using all factory services.
    ///
    /// This is the main entry point that replaces all manual logic in `main.rs`.
    pub async fn build_app_state(
        &self,
        db_pool: Option<Arc<PgPool>>,
    ) -> Result<AppState, DomainError> {
        // Database is REQUIRED in 100% blob storage model
        let pool = db_pool.clone().ok_or_else(|| {
            DomainError::internal_error(
                "Database",
                "PostgreSQL database is required for blob storage model",
            )
        })?;

        // 1. Core services (PgPool needed for DedupService index)
        let core = self.create_core_services(&pool).await?;

        // 2. Repository services (requires PgPool for all metadata)
        let repos = self.create_repository_services(&core, &pool);

        // 3. Trash service (needed before application services)
        let trash_service = self.create_trash_service(&repos).await;

        // 4. Application services (with trash already wired)
        let mut apps = self.create_application_services(&core, &repos, trash_service.clone());

        // 5. Share service
        let share_service = self.create_share_service(&repos);
        apps.share_service = share_service.clone();

        // 6. Database-dependent services (PgPool always available in blob model)
        let favorites_service: Option<Arc<dyn FavoritesUseCase>>;
        let recent_service: Option<Arc<dyn RecentItemsUseCase>>;
        let storage_usage_service: Option<
            Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>,
        >;
        let mut auth_services: Option<crate::common::di::AuthServices> = None;

        {
            let favs = self.create_favorites_service(&pool);
            favorites_service = Some(favs.clone());
            apps.favorites_service = Some(favs);

            let recent = self.create_recent_service(&pool);
            recent_service = Some(recent.clone());
            apps.recent_service = Some(recent);

            storage_usage_service = Some(self.create_storage_usage_service(&repos, &pool));

            // Auth services
            if self.config.features.enable_auth {
                match crate::infrastructure::auth_factory::create_auth_services(
                    &self.config,
                    pool.clone(),
                    Some(apps.folder_service_concrete.clone()),
                )
                .await
                {
                    Ok(services) => {
                        tracing::info!("Authentication services initialized successfully");
                        auth_services = Some(services);
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize authentication services: {}", e);
                    }
                }
            }
        }

        // 7. Preload translations
        self.preload_translations(&apps.i18n_service).await;

        // 8. Build the ZipService with real application services
        let zip_service: Arc<dyn crate::application::ports::zip_ports::ZipPort> = Arc::new(
            crate::infrastructure::services::zip_service::ZipService::new(
                apps.file_retrieval_service.clone(),
                apps.folder_service.clone(),
            ),
        );
        let mut core = core;
        core.zip_service = zip_service;

        // 9. Assemble final AppState
        let mut app_state = AppState {
            core,
            repositories: repos,
            applications: apps,
            db_pool: db_pool.clone(),
            auth_service: auth_services,
            admin_settings_service: None,
            trash_service,
            share_service,
            favorites_service,
            recent_service,
            storage_usage_service,
            calendar_service: None,
            contact_service: None,
            calendar_use_case: None,
            addressbook_use_case: None,
            contact_use_case: None,
            wopi_token_service: None,
            wopi_lock_service: None,
            wopi_discovery_service: None,
        };

        // 9b. Wire admin settings service when auth is available
        if let Some(auth_svc) = &app_state.auth_service {
            let settings_repo = Arc::new(
                crate::infrastructure::repositories::pg::SettingsPgRepository::new(pool.clone()),
            );
            let server_base_url = self.config.base_url();

            // Load OIDC config from env vars (the snapshot from startup)
            let env_oidc = crate::common::config::OidcConfig::from_env();

            let admin_svc = Arc::new(AdminSettingsService::new(
                settings_repo.clone(),
                env_oidc,
                auth_svc.auth_application_service.clone(),
                server_base_url,
            ));

            // Hot-reload OIDC from DB settings if configured
            match admin_svc.load_effective_oidc_config().await {
                Ok(eff)
                    if eff.enabled
                        && !eff.issuer_url.is_empty()
                        && !eff.client_id.is_empty()
                        && !eff.client_secret.is_empty() =>
                {
                    let oidc_svc = Arc::new(
                        crate::infrastructure::services::oidc_service::OidcService::new(
                            eff.clone(),
                        ),
                    );
                    auth_svc.auth_application_service.reload_oidc(oidc_svc, eff);
                    tracing::info!("OIDC config loaded from admin settings (database)");
                }
                Ok(_) => {
                    tracing::info!(
                        "No active OIDC config in admin settings — using env vars or defaults"
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to load OIDC settings from database (table may not exist yet): {}",
                        e
                    );
                }
            }

            app_state.admin_settings_service = Some(admin_svc);
        }

        // 10. Wire CalDAV/CardDAV services
        {
            // CalDAV
            let calendar_repo: Arc<
                dyn crate::domain::repositories::calendar_repository::CalendarRepository,
            > = Arc::new(
                crate::infrastructure::repositories::pg::CalendarPgRepository::new(pool.clone()),
            );
            let event_repo: Arc<
                dyn crate::domain::repositories::calendar_event_repository::CalendarEventRepository,
            > = Arc::new(
                crate::infrastructure::repositories::pg::CalendarEventPgRepository::new(
                    pool.clone(),
                ),
            );
            let calendar_storage = Arc::new(
                crate::infrastructure::adapters::calendar_storage_adapter::CalendarStorageAdapter::new(
                    calendar_repo,
                    event_repo,
                )
            );
            let calendar_service = Arc::new(
                crate::application::services::calendar_service::CalendarService::new(
                    calendar_storage,
                ),
            );
            app_state.calendar_use_case = Some(
                calendar_service
                    as Arc<dyn crate::application::ports::calendar_ports::CalendarUseCase>,
            );

            // CardDAV
            let address_book_repo: Arc<
                dyn crate::domain::repositories::address_book_repository::AddressBookRepository,
            > = Arc::new(
                crate::infrastructure::repositories::pg::AddressBookPgRepository::new(pool.clone()),
            );
            let contact_repo: Arc<
                dyn crate::domain::repositories::contact_repository::ContactRepository,
            > = Arc::new(
                crate::infrastructure::repositories::pg::ContactPgRepository::new(pool.clone()),
            );
            let group_repo: Arc<
                dyn crate::domain::repositories::contact_repository::ContactGroupRepository,
            > = Arc::new(
                crate::infrastructure::repositories::pg::ContactGroupPgRepository::new(
                    pool.clone(),
                ),
            );
            let contact_storage = Arc::new(
                crate::infrastructure::adapters::contact_storage_adapter::ContactStorageAdapter::new(
                    address_book_repo,
                    contact_repo,
                    group_repo,
                )
            );
            app_state.addressbook_use_case = Some(contact_storage.clone()
                as Arc<dyn crate::application::ports::carddav_ports::AddressBookUseCase>);
            app_state.contact_use_case = Some(
                contact_storage
                    as Arc<dyn crate::application::ports::carddav_ports::ContactUseCase>,
            );

            tracing::info!("CalDAV and CardDAV services initialized with PostgreSQL repositories");
        }

        // 11. Wire WOPI services if enabled
        if self.config.wopi.enabled {
            let discovery_url = &self.config.wopi.discovery_url;
            if discovery_url.is_empty() {
                tracing::error!(
                    "WOPI is enabled but WOPI_DISCOVERY_URL is empty — WOPI services will NOT be available"
                );
            } else {
                use crate::application::services::wopi_lock_service::WopiLockService;
                use crate::application::services::wopi_token_service::WopiTokenService;
                use crate::infrastructure::services::wopi_discovery_service::WopiDiscoveryService;

                let wopi_secret = if self.config.wopi.secret.is_empty() {
                    self.config.auth.jwt_secret.clone()
                } else {
                    self.config.wopi.secret.clone()
                };

                let wopi_token_service = Arc::new(WopiTokenService::new(
                    wopi_secret,
                    self.config.wopi.token_ttl_secs,
                ));

                let wopi_lock_service =
                    Arc::new(WopiLockService::new(self.config.wopi.lock_ttl_secs));
                wopi_lock_service.start_cleanup_task();

                let wopi_discovery_service = Arc::new(WopiDiscoveryService::new(
                    discovery_url.clone(),
                    86400, // 24 hour cache TTL
                ));

                app_state.wopi_token_service = Some(wopi_token_service);
                app_state.wopi_lock_service = Some(wopi_lock_service);
                app_state.wopi_discovery_service = Some(wopi_discovery_service);

                tracing::info!("WOPI services initialized (discovery: {})", discovery_url);
            }
        }

        Ok(app_state)
    }
}

/// Container for core services
#[derive(Clone)]
pub struct CoreServices {
    pub path_service: Arc<PathService>,
    pub file_content_cache: Arc<dyn ContentCachePort>,
    pub thumbnail_service: Arc<dyn ThumbnailPort>,
    pub chunked_upload_service: Arc<dyn ChunkedUploadPort>,
    pub image_transcode_service: Arc<dyn ImageTranscodePort>,
    pub dedup_service: Arc<dyn DedupPort>,
    pub zip_service: Arc<dyn ZipPort>,
    pub config: AppConfig,
}

/// Container for repository services
#[derive(Clone)]
pub struct RepositoryServices {
    pub folder_repository: Arc<dyn FolderStoragePort>,
    pub folder_repo_concrete: Arc<FolderDbRepository>,
    pub file_read_repository: Arc<dyn FileReadPort>,
    pub file_write_repository: Arc<dyn FileWritePort>,
    pub i18n_repository: Arc<dyn I18nService>,
    pub trash_repository:
        Option<Arc<dyn crate::domain::repositories::trash_repository::TrashRepository>>,
}

/// Container for application services
#[derive(Clone)]
pub struct ApplicationServices {
    // Concrete types for compatibility with existing handlers
    pub folder_service_concrete: Arc<FolderService>,
    // Traits for abstraction
    pub folder_service: Arc<dyn FolderUseCase>,
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

/// Container for authentication services
#[derive(Clone)]
pub struct AuthServices {
    pub token_service: Arc<dyn crate::application::ports::auth_ports::TokenServicePort>,
    pub auth_application_service: Arc<AuthApplicationService>,
}

/// Global application state for dependency injection
#[derive(Clone)]
pub struct AppState {
    pub core: CoreServices,
    pub repositories: RepositoryServices,
    pub applications: ApplicationServices,
    pub db_pool: Option<Arc<PgPool>>,
    pub auth_service: Option<AuthServices>,
    pub admin_settings_service: Option<Arc<AdminSettingsService>>,
    pub trash_service: Option<Arc<dyn TrashUseCase>>,
    pub share_service: Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>>,
    pub favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    pub recent_service: Option<Arc<dyn RecentItemsUseCase>>,
    pub storage_usage_service:
        Option<Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>>,
    pub calendar_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUseCase>>,
    pub contact_service: Option<Arc<dyn crate::application::ports::storage_ports::StorageUseCase>>,
    pub calendar_use_case:
        Option<Arc<dyn crate::application::ports::calendar_ports::CalendarUseCase>>,
    pub addressbook_use_case:
        Option<Arc<dyn crate::application::ports::carddav_ports::AddressBookUseCase>>,
    pub contact_use_case: Option<Arc<dyn crate::application::ports::carddav_ports::ContactUseCase>>,
    pub wopi_token_service:
        Option<Arc<crate::application::services::wopi_token_service::WopiTokenService>>,
    pub wopi_lock_service:
        Option<Arc<crate::application::services::wopi_lock_service::WopiLockService>>,
    pub wopi_discovery_service:
        Option<Arc<crate::infrastructure::services::wopi_discovery_service::WopiDiscoveryService>>,
}

impl Default for AppState {
    fn default() -> Self {
        // Minimal stub version for auth middleware and route construction.
        // Real services are wired in main.rs via AppServiceFactory.

        let config = crate::common::config::AppConfig::default();
        let path_service = Arc::new(
            crate::infrastructure::services::path_service::PathService::new(
                std::path::PathBuf::from("./storage"),
            ),
        );

        let i18n_repository = Arc::new(StubI18nService)
            as Arc<dyn crate::domain::services::i18n_service::I18nService>;
        let folder_service = Arc::new(StubFolderUseCase)
            as Arc<dyn crate::application::ports::inbound::FolderUseCase>;
        let file_upload_service = Arc::new(StubFileUploadUseCase)
            as Arc<dyn crate::application::ports::file_ports::FileUploadUseCase>;
        let file_retrieval_service = Arc::new(StubFileRetrievalUseCase)
            as Arc<dyn crate::application::ports::file_ports::FileRetrievalUseCase>;
        let file_management_service = Arc::new(StubFileManagementUseCase)
            as Arc<dyn crate::application::ports::file_ports::FileManagementUseCase>;
        let file_use_case_factory = Arc::new(StubFileUseCaseFactory)
            as Arc<dyn crate::application::ports::file_ports::FileUseCaseFactory>;

        // Create file content cache for stub
        let file_content_cache = Arc::new(FileContentCache::new(FileContentCacheConfig::default()));

        // Create dummy thumbnail service
        let dummy_thumbnail_service: Arc<dyn ThumbnailPort> = Arc::new(
            crate::infrastructure::services::thumbnail_service::ThumbnailService::new(
                &std::path::PathBuf::from("./storage"),
                100,
                10 * 1024 * 1024,
            ),
        );

        // Create dummy chunked upload service
        let dummy_chunked_upload_service: Arc<dyn ChunkedUploadPort> = Arc::new(
            crate::infrastructure::services::chunked_upload_service::ChunkedUploadService::new_stub(
                std::path::PathBuf::from("./storage/.uploads"),
            ),
        );

        // Create dummy image transcode service
        let dummy_image_transcode_service: Arc<dyn ImageTranscodePort> = Arc::new(
            crate::infrastructure::services::image_transcode_service::ImageTranscodeService::new(
                &std::path::PathBuf::from("./storage"),
                100,
                10 * 1024 * 1024,
            ),
        );

        // Stub dedup service (Default is only used for routing stubs, never for real I/O)
        let dummy_dedup_service: Arc<dyn DedupPort> = Arc::new(StubDedupPort);

        // Core services using stubs
        let core_services = CoreServices {
            path_service: path_service.clone(),
            file_content_cache,
            thumbnail_service: dummy_thumbnail_service,
            chunked_upload_service: dummy_chunked_upload_service,
            image_transcode_service: dummy_image_transcode_service,
            dedup_service: dummy_dedup_service,
            zip_service: Arc::new(StubZipPort) as Arc<dyn ZipPort>,
            config: config.clone(),
        };

        // Stub folder repo concrete — uses a dummy PgPool-less constructor
        // In practice, Default is only used for routing stubs; real app always goes through build_app_state
        let dummy_folder_repo_concrete = Arc::new(FolderDbRepository::new_stub());

        // Repository services using stubs
        let repository_services = RepositoryServices {
            folder_repository: Arc::new(StubFolderStoragePort)
                as Arc<dyn crate::application::ports::outbound::FolderStoragePort>,
            folder_repo_concrete: dummy_folder_repo_concrete,
            file_read_repository: Arc::new(StubFileReadPort)
                as Arc<dyn crate::application::ports::storage_ports::FileReadPort>,
            file_write_repository: Arc::new(StubFileWritePort)
                as Arc<dyn crate::application::ports::storage_ports::FileWritePort>,
            i18n_repository,
            trash_repository: None,
        };

        // Dummy concrete services for compatibility
        let dummy_folder_storage = Arc::new(StubFolderStoragePort)
            as Arc<dyn crate::application::ports::outbound::FolderStoragePort>;
        let folder_service_concrete = Arc::new(FolderService::new(dummy_folder_storage));

        // Dummy I18nApplicationService
        let dummy_i18n_app_service =
            crate::application::services::i18n_application_service::I18nApplicationService::new(
                Arc::new(StubI18nService)
                    as Arc<dyn crate::domain::services::i18n_service::I18nService>,
            );

        // Application services using stubs
        let application_services = ApplicationServices {
            folder_service_concrete: folder_service_concrete.clone(),
            folder_service,
            file_upload_service,
            file_retrieval_service,
            file_management_service,
            file_use_case_factory,
            i18n_service: Arc::new(dummy_i18n_app_service),
            trash_service: None,
            search_service: Some(Arc::new(StubSearchUseCase)
                as Arc<dyn crate::application::ports::inbound::SearchUseCase>),
            share_service: None,
            favorites_service: None,
            recent_service: None,
        };

        Self {
            core: core_services,
            repositories: repository_services,
            applications: application_services,
            db_pool: None,
            auth_service: None,
            admin_settings_service: None,
            trash_service: None,
            share_service: None,
            favorites_service: None,
            recent_service: None,
            storage_usage_service: None,
            calendar_service: None,
            contact_service: None,
            calendar_use_case: None,
            addressbook_use_case: None,
            contact_use_case: None,
            wopi_token_service: None,
            wopi_lock_service: None,
            wopi_discovery_service: None,
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
            admin_settings_service: None,
            trash_service: None,
            share_service: None,
            favorites_service: None,
            recent_service: None,
            storage_usage_service: None,
            calendar_service: None,
            contact_service: None,
            calendar_use_case: None,
            addressbook_use_case: None,
            contact_use_case: None,
            wopi_token_service: None,
            wopi_lock_service: None,
            wopi_discovery_service: None,
        }
    }

    pub fn with_database(mut self, db_pool: Arc<PgPool>) -> Self {
        self.db_pool = Some(db_pool);
        self
    }

    /// Creates a minimal AppState for route construction.
    ///
    /// Uses `Default` stubs for infrastructure services, then overlays the real
    /// application-level services that arrive as parameters from `main.rs`.
    /// This keeps `routes.rs` free of any `crate::infrastructure` references.
    #[allow(clippy::too_many_arguments)]
    pub fn for_routing(
        folder_service: Arc<FolderService>,
        file_retrieval_service: Arc<
            dyn crate::application::ports::file_ports::FileRetrievalUseCase,
        >,
        file_upload_service: Arc<dyn FileUploadUseCase>,
        file_management_service: Arc<dyn FileManagementUseCase>,
        folder_use_case: Arc<dyn crate::application::ports::inbound::FolderUseCase>,
        i18n_service: Option<
            Arc<crate::application::services::i18n_application_service::I18nApplicationService>,
        >,
        trash_service: Option<Arc<dyn TrashUseCase>>,
        search_service: Option<Arc<dyn crate::application::ports::inbound::SearchUseCase>>,
        share_service: Option<Arc<dyn crate::application::ports::share_ports::ShareUseCase>>,
        favorites_service: Option<Arc<dyn FavoritesUseCase>>,
        recent_service: Option<Arc<dyn RecentItemsUseCase>>,
    ) -> Self {
        let mut state = Self::default();

        // Override application services with real ones
        state.applications.folder_service_concrete = folder_service.clone();
        state.applications.folder_service = folder_use_case;
        state.applications.file_upload_service = file_upload_service;
        state.applications.file_retrieval_service = file_retrieval_service.clone();
        state.applications.file_management_service = file_management_service;

        if let Some(i18n) = i18n_service {
            state.applications.i18n_service = i18n;
        }

        state.applications.trash_service = trash_service.clone();
        state.applications.search_service = search_service.clone();
        state.applications.share_service = share_service.clone();
        state.applications.favorites_service = favorites_service.clone();
        state.applications.recent_service = recent_service.clone();

        // Also set top-level optional services
        state.trash_service = trash_service;
        state.share_service = share_service;
        state.favorites_service = favorites_service;
        state.recent_service = recent_service;

        // Create real ZipService with the actual file/folder services
        state.core.zip_service = Arc::new(
            crate::infrastructure::services::zip_service::ZipService::new(
                file_retrieval_service
                    as Arc<dyn crate::application::ports::file_ports::FileRetrievalUseCase>,
                folder_service.clone()
                    as Arc<dyn crate::application::ports::inbound::FolderUseCase>,
            ),
        );

        state
    }

    pub fn with_auth_services(mut self, auth_services: AuthServices) -> Self {
        self.auth_service = Some(auth_services);
        self
    }

    pub fn with_trash_service(mut self, trash_service: Arc<dyn TrashUseCase>) -> Self {
        self.trash_service = Some(trash_service);
        self
    }

    pub fn with_share_service(
        mut self,
        share_service: Arc<dyn crate::application::ports::share_ports::ShareUseCase>,
    ) -> Self {
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

    pub fn with_storage_usage_service(
        mut self,
        storage_usage_service: Arc<dyn crate::application::ports::storage_ports::StorageUsagePort>,
    ) -> Self {
        self.storage_usage_service = Some(storage_usage_service);
        self
    }

    pub fn with_calendar_service(
        mut self,
        calendar_service: Arc<dyn crate::application::ports::storage_ports::StorageUseCase>,
    ) -> Self {
        self.calendar_service = Some(calendar_service);
        self
    }

    pub fn with_contact_service(
        mut self,
        contact_service: Arc<dyn crate::application::ports::storage_ports::StorageUseCase>,
    ) -> Self {
        self.contact_service = Some(contact_service);
        self
    }

    pub fn with_calendar_use_case(
        mut self,
        calendar_use_case: Arc<dyn crate::application::ports::calendar_ports::CalendarUseCase>,
    ) -> Self {
        self.calendar_use_case = Some(calendar_use_case);
        self
    }

    pub fn with_addressbook_use_case(
        mut self,
        addressbook_use_case: Arc<dyn crate::application::ports::carddav_ports::AddressBookUseCase>,
    ) -> Self {
        self.addressbook_use_case = Some(addressbook_use_case);
        self
    }

    pub fn with_contact_use_case(
        mut self,
        contact_use_case: Arc<dyn crate::application::ports::carddav_ports::ContactUseCase>,
    ) -> Self {
        self.contact_use_case = Some(contact_use_case);
        self
    }

    pub fn with_zip_service(mut self, zip_service: Arc<dyn ZipPort>) -> Self {
        self.core.zip_service = zip_service;
        self
    }
}
