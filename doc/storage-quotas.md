# 11 - Storage Quotas

Per-user storage quotas track disk usage and can limit how much storage each user consumes. Controlled by the feature flag **OXICLOUD_ENABLE_USER_STORAGE_QUOTAS** (default: `false`).

## Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **StorageUsagePort** trait | `src/application/ports/storage_ports.rs` |
| Application Service | **StorageUsageService** | `src/application/services/storage_usage_service.rs` |
| Admin API | `/api/admin/users/{id}/quota` | `src/interfaces/api/handlers/admin_handler.rs` |

## Port Trait

```rust
#[async_trait]
pub trait StorageUsagePort: Send + Sync + 'static {
    async fn update_user_storage_usage(&self, user_id: &str) -> Result<i64, DomainError>;
    async fn update_all_users_storage_usage(&self) -> Result<(), DomainError>;
}
```

## How Usage is Calculated

1. Look up the user's username by ID.
2. Find the user's home folder: `"Mi Carpeta - {username}"` (naming convention).
3. Recursively traverse all subfolders, summing file sizes.
4. Skip directory entries (`mime_type = "directory"` or `"application/directory"`).
5. Update `auth.users.storage_used` via **UserStoragePort**.

`update_all_users_storage_usage()` processes all users concurrently via `tokio::spawn`.

## Admin Quota Management

Admins set per-user quotas through the admin API:

```bash
# Set 10 GB quota for a user
curl -X PUT -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"quota_bytes": 10737418240}' \
  "https://oxicloud.example.com/api/admin/users/{user_id}/quota"
```

## Dashboard Stats

The admin dashboard (`GET /api/admin/dashboard`) includes quota-related metrics:

```json
{
  "quotas_enabled": true,
  "total_quota_bytes": 107374182400,
  "total_used_bytes": 53687091200,
  "storage_usage_percent": 50.0,
  "users_over_80_percent": 5,
  "users_over_quota": 1
}
```

## Configuration

```bash
# Enable storage quotas (default: false)
OXICLOUD_ENABLE_USER_STORAGE_QUOTAS=true
```

Part of **FeaturesConfig** in `src/common/config.rs`:

```rust
pub struct FeaturesConfig {
    pub enable_user_storage_quotas: bool,  // default: false
    // ...
}
```
