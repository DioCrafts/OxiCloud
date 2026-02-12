# 04 - Caching Architecture

OxiCloud uses a multi-layer caching system spanning HTTP-level caching down to kernel-level memory mapping. Covers both uploads and downloads.

## Cache Layers Summary

```
┌─────────────────────────────────────────────────────┐
│  Layer 0: HTTP Cache Middleware (ETag + 304)         │  All endpoints
├─────────────────────────────────────────────────────┤
│  Layer 1: File Content Cache (LRU, <10MB files)     │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 2: MMAP (memmap2, 10-100MB files)            │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 3: Streaming (FramedRead, ≥100MB files)      │  Downloads
├─────────────────────────────────────────────────────┤
│  Layer 4: File Metadata Cache (adaptive TTL)        │  All file ops
├─────────────────────────────────────────────────────┤
│  Layer 5: Write-Behind Cache (<256KB uploads)       │  Uploads
├─────────────────────────────────────────────────────┤
│  Layer 6: Buffer Pool (reusable I/O buffers)        │  Compression
└─────────────────────────────────────────────────────┘
```

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
- `should_cache(size)` -- checks if file fits in cache
- `get(file_id)` → `Option<(Bytes, String, String)>` -- returns (data, etag, content_type)
- `put(file_id, content, etag, content_type)` -- inserts with LRU eviction
- `invalidate(file_id)`, `clear()`
- `stats()` → `CacheStats { current_size_bytes, max_size_bytes, hits, misses, hit_rate_percent }`

Port: implements **ContentCachePort** trait.

---

## Layer 2: MMAP (Download Tier 2)

**File**: `src/infrastructure/repositories/file_fs_read_repository.rs`

Memory-mapped I/O for medium files using `memmap2`.

| Parameter | Value |
|---|---|
| File range | 10 MB - 100 MB |
| Implementation | `memmap2::Mmap` via `spawn_blocking` |
| Latency | ~1-5ms |

Current implementation copies mmap'd data to `Bytes` (`Bytes::copy_from_slice(&mmap[..])`). Not true zero-copy, but still benefits from kernel page cache.

---

## Layer 3: Streaming (Download Tier 3)

**File**: `src/infrastructure/repositories/file_fs_read_repository.rs`

Chunked streaming for large files using tokio-util codecs.

| Parameter | Value |
|---|---|
| File range | ≥100 MB |
| Chunk size | 1 MB (configurable via **ResourceConfig.chunk_size_bytes**) |
| Implementation | `FramedRead` + `BytesCodec` |
| RAM usage | Near zero (one chunk at a time) |

---

## Layer 4: File Metadata Cache

**File**: `src/infrastructure/services/file_metadata_cache.rs`

Caches filesystem metadata (existence, size, MIME type, timestamps) to avoid repeated `stat()` calls.

| Parameter | Value |
|---|---|
| Default file TTL | 60 seconds |
| Default directory TTL | 120 seconds |
| Max entries | 10,000 |
| Adaptive TTL multiplier | 5x for popular entries (≥10 accesses) |
| LRU eviction | Frees 10% capacity when full |
| Cleanup | Background task runs periodically |

**CachedMetadata:**
```rust
pub struct FileMetadata {
    pub path: PathBuf,
    pub exists: bool,
    pub entry_type: CacheEntryType,     // File | Directory | Unknown
    pub size: Option<u64>,
    pub mime_type: Option<String>,
    pub created_at: Option<u64>,
    pub modified_at: Option<u64>,
    pub last_access: Instant,
    pub expires_at: Instant,
    pub access_count: usize,
}
```

**Adaptive TTL**: entries accessed ≥10 times get 5x the configured TTL, keeping frequently accessed file metadata in cache longer.

Port: implements **MetadataCachePort** trait.

---

## Layer 5: Write-Behind Cache

**File**: `src/infrastructure/services/write_behind_cache.rs`

Buffers small uploads in RAM and confirms immediately. Flushes to disk asynchronously.

| Parameter | Value |
|---|---|
| Max file size | 1 MB per file |
| Max total cache | 100 MB |
| Max pending duration | 30 seconds |
| Flush interval | 100 ms |
| Write strategy | Atomic (temp file + rename) |

Architecture:
- `put_pending(file_id, content, target_path)` stores bytes in `HashMap<String, PendingWrite>`
- Background `flush_worker` processes **FlushCommands** via `mpsc` channel
- Periodic checker force-flushes entries older than 30 seconds
- `get_pending(file_id)` serves reads while data is still in RAM (before flush)

Port: implements **WriteBehindCachePort** trait.

Statistics:
```rust
pub struct WriteBehindStatsDto {
    pub pending_count: usize,
    pub pending_bytes: usize,
    pub total_writes: u64,
    pub total_bytes_written: u64,
    pub cache_hits: u64,
    pub avg_flush_time_us: u64,
}
```

---

## Layer 6: Buffer Pool

**File**: `src/infrastructure/services/buffer_pool.rs`

Reusable byte buffer pool to reduce allocation pressure during compression operations.

| Parameter | Value |
|---|---|
| Buffer size | 64 KB |
| Max buffers | 100 |
| Buffer TTL | 60 seconds |
| Concurrency control | `tokio::sync::Semaphore` |

Features:
- `get_buffer()` -- borrows a buffer (blocks if pool exhausted)
- **BorrowedBuffer** auto-returns to pool on `Drop` via `tokio::spawn`
- Expired buffers are cleaned periodically via `start_cleaner()`
- Tracks stats: gets, hits, misses, returns, evictions, waits

---

## Configuration

All cache-related config in `src/common/config.rs`:

```rust
pub struct CacheConfig {
    pub file_ttl_ms: u64,        // default: 60,000 (1 min)
    pub directory_ttl_ms: u64,   // default: 120,000 (2 min)
    pub max_entries: usize,      // default: 10,000
}

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
       → file size < 100MB? → Tier 2: MMAP (kernel page cache)
       → file size ≥ 100MB  → Tier 3: Streaming (chunked)
```

---

## Range Requests (HTTP 206 Partial Content)

**Files**: `src/interfaces/api/handlers/file_handler.rs`, `src/infrastructure/repositories/file_fs_read_repository.rs`

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

### File Seek Implementation

`get_file_range_stream()` at the repository level:

```rust
async fn get_file_range_stream(
    &self, id: &str, start: u64, end: Option<u64>,
) -> Result<Box<dyn Stream<...> + Send>, DomainError>
```

1. Opens the file with `TokioFile::open()`
2. Seeks to `start` via `fh.seek(SeekFrom::Start(start))`
3. Limits read to `range_length` via `fh.take(range_length)`
4. Wraps in `FramedRead` + `BytesCodec`

Adaptive chunk size:

| Range size | Chunk size |
|---|---|
| ≤ 1 MB | 8 KB |
| > 1 MB | 1 MB (from **ResourceConfig.chunk_size_bytes**) |

### Tier Interaction

Range requests **bypass all download tiers** (LRU, MMAP, write-behind). They always use direct file seek + streaming. On stream creation error, the handler falls through to the normal `get_file_optimized()` 3-tier path.

### Limitations

- **Multipart ranges not supported**: only the first range in a multi-range request is served. Additional ranges are ignored.
- **`If-Range` not handled**: no conditional range support.
- **`If-Modified-Since` not handled**: only `If-None-Match` (ETag) is checked.

---

## Upload Flow

```
Request → file size < 256KB? → Write-behind cache (instant 201, async flush)
       → file size < 1MB?   → Buffered write (sync)
       → file size ≥ 1MB    → Streaming write (chunk-by-chunk to temp + rename)
```

### Upload Strategy Selection

**File**: `src/application/services/file_upload_service.rs`

```rust
pub enum UploadStrategy {
    WriteBehind,   // < 256 KB — instant response, async disk write
    Buffered,      // 256 KB – 1 MB — sync write to final path
    Streaming,     // ≥ 1 MB — chunk-by-chunk write to temp file + rename
}
```

| Constant | Value |
|---|---|
| `WRITE_BEHIND_THRESHOLD` | 256 KB |
| `STREAMING_UPLOAD_THRESHOLD` | 1 MB |

### Handler-Level Buffering

Both upload handlers (`upload_file` and `upload_file_with_cache`) buffer the **entire multipart body in RAM** as `Vec<Bytes>` before calling the service layer:

```rust
let mut chunks: Vec<Bytes> = Vec::new();
while let Some(chunk) = field.chunk().await {
    chunks.push(chunk);
}
// All bytes are now in RAM
upload_service.smart_upload(..., chunks, total_size).await
```

The "streaming" in `UploadStrategy::Streaming` refers to the **service→repository** path, not the HTTP-body→disk path. By the time `save_file_from_stream()` is called, data is already in memory.

### Streaming Path (≥ 1 MB): Service → Repository

`smart_upload()` converts the in-memory `Vec<Bytes>` into a `futures::stream::iter()` and passes it to `save_file_from_stream()`:

```rust
let chunk_stream = stream::iter(chunks.into_iter().map(|c| Ok(c)));
self.file_write.save_file_from_stream(name, folder_id, content_type, chunk_stream).await
```

**`save_file_from_stream()` implementation** (`file_fs_write_repository.rs`):

1. Resolves target path + generates unique name if collision
2. Creates temp file: `{target_path}.tmp.upload`
3. Iterates stream, writing each chunk with `fh.write_all(&chunk)`
4. Calls `fh.flush()` + `fh.sync_all()` for durability
5. Atomic rename: `fs::rename(temp_path, final_path)`
6. Post-write: ID mapping, cache invalidation, metadata update

### Buffered Path (256 KB - 1 MB)

Uses `save_file()` -- writes all bytes directly to the **final path** (no temp file). For larger content, writes in chunks of **ResourceConfig.chunk_size_bytes** (1 MB).

### Write-Behind Path (< 256 KB)

See **Layer 5** above. Instant `201`, background flush within 30 seconds.

### Dedup Pre-Check

Runs for **all upload strategies** before writing. Re-combines all chunks into a single `Vec<u8>` for hash computation, which means data is temporarily duplicated in RAM during dedup processing.
