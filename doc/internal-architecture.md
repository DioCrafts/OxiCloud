# 01 - Internal Architecture

OxiCloud follows a **hexagonal (ports & adapters) architecture** organized in four layers:

```
Domain  →  Application  →  Infrastructure  →  Interfaces
```

All cross-layer dependencies point inward via trait-based ports. The DI container (**AppServiceFactory**) wires concrete implementations at startup.

---

## Dependency Injection Container

### AppServiceFactory

**File:** `src/common/di.rs`

```rust
pub struct AppServiceFactory {
    storage_path: PathBuf,
    locales_path: PathBuf,
    config: AppConfig,
}
```

Initialization order in `build_app_state()`:

1. **Core services** -- path, caches, ID mapping, thumbnail, write-behind, chunked upload, transcode, dedup, compression
2. **Repository services** -- folder repo (stub mediator first), then **FileSystemStorageMediator** (real), file repos, metadata cache, buffer pool
3. **Trash service** (if **enable_trash** enabled)
4. **Application services** -- folder, file upload/retrieval/management, search, i18n
5. **Share service** (if **enable_file_sharing** enabled)
6. **DB-dependent services** -- favorites, recent, storage usage, auth (via **auth_factory**)
7. **Preload** translations + metadata cache
8. **ZIP service** (needs file retrieval + folder service, wired last)
9. **Assemble AppState** + admin settings + CalDAV/CardDAV

### AppState (Global State)

```rust
pub struct AppState {
    pub core: CoreServices,
    pub repositories: RepositoryServices,
    pub applications: ApplicationServices,
    pub db_pool: Option<Arc<PgPool>>,
    pub auth_service: Option<AuthServices>,
    pub admin_settings_service: Option<Arc<AdminSettingsService>>,
    pub trash_service: Option<Arc<dyn TrashUseCase>>,
    pub share_service: Option<Arc<dyn ShareUseCase>>,
    pub favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    pub recent_service: Option<Arc<dyn RecentItemsUseCase>>,
    pub storage_usage_service: Option<Arc<dyn StorageUsagePort>>,
    pub calendar_service: Option<Arc<dyn StorageUseCase>>,
    pub contact_service: Option<Arc<dyn StorageUseCase>>,
    pub calendar_use_case: Option<Arc<dyn CalendarUseCase>>,
    pub addressbook_use_case: Option<Arc<dyn AddressBookUseCase>>,
    pub contact_use_case: Option<Arc<dyn ContactUseCase>>,
}
```

Builder pattern: `new()` → `with_database()` → `with_auth_services()` → `with_trash_service()` → ... → `for_routing()`. The `Default` impl uses stubs from `crate::common::stubs`.

### Service Groups

```rust
pub struct CoreServices {
    pub path_service: Arc<PathService>,
    pub file_content_cache: Arc<dyn ContentCachePort>,
    pub id_mapping_service: Arc<dyn IdMappingPort>,       // folder IDs
    pub file_id_mapping_service: Arc<IdMappingService>,   // file IDs (concrete)
    pub id_mapping_optimizer: Arc<IdMappingOptimizer>,
    pub thumbnail_service: Arc<dyn ThumbnailPort>,
    pub write_behind_cache: Arc<dyn WriteBehindCachePort>,
    pub chunked_upload_service: Arc<dyn ChunkedUploadPort>,
    pub image_transcode_service: Arc<dyn ImageTranscodePort>,
    pub dedup_service: Arc<dyn DedupPort>,
    pub compression_service: Arc<dyn CompressionPort>,
    pub zip_service: Arc<dyn ZipPort>,
    pub config: AppConfig,
}

pub struct RepositoryServices {
    pub folder_repository: Arc<dyn FolderStoragePort>,
    pub file_read_repository: Arc<dyn FileReadPort>,
    pub file_write_repository: Arc<dyn FileWritePort>,
    pub i18n_repository: Arc<dyn I18nService>,
    pub storage_mediator: Arc<dyn StorageMediator>,
    pub metadata_cache: Arc<FileMetadataCache>,
    pub trash_repository: Option<Arc<dyn TrashRepository>>,
}

pub struct ApplicationServices {
    pub folder_service_concrete: Arc<FolderService>,
    pub folder_service: Arc<dyn FolderUseCase>,
    pub file_upload_service: Arc<dyn FileUploadUseCase>,
    pub file_retrieval_service: Arc<dyn FileRetrievalUseCase>,
    pub file_management_service: Arc<dyn FileManagementUseCase>,
    pub file_use_case_factory: Arc<dyn FileUseCaseFactory>,
    pub i18n_service: Arc<I18nApplicationService>,
    pub trash_service: Option<Arc<dyn TrashUseCase>>,
    pub search_service: Option<Arc<dyn SearchUseCase>>,
    pub share_service: Option<Arc<dyn ShareUseCase>>,
    pub favorites_service: Option<Arc<dyn FavoritesUseCase>>,
    pub recent_service: Option<Arc<dyn RecentItemsUseCase>>,
}

pub struct AuthServices {
    pub token_service: Arc<dyn TokenServicePort>,
    pub auth_application_service: Arc<AuthApplicationService>,
}
```

---

## ID Mapping System

Maps bidirectionally between **filesystem StoragePaths** and **UUID identifiers**. Two separate instances exist: one for folders (`folder_ids.json`), one for files (`file_ids.json`).

### StoragePath (Domain Value Object)

**File:** `src/domain/services/path_service.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StoragePath {
    segments: Vec<String>,  // e.g., ["Mi Carpeta - admin", "file.txt"]
}
```

| Method | Description |
|---|---|
| `root()` | Empty path (storage root) |
| `from_string(path)` | Parse from `/`-delimited string |
| `join(segment)` | Append a segment |
| `file_name()` | Last segment |
| `parent()` | All segments except last |
| `to_string()` | Join segments with `/` |

### IdMappingPort (Application Port)

**File:** `src/application/ports/outbound.rs`

```rust
#[async_trait]
pub trait IdMappingPort: Send + Sync + 'static {
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError>;
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError>;
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError>;
    async fn remove_id(&self, id: &str) -> Result<(), DomainError>;
    async fn save_changes(&self) -> Result<(), DomainError>;
    // Default impls for PathBuf variants:
    async fn get_file_path(&self, file_id: &str) -> Result<PathBuf, DomainError>;
    async fn update_file_path(&self, file_id: &str, new_path: &PathBuf) -> Result<(), DomainError>;
}
```

### IdMappingService (Base Implementation)

**File:** `src/infrastructure/services/id_mapping_service.rs`

```rust
pub struct IdMappingService {
    map_path: PathBuf,                 // e.g., storage/file_ids.json
    id_map: RwLock<IdMap>,
    save_mutex: Mutex<()>,
    timeouts: TimeoutConfig,
    pending_save: RwLock<bool>,
}

struct IdMap {
    path_to_id: HashMap<String, String>,
    id_to_path: HashMap<String, String>,
    version: u32,
}
```

Operations:
- `get_or_create_id(path)` -- Read-lock first (cache hit). Write-lock on miss, generates `Uuid::new_v4()`.
- `save_pending_changes()` -- Atomic write: serialize → write `.tmp` file → rename over original (with retry).
- `new(map_path)` -- Loads from JSON, rebuilds inverse map if inconsistent.

Persistence format (`storage/file_ids.json`):
```json
{
  "path_to_id": { "/Mi Carpeta - admin/doc.pdf": "a1b2c3d4-..." },
  "id_to_path": { "a1b2c3d4-...": "/Mi Carpeta - admin/doc.pdf" },
  "version": 42
}
```

### IdMappingOptimizer (Cache Layer)

**File:** `src/infrastructure/services/id_mapping_optimizer.rs`

Wraps **IdMappingService** with an in-memory TTL cache:

| Parameter | Value |
|---|---|
| Max cache entries | 10,000 |
| TTL | 300 s (5 min) |
| Cleanup interval | 150 s (2.5 min) |
| Batch threshold | ≥ 20 queued items |
| Max concurrent batches | 2 (semaphore) |

```rust
pub struct IdMappingOptimizer {
    base_service: Arc<IdMappingService>,
    path_to_id_cache: RwLock<HashMap<String, (String, Instant)>>,
    id_to_path_cache: RwLock<HashMap<String, (String, Instant)>>,
    stats: RwLock<OptimizerStats>,
    batch_limiter: Semaphore,
    pending_batch: Mutex<BatchQueue>,
}
```

Lookup flow: check cache → if miss, queue request → trigger batch if ≥ 20 pending → fallback to **base_service** → update cache.

On `update_path` / `remove_id`, cache entries are invalidated first, then delegated.

Used only for folder ID mapping. File ID mapping uses the base **IdMappingService** directly.

---

## Path Service

**File:** `src/infrastructure/services/path_service.rs`

```rust
pub struct PathService {
    root_path: PathBuf,   // e.g., ./storage
}
```

### Path Resolution

| Method | Description |
|---|---|
| `resolve_path(storage_path)` | Appends **StoragePath** segments to **root_path** → absolute `PathBuf` |
| `to_storage_path(physical_path)` | Strips **root_path** prefix → **StoragePath** (returns `None` if outside root) |
| `create_file_path(folder, name)` | Combines folder path + filename |
| `is_direct_child(parent, child)` | Check parent-child relationship |
| `is_in_root(path)` | Verify path is within storage root |

### Path Validation

`validate_path(path)` rejects:
- Empty path segments
- Segments containing dangerous characters: `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`
- Segments starting with `.` (exception: `.well-known` for WebDAV/CalDAV/CardDAV)

### Trait Implementations

- **StoragePort** -- `resolve_path()`, `ensure_directory()` (validates first, then `fs::create_dir_all`), `file_exists()`, `directory_exists()`
- **StorageMediator** -- Simplified stub variant. Folder lookup methods return `NotFound`.

---

## Storage Mediator

Bridges folder IDs to filesystem paths by combining folder repository, path service, and ID mapping.

### StorageMediator Trait

**File:** `src/application/services/storage_mediator.rs`

```rust
#[async_trait]
pub trait StorageMediator: Send + Sync + 'static {
    async fn get_folder_path(&self, folder_id: &str) -> StorageMediatorResult<PathBuf>;
    async fn get_folder_storage_path(&self, folder_id: &str) -> StorageMediatorResult<StoragePath>;
    async fn get_folder(&self, folder_id: &str) -> StorageMediatorResult<Folder>;
    async fn file_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool>;
    async fn file_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool>;
    async fn folder_exists_at_path(&self, path: &Path) -> StorageMediatorResult<bool>;
    async fn folder_exists_at_storage_path(&self, storage_path: &StoragePath) -> StorageMediatorResult<bool>;
    fn resolve_path(&self, relative_path: &Path) -> PathBuf;
    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf;
    async fn ensure_directory(&self, path: &Path) -> StorageMediatorResult<()>;
    async fn ensure_storage_directory(&self, storage_path: &StoragePath) -> StorageMediatorResult<()>;
}
```

### FileSystemStorageMediator

```rust
pub struct FileSystemStorageMediator {
    pub folder_storage_port: Arc<dyn FolderStoragePort>,
    pub path_service: Arc<dyn StoragePort>,
    pub id_mapping: Arc<dyn IdMappingPort>,
}
```

Folder ID → filesystem path resolution:

```
folder_id  ──►  FolderStoragePort.get_folder(id)
               ──►  Folder entity
               ──►  IdMappingPort.get_path_by_id(folder.id())
               ──►  StoragePath
               ──►  StoragePort.resolve_path(storage_path)
               ──►  PathBuf (absolute)
```

A **StubStorageMediator** also exists (returns `/tmp` paths) for DI bootstrap before the real mediator is available.

---

## Session Management

### Session Entity

**File:** `src/domain/entities/session.rs`

```rust
pub struct Session {
    id: String,              // UUID v4
    user_id: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
    revoked: bool,
}
```

Constructors:
- `Session::new(user_id, refresh_token, ip_address, user_agent, expires_in_days)` -- generates UUID, panics if **user_id** or **refresh_token** empty
- `Session::from_raw(...)` -- for DB reconstruction

### SessionRepository (Domain Port)

**File:** `src/domain/repositories/session_repository.rs`

```rust
#[async_trait]
pub trait SessionRepository: Send + Sync + 'static {
    async fn create_session(&self, session: Session) -> SessionRepositoryResult<Session>;
    async fn get_session_by_id(&self, id: &str) -> SessionRepositoryResult<Session>;
    async fn get_session_by_refresh_token(&self, token: &str) -> SessionRepositoryResult<Session>;
    async fn get_sessions_by_user_id(&self, user_id: &str) -> SessionRepositoryResult<Vec<Session>>;
    async fn revoke_session(&self, session_id: &str) -> SessionRepositoryResult<()>;
    async fn revoke_all_user_sessions(&self, user_id: &str) -> SessionRepositoryResult<u64>;
    async fn delete_expired_sessions(&self) -> SessionRepositoryResult<u64>;
}
```

### SessionStoragePort (Application Port)

**File:** `src/application/ports/auth_ports.rs`

```rust
#[async_trait]
pub trait SessionStoragePort: Send + Sync + 'static {
    async fn create_session(&self, session: Session) -> Result<Session, DomainError>;
    async fn get_session_by_refresh_token(&self, token: &str) -> Result<Session, DomainError>;
    async fn revoke_session(&self, session_id: &str) -> Result<(), DomainError>;
    async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DomainError>;
}
```

### SessionPgRepository (Infrastructure)

**File:** `src/infrastructure/repositories/pg/session_pg_repository.rs`

```rust
pub struct SessionPgRepository {
    pool: Arc<PgPool>,
}
```

Implements both **SessionRepository** and **SessionStoragePort**. Uses `with_transaction()` helper for write operations. `create_session` also updates `auth.users.last_login_at` within the same transaction.

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS auth.sessions (
    id             VARCHAR(36) PRIMARY KEY,
    user_id        VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    refresh_token  TEXT NOT NULL UNIQUE,
    expires_at     TIMESTAMP WITH TIME ZONE NOT NULL,
    ip_address     TEXT,
    user_agent     TEXT,
    created_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked        BOOLEAN NOT NULL DEFAULT FALSE
);

-- Indexes
CREATE INDEX idx_sessions_user_id ON auth.sessions(user_id);
CREATE INDEX idx_sessions_refresh_token ON auth.sessions(refresh_token);
CREATE INDEX idx_sessions_expires_at ON auth.sessions(expires_at);
CREATE INDEX idx_sessions_active ON auth.sessions(user_id, revoked)
    WHERE NOT revoked AND is_session_active(expires_at);
```

### Auth Service

**File:** `src/application/services/auth_application_service.rs`

**AuthApplicationService** orchestrates authentication using:
- **UserStoragePort** -- user CRUD
- **SessionStoragePort** -- session lifecycle
- **PasswordHasherPort** -- Argon2id hashing
- **TokenServicePort** -- JWT generation/validation
- `RwLock<OidcState>` -- hot-reloadable OIDC configuration
- `Mutex<HashMap<String, PendingOidcFlow>>` -- in-flight OIDC login states

Wired by `auth_factory.rs`: **UserPgRepository** + **SessionPgRepository** + **Argon2PasswordHasher** + **JwtTokenService** → **AuthApplicationService**.

---

## File Use Case Factory

**File:** `src/application/services/file_use_case_factory.rs`

```rust
pub trait FileUseCaseFactory: Send + Sync + 'static {
    fn create_file_upload_use_case(&self) -> Arc<dyn FileUploadUseCase>;
    fn create_file_retrieval_use_case(&self) -> Arc<dyn FileRetrievalUseCase>;
    fn create_file_management_use_case(&self) -> Arc<dyn FileManagementUseCase>;
}
```

**AppFileUseCaseFactory** creates lightweight service instances with only **FileReadPort** / **FileWritePort**. The main DI-wired services use `*_full()` constructors that inject write-behind cache, dedup, content cache, and transcode ports for full optimization.

### File Operation Port Hierarchy

| Port | Key Methods |
|---|---|
| **FileUploadUseCase** | `upload_file()`, `smart_upload()` (returns **UploadStrategy**: `WriteBehind` <256KB, `Buffered` 256KB-1MB, `Streaming` ≥1MB), `create_file()`, `update_file()` |
| **FileRetrievalUseCase** | `get_file()`, `get_file_content()`, `get_file_stream()`, `get_file_optimized()` (write-behind → content-cache → WebP transcode → mmap → streaming), `get_file_range_stream()` |
| **FileManagementUseCase** | `move_file()`, `rename_file()`, `delete_file()`, `delete_with_cleanup()` (trash-first with dedup reference cleanup) |

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Interfaces Layer                       │
│   Axum Router → API Routes + Middleware (Auth, Compress)    │
└─────────────────────┬───────────────────────────────────────┘
                      │ Arc<AppState>
┌─────────────────────▼───────────────────────────────────────┐
│                    Application Layer                        │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────────────┐ │
│  │ FileUpload   │ │ FolderService│ │ AuthApplication     │ │
│  │ FileRetrieval│ │ SearchService│ │ AdminSettings       │ │
│  │ FileMgmt     │ │ I18nService  │ │ TrashService        │ │
│  └──────┬───────┘ └──────┬───────┘ └──────────┬──────────┘ │
│         │ Ports (traits)  │                     │            │
└─────────┼────────────────┼─────────────────────┼────────────┘
          │                │                     │
┌─────────▼────────────────▼─────────────────────▼────────────┐
│                  Infrastructure Layer                       │
│  ┌────────────────┐ ┌──────────────┐ ┌──────────────────┐  │
│  │ FileFsRead/    │ │ IdMapping    │ │ SessionPg        │  │
│  │ FileFsWrite    │ │ + Optimizer  │ │ UserPg           │  │
│  │ FolderFs       │ │ PathService  │ │ JwtTokenService  │  │
│  │ TrashFs        │ │ StorageMed.  │ │ Argon2Hasher     │  │
│  └────────────────┘ └──────────────┘ └──────────────────┘  │
│  ┌────────────────┐ ┌──────────────┐ ┌──────────────────┐  │
│  │ ContentCache   │ │ Thumbnail    │ │ WriteBehind      │  │
│  │ MetadataCache  │ │ Transcode    │ │ BufferPool       │  │
│  │ BufferPool     │ │ Dedup        │ │ Compression      │  │
│  └────────────────┘ └──────────────┘ └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                      Domain Layer                           │
│  Entities: File, Folder, Session, User, Calendar, Contact   │
│  Value Objects: StoragePath                                 │
│  Repository Traits: SessionRepository, ...                  │
│  Domain Errors                                              │
└─────────────────────────────────────────────────────────────┘
```
