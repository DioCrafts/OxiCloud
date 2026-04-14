# 06 - Deduplication

OxiCloud uses **content-defined chunking (CDC)** via FastCDC for sub-file deduplication. Files are split into variable-size chunks (64 KB – 1 MB, average 256 KB) using the FastCDC 2020 algorithm. Each chunk is individually BLAKE3-hashed and stored in a pluggable blob backend (local FS, S3, Azure). A PostgreSQL *manifest* maps the whole-file BLAKE3 hash to the ordered list of chunk hashes that compose it. Identical chunks across any files are stored once and reference-counted.

Deduplication is always enabled and non-fatal — if dedup fails, file operations proceed normally with a warning log.

**Backward compatibility**: files uploaded before CDC (legacy whole-file blobs in `storage.blobs`) are served transparently. When no manifest row exists for a hash, the service falls back to direct blob reads.

## Architecture

```
┌─────────────────┐     ┌─────────────────────┐     ┌───────────────┐
│ storage.files   │────▶│ chunk_manifests      │────▶│ storage.blobs │──▶ Blob Store
│ (references)    │     │ (file→[chunk_hashes])│     │ (chunks)      │    (Local/S3/Azure)
└─────────────────┘     └─────────────────────┘     └───────────────┘
```

### Database Tables

| Table | Schema | Purpose |
|---|---|---|
| `storage.chunk_manifests` | `storage` | Maps file_hash → ordered chunk_hashes[] + chunk_sizes[] + ref_count |
| `storage.blobs` | `storage` | Per-chunk metadata: hash (PK), size, ref_count, content_type |

Defined in `migrations/20260414000000_chunk_manifests.sql`. Blobs table is part of the initial schema.

### Layer Placement

| Layer | Component | File |
|---|---|---|
| Application Port | **DedupPort** trait + DTOs | `src/application/ports/dedup_ports.rs` |
| Application Port | **BlobStorageBackend** trait | `src/application/ports/blob_storage_ports.rs` |
| Infrastructure | **DedupService** implementation | `src/infrastructure/services/dedup_service.rs` |
| Infrastructure | Blob backends (Local, S3, Azure, Retry, Encrypted, Cached, Migration) | `src/infrastructure/services/*_blob_backend.rs` |
| Interfaces | **DedupHandler** REST endpoints | `src/interfaces/api/handlers/dedup_handler.rs` |
| Integration | **FileBlobWriteRepository** (dedup on upload) | `src/infrastructure/repositories/pg/file_blob_write_repository.rs` |
| Integration | **FileBlobReadRepository** (dedup reads) | `src/infrastructure/repositories/pg/file_blob_read_repository.rs` |

## Constants

| Constant | Value | Description |
|---|---|---|
| `CDC_MIN_CHUNK` | 64 KB (`65_536`) | Minimum CDC chunk size |
| `CDC_AVG_CHUNK` | 256 KB (`262_144`) | Average / target CDC chunk size |
| `CDC_MAX_CHUNK` | 1 MB (`1_048_576`) | Maximum CDC chunk size |
| `CHUNK_UPLOAD_CONCURRENCY` | 8 | Maximum parallel chunk uploads to blob backend |

Hardcoded in `dedup_service.rs`.

## Write Path: `store_from_file`

The core write operation follows a **write-first strategy** that never holds a PG connection during disk I/O:

```
store_from_file(source_path, content_type, pre_computed_hash)
  │
  ├─ Fast path: pre_computed_hash provided?
  │    └─ try_dedup_hit() → check manifest + legacy blob
  │         └─ Hit? → bump ref_count, delete source, return ExistingBlob
  │
  ├─ CDC analysis (single mmap pass, spawn_blocking):
  │    ├─ Memory-map the file (memmap2)
  │    ├─ FastCDC 2020 boundary detection → ChunkMeta[]
  │    └─ BLAKE3 whole-file hash (concurrent with chunking)
  │
  ├─ Second dedup check with computed hash (if no pre_computed_hash)
  │
  ├─ store_chunks() — 3-phase pipeline:
  │    │
  │    ├─ Phase 0: Batch-check existing chunks (single PG query)
  │    │    SELECT hash FROM storage.blobs WHERE hash = ANY($1)
  │    │    → HashSet<String> of already-stored chunk hashes
  │    │
  │    ├─ Phase 1: Selective disk read (sequential, one pass)
  │    │    For each chunk:
  │    │      existing? → skip read (None)
  │    │      new?      → seek + read_exact → Some(Bytes)
  │    │
  │    └─ Phase 2: Parallel operations (buffer_unordered × 8)
  │         new chunk:      put_blob_from_bytes + INSERT ON CONFLICT
  │         existing chunk: UPDATE ref_count + 1 (no disk I/O)
  │
  ├─ INSERT manifest into storage.chunk_manifests
  │    (file_hash, chunk_hashes[], chunk_sizes[], total_size, chunk_count)
  │
  └─ Delete source file, return NewBlob { hash, size }
```

### Dedup Skip Optimization

The **biggest I/O saving** for versioned files. Before reading any chunk from disk or uploading it to the blob backend, `store_chunks` batch-queries PG to discover which chunk hashes already exist:

```sql
SELECT hash FROM storage.blobs WHERE hash = ANY($1)
```

This single round-trip returns all known chunks. For each existing chunk, the service skips:
- `seek()` + `read_exact()` from the source file (no disk I/O)
- `put_blob_from_bytes()` to the backend (no network I/O for S3/Azure)

Only a lightweight `UPDATE ref_count + 1` is executed in PG (~0.1 ms per chunk).

**Impact**: for a 100 MB versioned file where 95% of chunks are unchanged, only ~5 MB is read from disk and uploaded. The remaining 95% costs only PG ref-count bumps.

### Parallel Chunk Storage

Phase 2 of `store_chunks` uses `futures::stream::buffer_unordered(8)` to execute up to 8 concurrent chunk operations. This is a major win for S3/Azure backends where each PUT has 50-200 ms of network latency.

Chunk order in the returned `(chunk_hashes, chunk_sizes)` is preserved by deriving both from the original `ChunkMeta` slice (CDC order), not from the unordered parallel results.

### Full-File Dedup Hit (Fast Path)

When a file with the exact same BLAKE3 hash already has a manifest, `try_dedup_hit` returns immediately:
- Bumps `chunk_manifests.ref_count`
- Deletes the source file
- Returns `ExistingBlob` — **zero chunk I/O**

Also checks legacy whole-file blobs in `storage.blobs` for backward compatibility.

## Read Path

### Streaming Read (`read_blob_stream`)

CDC-aware with legacy fallback:

1. Query `chunk_manifests` for `chunk_hashes[]`
2. If found: stream chunks in order via `backend.get_blob_stream(chunk_hash)`, concatenated into a single byte stream with `buffered(1) + try_flatten`
3. If not found: fall back to `backend.get_blob_stream(hash)` for legacy blobs

### Range Read (`read_blob_range_stream`)

For HTTP Range requests (and WOPI/WebDAV partial reads):

1. Query manifest for `chunk_hashes[]`, `chunk_sizes[]`, `total_size`
2. Calculate which chunks overlap `[start, end)` using cumulative offsets
3. For each overlapping chunk, compute the sub-range within that chunk
4. Stream only the relevant chunk portions via `backend.get_blob_range_stream()`

### Blob Size (`blob_size`)

Returns `total_size` from the manifest (O(1) PG lookup). Falls back to `backend.blob_size()` for legacy blobs. Used by HEAD requests for Content-Length.

## Reference Counting

### Adding References (`add_reference`)

Manifest-aware with legacy fallback:
1. Try `UPDATE chunk_manifests SET ref_count = ref_count + 1 WHERE file_hash = $1`
2. If no rows affected, try `UPDATE storage.blobs SET ref_count + 1 WHERE hash = $1`
3. If neither exists, return NotFound error

### Removing References (`remove_reference`)

**CDC manifest path** (transactional):
1. Check `chunk_manifests` for the file hash
2. If `ref_count > 1`: decrement manifest ref_count → commit
3. If `ref_count == 1` (last reference):
   - `SELECT ... FOR UPDATE` to lock the manifest row
   - `DELETE FROM chunk_manifests`
   - `UPDATE storage.blobs SET ref_count = ref_count - 1 WHERE hash = ANY(chunk_hashes)`
   - `DELETE FROM storage.blobs WHERE hash = ANY(chunk_hashes) AND ref_count <= 0 RETURNING hash`
   - Commit TX
   - Delete orphaned chunk blob files from backend (after commit)

**Legacy blob path** (transactional):
1. `SELECT ref_count, size FROM storage.blobs WHERE hash = $1 FOR UPDATE`
2. If `ref_count == 1`: `DELETE FROM storage.blobs` + delete blob file
3. If `ref_count > 1`: `UPDATE SET ref_count = ref_count - 1`

## Port: DedupPort Trait

Defined in `src/application/ports/dedup_ports.rs`:

```rust
pub trait DedupPort: Send + Sync + 'static {
    /// Store content with CDC deduplication (from file).
    async fn store_from_file(
        &self,
        source_path: &Path,
        content_type: Option<String>,
        pre_computed_hash: Option<String>,
    ) -> Result<DedupResultDto, DomainError>;

    /// Check if a blob exists by hash (manifest or legacy).
    async fn blob_exists(&self, hash: &str) -> bool;

    /// Get metadata for a blob.
    async fn get_blob_metadata(&self, hash: &str) -> Option<BlobMetadataDto>;

    /// Stream blob content — CDC-aware with legacy fallback.
    async fn read_blob_stream(&self, hash: &str)
        -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>, DomainError>;

    /// Stream a byte range — CDC-aware with legacy fallback.
    async fn read_blob_range_stream(&self, hash: &str, start: u64, end: Option<u64>)
        -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>, DomainError>;

    /// Get blob size without reading content.
    async fn blob_size(&self, hash: &str) -> Result<u64, DomainError>;

    /// Increment reference count (manifest-aware).
    async fn add_reference(&self, hash: &str) -> Result<(), DomainError>;

    /// Decrement reference count. Returns true if blob was deleted.
    async fn remove_reference(&self, hash: &str) -> Result<bool, DomainError>;

    /// Calculate BLAKE3 hash of a file (mmap + rayon).
    async fn hash_file(&self, path: &Path) -> Result<String, DomainError>;

    /// Get local filesystem path for a blob hash.
    fn blob_path(&self, hash: &str) -> PathBuf;

    /// Get deduplication statistics (computed from PG).
    async fn get_stats(&self) -> DedupStatsDto;

    /// Flush index to persistent storage (no-op for PG backend).
    async fn flush(&self) -> Result<(), DomainError>;

    /// Verify integrity of all stored blobs and manifests.
    async fn verify_integrity(&self) -> Result<Vec<String>, DomainError>;
}
```

### Port DTOs

```rust
/// Result of a dedup store operation.
pub enum DedupResultDto {
    NewBlob { hash: String, size: u64 },
    ExistingBlob { hash: String, size: u64, saved_bytes: u64 },
}
// Methods: hash(), size(), was_deduplicated()

/// Metadata for a stored blob.
pub struct BlobMetadataDto {
    pub hash: String,           // BLAKE3 hex string
    pub size: u64,
    pub ref_count: u32,
    pub content_type: Option<String>,
}

/// Aggregate dedup statistics (computed from PG).
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
    backend: Arc<dyn BlobStorageBackend>,  // Pluggable blob storage (Local/S3/Azure/...)
    pool: Arc<PgPool>,                     // Primary pool (request-path operations)
    maintenance_pool: Arc<PgPool>,         // Isolated pool (verify_integrity, GC)
}
```

### Key Methods

| Method | Description |
|---|---|
| `new(backend, pool, maintenance_pool)` | Construct — wires pluggable backend + dual PG pools |
| `initialize()` | Initialize backend + log blob/manifest counts from PG |
| `cdc_hash_and_chunk_file(path)` | Single mmap pass: BLAKE3 whole-file hash + FastCDC chunk boundaries + per-chunk BLAKE3 |
| `cdc_chunk_file(path)` | CDC without whole-file hash (when hash is pre-computed) |
| `hash_file(path)` | BLAKE3 hash via mmap + rayon parallelism |
| `store_from_file(path, ct, hash)` | CDC → store_chunks → manifest INSERT (main write path) |
| `try_dedup_hit(hash, path)` | Check manifest/legacy for full-file dedup hit |
| `store_chunks(path, chunks)` | 3-phase: batch-check → selective read → parallel upload |
| `blob_exists(hash)` | Check manifest + legacy blob existence |
| `user_owns_blob_reference(hash, user_id)` | Authorization: check file ownership |
| `get_blob_metadata(hash)` | Manifest-aware metadata with legacy fallback |
| `add_reference(hash)` | Manifest-aware ref_count increment |
| `remove_reference(hash)` | Manifest-aware ref_count decrement + cascade cleanup |
| `read_blob_stream(hash)` | CDC chunk-streaming with legacy fallback |
| `read_blob_range_stream(hash, start, end)` | CDC range-streaming with legacy fallback |
| `blob_size(hash)` | O(1) from manifest, fallback to backend |
| `get_stats()` | Compute stats from PG (blobs + manifests) |
| `verify_integrity()` | Verify manifests (counts, sizes) + blobs (existence, size, re-hash) |
| `garbage_collect()` | Batch-delete orphaned manifests/blobs (uses maintenance pool) |

### Key Behaviors

- **CDC analysis in `spawn_blocking`**: mmap + FastCDC runs off the async runtime to avoid blocking the event loop
- **Dual PG pools**: request-path operations use the primary pool; `verify_integrity` and `garbage_collect` use the maintenance pool to prevent starvation
- **Pluggable blob backend**: all chunk I/O goes through `Arc<dyn BlobStorageBackend>` — works with local FS, S3, Azure, or any composed backend (retry, encryption, caching)
- **Atomic chunk storage**: `put_blob_from_bytes` is idempotent; `INSERT ON CONFLICT` handles concurrent uploads of the same chunk
- **Delete-after-commit**: blob files are deleted from the backend only after the PG transaction commits, preventing orphaned PG rows
- **Flush is no-op**: PG handles durability via WAL/commit — no explicit index persistence needed

## REST API Endpoints

All routes under `/api/dedup`, authentication required.

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/dedup/check/{hash}` | `DedupHandler::check_hash` | Check if a blob exists by BLAKE3 hash |
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

## DI Wiring

In `src/common/di.rs`:

```rust
// Blob backend is assembled from layered backends:
// LocalBlobBackend / S3BlobBackend / AzureBlobBackend
// → RetryBlobBackend → EncryptedBlobBackend → CachedBlobBackend
let blob_backend: Arc<dyn BlobStorageBackend> = /* ... */;

// DedupService receives the composed blob backend + dual PG pools
let dedup_service = Arc::new(DedupService::new(
    blob_backend,
    db_pool.clone(),
    maintenance_pool.clone(),
));
dedup_service.initialize().await?;

// Stored in CoreServices as:
pub struct CoreServices {
    pub dedup_service: Arc<dyn DedupPort>,
    // ...
}

// Injected into blob repositories:
FileBlobReadRepository::new(pool, core.dedup_service.clone(), folder_repo)
FileBlobWriteRepository::new(pool, core.dedup_service.clone(), folder_repo)
```

## Maintenance

### Garbage Collection (`garbage_collect`)

Two-phase batch deletion using the maintenance pool:

1. **Phase 1 — Orphaned manifests**: `DELETE FROM chunk_manifests WHERE ref_count <= 0` (batches of 500). For each deleted manifest, `UPDATE storage.blobs SET ref_count = ref_count - 1` for its chunks.
2. **Phase 2 — Orphaned blobs**: `DELETE FROM storage.blobs WHERE ref_count <= 0` (batches of 500). Deletes blob files from backend + thumbnail cleanup (best-effort).

Uses `tokio::task::yield_now()` between batches to avoid starving other tasks.

### Integrity Verification (`verify_integrity`)

Phase 1 — Verify CDC manifests:
- `chunk_hashes.len() == chunk_sizes.len()`
- `SUM(chunk_sizes) == total_size`
- Every referenced chunk exists in the blob backend with correct size

Phase 2 — Verify blobs (chunks + legacy):
- Blob file exists in backend
- Actual size matches PG record
- (Local backends only) Re-hash file content to verify BLAKE3 integrity
- Processes 16 blobs concurrently via `buffer_unordered`

## Tests

Located at the bottom of `src/infrastructure/services/dedup_service.rs` (12 tests):

| Test | Description |
|---|---|
| `test_cdc_deterministic_same_content` | Same content → same file hash + same chunk hashes/offsets/lengths |
| `test_cdc_empty_file` | Empty file → zero chunks, correct BLAKE3 empty hash |
| `test_cdc_small_file_single_chunk` | File below min chunk → single chunk covering entire file |
| `test_cdc_chunk_sizes_within_bounds` | All non-last chunks are within [64 KB, 1 MB] |
| `test_cdc_file_hash_matches_hash_file` | CDC whole-file hash matches standalone `hash_file()` |
| `test_cdc_chunk_hashes_are_correct` | Each chunk hash == BLAKE3 of that chunk's data |
| `test_cdc_reassembly_matches_original` | Concatenating chunks reproduces original file |
| `test_cdc_chunks_are_contiguous` | Chunks cover entire file with no gaps or overlaps |
| `test_cdc_similar_files_share_chunks` | Editing last 64 KB of 2 MB file → most chunks shared |
| `test_cdc_chunk_file_matches_full` | `cdc_chunk_file` produces same chunks as `cdc_hash_and_chunk_file` |
| `test_cdc_large_file_chunk_count` | 8 MB file produces 8-128 chunks (avg ~256 KB) |
| `test_cdc_insert_at_beginning_preserves_later_chunks` | 128 KB prefix insert → CDC resynchronizes, later chunks shared |

## Performance Characteristics

| Scenario | Behavior |
|---|---|
| **First upload of new file** | Single mmap pass (CDC + hash) → parallel chunk upload → manifest INSERT |
| **Re-upload of identical file** | `try_dedup_hit` → manifest ref_count bump → zero chunk I/O |
| **Upload of edited file (5% changed)** | CDC → batch-check finds 95% existing → reads only 5% → uploads 5% → ref-bumps 95% |
| **Range read (1 MB from 1 GB file)** | Manifest lookup → identify overlapping chunks → stream only those portions |
| **Delete last reference** | TX: delete manifest → batch-decrement chunks → delete zero-ref chunks → commit → delete blob files |
| **Garbage collection** | Maintenance pool, batches of 500, yields between batches |

## Client Usage Example

```bash
# 1. Check if file already exists by hash
HASH=$(b3sum myfile.txt | cut -d' ' -f1)
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
