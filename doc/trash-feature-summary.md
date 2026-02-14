# 14 - Trash Feature

Soft-delete for files and folders. Items go to a per-user trash bin instead of being permanently removed. Configurable retention period with automatic cleanup.

## Architecture

Follows the hexagonal architecture:

1. **Domain Layer** (`/src/domain/`):
   - Entities: **TrashedItem** representing files and folders in the trash
   - Repository interfaces: **TrashRepository** defining trash management operations

2. **Application Layer** (`/src/application/`):
   - DTOs: **TrashedItemDto** for data transfer between layers
   - Ports: **TrashUseCase** defining available operations
   - Services: **TrashService** implementing the trash use cases

3. **Infrastructure Layer** (`/src/infrastructure/`):
   - Repositories: **TrashDbRepository** (PostgreSQL) — reads from `storage.trash_items` VIEW, manages soft-delete flags
   - Trash-related methods in file/folder repositories: `FileBlobWriteRepository::move_to_trash()`, `FolderDbRepository::move_to_trash()`, etc.
   - Services: **TrashCleanupService** for automatic cleanup of expired items

4. **Interface Layer** (`/src/interfaces/`):
   - API handlers: `trash_handler.rs` with HTTP endpoints for trash operations
   - Routes: updated `routes.rs` to include trash endpoints

## Storage Model

Trash uses a **soft-delete** model in PostgreSQL:

- Files and folders have `is_trashed` (BOOLEAN) and `trashed_at` (TIMESTAMPTZ) columns
- When an item is trashed, `is_trashed` is set to `TRUE` and `trashed_at` records the timestamp
- `original_parent_id` / `original_folder_id` stores the original location for restore
- The `storage.trash_items` VIEW provides a unified list of all trashed items (files + folders)
- No physical file movement occurs — blob content stays at `.blobs/{prefix}/{hash}.blob`
- Permanent deletion removes the DB row and decrements the blob reference counter

```sql
CREATE OR REPLACE VIEW storage.trash_items AS
    SELECT id, name, 'file' AS item_type, folder_id AS parent_id,
           user_id, size, mime_type, trashed_at, created_at
    FROM storage.files WHERE is_trashed = TRUE
  UNION ALL
    SELECT id, name, 'folder' AS item_type, parent_id,
           user_id, 0 AS size, NULL AS mime_type, trashed_at, created_at
    FROM storage.folders WHERE is_trashed = TRUE;
```

## Key Features

1. **Soft Deletion** — files and folders are flagged as trashed, not immediately deleted
2. **Per-User Trash** — each user has an isolated trash bin (filtered by `user_id`)
3. **Retention Policy** — items auto-delete after a configurable period
4. **Restoration** — items can be restored to their original location via `original_parent_id`/`original_folder_id`
5. **Permanent Deletion** — items can be permanently deleted before retention expires (removes DB row + decrements blob ref)
6. **Empty Trash** — wipe everything in the trash at once

## API Endpoints

- `GET /api/trash` or `GET /api/trash/` — list all items in the user's trash
- `DELETE /api/trash/files/:id` — move a file to trash
- `DELETE /api/trash/folders/:id` — move a folder to trash
- `POST /api/trash/:id/restore` — restore an item to its original location
- `DELETE /api/trash/:id` — permanently delete an item from trash
- `DELETE /api/trash/empty` — empty the entire trash bin

## Implementation Details

### TrashDbRepository

**File:** `src/infrastructure/repositories/pg/trash_db_repository.rs`

```rust
pub struct TrashDbRepository {
    pool: Arc<PgPool>,
    retention_days: u32,
}
```

Key methods:
- `get_trash_items(user_id)` — SELECT from `storage.trash_items` WHERE `user_id = $1`
- `clear_trash(user_id)` — DELETE from `storage.files` and `storage.folders` WHERE `is_trashed = TRUE AND user_id = $1`
- `get_expired_items()` — finds items where `trashed_at + retention_days < NOW()`

### TrashService

**File:** `src/application/services/trash_service.rs`

Constructor: `TrashService::new(trash_repo, file_read, file_write, folder_repo, retention_days)`

Orchestrates trash operations by delegating to the appropriate repository:
- Moving a file to trash → `FileBlobWriteRepository::move_to_trash()`
- Moving a folder to trash → `FolderDbRepository::move_to_trash()`
- Permanent deletion → removes DB row + calls `DedupService::decrement_ref()` to clean blob if unreferenced

### TrashCleanupService

**File:** `src/infrastructure/services/trash_cleanup_service.rs`

Background job that runs every 24 hours to permanently delete items past the retention period.

## Testing

1. **Unit Tests** — testing **TrashService**:
   - Move files and folders to trash
   - Restore items from trash
   - Permanent deletion
   - Empty trash operation

2. **Integration Tests** — Python script hitting the API endpoints:
   - End-to-end testing of all trash operations
   - Verification of move, list, restore, and delete behavior

## Configuration

- **OXICLOUD_ENABLE_TRASH**: enable/disable the trash feature via **FeaturesConfig** (default: true)
- **OXICLOUD_TRASH_RETENTION_DAYS**: days to keep items before automatic deletion (default: 30, via **StorageConfig**)
