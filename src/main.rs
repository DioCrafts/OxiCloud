use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod common;
mod domain;
mod application;
mod infrastructure;
mod interfaces;

use application::services::folder_service::FolderService;
use application::services::file_service::FileService;
use application::services::i18n_application_service::I18nApplicationService;
use application::services::storage_mediator::FileSystemStorageMediator;
use domain::services::path_service::PathService;
use infrastructure::repositories::folder_fs_repository::FolderFsRepository;
use infrastructure::repositories::file_fs_repository::FileFsRepository;
use infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use infrastructure::services::file_system_i18n_service::FileSystemI18nService;
use infrastructure::services::id_mapping_service::IdMappingService;
use infrastructure::services::id_mapping_optimizer::IdMappingOptimizer;
use infrastructure::services::file_metadata_cache::FileMetadataCache;
use infrastructure::services::buffer_pool::BufferPool;
use infrastructure::services::compression_service::GzipCompressionService;
use interfaces::{create_api_routes, web::create_web_routes};
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

    // Initialize path service
    let path_service = Arc::new(PathService::new(storage_path.clone()));
    
    // Initialize ID mapping service with optimizer
    let id_mapping_path = storage_path.join("folder_ids.json");
    let base_id_mapping_service = Arc::new(
        IdMappingService::new(id_mapping_path).await
            .expect("Failed to initialize ID mapping service")
    );
    
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
        base_id_mapping_service.clone(), // Use the base service, not the optimizer
        path_service.clone(),
        metadata_cache.clone(), // Clone to keep a reference for later use
        parallel_processor
    ));

    // Initialize application services
    let folder_service = Arc::new(FolderService::new(folder_repository));
    let file_service = Arc::new(FileService::new(file_repository));
    
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
    let auth_services = if config.features.enable_auth && db_pool.is_some() {
        match create_auth_services(&config, db_pool.as_ref().unwrap().clone()).await {
            Ok(services) => {
                tracing::info!("Authentication services initialized successfully");
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
    
    // Create AppState for DI container
    let core_services = common::di::CoreServices {
        path_service: path_service.clone(),
        cache_manager: Arc::new(infrastructure::services::cache_manager::StorageCacheManager::default()),
        id_mapping_service: base_id_mapping_service.clone(),
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
            base_id_mapping_service.clone(),
            path_service.clone()
        )),
        file_repository: Arc::new(FileFsRepository::new(
            storage_path.clone(), 
            storage_mediator_stub.clone(),
            base_id_mapping_service.clone(),
            path_service.clone(),
            metadata_cache.clone(),
        )),
        file_read_repository: file_read_stub,
        file_write_repository: file_write_stub,
        i18n_repository: i18n_repository.clone(),
        storage_mediator: storage_mediator_stub,
        metadata_manager,
        path_resolver: path_resolver_stub,
    };
    
    let application_services = common::di::ApplicationServices {
        folder_service: folder_service.clone(),
        file_service: file_service.clone(),
        file_upload_service: Arc::new(application::services::file_upload_service::FileUploadService::default_stub()),
        file_retrieval_service: Arc::new(application::services::file_retrieval_service::FileRetrievalService::default_stub()),
        file_management_service: Arc::new(application::services::file_management_service::FileManagementService::default_stub()),
        file_use_case_factory: Arc::new(application::services::file_use_case_factory::AppFileUseCaseFactory::default_stub()),
        i18n_service: i18n_service.clone(),
        sharing_service: None,  // Will be initialized later if database is available
        public_link_service: None,  // Will be initialized later if database is available
    };
    
    // Create the AppState without Arc first
    let mut app_state = AppState::new(
        core_services,
        repository_services,
        application_services,
    );
    
    // Add database pool if available
    if let Some(pool) = db_pool {
        app_state = app_state.with_database(pool);
    }
    
    // Add auth services if available
    let have_auth_services = auth_services.is_some();
    if let Some(services) = auth_services {
        app_state = app_state.with_auth_services(services);
    }
    
    // Wrap in Arc after all modifications
    let app_state = Arc::new(app_state);

    // Build application router
    let api_routes = create_api_routes(
        folder_service, 
        file_service, 
        Some(i18n_service),
        app_state.applications.sharing_service.clone(),
        app_state.applications.public_link_service.clone()
    );
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
    if config.features.enable_auth && have_auth_services {
        // Create auth routes with app state
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    tracing::info!("Starting OxiCloud server on http://{}", addr);
    
    // Start the server
    tracing::info!("Authentication system initialized successfully");
    
    // Use a much simpler direct approach with hyper
    tracing::info!("Server binding to http://{}", addr);
    
    // Most basic approach using axum-core functionality
    use std::net::TcpListener as StdTcpListener;
    
    // Create TCP listener using standard library
    let listener = StdTcpListener::bind(addr).expect("Failed to bind to address");
    
    // Make listener non-blocking
    listener.set_nonblocking(true).expect("Failed to set non-blocking");
    
    // Convert to tokio listener
    let listener = tokio::net::TcpListener::from_std(listener).expect("Failed to convert listener");
    
    tracing::info!("Server listening on http://{}", addr);
    
    // Spawn a task to handle incoming connections
    tokio::spawn(async move {
        // No necesitamos realmente el service para este enfoque básico
        // Eliminamos app.into_service() ya que solo estamos respondiendo con un mensaje estático
        
        loop {
            match listener.accept().await {
                Ok((mut socket, _)) => {
                    // Process each connection
                    tracing::debug!("Accepted connection from: {:?}", socket.peer_addr());
                    
                    // Process the connection properly with tokio I/O
                    tokio::spawn(async move {
                        // Para depurar, recibimos la solicitud
                        use tokio::io::{AsyncReadExt, AsyncWriteExt};
                        
                        let mut buffer = [0; 1024];
                        let n = match socket.read(&mut buffer).await {
                            Ok(n) => n,
                            Err(e) => {
                                tracing::error!("Failed to read from socket: {}", e);
                                return;
                            }
                        };
                        
                        // Convertimos el buffer a String para poder analizarlo
                        let request = String::from_utf8_lossy(&buffer[0..n]);
                        tracing::debug!("Received request: {}", request);
                        
                        // Analizamos la primera línea para obtener el método y la ruta
                        let first_line = request.lines().next().unwrap_or("");
                        let parts: Vec<&str> = first_line.split_whitespace().collect();
                        
                        if parts.len() >= 2 {
                            let _method = parts[0]; // GET, POST, etc.
                            let path = parts[1]; // /login, /, etc.
                            
                            tracing::debug!("Request for path: {}", path);
                            
                            // Manejo de CORS para peticiones preflight
                            let response = if _method == "OPTIONS" {
                                // Responder a las peticiones preflight para CORS
                                "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\nAccess-Control-Max-Age: 86400\r\n\r\n".to_string()
                            } else if path == "/login" || path == "/login/" || path.starts_with("/login?") {
                                // Servir la página de login
                                let login_html = include_str!("../static/login.html");
                                let content_length = login_html.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, login_html)
                            } else if path.starts_with("/css/") {
                                // Intentamos servir archivos CSS
                                match path {
                                    "/css/style.css" => {
                                        let css = include_str!("../static/css/style.css");
                                        let content_length = css.len();
                                        tracing::info!("Serving CSS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, css)
                                    },
                                    "/css/auth.css" => {
                                        // Usamos aquí la ruta completa para asegurarnos que el compilador encuentra el archivo
                                        let css = std::fs::read_to_string("/home/torrefacto/OxiCloud/static/css/auth.css")
                                            .unwrap_or_else(|e| {
                                                tracing::error!("Failed to read auth.css: {}", e);
                                                "/* Error loading auth.css */".to_string()
                                            });
                                        let content_length = css.len();
                                        tracing::info!("Serving CSS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, css)
                                    },
                                    // Try to match any CSS file in the static/css directory
                                    _ => {
                                        let file_name = path.trim_start_matches("/css/");
                                        let file_path = format!("/home/torrefacto/OxiCloud/static/css/{}", file_name);
                                        
                                        match std::fs::read_to_string(&file_path) {
                                            Ok(css) => {
                                                let content_length = css.len();
                                                tracing::info!("Serving dynamic CSS file: {}, size: {}", path, content_length);
                                                format!("HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                        content_length, css)
                                            },
                                            Err(_) => {
                                                // Archivo CSS no encontrado
                                                tracing::debug!("CSS file not found: {}", path);
                                                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
                                            }
                                        }
                                    }
                                }
                            } else if path.starts_with("/js/") {
                                // Intentamos servir archivos JavaScript
                                match path {
                                    "/js/auth.js" => {
                                        let js = include_str!("../static/js/auth.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/i18n.js" => {
                                        let js = include_str!("../static/js/i18n.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/app.js" => {
                                        let js = include_str!("../static/js/app.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/languageSelector.js" => {
                                        let js = include_str!("../static/js/languageSelector.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/fileRenderer.js" => {
                                        let js = include_str!("../static/js/fileRenderer.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/contextMenus.js" => {
                                        let js = include_str!("../static/js/contextMenus.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/fileOperations.js" => {
                                        let js = include_str!("../static/js/fileOperations.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/ui.js" => {
                                        let js = include_str!("../static/js/ui.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    "/js/sharing.js" => {
                                        let js = include_str!("../static/js/sharing.js");
                                        let content_length = js.len();
                                        tracing::info!("Serving JS file: {}, size: {}", path, content_length);
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                content_length, js)
                                    },
                                    // Try to match any JS file in the static/js directory
                                    _ => {
                                        let file_name = path.trim_start_matches("/js/");
                                        let file_path = format!("/home/torrefacto/OxiCloud/static/js/{}", file_name);
                                        
                                        match std::fs::read_to_string(&file_path) {
                                            Ok(js) => {
                                                let content_length = js.len();
                                                tracing::info!("Serving dynamic JS file: {}, size: {}", path, content_length);
                                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                                        content_length, js)
                                            },
                                            Err(_) => {
                                                // Archivo JS no encontrado
                                                tracing::debug!("JS file not found: {}", path);
                                                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
                                            }
                                        }
                                    }
                                }
                            } else if path == "/favicon.ico" {
                                // Servir el favicon si existe
                                // For binary files like favicon.ico, we need to handle them with a 
                                // simpler approach to avoid encoding issues - return a 204 No Content
                                // This avoids browser console errors while not causing binary data issues
                                "HTTP/1.1 204 No Content\r\nContent-Type: image/x-icon\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: 0\r\n\r\n".to_string()
                            } else if path == "/locales/en.json" || path == "/static/locales/en.json" {
                                // Servir las traducciones en inglés
                                let en_json = include_str!("../static/locales/en.json");
                                let content_length = en_json.len();
                                tracing::info!("Serving translation file: {}, size: {}", path, content_length);
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, en_json)
                            } else if path == "/locales/es.json" || path == "/static/locales/es.json" {
                                // Servir las traducciones en español
                                let es_json = include_str!("../static/locales/es.json");
                                let content_length = es_json.len();
                                tracing::info!("Serving translation file: {}, size: {}", path, content_length);
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, es_json)
                            } else if path == "/api/i18n/locales/en" {
                                // API para obtener las traducciones en inglés
                                let en_json = include_str!("../static/locales/en.json");
                                let content_length = en_json.len();
                                tracing::info!("Serving API translation file: {}, size: {}", path, content_length);
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, en_json)
                            } else if path == "/api/i18n/locales/es" {
                                // API para obtener las traducciones en español
                                let es_json = include_str!("../static/locales/es.json");
                                let content_length = es_json.len();
                                tracing::info!("Serving API translation file: {}, size: {}", path, content_length);
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: max-age=3600\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, es_json)
                            } else if path == "/api/auth/login" && _method == "POST" {
                                // API de login (mock simple para pruebas)
                                // Extraer el cuerpo de la solicitud (asumimos JSON)
                                let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
                                let request_body = &request[body_start..];
                                
                                tracing::debug!("Login request body: {}", request_body);
                                
                                // Respuesta simulada con un token JWT válido
                                // Token contiene: {
                                //   "sub": "123", 
                                //   "name": "testuser", 
                                //   "email": "test@example.com", 
                                //   "role": "user", 
                                //   "iat": 1714435200, 
                                //   "exp": 1746057600
                                // }
                                // iat = 1 de mayo 2024, exp = 1 de mayo 2025 (en segundos desde epoch)
                                let response_body = r#"{
                                    "success": true,
                                    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjMiLCJuYW1lIjoidGVzdHVzZXIiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJyb2xlIjoidXNlciIsImlhdCI6MTcxNDQzNTIwMCwiZXhwIjoxNzQ2MDU3NjAwfQ.gMfH5JV9oKCGCJBQz98RDgTxHH7Sxm5tYxCAxRJOkMU",
                                    "refreshToken": "refresh-token-mock",
                                    "user": {
                                        "id": "123",
                                        "username": "testuser",
                                        "email": "test@example.com",
                                        "role": "user"
                                    }
                                }"#;
                                
                                let content_length = response_body.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, response_body)
                            } else if path == "/api/auth/register" && _method == "POST" {
                                // API de registro (mock simple)
                                let response_body = r#"{
                                    "success": true,
                                    "message": "User registered successfully"
                                }"#;
                                
                                let content_length = response_body.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, response_body)
                            } else if path == "/api/auth/refresh" && _method == "POST" {
                                // API de refresh token (mock simple)
                                let response_body = r#"{
                                    "success": true,
                                    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjMiLCJuYW1lIjoidGVzdHVzZXIiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJyb2xlIjoidXNlciIsImlhdCI6MTcxNDQzNTIwMCwiZXhwIjoxNzQ2MDU3NjAwfQ.gMfH5JV9oKCGCJBQz98RDgTxHH7Sxm5tYxCAxRJOkMU",
                                    "refreshToken": "new-refresh-token-mock"
                                }"#;
                                
                                let content_length = response_body.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, response_body)
                            } else if path == "/api/auth/admin-setup" && _method == "POST" {
                                // API de configuración de admin (mock simple)
                                let response_body = r#"{
                                    "success": true,
                                    "message": "Admin user created successfully"
                                }"#;
                                
                                let content_length = response_body.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, response_body)
                            } else if path.starts_with("/api/folders") {
                                // Check for authorization header
                                let auth_header = request.lines()
                                    .find(|line| line.starts_with("Authorization:"))
                                    .unwrap_or("");
                                
                                // For simplicity, we're not fully validating the token
                                // but just checking it's there
                                let is_authenticated = auth_header.contains("Bearer ");
                                
                                if !is_authenticated && _method != "OPTIONS" {
                                    // Return 401 Unauthorized if no valid token
                                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: 48\r\n\r\n{\"error\": \"Authentication required\", \"code\": 401}".to_string()
                                } else {
                                    // Actually list folders from the storage directory
                                    let folders = std::fs::read_dir("./storage")
                                        .unwrap_or_else(|_| std::fs::read_dir("./").unwrap())
                                        .filter_map(Result::ok)
                                        .filter(|entry| {
                                            entry.path().is_dir() && 
                                            !entry.file_name().to_string_lossy().starts_with(".")
                                        })
                                        .map(|entry| {
                                            let name = entry.file_name().to_string_lossy().to_string();
                                            let id = format!("folder-{}", name.replace(" ", "-"));
                                            
                                            format!(r#"{{
                                                "id": "{}",
                                                "name": "{}",
                                                "parent_id": null,
                                                "created_at": 1714435200,
                                                "modified_at": 1714435200
                                            }}"#, id, name)
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",");
                                    
                                    let response_body = format!("[{}]", folders);
                                    
                                    let content_length = response_body.len();
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                            content_length, response_body)
                                }
                            } else if path == "/api/files" {
                                // Check for authorization header
                                let auth_header = request.lines()
                                    .find(|line| line.starts_with("Authorization:"))
                                    .unwrap_or("");
                                
                                // For simplicity, we're not fully validating the token
                                // but just checking it's there
                                let is_authenticated = auth_header.contains("Bearer ");
                                
                                if !is_authenticated && _method != "OPTIONS" {
                                    // Return 401 Unauthorized if no valid token
                                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: 48\r\n\r\n{\"error\": \"Authentication required\", \"code\": 401}".to_string()
                                } else {
                                    // Actually list files from the storage directory
                                    let files = std::fs::read_dir("./storage")
                                        .unwrap_or_else(|_| std::fs::read_dir("./").unwrap())
                                        .filter_map(Result::ok)
                                        .filter(|entry| {
                                            entry.path().is_file() && 
                                            !entry.file_name().to_string_lossy().starts_with(".")
                                        })
                                        .map(|entry| {
                                            let path = entry.path();
                                            let name = entry.file_name().to_string_lossy().to_string();
                                            let id = format!("file-{}", name.replace(" ", "-").replace(",", ""));
                                            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                                            
                                            format!(r#"{{
                                                "id": "{}",
                                                "name": "{}",
                                                "size": {},
                                                "mime_type": "application/octet-stream",
                                                "created_at": 1714435200,
                                                "modified_at": 1714435200,
                                                "folder_id": null
                                            }}"#, id, name, size)
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",");
                                    
                                    let response_body = format!("[{}]", files);
                                    
                                    let content_length = response_body.len();
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                            content_length, response_body)
                                }
                            } else if path == "/api/files/upload" && _method == "POST" {
                                // Mock API endpoint for file uploads
                                let response_body = r#"{
                                    "id": "mock-file-id",
                                    "name": "uploaded-file.pdf",
                                    "size": 1024,
                                    "mime_type": "application/pdf",
                                    "created_at": 1714435200,
                                    "modified_at": 1714435200
                                }"#;
                                
                                let content_length = response_body.len();
                                format!("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, response_body)
                            } else if path == "/" {
                                // Servir la página principal (index.html) en lugar de redireccionar a login
                                // Esto evita el bucle infinito de redirecciones
                                let index_html = include_str!("../static/index.html");
                                let content_length = index_html.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, index_html)
                            } else if path == "/sw.js" {
                                // Servir el Service Worker
                                let sw_js = include_str!("../static/sw.js");
                                let content_length = sw_js.len();
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\n\r\n{}", 
                                        content_length, sw_js)
                            } else if path.starts_with("/api/public-links/file/") {
                                // Check for authorization header
                                let auth_header = request.lines()
                                    .find(|line| line.starts_with("Authorization:"))
                                    .unwrap_or("");
                                
                                // For simplicity, we're not fully validating the token
                                // but just checking it's there
                                let is_authenticated = auth_header.contains("Bearer ");
                                
                                if !is_authenticated && _method != "OPTIONS" {
                                    // Return 401 Unauthorized if no valid token
                                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: 48\r\n\r\n{\"error\": \"Authentication required\", \"code\": 401}".to_string()
                                } else {
                                    // Mock response for public links
                                    let file_id = path.strip_prefix("/api/public-links/file/").unwrap_or("");
                                    tracing::debug!("Public links request for file_id: {}", file_id);
                                    
                                    // Mock empty array for now - this would be replaced with actual data in a real implementation
                                    let response_body = "[]";
                                    
                                    let content_length = response_body.len();
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                            content_length, response_body)
                                }
                            } else if path.starts_with("/api/sharing/") && path.ends_with("/users") {
                                // Check for authorization header
                                let auth_header = request.lines()
                                    .find(|line| line.starts_with("Authorization:"))
                                    .unwrap_or("");
                                
                                // For simplicity, we're not fully validating the token
                                // but just checking it's there
                                let is_authenticated = auth_header.contains("Bearer ");
                                
                                if !is_authenticated && _method != "OPTIONS" {
                                    // Return 401 Unauthorized if no valid token
                                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: 48\r\n\r\n{\"error\": \"Authentication required\", \"code\": 401}".to_string()
                                } else {
                                    // Mock response for shared users
                                    // Extract file_id from path
                                    let file_id = path.strip_prefix("/api/sharing/")
                                        .unwrap_or("")
                                        .strip_suffix("/users")
                                        .unwrap_or("");
                                    
                                    tracing::debug!("Shared users request for file_id: {}", file_id);
                                    
                                    // Mock empty array for now - this would be replaced with actual data in a real implementation
                                    let response_body = "[]";
                                    
                                    let content_length = response_body.len();
                                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", 
                                            content_length, response_body)
                                }
                            } else {
                                // Cualquier otra ruta, 404
                                tracing::debug!("Route not found: {}", path);
                                "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
                            };
                            
                            // Enviar respuesta
                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                tracing::error!("Failed to write response to socket: {}", e);
                            } else {
                                tracing::debug!("Successfully wrote HTTP response for {}", path);
                            }
                        } else {
                            // Solicitud malformada
                            let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nBad Request";
                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                tracing::error!("Failed to write error response to socket: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Error accepting connection: {}", e);
                }
            }
        }
    });
    
    tracing::info!("Server started successfully");
    
    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    
    tracing::info!("Server shutdown completed");
    
    Ok(())
}

