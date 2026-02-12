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
   - Repositories: **TrashFsRepository** for filesystem-based trash storage
   - Trash-related methods in existing repositories: `FileWriteRepository::move_to_trash()`, `FolderRepository::move_to_trash()`, etc.
   - Services: **TrashCleanupService** for automatic cleanup of expired items

4. **Interface Layer** (`/src/interfaces/`):
   - API handlers: `trash_handler.rs` with HTTP endpoints for trash operations
   - Routes: updated `routes.rs` to include trash endpoints

## Key Features

1. **Soft Deletion** -- files and folders move to trash, not immediately deleted
2. **Per-User Trash** -- each user has an isolated trash bin
3. **Retention Policy** -- items auto-delete after a configurable period
4. **Restoration** -- items can be restored to their original location
5. **Permanent Deletion** -- items can be permanently deleted before retention expires
6. **Empty Trash** -- wipe everything in the trash at once

## API Endpoints

- `GET /api/trash` or `GET /api/trash/` -- list all items in the user's trash
- `DELETE /api/trash/files/:id` -- move a file to trash
- `DELETE /api/trash/folders/:id` -- move a folder to trash
- `POST /api/trash/:id/restore` -- restore an item to its original location
- `DELETE /api/trash/:id` -- permanently delete an item from trash
- `DELETE /api/trash/empty` -- empty the entire trash bin

## Testing

1. **Unit Tests** -- testing **TrashService**:
   - Move files and folders to trash
   - Restore items from trash
   - Permanent deletion
   - Empty trash operation

2. **Integration Tests** -- Python script hitting the API endpoints:
   - End-to-end testing of all trash operations
   - Verification of move, list, restore, and delete behavior

3. **Shell Script** -- for manual testing and demonstration

## Configuration

- **OXICLOUD_ENABLE_TRASH**: enable/disable the trash feature via **FeaturesConfig** (default: true)
- **OXICLOUD_TRASH_RETENTION_DAYS**: days to keep items before automatic deletion (default: 30, via **StorageConfig**)

## Implementation Details

1. **Physical File Storage** -- when items are trashed, they physically move to a `.trash` directory
2. **Metadata Storage** -- trashed item info stored in a separate database table or file
3. **User Isolation** -- trash items are isolated by user ID
4. **Automatic Cleanup** -- a background job runs periodically to clean up expired items
5. **Transaction Safety** -- operations are atomic with proper error handling

## Future Enhancements

1. **Trash Quotas** -- limit trash storage per user
2. **Batch Operations** -- trash, restore, or delete multiple items at once
3. **Storage Optimization** -- deduplication for trashed items
4. **Version Control** -- track file versions when moving to trash
5. **Scheduled Cleanup** -- let users configure custom retention periods
6. **Trash Monitoring** -- metrics and alerts for trash usage and cleanup
