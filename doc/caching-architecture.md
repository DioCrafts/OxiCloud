# 04 - Caching Architecture

OxiCloud uses a multi-layer caching system spanning HTTP-level caching down to kernel-level memory mapping. Covers both uploads and downloads.

## Cache Layers Summary

```
┌─────────────────────────────────────────────────────┐
│  Layer 0: HTTP Cache Middleware (ETag + 304)         │  All endpoints
├─────────────────────────────────────────────────────┤
│  Layer 1: File Content Cache (LRU, <10MB files)     │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 2: MMAP (memmap2, 10-100MB blobs)            │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 3: Streaming (FramedRead, ≥100MB blobs)      │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 4: Buffer Pool (reusable I/O buffers)        │  Compression
└─────────────────────────────────────────────────────┘
```

> **Note:** File metadata (name, size, MIME type, folder) is served from PostgreSQL — no separate filesystem metadata cache is needed.

---

## Layer 0: HTTP Cache Middleware

**File**: `src/interfaces/middleware/cache.rs`

Generic HTTP caching layer applied to API endpoints.

| Parameter | Value |
|---|---|
| Max entries | 1,000 |
| Default max-age | 60 seconds |
| Eviction | LRU (oldest 10% when full) |
| Cleanup | Background task every 5 minutes |

Features:
- ETag-based conditional requests (`If-None-Match` → `304 Not Modified`)
- `Cache-Control` header injection
- Implements Tower `Layer` + `Service` traits for Axum integration
- Per-request key: method + URI

---

## Layer 1: File Content Cache (Download Tier 1)

**File**: `src/infrastructure/services/file_content_cache.rs`

In-memory LRU cache for small files, served directly from RAM.

| Parameter | Value |
|---|---|
| Max file size | 10 MB per file |
| Max total cache size | 512 MB |
| Max entries | 10,000 |
| Structure | `lru::LruCache<String, CacheEntry>` |
| Latency | ~0.1ms |

**CacheEntry**: `{ data: Bytes, etag: String, content_type: String, size: usize }`

Methods:
- `should_cache(size)` — checks if file fits in cache
- `get(file_id)` → `Option<(Bytes, String, String)>` — returns (data, etag, content_type)
- `put(file_id, content, etag, content_type)` — inserts with LRU eviction
- `invalidate(file_id)`, `clear()`
- `stats()` → `CacheStats { current_size_bytes, max_size_bytes, hits, misses, hit_rate_percent }`

Port: implements **ContentCachePort** trait.

---

## Layer 2: MMAP (Download Tier 2)

**File**: `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

Memory-mapped I/O for medium blobs using `memmap2`.

| Parameter | Value |
|---|---|
| File range | 10 MB - 100 MB |
| Implementation | `memmap2::Mmap` via `spawn_blocking` |
| Latency | ~1-5ms |

The blob file (`.blobs/{prefix}/{hash}.blob`) is memory-mapped and its contents copied to `Bytes`. Benefits from kernel page cache for frequently accessed blobs.

---

## Layer 3: Streaming (Download Tier 3)

**File**: `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

Chunked streaming for large blobs using tokio-util codecs.

| Parameter | Value |
|---|---|
| File range | ≥100 MB |
| Chunk size | 1 MB (configurable via **ResourceConfig.chunk_size_bytes**) |
| Implementation | `FramedRead` + `BytesCodec` |
| RAM usage | Near zero (one chunk at a time) |

---

## Layer 4: Buffer Pool

**File**: `src/infrastructure/services/buffer_pool.rs`

Reusable byte buffer pool to reduce allocation pressure during compression operations.

| Parameter | Value |
|---|---|
| Buffer size | 64 KB |
| Max buffers | 100 |
| Buffer TTL | 60 seconds |
| Concurrency control | `tokio::sync::Semaphore` |

Features:
- `get_buffer()` — borrows a buffer (blocks if pool exhausted)
- **BorrowedBuffer** auto-returns to pool on `Drop` via `tokio::spawn`
- Expired buffers are cleaned periodically via `start_cleaner()`
- Tracks stats: gets, hits, misses, returns, evictions, waits

---

## Configuration

All cache-related config in `src/common/config.rs`:

```rust
pub struct ResourceConfig {
    pub large_file_threshold_mb: u64,      // 100 MB (mmap→streaming boundary)
    pub chunk_size_bytes: usize,           // 1 MB (streaming chunk size)
    pub max_in_memory_file_size_mb: u64,   // 50 MB
}
```

## Download Flow

```
Request → ETag check (304?) → Range request (206?)
       → file size < 10MB?  → Tier 1: LRU cache (RAM)
       → file size < 100MB? → Tier 2: MMAP (kernel page cache on blob file)
       → file size ≥ 100MB  → Tier 3: Streaming (chunked from blob file)
```

In all tiers, metadata (file name, size, MIME type) comes from a PostgreSQL `SELECT` on `storage.files`. Content is read from the DedupService blob at `.blobs/{prefix}/{hash}.blob`.

---

## Range Requests (HTTP 206 Partial Content)

**Files**: `src/interfaces/api/handlers/file_handler.rs`, `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

**Crate**: `http-range-header = "0.4"` for parsing.

### Request Processing Flow

```
Range header present?
  ├─ parse_range_header(range_str)
  │    ├─ Parse OK → ranges.validate(file_size)
  │    │    ├─ Valid   → take first range → get_file_range_stream(start, end+1)
  │    │    │    ├─ Stream OK → 206 Partial Content
  │    │    │    └─ Stream Err → fall through to normal download (200)
  │    │    └─ Invalid → 416 Range Not Satisfiable
  │    └─ Parse Err → fall through to normal download (200)
  └─ No Range header → normal 3-tier download
```

### Response Headers (206)

| Header | Value |
|---|---|
| `Content-Type` | File MIME type |
| `Content-Range` | `bytes {start}-{end}/{total_size}` |
| `Content-Length` | Range length (end - start + 1) |
| `Accept-Ranges` | `bytes` |
| `ETag` | `"{file_id}-{modified_at}"` |
| `Cache-Control` | `private, max-age=3600, must-revalidate` |

### 416 Range Not Satisfiable

Returned when `ranges.validate(file_size)` fails:
```
HTTP/1.1 416 Range Not Satisfiable
Content-Range: bytes */12345
```

### Blob File Seek Implementation

`get_file_range_stream()` at the repository level:

1. Resolves blob path from `blob_hash` via DedupService
2. Opens the blob file with `TokioFile::open()`
3. Seeks to `start` via `fh.seek(SeekFrom::Start(start))`
4. Limits read to `range_length` via `fh.take(range_length)`
5. Wraps in `FramedRead` + `BytesCodec`

Adaptive chunk size:

| Range size | Chunk size |
|---|---|
| ≤ 1 MB | 8 KB |
| > 1 MB | 1 MB (from **ResourceConfig.chunk_size_bytes**) |

### Tier Interaction

Range requests **bypass all download tiers** (LRU, MMAP). They always use direct blob file seek + streaming. On stream creation error, the handler falls through to the normal `get_file_optimized()` 3-tier path.

### Limitations

- **Multipart ranges not supported**: only the first range in a multi-range request is served.
- **`If-Range` not handled**: no conditional range support.
- **`If-Modified-Since` not handled**: only `If-None-Match` (ETag) is checked.

---

## Upload Flow

```
Request → file size < 1MB?   → Buffered write (sync to blob store)
       → file size ≥ 1MB    → Streaming write (chunk-by-chunk to blob store)
```

### Upload Strategy Selection

**File**: `src/application/services/file_upload_service.rs`

```rust
pub enum UploadStrategy {
    Buffered,      // < 1 MB — collect bytes, write to blob store
    Streaming,     // ≥ 1 MB — chunk-by-chunk write via save_file_from_stream
}
```

| Constant | Value |
|---|---|
| `STREAMING_UPLOAD_THRESHOLD` | 1 MB |

### Handler-Level Buffering

Upload handlers buffer the multipart body in RAM as `Vec<Bytes>` before calling the service layer:

```rust
let mut chunks: Vec<Bytes> = Vec::new();
while let Some(chunk) = field.chunk().await {
    chunks.push(chunk);
}
upload_service.smart_upload(..., chunks, total_size).await
```

### Buffered Path (< 1 MB)

Uses `save_file()` — all bytes are passed to `FileBlobWriteRepository`, which calls `DedupService.store_bytes()` to compute hash and store the blob, then INSERTs metadata into `storage.files`.

### Streaming Path (≥ 1 MB)

`smart_upload()` converts the in-memory `Vec<Bytes>` into a `futures::stream::iter()` and passes it to `save_file_from_stream()`:

```rust
let chunk_stream = stream::iter(chunks.into_iter().map(|c| Ok(c)));
self.file_write.save_file_from_stream(name, folder_id, content_type, chunk_stream).await
```

`FileBlobWriteRepository.save_file_from_stream()` collects the stream, stores via DedupService, and INSERTs metadata.

### Dedup Integration

Deduplication is handled at the **repository layer** (not the service layer) for all upload strategies. `FileBlobWriteRepository` always calls `DedupService.store_bytes()` which:

1. Computes SHA-256 hash of content
2. Checks if blob already exists (dedup hit → increment ref count, skip write)
3. If new → atomic write to `.blobs/{prefix}/{hash}.blob`
4. Returns the hash for storage in `storage.files.blob_hash`
