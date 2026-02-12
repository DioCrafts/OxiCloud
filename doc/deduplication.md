# 06 - Deduplication

OxiCloud uses **content-addressable deduplication** via SHA-256 hashing. Uploaded file content is hashed and stored in a central blob store. Identical files share the same blob, tracked by a reference counter. Disk savings scale with the number of duplicates.

Deduplication is always enabled and non-fatal -- if dedup fails, file operations proceed normally with a warning log.

## Architecture

```
User Files (references) ──▶ Dedup Index (hash→metadata) ──▶ Blob Store (actual data)
```

### Storage Layout

```
<storage_path>/
  .blobs/
    00/ .. ff/              ← 256 prefix directories (hex) for FS distribution
      <sha256>.blob         ← actual blob files
  .dedup_temp/              ← temp staging directory for atomic writes
  .dedup_index.json         ← persistent JSON index (hash → metadata)
```

### Layer Placement

| Layer | Component | File |
|---|---|---|
| Application Port | **DedupPort** trait + DTOs | `src/application/ports/dedup_ports.rs` |
| Infrastructure | **DedupService** implementation | `src/infrastructure/services/dedup_service.rs` |
| Interfaces | **DedupHandler** REST endpoints | `src/interfaces/api/handlers/dedup_handler.rs` |
| Integration | **FileUploadService** (dedup on upload) | `src/application/services/file_upload_service.rs` |
| Integration | **FileManagementService** (ref-count on delete) | `src/application/services/file_management_service.rs` |

## Constants

| Constant | Value | Description |
|---|---|---|
| `HASH_CHUNK_SIZE` | 256 KB (`256 * 1024`) | Chunk size for streaming SHA-256 computation |
| `MIN_DEDUP_SIZE` | 4 KB (`4096`) | Files below this size skip deduplication |

Hardcoded in `dedup_service.rs`. No runtime configuration beyond **storage_path**.

## Port: DedupPort Trait

Defined in `src/application/ports/dedup_ports.rs`:

```rust
#[async_trait]
pub trait DedupPort: Send + Sync + 'static {
    /// Store content from bytes, returning dedup result
    async fn store_bytes(&self, content: &[u8], content_type: Option<String>) -> Result<DedupResultDto, DomainError>;

    /// Store content from an existing file path
    async fn store_from_file(&self, source_path: &Path, content_type: Option<String>) -> Result<DedupResultDto, DomainError>;

    /// Check if a blob exists by hash
    async fn blob_exists(&self, hash: &str) -> bool;

    /// Get metadata for a blob
    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto>;

    /// Read blob content as Vec<u8>
    async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, DomainError>;

    /// Read blob content as Bytes
    async fn read_blob_bytes(&self, hash: &str) -> Result<Bytes, DomainError>;

    /// Increment reference count for a blob
    async fn add_reference(&self, hash: &str) -> Result<(), DomainError>;

    /// Decrement reference count; deletes blob if it reaches 0. Returns true if deleted.
    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError>;

    /// Compute SHA-256 hash of bytes (synchronous)
    fn hash_bytes(&self, content: &[u8]) -> String;

    /// Compute SHA-256 hash of file (streaming)
    async fn hash_file(&self, path: &Path) -> Result<String, DomainError>;

    /// Get deduplication statistics
    async fn get_stats(&self) -> DedupStatsDto;

    /// Persist index to disk
    async fn flush(&self) -> Result<(), DomainError>;

    /// Verify integrity of all blobs (existence, hash, size)
    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError>;
}
```

### Port DTOs

```rust
/// Result of a dedup store operation
pub enum DedupResultDto {
    NewBlob { hash: String, size: u64, blob_path: PathBuf },
    ExistingBlob { hash: String, size: u64, blob_path: PathBuf, saved_bytes: u64 },
}
// Methods: hash(), size(), blob_path(), was_deduplicated()

/// Metadata for a stored blob
pub struct BlobMetadataDto {
    pub hash: String,           // SHA-256 hex string
    pub size: u64,
    pub ref_count: u32,
    pub content_type: Option<String>,
}

/// Aggregate dedup statistics
pub struct DedupStatsDto {
    pub total_blobs: u64,
    pub total_bytes_stored: u64,
    pub total_bytes_referenced: u64,
    pub bytes_saved: u64,
    pub dedup_hits: u64,
    pub dedup_ratio: f64,
}
```

## Infrastructure: DedupService

Implemented in `src/infrastructure/services/dedup_service.rs`.

### Struct

```rust
pub struct DedupService {
    blob_root: PathBuf,                                  // <storage>/.blobs
    temp_root: PathBuf,                                  // <storage>/.dedup_temp
    index: Arc<RwLock<HashMap<String, BlobMetadata>>>,   // in-memory index
    index_path: PathBuf,                                 // <storage>/.dedup_index.json
    stats: Arc<RwLock<DedupStats>>,
}
```

### Key Methods

| Method | Description |
|---|---|
| `new(storage_root: &Path)` | Constructs paths, initializes empty index |
| `initialize()` | Creates `.blobs/` (256 prefix dirs), `.dedup_temp/`, loads index JSON |
| `blob_path(hash: &str)` | Returns `<blob_root>/<first2chars>/<hash>.blob` |
| `hash_bytes(content: &[u8])` | Static SHA-256 → hex string |
| `hash_file(path: &Path)` | Streaming SHA-256 of file (256 KB chunks) |
| `store_bytes(content, content_type)` | Hash → check existing → increment ref or write new blob atomically |
| `store_from_file(source_path, content_type)` | Hash file → dedup check → move file to blob store |
| `add_reference(hash)` | Increments **ref_count** + updates stats |
| `remove_reference(hash)` | Decrements **ref_count**. If 0, deletes blob file + removes from index |
| `read_blob(hash)` | Reads blob file content |
| `get_stats()` | Returns current dedup statistics |
| `flush()` | Saves index to JSON atomically (write to `.json.tmp` then rename) |
| `verify_integrity()` | Checks every blob: file exists, hash matches, size matches |
| `garbage_collect()` | Removes blobs with `ref_count == 0`. Returns `(deleted_count, deleted_bytes)` |

### Key Behaviors

- **Atomic writes**: new blobs are written to `.dedup_temp/<uuid>.tmp` then renamed into `.blobs/<prefix>/<hash>.blob`
- **Index persistence**: auto-saved every 100 new blobs, also saved explicitly via `flush()`
- **Small file bypass**: files < 4 KB skip deduplication
- **File move optimization**: `store_from_file` uses `fs::rename` to move the source file into the blob store (zero-copy on same filesystem)
- **Thread safety**: index and stats are protected by `Arc<RwLock<...>>`

## REST API Endpoints

All routes under `/api/dedup`, authentication required.

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/dedup/check/{hash}` | `DedupHandler::check_hash` | Check if a blob exists by SHA-256 hash |
| `POST` | `/api/dedup/upload` | `DedupHandler::upload_with_dedup` | Multipart upload with automatic dedup |
| `GET` | `/api/dedup/stats` | `DedupHandler::get_stats` | Get deduplication statistics |
| `GET` | `/api/dedup/blob/{hash}` | `DedupHandler::get_blob` | Retrieve raw blob content by hash |
| `DELETE` | `/api/dedup/blob/{hash}` | `DedupHandler::remove_reference` | Decrement ref-count (deletes blob if 0) |
| `POST` | `/api/dedup/recalculate` | `DedupHandler::recalculate_stats` | Run integrity verification + refresh stats |

### API Response Types

**Hash Check** (`GET /api/dedup/check/{hash}`):
```json
{
  "exists": true,
  "hash": "a1b2c3d4...",
  "existing_size": 1048576,
  "ref_count": 3
}
```
- Validates 64-character hex format for the hash parameter
- `existing_size` and `ref_count` are omitted when `exists` is `false`

**Dedup Upload** (`POST /api/dedup/upload`):
```json
{
  "is_new": false,
  "hash": "a1b2c3d4...",
  "size": 1048576,
  "bytes_saved": 1048576,
  "ref_count": 2
}
```
- Returns `201 Created` for new blobs, `200 OK` for deduplicated content
- Accepts multipart form data

**Stats** (`GET /api/dedup/stats`):
```json
{
  "unique_blobs": 150,
  "total_references": 300,
  "bytes_saved": 524288000,
  "total_logical_bytes": 1073741824,
  "total_physical_bytes": 549453824,
  "dedup_ratio": 2.0,
  "savings_percentage": 48.8
}
```

**Get Blob** (`GET /api/dedup/blob/{hash}`):
- Returns raw blob content with `Content-Type` from metadata
- Adds `X-Dedup-Hash` response header

**Remove Reference** (`DELETE /api/dedup/blob/{hash}`):
```json
{
  "success": true,
  "deleted": true,
  "message": "Blob deleted (ref count reached 0)"
}
```

## Integration with File Upload

**FileUploadService** holds `dedup: Option<Arc<dyn DedupPort>>`.

During `smart_upload()`, dedup runs for **all upload tiers** (write-behind, buffered, streaming):

```rust
// Inside smart_upload() — dedup runs after data is collected
{
    let dedup_data: Vec<u8> = { /* combine all chunks */ };
    self.run_dedup(&dedup_data, &content_type).await;
}
```

The private `run_dedup` method:

```rust
async fn run_dedup(&self, data: &[u8], content_type: &str) {
    let Some(dedup) = &self.dedup else { return };
    match dedup.store_bytes(data, Some(content_type.to_string())).await {
        Ok(result) => { /* log new or dedup hit */ }
        Err(e) => { warn!("DEDUP: Failed to store in blob store: {}", e); }
    }
}
```

Dedup is non-fatal -- failures are only logged as warnings. The file upload always completes regardless of dedup outcome.

## Integration with File Deletion

**FileManagementService** holds `dedup_service: Option<Arc<dyn DedupPort>>`.

In `delete_with_cleanup()`:

1. **Compute content hash** -- reads file via **FileReadPort**, calls `dedup.hash_bytes(&content)`
2. **Delete file** -- tries trash (soft delete) first, falls back to permanent delete
3. **Decrement dedup ref-count** -- calls `dedup.remove_reference(hash)` which may delete the blob if **ref_count** reaches 0

```rust
// Private helpers in FileManagementService
async fn compute_content_hash(&self, id: &str) -> Option<String>
async fn decrement_dedup_ref(&self, hash: &str)
```

## DI Wiring

In `src/common/di.rs`:

```rust
// Initialization (in create_core_services)
let dedup_service = Arc::new(DedupService::new(&self.storage_path));
dedup_service.initialize().await?;

// Stored in CoreServices as:
pub struct CoreServices {
    pub dedup_service: Arc<dyn DedupPort>,
    // ...
}

// Injected into application services:
FileUploadService::new_full(... core.dedup_service.clone())
FileManagementService::new_full(... core.dedup_service.clone())
```

## Persistence

Deduplication uses a **file-based JSON index** (`<storage>/.dedup_index.json`), NOT a database table. The index is loaded into memory at startup and flushed to disk:

- Automatically every 100 new blobs
- Explicitly via `flush()`
- Uses atomic write (write to `.json.tmp` then rename) for crash safety

## Tests

Located at the bottom of `src/infrastructure/services/dedup_service.rs`:

| Test | Description |
|---|---|
| `test_dedup_identical_content` | Stores same content (>4KB) twice. Verifies second is deduplicated, hashes match, `stats.dedup_hits == 1` |
| `test_reference_counting` | Stores twice (ref_count=2), removes one ref (not deleted), removes second (blob deleted) |

## Client Usage Example

```bash
# 1. Check if file already exists by hash
HASH=$(sha256sum myfile.txt | cut -d' ' -f1)
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/dedup/check/$HASH"

# 2. Upload with dedup (if not exists)
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -F "file=@myfile.txt" \
  "https://oxicloud.example.com/api/dedup/upload"

# 3. Get dedup statistics
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/dedup/stats"

# 4. Retrieve blob content
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/dedup/blob/$HASH" -o output.bin

# 5. Recalculate stats with integrity check
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/dedup/recalculate"
```
