# Trash & Recycle Bin

OxiCloud provides a trash system that soft-deletes files and folders, allowing users to restore or permanently remove them.

## How It Works

1. When a file or folder is deleted, it's **soft-deleted** — a flag (`is_trashed`) is set and a `trashed_at` timestamp is recorded
2. Trashed items are hidden from normal file listings but remain on disk and in the database
3. Users can browse the trash, restore items, or permanently delete them
4. Items older than the retention period (default: **30 days**) are automatically purged

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/trash` | List trashed items |
| POST | `/api/trash/restore/{id}` | Restore a trashed item |
| DELETE | `/api/trash/{id}` | Permanently delete |
| DELETE | `/api/trash/empty` | Empty the entire trash |

## Deduplication Interaction

Permanent deletion decrements the blob reference count. If no other file points to the same blob, the blob is removed from disk.

## Feature Flag

Trash can be disabled via `OXICLOUD_ENABLE_TRASH=false`. When disabled, deletions are permanent.
