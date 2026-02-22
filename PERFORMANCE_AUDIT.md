# OxiCloud — Comprehensive Architecture & Performance Audit

> **Scope**: Full source-level analysis of all layers (domain → infrastructure → application → interfaces).
> **Methodology**: Static analysis of every critical `.rs` file. No code changes made.

---

## Table of Contents

1. [CRITICAL — `std::sync::Mutex` Blocking the Tokio Runtime](#1-critical--stdsyncmutex-blocking-the-tokio-runtime)
2. [CRITICAL — ZIP Service Loads Entire Files into Memory](#2-critical--zip-service-loads-entire-files-into-memory)
3. [CRITICAL — Share Repository: JSON File I/O per Operation](#3-critical--share-repository-json-file-io-per-operation)
4. [HIGH — Blocking Filesystem Calls in Async Context](#4-high--blocking-filesystem-calls-in-async-context)
5. [HIGH — Unbounded Task Spawning in Recursive Search](#5-high--unbounded-task-spawning-in-recursive-search)
6. [HIGH — Thumbnail Cache Write-Lock Contention on Reads](#6-high--thumbnail-cache-write-lock-contention-on-reads)
7. [HIGH — HTTP Cache Middleware Buffers Entire Response Bodies](#7-high--http-cache-middleware-buffers-entire-response-bodies)
8. [MEDIUM — N+1 Queries / Extra Database Round Trips](#8-medium--n1-queries--extra-database-round-trips)
9. [MEDIUM — Unnecessary String Allocations in Error Paths](#9-medium--unnecessary-string-allocations-in-error-paths)
10. [MEDIUM — Redundant Path String in Domain Entities](#10-medium--redundant-path-string-in-domain-entities)
11. [MEDIUM — Unbounded Parallel Tasks in Storage Usage Update](#11-medium--unbounded-parallel-tasks-in-storage-usage-update)
12. [MEDIUM — Upload Handler Re-parses Its Own HTTP Response](#12-medium--upload-handler-re-parses-its-own-http-response)
13. [LOW — One-Shot Cache Pattern Defeats Caching Purpose](#13-low--one-shot-cache-pattern-defeats-caching-purpose)
14. [LOW — Duplicated SQL in Paginated Search](#14-low--duplicated-sql-in-paginated-search)
15. [LOW — Sequential Trash Cleanup Without Batching](#15-low--sequential-trash-cleanup-without-batching)
16. [LOW — Search Cache Key Serializes Entire DTO to JSON](#16-low--search-cache-key-serializes-entire-dto-to-json)
17. [Positive Patterns — What's Done Well](#17-positive-patterns--whats-done-well)
18. [Summary Matrix](#18-summary-matrix)

---

## 1. CRITICAL — `std::sync::Mutex` Blocking the Tokio Runtime

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/application/services/search_service.rs` | 4, 56 | `search_cache: Arc<Mutex<HashMap<…>>>` |
| `src/interfaces/middleware/cache.rs` | 14, 51 | `cache: Arc<Mutex<HashMap<…>>>` |
| `src/application/services/auth_application_service.rs` | 62–63 | `pending_oidc_flows`, `pending_oidc_tokens` |

### Problematic Pattern

```rust
// search_service.rs:4
use std::sync::Mutex;

// search_service.rs:56
search_cache: Arc<Mutex<HashMap<SearchCacheKey, CachedSearchResult>>>,
```

Every call to `.lock()` on a `std::sync::Mutex` across an `.await` boundary **blocks the entire Tokio worker thread**. If all Tokio workers are blocked on the Mutex simultaneously, the runtime deadlocks.

In `search_service.rs`, `get_from_cache()` and `store_in_cache()` both call `.lock()`, and `store_in_cache()` does eviction work (iteration + removal) while holding the lock. The cleanup task (`start_cache_cleanup_task`) also locks the Mutex inside a `tokio::spawn` future.

In `cache.rs`, every HTTP GET request passes through `.get()` or `.set()`, each calling `self.cache.lock().unwrap()`. The `evict_oldest()` method sorts all entries by timestamp while the parent lock is held.

### Impact

- **Severity**: CRITICAL under concurrent load.
- Under 50+ concurrent requests, Tokio worker threads park on the Mutex, causing tail-latency spikes (p99 > 100ms) and potential deadlock.
- The cleanup tasks also lock, creating periodic contention peaks.

### Fix Sketch

**Option A — Replace with `tokio::sync::RwLock`** (minimal change):
```rust
use tokio::sync::RwLock;
search_cache: Arc<RwLock<HashMap<SearchCacheKey, CachedSearchResult>>>,
```

**Option B — Replace with `moka` (lock-free, recommended)**:
```rust
use moka::future::Cache;

// In SearchService
search_cache: Cache<SearchCacheKey, SearchResultsDto>,

// Construction
let search_cache = Cache::builder()
    .max_capacity(max_cache_size as u64)
    .time_to_live(Duration::from_secs(cache_ttl))
    .build();
```
This eliminates all manual eviction logic and the cleanup task entirely. Already used successfully in `image_transcode_service.rs` and `file_content_cache.rs`.

For the HTTP cache middleware: consider replacing with `moka::future::Cache<String, CacheEntry>`.

For the auth service: the OIDC maps are short-lived and low-contention, so `tokio::sync::Mutex` would suffice, or use `dashmap::DashMap`.

---

## 2. CRITICAL — ZIP Service Loads Entire Files into Memory

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/services/zip_service.rs` | 209–230 | `add_file_to_zip()` |

### Problematic Pattern

```rust
// zip_service.rs: add_file_to_zip()
async fn add_file_to_zip(
    &self,
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    // ...
) -> Result<()> {
    // Loads ENTIRE file content into memory
    let content = self.file_service.get_file_content(&file_id).await?;
    zip.write_all(&content)?;
}
```

For a folder download containing N files of size S, memory usage is `O(N × S)` **plus** the ZIP buffer itself. A folder with 100 × 100MB files = 10GB in RAM.

### Impact

- **Severity**: CRITICAL for large folders. OOM-kill risk in production.
- The ZIP buffer (`Cursor<Vec<u8>>`) also holds the entire compressed output in memory.

### Fix Sketch

Use `tokio::io::AsyncRead` + streaming ZIP writer (e.g., `async_zip` crate):
```rust
// Stream-based approach:
let blob_stream = self.dedup_service.read_blob_stream(&blob_hash).await?;
// Pipe directly to zip writer without buffering the entire file
zip.write_entry_stream(file_name, blob_stream).await?;
```
Alternatively, use `read_blob_stream()` (which already exists in `DedupService` with 256KB chunks) and write chunks incrementally to the ZIP.

---

## 3. CRITICAL — Share Repository: JSON File I/O per Operation

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/repositories/share_fs_repository.rs` | 1–286 | `ShareFsRepository` |

### Problematic Pattern

```rust
// Every read operation:
async fn get_share(&self, id: &str) -> Result<…> {
    let shares = self.read_shares().await?;  // Read ENTIRE file
    shares.into_iter().find(|s| s.id == id)  // Linear scan
}

// Every write operation:
async fn create_share(&self, share: Share) -> Result<…> {
    let mut shares = self.read_shares().await?;  // Read ENTIRE file
    shares.push(share);
    self.write_shares(&shares).await?;            // Write ENTIRE file
}
```

### Impact

- **Severity**: CRITICAL for concurrent users.
- **Race condition**: Two concurrent `create_share()` calls read the same file, each appends its share, and the second write loses the first share.
- **O(n)** per operation — every read scans all shares.
- **Blocking I/O**: `tokio::fs::read` / `tokio::fs::write` are async but the entire file is serialized/deserialized on every call.

### Fix Sketch

**Option A — Migrate to PostgreSQL** (recommended, consistent with other repos):
```sql
CREATE TABLE storage.shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resource_id UUID NOT NULL,
    resource_type TEXT NOT NULL,
    token TEXT UNIQUE NOT NULL,
    password_hash TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_shares_token ON storage.shares(token);
```

**Option B — Add file locking + in-memory index** (minimal change):
```rust
struct ShareFsRepository {
    shares: Arc<RwLock<HashMap<String, Share>>>,  // In-memory index
    path: PathBuf,
    file_lock: tokio::sync::Mutex<()>,            // Serialize writes
}
```

---

## 4. HIGH — Blocking Filesystem Calls in Async Context

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/services/path_service.rs` | 137, 148, 165, 172 | `physical_path.exists()`, `.is_file()`, `.is_dir()` |
| `src/main.rs` | 60, 64 | `std::fs::create_dir_all()` |
| `src/infrastructure/services/dedup_service.rs` | 224 | `std::fs::remove_file()` |

### Problematic Pattern

```rust
// path_service.rs — inside async fn file_exists()
async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
    let physical_path = self.resolve_path(storage_path);
    let exists = physical_path.exists() && physical_path.is_file();  // BLOCKING
    Ok(exists)
}

// Also in directory_exists() and ensure_directory()
```

`Path::exists()`, `.is_file()`, and `.is_dir()` perform synchronous `stat()` syscalls. On network-attached storage (NFS, CIFS) or slow disks, these can take 10–100ms, blocking a Tokio worker.

### Impact

- **Severity**: HIGH on NFS/CIFS storage; moderate on local SSD.
- `path_service.rs` is called by the StoragePort trait used throughout the application.

### Fix Sketch

```rust
async fn file_exists(&self, storage_path: &StoragePath) -> Result<bool, DomainError> {
    let physical_path = self.resolve_path(storage_path);
    match tokio::fs::metadata(&physical_path).await {
        Ok(meta) => Ok(meta.is_file()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(DomainError::from(e)),
    }
}
```

For `main.rs` (startup-only), the blocking calls are acceptable but could use `tokio::fs::create_dir_all()` for consistency.

---

## 5. HIGH — Unbounded Task Spawning in Recursive Search

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/application/services/search_service.rs` | 310–360 | `search_parallel()` |

### Problematic Pattern

```rust
fn search_parallel(…) -> Pin<Box<dyn Future<…> + Send>> {
    Box::pin(async move {
        let folders = folder_repo.list_folders(current_folder_id.as_deref()).await?;
        
        // Spawns one task PER subfolder — NO concurrency limit
        let mut handles = Vec::with_capacity(folder_dtos.len());
        for subfolder in &folder_dtos {
            handles.push(tokio::spawn(async move {
                Self::search_parallel(fr, fdr, Some(folder_id), crit).await
            }));
        }
        // ...joins all
    })
}
```

For a directory tree of depth D with branching factor B, this spawns `B^D` tasks. A user with 1000 folders in a flat structure spawns 1000 concurrent tasks, each making DB queries.

### Impact

- **Severity**: HIGH — DB connection pool exhaustion (max 20 connections), Tokio task backlog.
- Contrast with `batch_operations.rs` which correctly uses `Semaphore::new(10)`.

### Fix Sketch

```rust
use tokio::sync::Semaphore;

fn search_parallel(
    semaphore: Arc<Semaphore>,
    // ... other args
) {
    Box::pin(async move {
        let _permit = semaphore.acquire().await.unwrap();
        // ... existing logic, pass semaphore to recursive calls
    })
}
```

Or better: for non-recursive search, the DB-level pagination path is already used. For recursive search, consider a single recursive SQL CTE instead of application-level recursion.

---

## 6. HIGH — Thumbnail Cache Write-Lock Contention on Reads

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/services/thumbnail_service.rs` | ~25, 200–280 | `cache: Arc<RwLock<LruCache<…>>>` |

### Problematic Pattern

```rust
// LruCache requires write access on EVERY read (LRU promotion)
pub async fn get_thumbnail(&self, …) -> Result<Bytes, …> {
    // Read from cache — but LRU needs write lock!
    let cache = self.cache.read().await;  // Can't actually use read lock for LRU
    // ...
}

pub async fn add_to_cache(&self, key: ThumbnailCacheKey, data: Bytes) {
    let mut current_size = self.current_cache_bytes.write().await;  // Lock #1
    // ... eviction loop also acquires:
    let mut cache = self.cache.write().await;                       // Lock #2
    // TWO write locks held simultaneously
}
```

Every cache hit and miss requires a write lock. Under concurrent image requests, this creates a bottleneck.

### Impact

- **Severity**: HIGH for image-heavy workloads.
- Two separate `RwLock` acquisitions in `add_to_cache()` — potential for deadlock if ordering is inconsistent.

### Fix Sketch

Replace with `moka::future::Cache` (already used in `image_transcode_service.rs`):

```rust
use moka::future::Cache;

pub struct ThumbnailService {
    cache: Cache<ThumbnailCacheKey, Bytes>,  // Lock-free reads, weight-based eviction
    // Remove current_cache_bytes — moka tracks weight internally
}

// Construction
let cache = Cache::builder()
    .max_capacity(max_cache_bytes as u64)
    .weigher(|_k: &ThumbnailCacheKey, v: &Bytes| v.len() as u32)
    .time_to_idle(Duration::from_secs(300))
    .build();
```

---

## 7. HIGH — HTTP Cache Middleware Buffers Entire Response Bodies

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/interfaces/middleware/cache.rs` | 247–259, 450–470 | `cache_middleware()`, `HttpCacheService::call()` |

### Problematic Pattern

```rust
// cache.rs: cache_middleware()
let bytes = axum::body::to_bytes(_body, 1024 * 1024 * 10)  // Buffer up to 10MB
    .await
    .unwrap_or_default();

let etag = cache.calculate_etag_for_bytes(&bytes);  // Hash all bytes
cache.set(cache_key, etag, Some(bytes.clone()), …);  // Clone + store
```

Every non-cached GET response is fully buffered to calculate an ETag, even for responses that shouldn't be cached (large file listings, etc.).

### Impact

- **Severity**: HIGH — 10MB max buffer per concurrent request × N concurrent requests.
- The `bytes.clone()` doubles peak memory per response.

### Fix Sketch

1. Only cache small responses (check `Content-Length` first).
2. Use streaming hash (SHA-256) to compute ETag without buffering.
3. Skip caching for responses > 1MB.
4. Replace `std::sync::Mutex` backing the cache (see Issue #1).

---

## 8. MEDIUM — N+1 Queries / Extra Database Round Trips

### Location

| File | Line(s) | Symbol | Issue |
|------|---------|--------|-------|
| `src/infrastructure/repositories/pg/folder_db_repository.rs` | ~rename_folder | `rename_folder()` | UPDATE + separate SELECT |
| `src/infrastructure/repositories/pg/folder_db_repository.rs` | ~move_folder | `move_folder()` | UPDATE + separate SELECT |
| `src/infrastructure/repositories/pg/file_blob_write_repository.rs` | ~lookup_folder_path | `lookup_folder_path()` | Extra query per file write |
| `src/infrastructure/repositories/pg/trash_db_repository.rs` | ~clear_trash | `clear_trash()` | 2 separate DELETEs |

### Problematic Pattern

```rust
// folder_db_repository.rs: rename_folder()
async fn rename_folder(&self, id: &str, new_name: &str) -> Result<Folder, DomainError> {
    // Query 1: UPDATE
    sqlx::query("UPDATE storage.folders SET name = $1 WHERE id = $2::uuid")
        .execute(self.pool.as_ref()).await?;
    
    // Query 2: SELECT (separate round trip)
    self.get_folder(id).await
}
```

### Impact

- **Severity**: MEDIUM — adds 1–5ms per extra round trip, compounded in batch operations.
- `lookup_folder_path()` is called per file write; in batch uploads of N files to the same folder, it makes N identical queries.

### Fix Sketch

```sql
-- Use RETURNING to get the updated row in a single query
UPDATE storage.folders SET name = $1
WHERE id = $2::uuid
RETURNING id::text, name, parent_id::text, path, …
```

For `lookup_folder_path()` in batch operations, cache the folder path for the duration of the batch.

---

## 9. MEDIUM — Unnecessary String Allocations in Error Paths

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/domain/errors.rs` | throughout | `DomainError` factory methods |
| `src/infrastructure/repositories/pg/*.rs` | throughout | `.map_err(|e| DomainError::internal_error(…, format!("…: {e}")))` |

### Problematic Pattern

```rust
// domain/errors.rs
pub fn not_found(entity_type: &'static str, id: impl Into<String>) -> Self {
    let entity_id = id.into();
    Self {
        message: format!("{} not found: {}", entity_type, entity_id),  // ALLOCATION
        entity_id: Some(entity_id),                                     // ALLOCATION
        // ...
    }
}
```

Every error — even `NotFound` which may be a normal control flow path (e.g., checking if a file exists) — allocates 2 strings via `format!()` and `into()`.

### Impact

- **Severity**: MEDIUM — hot error paths (404 checks, duplicate detection) trigger allocations.
- In batch operations checking 1000 files, this creates thousands of unnecessary allocations.

### Fix Sketch

Use `Cow<'static, str>` for common messages:
```rust
pub fn not_found(entity_type: &'static str, id: impl Into<String>) -> Self {
    Self {
        message: Cow::Borrowed(""),  // Defer formatting to Display impl
        entity_id: Some(id.into()),
        kind: ErrorKind::NotFound,
        entity_type,
        source: None,
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format lazily only when displayed
        write!(f, "{} {}: {}", self.entity_type, self.kind, 
               self.entity_id.as_deref().unwrap_or("unknown"))
    }
}
```

---

## 10. MEDIUM — Redundant Path String in Domain Entities

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/domain/entities/file.rs` | ~30–50 | `storage_path: StoragePath` + `path_string: String` |
| `src/domain/entities/folder.rs` | ~30–50 | Same pattern |

### Problematic Pattern

```rust
pub struct File {
    storage_path: StoragePath,
    path_string: String,  // Redundant: same data as storage_path.to_string()
    // ...
}
```

Every `File` and `Folder` entity carries both a `StoragePath` (which internally holds `Vec<String>`) **and** a pre-rendered `String` copy. This doubles the path memory per entity.

### Impact

- **Severity**: MEDIUM — when listing 10,000 files, each path is stored twice.

### Fix Sketch

Remove `path_string` and derive it on demand:
```rust
impl File {
    pub fn path_string(&self) -> String {
        self.storage_path.to_string()
    }
}
```

Or cache it lazily with `OnceCell<String>` if `.to_string()` is called frequently.

---

## 11. MEDIUM — Unbounded Parallel Tasks in Storage Usage Update

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/application/services/storage_usage_service.rs` | 138–165 | `update_all_users_storage_usage()` |

### Problematic Pattern

```rust
async fn update_all_users_storage_usage(&self) -> Result<(), DomainError> {
    let users = self.user_repository.list_users(1000, 0).await?;
    
    let mut update_tasks = Vec::new();
    for user in users {
        let service_clone = self.clone();
        // Spawn one task per user — NO concurrency limit
        let task = task::spawn(async move {
            service_clone.update_user_storage_usage(&user_id).await
        });
        update_tasks.push(task);
    }
    // joins all
}
```

### Impact

- **Severity**: MEDIUM — 1000 users = 1000 concurrent DB queries. DB pool has max 20 connections, so 980 tasks queue, but Tokio task overhead + connection wait time is wasteful.

### Fix Sketch

```rust
use futures::stream::{self, StreamExt};

stream::iter(users)
    .map(|user| {
        let svc = self.clone();
        async move { svc.update_user_storage_usage(&user.id).await }
    })
    .buffer_unordered(10)  // Max 10 concurrent
    .collect::<Vec<_>>()
    .await;
```

---

## 12. MEDIUM — Upload Handler Re-parses Its Own HTTP Response

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/interfaces/api/handlers/file_handler.rs` | ~upload_file_with_thumbnails | `upload_file_with_thumbnails()` |

### Problematic Pattern

The `upload_file_with_thumbnails` handler calls the upload logic, gets back an HTTP response, then reads the response body back to extract the file ID for thumbnail generation. This means:

1. Serialize file info → JSON response body
2. Read response body → bytes
3. Deserialize bytes → file info
4. Use file info for thumbnail generation

### Impact

- **Severity**: MEDIUM — unnecessary serialize → deserialize round trip per upload.

### Fix Sketch

Call the upload service directly and pass the result to thumbnail generation, instead of going through HTTP serialization:

```rust
let file = upload_service.upload_file(…).await?;
thumbnail_service.generate_all_sizes_background(file.id.clone(), path);
Ok(Json(FileDto::from(file)))
```

---

## 13. LOW — One-Shot Cache Pattern Defeats Caching Purpose

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/repositories/pg/file_blob_read_repository.rs` | ~95–115 | `resolve_blob_hash()` |

### Problematic Pattern

```rust
async fn resolve_blob_hash(&self, file_id: &str) -> Result<String, DomainError> {
    // Check moka cache
    if let Some(hash) = self.hash_cache.get(file_id) {
        self.hash_cache.invalidate(file_id);  // Immediately invalidate!
        return Ok(hash);
    }
    // ... DB query
}
```

The hash is cached then immediately invalidated after first use. This means repeated reads of the same file always hit the database.

### Impact

- **Severity**: LOW — the pattern only provides "write-behind" benefit (avoiding a DB query between upload and first download). Repeated downloads bypass cache.

### Fix Sketch

Remove the invalidation and let moka's TTI (30s) handle expiry:
```rust
if let Some(hash) = self.hash_cache.get(file_id) {
    return Ok(hash);  // Let TTI handle expiry
}
```

---

## 14. LOW — Duplicated SQL in Paginated Search

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/repositories/pg/file_blob_read_repository.rs` | 400–620 | `search_files_paginated()` |

### Problematic Pattern

Four nearly identical `match` arms containing:
- Copy-pasted SQL with minor WHERE clause differences
- Each arm has a COUNT query + SELECT query (2 DB round trips per search)
- SQL ORDER BY built via `format!()` string interpolation

### Impact

- **Severity**: LOW (correctness) to MEDIUM (maintenance burden).
- The COUNT query is always executed even when the result set is smaller than the limit (i.e., total count could be inferred).

### Fix Sketch

Build the query dynamically with a query builder:
```rust
let mut conditions = vec!["fi.user_id = $1::uuid", "fi.is_trashed = false"];
let mut bind_idx = 2;

if let Some(fid) = folder_id {
    conditions.push(&format!("fi.folder_id = ${bind_idx}::uuid"));
    bind_idx += 1;
}
if let Some(name) = &criteria.name_contains {
    conditions.push(&format!("LOWER(fi.name) LIKE ${bind_idx}"));
    bind_idx += 1;
}
// ... single query construction
```

Use `COUNT(*) OVER()` window function to get total count in a single query:
```sql
SELECT fi.*, COUNT(*) OVER() as total_count
FROM storage.files fi
WHERE …
ORDER BY … LIMIT $N OFFSET $M
```

---

## 15. LOW — Sequential Trash Cleanup Without Batching

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/infrastructure/services/trash_cleanup_service.rs` | 75–95 | `cleanup_expired_items()` |

### Problematic Pattern

```rust
for item in expired_items {
    trash_service.delete_permanently(&trash_id, &user_id).await;  // One at a time
}
```

### Impact

- **Severity**: LOW — cleanup runs periodically in the background, not in the request path.

### Fix Sketch

Use `futures::stream::buffer_unordered()` for concurrent deletion, or batch-delete at the SQL level:
```sql
DELETE FROM storage.files WHERE is_trashed = true AND trashed_at < NOW() - INTERVAL '30 days';
```

---

## 16. LOW — Search Cache Key Serializes Entire DTO to JSON

### Location

| File | Line(s) | Symbol |
|------|---------|--------|
| `src/application/services/search_service.rs` | 170–180 | `create_cache_key()` |

### Problematic Pattern

```rust
fn create_cache_key(&self, criteria: &SearchCriteriaDto, user_id: &str) -> Result<SearchCacheKey> {
    let criteria_str = serde_json::to_string(criteria).map_err(…)?;  // Full JSON serialization
    Ok(SearchCacheKey {
        criteria_hash: criteria_str,  // Stored as full JSON string, not a hash
        user_id: user_id.to_string(),
    })
}
```

The "hash" field is actually the full JSON string, not a hash. This means:
1. Full serde serialization per search request
2. HashMap key comparison is O(n) on string length
3. Unnecessary memory for cache keys

### Impact

- **Severity**: LOW — search requests are human-speed, not high-throughput.

### Fix Sketch

```rust
use std::hash::{Hash, Hasher, DefaultHasher};

fn create_cache_key(&self, criteria: &SearchCriteriaDto, user_id: &str) -> SearchCacheKey {
    let mut hasher = DefaultHasher::new();
    criteria.hash(&mut hasher);  // Derive Hash on SearchCriteriaDto
    user_id.hash(&mut hasher);
    SearchCacheKey(hasher.finish())
}
```

---

## 17. Positive Patterns — What's Done Well

These are worth calling out as **exemplary** implementations:

| Component | File | Pattern |
|-----------|------|---------|
| **Image Transcoding** | `image_transcode_service.rs` | Dedicated `rayon` thread pool (not Tokio blocking pool), `moka` lock-free cache, `AtomicU64` stats, fire-and-forget disk cache writes. **Best-in-class design.** |
| **File Content Cache** | `file_content_cache.rs` | `moka` weight-based cache, lock-free reads, automatic eviction. Clean. |
| **Batch Operations** | `batch_operations.rs` | `Semaphore`-based concurrency control. Correct pattern. |
| **Thumbnail Generation** | `thumbnail_service.rs` | Uses `spawn_blocking` for image processing. Correct (but cache should be moka). |
| **Compression** | `compression_service.rs` | `spawn_blocking` for CPU-bound gzip. Streaming compress. Correct. |
| **Dedup Service** | `dedup_service.rs` | Atomic write via temp+rename, `SELECT FOR UPDATE` for blob refcounting, 2-char hash prefix sharding. Solid CAS implementation. |
| **Multi-Tier Download** | `file_retrieval_service.rs` | Write-behind → hot cache + WebP → mmap → streaming. Well-designed tiered strategy. |
| **Streaming Upload** | `file_handler.rs` | SHA-256 computed during spool, 512KB BufWriter, pre-allocation hints. Good. |
| **DB Pagination** | `file_blob_read_repository.rs` | Non-recursive search uses LIMIT/OFFSET at DB level. Correct. |
| **Content Dedup** | `file_blob_write_repository.rs` | `copy_file` uses CTE for zero-copy blob dedup. `update_file_content` uses atomic CTE with `FOR UPDATE`. |

---

## 18. Summary Matrix

| # | Issue | Severity | Impact Area | Effort to Fix |
|---|-------|----------|-------------|---------------|
| 1 | `std::sync::Mutex` in async | **CRITICAL** | Latency, deadlock | Small (swap to moka) |
| 2 | ZIP loads files into memory | **CRITICAL** | OOM risk | Medium (streaming ZIP) |
| 3 | Share repo: JSON file I/O | **CRITICAL** | Data loss, O(n) | Medium (migrate to PG) |
| 4 | Blocking FS in async | **HIGH** | Latency on slow storage | Small (use tokio::fs) |
| 5 | Unbounded search tasks | **HIGH** | DB pool exhaustion | Small (add Semaphore) |
| 6 | Thumbnail cache write-lock | **HIGH** | Contention | Small (swap to moka) |
| 7 | HTTP cache buffers 10MB | **HIGH** | Memory | Medium (streaming hash) |
| 8 | N+1 queries | **MEDIUM** | Latency | Small (use RETURNING) |
| 9 | Error string allocations | **MEDIUM** | Allocator pressure | Medium (Cow/lazy) |
| 10 | Redundant path string | **MEDIUM** | Memory | Small (remove field) |
| 11 | Unbounded storage tasks | **MEDIUM** | DB pool | Small (add Semaphore) |
| 12 | Handler re-parses response | **MEDIUM** | CPU waste | Small (refactor) |
| 13 | One-shot cache invalidation | **LOW** | Cache miss rate | Trivial |
| 14 | Duplicated search SQL | **LOW** | Maintenance | Medium |
| 15 | Sequential trash cleanup | **LOW** | Cleanup speed | Small |
| 16 | JSON cache key | **LOW** | Minor alloc | Small |

### Recommended Priority Order

1. **Issues 1, 2, 3** — Fix immediately. These can cause production incidents (deadlocks, OOM, data loss).
2. **Issues 4, 5, 6** — Fix before scaling. These create bottlenecks under load.
3. **Issue 7** — Fix when observing memory pressure.
4. **Issues 8–12** — Address as part of normal development.
5. **Issues 13–16** — Clean up opportunistically.
