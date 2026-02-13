use std::time::Duration;
use std::path::PathBuf;
use std::env;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// TTL for file cache entries (ms)
    pub file_ttl_ms: u64,
    /// TTL for directory cache entries (ms)
    pub directory_ttl_ms: u64,
    /// Maximum number of cache entries
    pub max_entries: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            file_ttl_ms: 60_000,     // 1 minute
            directory_ttl_ms: 120_000, // 2 minutes
            max_entries: 10_000,      // 10,000 entries
        }
    }
}

/// Timeout configuration for different operations
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Timeout for file operations (ms)
    pub file_operation_ms: u64,
    /// Timeout for directory operations (ms)
    pub dir_operation_ms: u64,
    /// Timeout for lock acquisition (ms)
    pub lock_acquisition_ms: u64,
    /// Timeout for network operations (ms)
    pub network_operation_ms: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            file_operation_ms: 10000,    // 10 seconds
            dir_operation_ms: 30000,     // 30 seconds
            lock_acquisition_ms: 5000,   // 5 seconds
            network_operation_ms: 15000, // 15 seconds
        }
    }
}

impl TimeoutConfig {
    /// Gets a Duration for file operations
    pub fn file_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file write operations
    pub fn file_write_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file read operations
    pub fn file_read_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for file delete operations
    pub fn file_delete_timeout(&self) -> Duration {
        Duration::from_millis(self.file_operation_ms)
    }

    /// Gets a Duration for directory operations
    pub fn dir_timeout(&self) -> Duration {
        Duration::from_millis(self.dir_operation_ms)
    }

    /// Gets a Duration for lock acquisition
    pub fn lock_timeout(&self) -> Duration {
        Duration::from_millis(self.lock_acquisition_ms)
    }

    /// Gets a Duration for network operations
    pub fn network_timeout(&self) -> Duration {
        Duration::from_millis(self.network_operation_ms)
    }
}

/// Configuration for large resource handling
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Threshold in MB to consider a file as large
    pub large_file_threshold_mb: u64,
    /// Entry threshold to consider a directory as large
    pub large_dir_threshold_entries: usize,
    /// Chunk size for large file processing (bytes)
    pub chunk_size_bytes: usize,
    /// File size limit for loading into memory (MB)
    pub max_in_memory_file_size_mb: u64,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            large_file_threshold_mb: 100,       // 100 MB
            large_dir_threshold_entries: 1000,  // 1000 entries
            chunk_size_bytes: 1024 * 1024,      // 1 MB
            max_in_memory_file_size_mb: 50,     // 50 MB
        }
    }
}

impl ResourceConfig {
    /// Converts a size in bytes to MB
    pub fn bytes_to_mb(&self, bytes: u64) -> u64 {
        bytes / (1024 * 1024)
    }

    /// Determines if a file is considered large
    pub fn is_large_file(&self, size_bytes: u64) -> bool {
        self.bytes_to_mb(size_bytes) >= self.large_file_threshold_mb
    }
    
    /// Determines if a file is large enough for parallel processing
    pub fn needs_parallel_processing(&self, size_bytes: u64, config: &ConcurrencyConfig) -> bool {
        self.bytes_to_mb(size_bytes) >= config.min_size_for_parallel_chunks_mb
    }

    /// Determines if a file can be fully loaded into memory
    pub fn can_load_in_memory(&self, size_bytes: u64) -> bool {
        self.bytes_to_mb(size_bytes) <= self.max_in_memory_file_size_mb
    }

    /// Determines if a directory is considered large
    pub fn is_large_directory(&self, entry_count: usize) -> bool {
        entry_count >= self.large_dir_threshold_entries
    }
    
    /// Calculates the number of chunks for parallel processing
    pub fn calculate_optimal_chunks(&self, size_bytes: u64, config: &ConcurrencyConfig) -> usize {
        // If the file is not large enough, return 1
        if !self.needs_parallel_processing(size_bytes, config) {
            return 1;
        }
        
        // Calculate the number of chunks based on size
        let chunk_count = (size_bytes as usize + config.parallel_chunk_size_bytes - 1) 
                         / config.parallel_chunk_size_bytes;
                         
        // Limit to the maximum number of parallel chunks
        chunk_count.min(config.max_parallel_chunks)
    }
    
    /// Calculates the optimal size of each chunk for parallel processing
    pub fn calculate_chunk_size(&self, file_size: u64, chunk_count: usize) -> usize {
        if chunk_count <= 1 {
            return file_size as usize;
        }
        
        // Distribute the size evenly among the chunks
        ((file_size as usize) + chunk_count - 1) / chunk_count
    }
}

/// Configuration for concurrent operations
#[derive(Debug, Clone)]
pub struct ConcurrencyConfig {
    /// Maximum concurrent file tasks
    pub max_concurrent_files: usize,
    /// Maximum concurrent directory tasks
    pub max_concurrent_dirs: usize,
    /// Maximum concurrent IO operations
    pub max_concurrent_io: usize,
    /// Maximum chunks to process in parallel per file
    pub max_parallel_chunks: usize,
    /// Minimum file size (MB) to apply parallel chunk processing
    pub min_size_for_parallel_chunks_mb: u64,
    /// Chunk size for parallel processing (bytes)
    pub parallel_chunk_size_bytes: usize,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_files: 10,
            max_concurrent_dirs: 5,
            max_concurrent_io: 20,
            max_parallel_chunks: 8,
            min_size_for_parallel_chunks_mb: 200, // 200 MB
            parallel_chunk_size_bytes: 8 * 1024 * 1024, // 8 MB
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Root directory for storage
    pub root_dir: String,
    /// Chunk size for file processing
    pub chunk_size: usize,
    /// Threshold for parallel processing
    pub parallel_threshold: usize,
    /// Retention days for files in the trash
    pub trash_retention_days: u32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            root_dir: "storage".to_string(),
            chunk_size: 1024 * 1024,      // 1 MB
            parallel_threshold: 100 * 1024 * 1024, // 100 MB
            trash_retention_days: 30,     // 30 days
        }
    }
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            // Updated connection string with default credentials that PostgreSQL often uses
            connection_string: "postgres://postgres:postgres@localhost:5432/oxicloud".to_string(),
            max_connections: 20,
            min_connections: 5,
            connect_timeout_secs: 10,
            idle_timeout_secs: 300,
            max_lifetime_secs: 1800,
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_expiry_secs: i64,
    pub refresh_token_expiry_secs: i64,
    pub hash_memory_cost: u32,
    pub hash_time_cost: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            // SECURITY: This default is intentionally insecure to force operators
            // to set OXICLOUD_JWT_SECRET in production. The from_env() method
            // will validate this and warn/panic if not configured.
            jwt_secret: String::new(),
            access_token_expiry_secs: 3600, // 1 hour
            refresh_token_expiry_secs: 2592000, // 30 days
            hash_memory_cost: 65536, // 64MB
            hash_time_cost: 3,
        }
    }
}

/// OpenID Connect (OIDC) configuration
#[derive(Debug, Clone)]
pub struct OidcConfig {
    /// Whether OIDC authentication is enabled
    pub enabled: bool,
    /// OIDC Issuer URL (e.g. https://authentik.example.com/application/o/oxicloud/)
    pub issuer_url: String,
    /// OIDC Client ID
    pub client_id: String,
    /// OIDC Client Secret
    pub client_secret: String,
    /// Redirect URI after OIDC authentication (must match IdP config)
    pub redirect_uri: String,
    /// OIDC scopes to request
    pub scopes: String,
    /// Frontend URL to redirect after successful OIDC login (tokens appended as fragment)
    pub frontend_url: String,
    /// Whether to auto-create users on first OIDC login (JIT provisioning)
    pub auto_provision: bool,
    /// Comma-separated list of OIDC groups that map to admin role
    pub admin_groups: String,
    /// Whether to disable password-based login entirely
    pub disable_password_login: bool,
    /// OIDC provider display name (shown in UI)
    pub provider_name: String,
}

impl Default for OidcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            issuer_url: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: "http://localhost:8086/api/auth/oidc/callback".to_string(),
            scopes: "openid profile email".to_string(),
            frontend_url: "http://localhost:8086".to_string(),
            auto_provision: true,
            admin_groups: String::new(),
            disable_password_login: false,
            provider_name: "SSO".to_string(),
        }
    }
}

impl OidcConfig {
    /// Load OIDC configuration from environment variables only
    pub fn from_env() -> Self {
        use std::env;
        let mut cfg = Self::default();
        if let Ok(v) = env::var("OXICLOUD_OIDC_ENABLED") {
            cfg.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_ISSUER_URL") { cfg.issuer_url = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_CLIENT_ID") { cfg.client_id = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_CLIENT_SECRET") { cfg.client_secret = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_REDIRECT_URI") { cfg.redirect_uri = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_SCOPES") { cfg.scopes = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_FRONTEND_URL") { cfg.frontend_url = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_AUTO_PROVISION") {
            cfg.auto_provision = v.parse::<bool>().unwrap_or(true);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_ADMIN_GROUPS") { cfg.admin_groups = v; }
        if let Ok(v) = env::var("OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN") {
            cfg.disable_password_login = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_PROVIDER_NAME") { cfg.provider_name = v; }
        cfg
    }
}

/// Feature configuration (feature flags)
#[derive(Debug, Clone)]
pub struct FeaturesConfig {
    pub enable_auth: bool,
    pub enable_user_storage_quotas: bool,
    pub enable_file_sharing: bool,
    pub enable_trash: bool,
    pub enable_search: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enable_auth: true,  // Enable authentication by default
            enable_user_storage_quotas: false,
            enable_file_sharing: true,  // Enable file sharing by default
            enable_trash: true,  // Enable trash feature
            enable_search: true, // Enable search feature
        }
    }
}

/// Global application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Storage directory path
    pub storage_path: PathBuf,
    /// Static files directory path
    pub static_path: PathBuf,
    /// Server port
    pub server_port: u16,
    /// Server host
    pub server_host: String,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Timeout configuration
    pub timeouts: TimeoutConfig,
    /// Resource configuration
    pub resources: ResourceConfig,
    /// Concurrency configuration
    pub concurrency: ConcurrencyConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Feature configuration
    pub features: FeaturesConfig,
    /// OIDC configuration
    pub oidc: OidcConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./storage"),
            static_path: PathBuf::from("./static"),
            server_port: 8085,
            server_host: "127.0.0.1".to_string(),
            cache: CacheConfig::default(),
            timeouts: TimeoutConfig::default(),
            resources: ResourceConfig::default(),
            concurrency: ConcurrencyConfig::default(),
            storage: StorageConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            features: FeaturesConfig::default(),
            oidc: OidcConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Use environment variables to override default values
        if let Ok(storage_path) = env::var("OXICLOUD_STORAGE_PATH") {
            config.storage_path = PathBuf::from(storage_path);
        }
            
        if let Ok(static_path) = env::var("OXICLOUD_STATIC_PATH") {
            config.static_path = PathBuf::from(static_path);
        }
            
        if let Ok(server_port) = env::var("OXICLOUD_SERVER_PORT") {
            if let Ok(port) = server_port.parse::<u16>() {
                config.server_port = port;
            }
        }
            
        if let Ok(server_host) = env::var("OXICLOUD_SERVER_HOST") {
            config.server_host = server_host;
        }
        
        // Database configuration
        if let Ok(connection_string) = env::var("OXICLOUD_DB_CONNECTION_STRING") {
            config.database.connection_string = connection_string;
        }
            
        if let Ok(max_connections) = env::var("OXICLOUD_DB_MAX_CONNECTIONS")
            .map(|v| v.parse::<u32>()) {
            if let Ok(val) = max_connections {
                config.database.max_connections = val;
            }
        }
            
        if let Ok(min_connections) = env::var("OXICLOUD_DB_MIN_CONNECTIONS")
            .map(|v| v.parse::<u32>()) {
            if let Ok(val) = min_connections {
                config.database.min_connections = val;
            }
        }
        
        // Auth configuration
        if let Ok(jwt_secret) = env::var("OXICLOUD_JWT_SECRET") {
            config.auth.jwt_secret = jwt_secret;
        }
        
        // SECURITY: Validate JWT secret when auth is enabled
        if config.features.enable_auth && config.auth.jwt_secret.is_empty() {
            // Generate a random secret for this session and warn loudly
            use rand_core::{OsRng, RngCore};
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);
            let generated_secret: String = key.iter().map(|b| format!("{:02x}", b)).collect();
            config.auth.jwt_secret = generated_secret;
            
            tracing::warn!("==========================================================");
            tracing::warn!("OXICLOUD_JWT_SECRET is not set.");
            tracing::warn!("A random secret has been generated for this session.");
            tracing::warn!("All tokens will be INVALIDATED on restart.");
            tracing::warn!("Set OXICLOUD_JWT_SECRET env var for production use.");
            tracing::warn!("==========================================================");
        }
            
        if let Ok(access_token_expiry) = env::var("OXICLOUD_ACCESS_TOKEN_EXPIRY_SECS")
            .map(|v| v.parse::<i64>()) {
            if let Ok(val) = access_token_expiry {
                config.auth.access_token_expiry_secs = val;
            }
        }
            
        if let Ok(refresh_token_expiry) = env::var("OXICLOUD_REFRESH_TOKEN_EXPIRY_SECS")
            .map(|v| v.parse::<i64>()) {
            if let Ok(val) = refresh_token_expiry {
                config.auth.refresh_token_expiry_secs = val;
            }
        }
        
        // Feature flags
        if let Ok(enable_auth) = env::var("OXICLOUD_ENABLE_AUTH")
            .map(|v| v.parse::<bool>()) {
            if let Ok(val) = enable_auth {
                config.features.enable_auth = val;
            }
        }
        
        if let Ok(enable_user_storage_quotas) = env::var("OXICLOUD_ENABLE_USER_STORAGE_QUOTAS")
            .map(|v| v.parse::<bool>()) {
            if let Ok(val) = enable_user_storage_quotas {
                config.features.enable_user_storage_quotas = val;
            }
        }
        
        if let Ok(enable_file_sharing) = env::var("OXICLOUD_ENABLE_FILE_SHARING")
            .map(|v| v.parse::<bool>()) {
            if let Ok(val) = enable_file_sharing {
                config.features.enable_file_sharing = val;
            }
        }
        
        if let Ok(enable_trash) = env::var("OXICLOUD_ENABLE_TRASH")
            .map(|v| v.parse::<bool>()) {
            if let Ok(val) = enable_trash {
                config.features.enable_trash = val;
            }
        }
        
        if let Ok(enable_search) = env::var("OXICLOUD_ENABLE_SEARCH")
            .map(|v| v.parse::<bool>()) {
            if let Ok(val) = enable_search {
                config.features.enable_search = val;
            }
        }
        
        // OIDC configuration
        if let Ok(v) = env::var("OXICLOUD_OIDC_ENABLED") {
            config.oidc.enabled = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_ISSUER_URL") {
            config.oidc.issuer_url = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_CLIENT_ID") {
            config.oidc.client_id = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_CLIENT_SECRET") {
            config.oidc.client_secret = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_REDIRECT_URI") {
            config.oidc.redirect_uri = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_SCOPES") {
            config.oidc.scopes = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_FRONTEND_URL") {
            config.oidc.frontend_url = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_AUTO_PROVISION") {
            config.oidc.auto_provision = v.parse::<bool>().unwrap_or(true);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_ADMIN_GROUPS") {
            config.oidc.admin_groups = v;
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN") {
            config.oidc.disable_password_login = v.parse::<bool>().unwrap_or(false);
        }
        if let Ok(v) = env::var("OXICLOUD_OIDC_PROVIDER_NAME") {
            config.oidc.provider_name = v;
        }

        // Validate OIDC config when enabled
        if config.oidc.enabled {
            if config.oidc.issuer_url.is_empty() || config.oidc.client_id.is_empty() || config.oidc.client_secret.is_empty() {
                tracing::error!("OIDC is enabled but OXICLOUD_OIDC_ISSUER_URL, OXICLOUD_OIDC_CLIENT_ID, or OXICLOUD_OIDC_CLIENT_SECRET are not set");
                config.oidc.enabled = false;
            }
        }

        config
    }
    
    pub fn with_features(mut self, features: FeaturesConfig) -> Self {
        self.features = features;
        self
    }
    
    pub fn db_enabled(&self) -> bool {
        self.features.enable_auth
    }
    
    pub fn auth_enabled(&self) -> bool {
        self.features.enable_auth
    }

    /// Build the public base URL for generating share links and other external URLs.
    ///
    /// Priority:
    /// 1. `OXICLOUD_BASE_URL` env var (used as-is)
    /// 2. If `server_host` already contains a scheme (`http://` or `https://`),
    ///    treat it as a full origin and do **not** prepend a scheme or append a port.
    /// 3. Otherwise, fall back to `http://{server_host}:{server_port}`.
    pub fn base_url(&self) -> String {
        if let Ok(explicit) = std::env::var("OXICLOUD_BASE_URL") {
            return explicit.trim_end_matches('/').to_string();
        }

        let host = self.server_host.trim_end_matches('/');

        if host.starts_with("http://") || host.starts_with("https://") {
            // The user already provided a full origin — use it directly.
            host.to_string()
        } else {
            format!("http://{}:{}", host, self.server_port)
        }
    }
}

/// Gets a default global configuration
pub fn default_config() -> AppConfig {
    AppConfig::default()
}