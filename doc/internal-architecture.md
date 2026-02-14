# 01 - Internal Architecture

OxiCloud follows a **hexagonal (ports & adapters) architecture** organized in four layers:

```
Domain  →  Application  →  Infrastructure  →  Interfaces
```

All cross-layer dependencies point inward via trait-based ports. The DI container (**AppServiceFactory**) wires concrete implementations at startup.

---

## Storage Model: 100% Blob Storage

OxiCloud uses a **100% blob storage model** where:

- **File metadata** (name, folder, size, user, timestamps, trash status) is stored in **PostgreSQL** (`storage.files` table).
- **File content** is stored as content-addressed blobs via **DedupService** at `.blobs/{prefix}/{hash}.blob`.
- **Folder structure** is purely virtual — represented as rows in `storage.folders` (no filesystem directories per user).
- **Trash** is a soft-delete flag (`is_trashed`, `trashed_at`) on files and folders, exposed via `storage.trash_items` VIEW.

There are no filesystem-based ID mappings, no `folder_ids.json`/`file_ids.json`, and no storage mediator.

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

1. **Core services** — path, content cache, thumbnail, chunked upload, transcode, dedup, compression
2. **Repository services** — `FolderDbRepository`, `FileBlobReadRepository`, `FileBlobWriteRepository`, `TrashDbRepository` (all PgPool-backed)
3. **Trash service** (if **enable_trash** enabled)
4. **Application services** — folder, file upload/retrieval/management, search, i18n
5. **Share service** (if **enable_file_sharing** enabled)
6. **DB-dependent services** — favorites, recent, storage usage, auth (via **auth_factory**)
7. **Preload** translations
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
    pub thumbnail_service: Arc<dyn ThumbnailPort>,
    pub chunked_upload_service: Arc<dyn ChunkedUploadPort>,
    pub image_transcode_service: Arc<dyn ImageTranscodePort>,
    pub dedup_service: Arc<dyn DedupPort>,
    pub compression_service: Arc<dyn CompressionPort>,
    pub zip_service: Arc<dyn ZipPort>,
    pub config: AppConfig,
}

pub struct RepositoryServices {
    pub folder_repository: Arc<dyn FolderStoragePort>,
    pub folder_repo_concrete: Arc<FolderDbRepository>,
    pub file_read_repository: Arc<dyn FileReadPort>,
    pub file_write_repository: Arc<dyn FileWritePort>,
    pub i18n_repository: Arc<dyn I18nService>,
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

## Database Schema (Storage)

All file and folder metadata lives in the `storage` PostgreSQL schema:

```sql
CREATE SCHEMA IF NOT EXISTS storage;

CREATE TABLE storage.folders (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    parent_id     UUID REFERENCES storage.folders(id) ON DELETE CASCADE,
    user_id       VARCHAR(36) NOT NULL REFERENCES auth.users(id),
    is_trashed    BOOLEAN NOT NULL DEFAULT FALSE,
    trashed_at    TIMESTAMPTZ,
    original_parent_id UUID,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE storage.files (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    folder_id     UUID NOT NULL REFERENCES storage.folders(id) ON DELETE CASCADE,
    user_id       VARCHAR(36) NOT NULL REFERENCES auth.users(id),
    blob_hash     TEXT NOT NULL,
    size          BIGINT NOT NULL DEFAULT 0,
    mime_type     TEXT NOT NULL DEFAULT 'application/octet-stream',
    is_trashed    BOOLEAN NOT NULL DEFAULT FALSE,
    trashed_at    TIMESTAMPTZ,
    original_folder_id UUID,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE VIEW storage.trash_items AS
    SELECT id, name, 'file' AS item_type, folder_id AS parent_id,
           user_id, size, mime_type, trashed_at, created_at
    FROM storage.files WHERE is_trashed = TRUE
  UNION ALL
    SELECT id, name, 'folder' AS item_type, parent_id,
           user_id, 0 AS size, NULL AS mime_type, trashed_at, created_at
    FROM storage.folders WHERE is_trashed = TRUE;
```

---

## Repository Layer (Infrastructure)

All repositories use **PgPool** for metadata and **DedupService** for blob content.

### FolderDbRepository

**File:** `src/infrastructure/repositories/pg/folder_db_repository.rs`

```rust
pub struct FolderDbRepository {
    pool: Option<Arc<PgPool>>,
}
```

Implements `FolderRepository`. Uses recursive CTEs for path building, unique constraints for name dedup within parent, and soft-delete flags for trash operations.

Key methods: `create_folder`, `get_folder`, `get_folder_by_path`, `list_folders`, `rename_folder`, `move_folder`, `delete_folder`, `move_to_trash`, `restore_from_trash`, `create_home_folder`, `get_folder_user_id`.

`new_stub()` creates a pool-less instance for `AppState::default()`.

### FileBlobReadRepository

**File:** `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

```rust
pub struct FileBlobReadRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    folder_repo: Arc<FolderDbRepository>,
}
```

Implements `FileReadPort`. Reads metadata from `storage.files` and content from blob store via `dedup.read_blob()` / `read_blob_bytes()`.

Key methods: `get_file`, `list_files`, `get_file_content`, `get_file_stream`, `get_file_range_stream`, `get_file_mmap`, `get_file_path`, `get_parent_folder_id`.

### FileBlobWriteRepository

**File:** `src/infrastructure/repositories/pg/file_blob_write_repository.rs`

```rust
pub struct FileBlobWriteRepository {
    pool: Arc<PgPool>,
    dedup: Arc<dyn DedupPort>,
    folder_repo: Arc<FolderDbRepository>,
}
```

Implements `FileWritePort`. Stores content via `dedup.store_bytes()` (returns hash), then INSERTs metadata into `storage.files`.

Key methods: `save_file`, `save_file_from_stream`, `move_file`, `rename_file`, `delete_file`, `update_file_content`, `move_to_trash`, `restore_from_trash`, `delete_file_permanently`.

### TrashDbRepository

**File:** `src/infrastructure/repositories/pg/trash_db_repository.rs`

```rust
pub struct TrashDbRepository {
    pool: Arc<PgPool>,
    retention_days: u32,
}
```

Implements `TrashRepository`. Reads from `storage.trash_items` VIEW. `clear_trash` DELETEs rows where `is_trashed = TRUE`. `get_expired_items` checks `trashed_at` against the configured retention period.

---

## Path Service

**File:** `src/infrastructure/services/path_service.rs`

```rust
pub struct PathService {
    root_path: PathBuf,   // e.g., ./storage
}
```

Used for resolving storage root paths (blob storage directory, thumbnail paths, etc.). Not used for per-user folder resolution — that is handled by `FolderDbRepository` via PostgreSQL.

### StoragePath (Domain Value Object)

**File:** `src/domain/services/path_service.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StoragePath {
    segments: Vec<String>,
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

### Trait Implementations

- **StoragePort** — `resolve_path()`, `ensure_directory()`, `file_exists()`, `directory_exists()`

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
- `Session::new(user_id, refresh_token, ip_address, user_agent, expires_in_days)` — generates UUID, panics if **user_id** or **refresh_token** empty
- `Session::from_raw(...)` — for DB reconstruction

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

CREATE INDEX idx_sessions_user_id ON auth.sessions(user_id);
CREATE INDEX idx_sessions_refresh_token ON auth.sessions(refresh_token);
CREATE INDEX idx_sessions_expires_at ON auth.sessions(expires_at);
CREATE INDEX idx_sessions_active ON auth.sessions(user_id, revoked)
    WHERE NOT revoked AND is_session_active(expires_at);
```

### Auth Service

**File:** `src/application/services/auth_application_service.rs`

**AuthApplicationService** orchestrates authentication using:
- **UserStoragePort** — user CRUD
- **SessionStoragePort** — session lifecycle
- **PasswordHasherPort** — Argon2id hashing
- **TokenServicePort** — JWT generation/validation
- `RwLock<OidcState>` — hot-reloadable OIDC configuration
- `Mutex<HashMap<String, PendingOidcFlow>>` — in-flight OIDC login states

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

**AppFileUseCaseFactory** creates lightweight service instances with only **FileReadPort** / **FileWritePort**.

### File Operation Port Hierarchy

| Port | Key Methods |
|---|---|
| **FileUploadUseCase** | `upload_file()`, `smart_upload()` (returns **UploadStrategy**: `Buffered` <1MB, `Streaming` ≥1MB), `create_file()`, `update_file()` |
| **FileRetrievalUseCase** | `get_file()`, `get_file_content()`, `get_file_stream()`, `get_file_optimized()` (content-cache → WebP transcode → mmap → streaming), `get_file_range_stream()` |
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
│  │ FileBlobRead   │ │ PathService  │ │ SessionPg        │  │
│  │ FileBlobWrite  │ │ DedupService │ │ UserPg           │  │
│  │ FolderDb       │ │ Thumbnail    │ │ JwtTokenService  │  │
│  │ TrashDb        │ │ Transcode    │ │ Argon2Hasher     │  │
│  └────────────────┘ └──────────────┘ └──────────────────┘  │
│  ┌────────────────┐ ┌──────────────┐ ┌──────────────────┐  │
│  │ ContentCache   │ │ Compression  │ │ ChunkedUpload    │  │
│  │ BufferPool     │ │ ZipService   │ │ ShareFsRepo      │  │
│  └────────────────┘ └──────────────┘ └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                      Domain Layer                           │
│  Entities: File, Folder, Session, User, Calendar, Contact   │
│  Value Objects: StoragePath                                 │
│  Repository Traits: FolderRepository, TrashRepository, ...  │
│  Domain Errors                                              │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow: File Upload

```
HTTP Request (multipart)
  → FileUploadService.smart_upload()
    → FileBlobWriteRepository.save_file() / save_file_from_stream()
      → DedupService.store_bytes() → .blobs/{prefix}/{hash}.blob
      → INSERT INTO storage.files (name, folder_id, blob_hash, size, ...)
  → 201 Created (FileDto)
```

### Data Flow: File Download

```
HTTP Request (GET /api/files/{id}/download)
  → FileRetrievalService.get_file_optimized()
    → ContentCache hit? → serve from RAM
    → FileBlobReadRepository.get_file_content() / get_file_stream()
      → SELECT blob_hash FROM storage.files WHERE id = $1
      → DedupService.read_blob(hash) → bytes from .blobs/
    → Optional WebP transcode → response
```

### Data Flow: Folder Operations

```
HTTP Request (POST /api/folders)
  → FolderService.create_folder()
    → FolderDbRepository.create_folder()
      → INSERT INTO storage.folders (name, parent_id, user_id, ...)
  → 201 Created (FolderDto)
```
