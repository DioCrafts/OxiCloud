# 09 - Batch Operations

Batch endpoints let you perform bulk file and folder operations (move, copy, delete, get, create) in a single request. Operations run concurrently behind a configurable semaphore. Each response includes detailed per-item success/failure info.

## Architecture

| Layer | Component | File |
|---|---|---|
| Application Service | **BatchOperationService** | `src/application/services/batch_operations.rs` |
| Interfaces | `batch_handler` functions | `src/interfaces/api/handlers/batch_handler.rs` |

## REST API

All routes live under `/api/batch` and require authentication.

### File Operations

| Method | Path | Handler | Description |
|---|---|---|---|
| `POST` | `/api/batch/files/move` | `move_files_batch` | Move files to target folder |
| `POST` | `/api/batch/files/copy` | `copy_files_batch` | Copy files to target folder |
| `POST` | `/api/batch/files/delete` | `delete_files_batch` | Delete multiple files |
| `POST` | `/api/batch/files/get` | `get_files_batch` | Get metadata for multiple files |

### Folder Operations

| Method | Path | Handler | Description |
|---|---|---|---|
| `POST` | `/api/batch/folders/delete` | `delete_folders_batch` | Delete multiple folders |
| `POST` | `/api/batch/folders/create` | `create_folders_batch` | Create multiple folders |
| `POST` | `/api/batch/folders/get` | `get_folders_batch` | Get metadata for multiple folders |

## Request DTOs

### File Operations

```json
{
  "file_ids": ["id-1", "id-2", "id-3"],
  "target_folder_id": "folder-abc"      // required for move/copy
}
```

### Folder Delete

```json
{
  "folder_ids": ["folder-1", "folder-2"],
  "recursive": true,
  "target_folder_id": null
}
```

### Folder Create

```json
{
  "folders": [
    { "name": "Documents", "parent_id": null },
    { "name": "Photos", "parent_id": "folder-abc" }
  ]
}
```

## Response Format

Every batch endpoint returns the same structure:

```json
{
  "successful": [ ... ],
  "failed": [
    { "id": "bad-id", "error": "File not found" }
  ],
  "stats": {
    "total": 5,
    "successful": 4,
    "failed": 1,
    "execution_time_ms": 245
  }
}
```

### Status Codes

| Code | Meaning |
|---|---|
| `200 OK` / `201 Created` | All operations succeeded |
| `206 Partial Content` | Some succeeded, some failed |
| `400 Bad Request` | All operations failed |

## Concurrency

Operations run concurrently via `tokio::sync::Semaphore`, capped by **max_concurrent_files** (default: 10).

```rust
pub struct BatchOperationService {
    file_retrieval: Arc<dyn FileRetrievalUseCase>,
    file_management: Arc<dyn FileManagementUseCase>,
    folder_service: Arc<FolderService>,
    config: AppConfig,
    semaphore: Arc<Semaphore>,
}
```

## Error Handling

```rust
pub enum BatchOperationError {
    Domain(DomainError),                    // individual operation error
    Cancelled(String),                      // operation was cancelled
    ConcurrencyLimit(String),               // semaphore exhausted
    PartialFailure(String, usize, usize),   // message, success_count, fail_count
    Internal(String),
}
```

Individual failures do not abort the batch. They get collected in the `failed` array and the response returns `206 Partial Content`.

## Example

```bash
# Move 3 files to a folder
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"file_ids":["id-1","id-2","id-3"],"target_folder_id":"folder-abc"}' \
  "https://oxicloud.example.com/api/batch/files/move"

# Delete folders recursively
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"folder_ids":["old-1","old-2"],"recursive":true}' \
  "https://oxicloud.example.com/api/batch/folders/delete"

# Create multiple folders
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"folders":[{"name":"Docs","parent_id":null},{"name":"Photos","parent_id":"root-id"}]}' \
  "https://oxicloud.example.com/api/batch/folders/create"
```
