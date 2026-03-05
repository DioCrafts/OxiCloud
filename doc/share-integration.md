# 15 - Share Integration

File and folder sharing via public links. Users generate access links that work even for people without accounts. Supports optional password protection, expiration, and granular permissions. Follows hexagonal architecture throughout.

## Domain Entities

**Share** (`src/domain/entities/share.rs`) -- the core entity representing a shared resource:

```rust
pub struct Share {
    pub id: String,                         // unique link identifier
    pub item_id: String,                    // ID of shared file or folder
    pub item_type: ShareItemType,           // File or Folder
    pub token: String,                      // unique token for public access
    pub password_hash: Option<String>,      // optional password hash
    pub expires_at: Option<u64>,            // optional expiration timestamp
    pub permissions: SharePermissions,      // granted permissions
    pub created_at: u64,                    // creation timestamp
    pub created_by: String,                 // creator user ID
    pub access_count: u64,                  // access counter
}

pub enum ShareItemType {
    File,
    Folder
}

pub struct SharePermissions {
    pub read: bool,     // read permission
    pub write: bool,    // write permission
    pub reshare: bool,  // re-share permission
}
```

The entity has methods to validate expiration, verify passwords, increment the access counter, and modify properties (permissions, password, expiration).

## Repository Interface

**ShareRepository** (`src/domain/repositories/share_repository.rs`) defines persistence operations:

```rust
#[async_trait]
pub trait ShareRepository: Send + Sync + 'static {
    async fn save(&self, share: &Share) -> Result<Share, ShareRepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Share, ShareRepositoryError>;
    async fn find_by_token(&self, token: &str) -> Result<Share, ShareRepositoryError>;
    async fn find_by_item(&self, item_id: &str, item_type: &ShareItemType) -> Result<Vec<Share>, ShareRepositoryError>;
    async fn update(&self, share: &Share) -> Result<Share, ShareRepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), ShareRepositoryError>;
    async fn find_by_user(&self, user_id: &str, offset: usize, limit: usize) -> Result<(Vec<Share>, usize), ShareRepositoryError>;
}
```

## Application Ports

**ShareUseCase** and **ShareStoragePort** (`src/application/ports/share_ports.rs`):

```rust
#[async_trait]
pub trait ShareUseCase: Send + Sync + 'static {
    async fn create_shared_link(&self, user_id: &str, dto: CreateShareDto) -> Result<ShareDto, DomainError>;
    async fn get_shared_link(&self, id: &str) -> Result<ShareDto, DomainError>;
    async fn get_shared_link_by_token(&self, token: &str) -> Result<ShareDto, DomainError>;
    async fn get_shared_links_for_item(&self, item_id: &str, item_type: &ShareItemType) -> Result<Vec<ShareDto>, DomainError>;
    async fn update_shared_link(&self, id: &str, dto: UpdateShareDto) -> Result<ShareDto, DomainError>;
    async fn delete_shared_link(&self, id: &str) -> Result<(), DomainError>;
    async fn get_user_shared_links(&self, user_id: &str, page: usize, per_page: usize) -> Result<PaginatedResponseDto<ShareDto>, DomainError>;
    async fn verify_shared_link_password(&self, token: &str, password: &str) -> Result<bool, DomainError>;
    async fn register_shared_link_access(&self, token: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ShareStoragePort: Send + Sync + 'static {
    async fn save_share(&self, share: &Share) -> Result<Share, DomainError>;
    async fn find_share_by_id(&self, id: &str) -> Result<Share, DomainError>;
    // ... other methods
}
```

## DTOs

**DTOs** (`src/application/dtos/share_dto.rs`):

```rust
pub struct CreateShareDto {
    pub item_id: String,
    pub item_type: String,
    pub password: Option<String>,
    pub expires_at: Option<u64>,
    pub permissions: Option<SharePermissionsDto>,
}

pub struct UpdateShareDto {
    pub password: Option<String>,
    pub expires_at: Option<u64>,
    pub permissions: Option<SharePermissionsDto>,
}

pub struct SharePermissionsDto {
    pub read: bool,
    pub write: bool,
    pub reshare: bool,
}

pub struct ShareDto {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub token: String,
    pub url: String,
    pub password_protected: bool,
    pub expires_at: Option<u64>,
    pub permissions: SharePermissionsDto,
    pub created_at: u64,
    pub created_by: String,
    pub access_count: u64,
}
```

## Application Service

**ShareService** (`src/application/services/share_service.rs`) implements the business logic:

```rust
pub struct ShareService {
    config: Arc<AppConfig>,
    share_repository: Arc<dyn ShareStoragePort>,
    file_repository: Arc<dyn FileReadPort>,
    folder_repository: Arc<dyn FolderStoragePort>,
}
```

Handles: shared element validation, permission management, unique link/token generation, password protection, expiration control, and access tracking.

## Infrastructure

**ShareFsRepository** (`src/infrastructure/repositories/share_fs_repository.rs`) persists share link metadata to a local JSON file:

```rust
pub struct ShareFsRepository {
    config: Arc<AppConfig>,
}

struct ShareRecord {
    id: String,
    item_id: String,
    item_type: String,
    token: String,
    password_hash: Option<String>,
    expires_at: Option<u64>,
    permissions_read: bool,
    permissions_write: bool,
    permissions_reshare: bool,
    created_at: u64,
    created_by: String,
    access_count: u64,
}
```

Stores share link records in a JSON file. Supports queries and updates, search by ID/token/user, and pagination.

> **Note:** This repository stores *share link metadata* only (tokens, permissions, expiration). The actual file/folder content is accessed via `FileReadPort` / `FolderStoragePort` which use the blob storage model (PostgreSQL metadata + DedupService blobs).

## API Handlers and Routes

**Handlers** (`src/interfaces/api/handlers/share_handler.rs`):

```rust
pub async fn create_shared_link(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Json(dto): Json<CreateShareDto>,
) -> impl IntoResponse {
    // ...
}

pub async fn get_shared_link(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // ...
}

pub async fn get_user_shares(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Query(query): Query<GetSharesQuery>,
) -> impl IntoResponse {
    // ...
}

pub async fn update_shared_link(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateShareDto>,
) -> impl IntoResponse {
    // ...
}

pub async fn delete_shared_link(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // ...
}

pub async fn access_shared_item(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    // ...
}

pub async fn verify_shared_item_password(
    State(share_use_case): State<Arc<dyn ShareUseCase>>,
    Path(token): Path<String>,
    Json(req): Json<VerifyPasswordRequest>,
) -> impl IntoResponse {
    // ...
}
```

**Routes** (`src/interfaces/api/routes.rs`):

```rust
// Private routes for managing shared links
let share_router = Router::new()
    .route("/", post(share_handler::create_shared_link))
    .route("/", get(share_handler::get_user_shares))
    .route("/{id}", get(share_handler::get_shared_link))
    .route("/{id}", put(share_handler::update_shared_link))
    .route("/{id}", delete(share_handler::delete_shared_link));

// Public routes for accessing shared links
let public_share_router = Router::new()
    .route("/{token}", get(share_handler::access_shared_item))
    .route("/{token}/verify", post(share_handler::verify_shared_item_password));

// Main router configuration
router
    .nest("/shares", share_router)       // private API: /api/shares/...
    .nest("/s", public_share_router);    // public API: /api/s/...
```

## System Integration

### Configuration

```rust
pub struct FeaturesConfig {
    // ...
    pub enable_file_sharing: bool,
    // ...
}
```

### Dependency Injection

The service is instantiated via **AppServiceFactory** in `src/common/di.rs` and injected into **AppState**:

```rust
// In AppServiceFactory::create_share_service()
pub fn create_share_service(&self, repos: &RepositoryServices)
    -> Option<Arc<dyn ShareUseCase>>
{
    if !self.config.features.enable_file_sharing {
        return None;
    }

    let share_repository = Arc::new(ShareFsRepository::new(
        Arc::new(self.config.clone())
    ));

    let password_hasher: Arc<dyn PasswordHasherPort> =
        Arc::new(Argon2PasswordHasher::new());

    let service = Arc::new(ShareService::new(
        Arc::new(self.config.clone()),
        share_repository,
        repos.file_read_repository.clone(),   // FileBlobReadRepository
        repos.folder_repository.clone(),       // FolderDbRepository
        password_hasher,
    ));

    Some(service)
}
```

## Workflows

### Creating a Shared Link

1. User selects a file or folder to share.
2. Frontend sends a POST to `/api/shares/` with details (optional password, expiration, permissions).
3. `ShareService.create_shared_link()` validates data and verifies the item exists.
4. A unique token and access URL are generated.
5. The link is saved to the repository.
6. The URL and link details are returned.

### Accessing a Shared Resource

1. Someone opens a shared link (e.g., `http://oxicloud.example/api/s/{token}`).
2. Backend checks: valid token, not expired, password-protected or not.
3. If password-protected, the user is prompted.
4. Access counter increments.
5. Resource metadata is returned for display in the UI.
6. The user can access content according to the granted permissions.

## Security

**Password Protection** -- passwords are stored as hashes, not plaintext. Currently uses a simple hash but the design supports stronger algorithms like bcrypt.

**Expiration Control** -- links can be configured to expire automatically. The system checks expiration before granting access.

**Permission Control** -- granular permission model (read, write, reshare). Each operation validates permissions before allowing the action.

## Error Handling

```rust
pub enum ShareServiceError {
    #[error("Share not found: {0}")]
    NotFound(String),
    
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    
    #[error("Share expired")]
    Expired,
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Invalid item type: {0}")]
    InvalidItemType(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}
```

HTTP status code mapping:
- `NotFound` -> HTTP 404
- `PasswordRequired` -> HTTP 401 + metadata
- `Expired` -> HTTP 410 Gone
- `AccessDenied` -> HTTP 403
- `ValidationError` -> HTTP 400

## Future Enhancements

1. **Notifications** -- alert users when their shared resources are accessed
2. **Activity Log** -- detailed audit trail of who accessed what and when
3. **Usage Limits** -- max access count or bandwidth per shared link
4. **Advanced Statistics** -- detailed metrics on shared resource usage
5. **Alternative Persistence** -- database or cloud storage backends (same interface)

## Technical Notes

- **Share metadata** is stored in a local JSON file via `ShareFsRepository`. This is separate from the 100% blob storage model used for file content.
- **File/folder lookups** during share access go through `FileReadPort` / `FolderStoragePort`, which read metadata from PostgreSQL and content from the DedupService blob store.
- **Scalability**: for higher load, share metadata could be migrated to PostgreSQL using the same hexagonal architecture (implement `ShareRepository` with PgPool).
- **Maintenance**: clear separation of concerns makes testing and maintenance straightforward.

The sharing feature is enabled via `OXICLOUD_ENABLE_FILE_SHARING` configuration flag.
