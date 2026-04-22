# Batch Operations

OxiCloud exposes batch endpoints for bulk file and folder operations under `/api/batch`. Batch requests reduce round-trips, run concurrently behind a semaphore, and return per-item success and failure details instead of aborting on the first error.

## What You Can Do

### File operations

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/api/batch/files/move` | Move multiple files into a target folder |
| `POST` | `/api/batch/files/copy` | Copy multiple files into a target folder |
| `POST` | `/api/batch/files/delete` | Delete multiple files |
| `POST` | `/api/batch/files/get` | Fetch metadata for multiple files |

### Folder operations

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/api/batch/folders/delete` | Delete multiple folders |
| `POST` | `/api/batch/folders/create` | Create multiple folders |
| `POST` | `/api/batch/folders/get` | Fetch metadata for multiple folders |
| `POST` | `/api/batch/folders/move` | Move multiple folders |

### Additional batch endpoints

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/api/batch/trash` | Trash multiple items in one request |
| `POST` | `/api/batch/download` | Build a batch download |

## Request Shapes

### File move or copy

```json
{
  "file_ids": ["id-1", "id-2", "id-3"],
  "target_folder_id": "folder-abc"
}
```

### Folder delete

```json
{
  "folder_ids": ["folder-1", "folder-2"],
  "recursive": true
}
```

### Folder create

```json
{
  "folders": [
    { "name": "Documents", "parent_id": null },
    { "name": "Photos", "parent_id": "folder-abc" }
  ]
}
```

## Response Format

All batch endpoints return the same envelope:

```json
{
  "successful": [
    { "id": "id-1" }
  ],
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

### Status codes

| Code | Meaning |
| --- | --- |
| `200 OK` or `201 Created` | Every operation succeeded |
| `206 Partial Content` | Some operations succeeded and some failed |
| `400 Bad Request` | Every operation failed |

## Concurrency Model

Batch work is coordinated by `BatchOperationService` and a `tokio::sync::Semaphore`. By default, OxiCloud caps concurrent work with `max_concurrent_files = 10` so large batches do not starve the rest of the application.

Individual failures are collected in the `failed` array. One bad item does not cancel the whole request unless the batch cannot start at all.

## Example

```bash
# Move three files into a folder
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"file_ids":["id-1","id-2","id-3"],"target_folder_id":"folder-abc"}' \
  "https://oxicloud.example.com/api/batch/files/move"

# Delete multiple folders recursively
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"folder_ids":["old-1","old-2"],"recursive":true}' \
  "https://oxicloud.example.com/api/batch/folders/delete"
```

## Related Pages

- [Search](/guide/search)
- [Trash & Recycle Bin](/guide/trash)
- [ZIP and Compression](/guide/zip-and-compression)