# Trash & Recycle Bin

OxiCloud provides a trash system that soft-deletes files and folders, allowing users to restore or permanently remove them.

## How It Works

1. When a file or folder is deleted, it's **soft-deleted** — a flag (`is_trashed`) is set and a `trashed_at` timestamp is recorded
2. Trashed items are hidden from normal file listings but remain on disk and in the database
3. Users can browse the trash, restore items, or permanently delete them
4. Items older than the retention period (default: **30 days**) are automatically purged

## Storage Model

- files and folders keep their original rows in PostgreSQL
- deletion into trash only flips soft-delete state and records the original parent location for restore
- blob content is not moved when an item enters the trash
- the unified `storage.trash_items` view is used to list trashed files and folders together

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/trash` | List trashed items |
| DELETE | `/api/trash/files/{id}` | Move a file to the trash |
| DELETE | `/api/trash/folders/{id}` | Move a folder to the trash |
| POST | `/api/trash/{id}/restore` | Restore a trashed item |
| DELETE | `/api/trash/{id}` | Permanently delete |
| DELETE | `/api/trash/empty` | Empty the entire trash |

## Deduplication Interaction

Permanent deletion decrements the blob reference count. If no other file points to the same blob, the blob is removed from disk.

## Feature Flag

Trash can be disabled via `OXICLOUD_ENABLE_TRASH=false`. When disabled, deletions are permanent.

Retention is controlled by `OXICLOUD_TRASH_RETENTION_DAYS`.
