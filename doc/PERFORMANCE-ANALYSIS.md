# OxiCloud — Deep Performance Analysis

> Extreme‑optimization audit of every hot path, allocation pattern, and
> concurrency strategy across 22 source files.

---

## Executive Summary

OxiCloud is **already well‑architected** for performance: moka lock‑free caches
everywhere, BLAKE3 hashing, dedicated rayon pool for image work, ltree GiST
indexes for subtree queries, streaming I/O, and zero‑copy `Bytes` clones. The
findings below target the **remaining ~15–25 % of allocatable overhead** that
separates "good" from "extreme."

**Impact tiers:**
- 🔴 **High** — measurable latency or throughput regression on every request
- 🟡 **Medium** — wasteful but amortised across many requests
- 🟢 **Low** — micro‑optimisation, only matters at ≥ 10 k req/s

---

## 1. Avoidable `.clone()` calls

### 🔴 1a. `CurrentUser` cloned on every authenticated request

**File:** `src/interfaces/middleware/auth.rs`

The middleware extracts a `CurrentUser` (4 owned `String` fields) into Axum's
request extensions. Every handler that reads it clones the struct:

```rust
// auth.rs — CurrentUser has 4 String fields
pub struct CurrentUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}
```

**Fix:** Replace with `Arc<CurrentUser>` in request extensions. All downstream
handlers receive `Arc::clone()` (8‑byte refcount bump) instead of 4 heap
allocations:

```rust
request.extensions_mut().insert(Arc::new(current_user));
// handlers: Extension(user): Extension<Arc<CurrentUser>>
```

**Estimated saving:** ~160–320 ns per request (4 × String clone of ~20‑byte
UUIDs/emails).

---

### 🟡 1b. `config.clone()` during `CoreServices` construction

**File:** `src/common/di.rs`

```rust
// di.rs — CoreServices creation
let core = CoreServices {
    config: config.clone(), // AppConfig is large: ~60 fields, many Strings
    ...
};
```

`AppConfig` contains ~60 fields including nested structs with owned `String`s.
This only runs at startup, so impact is negligible — but it leaks into any
service that receives `AppConfig` by value instead of `Arc<AppConfig>`.

**Fix:** Pass `Arc<AppConfig>` everywhere. Most services already take
`Arc<AppConfig>`; unify the remaining call sites.

---

### 🟡 1c. `mime_type.clone()` in file retrieval return paths

**File:** `src/application/services/file_retrieval_service.rs`

```rust
// file_retrieval_service.rs — return path
Ok(FileContentDto {
    content,
    mime_type: mime_type.clone(), // repeated in match arms
    ...
})
```

Mime type strings are typically < 30 bytes (`"image/jpeg"`) so each clone is
cheap, but this happens per‑download. Using `Arc<str>` or keeping the MIME as
`&'static str` (from a lookup table of the ~30 common types) would eliminate
the allocation entirely.

---

### 🟡 1d. `file.clone()` in search suggest

**File:** `src/application/services/search_service.rs`

```rust
// search_service.rs — suggest()
results.iter().map(|file| {
    FileDto::from(file.clone()) // full File entity clone per suggestion
}).collect()
```

**Fix:** `FileDto::from(&file)` — take by reference, build DTO fields directly.

---

### 🟡 1e. `target_folder.map(|s| s.to_string())` in batch operations

**File:** `src/application/services/batch_operations.rs`

```rust
// batch_operations.rs — copy_files/move_files
let target_folder: Option<Arc<str>> = target_folder_id.map(|s| Arc::from(s.as_str()));
// ...per-item:
target_folder.map(|s| s.to_string()) // re-allocates a String from Arc<str> per item
```

`Arc<str>` is correctly used to avoid N clones, but the inner closure converts
it back to `String` on each iteration — allocating N identical Strings.

**Fix:** Accept `Option<&str>` in the downstream service method, or if it
requires `String`, store `Arc<String>` and call `.as_ref()`.

---

## 2. String allocations replaceable by `&str` / `Cow` / `&'static str`

### 🔴 2a. `DomainError` allocates on every construction

**File:** `src/domain/errors.rs`

```rust
// errors.rs
pub struct DomainError {
    pub entity_id: Option<String>,   // heap alloc
    pub message: String,             // heap alloc
    pub source: Option<Box<dyn StdError + Send + Sync>>, // heap alloc
    ...
}

pub fn not_found(entity_type: &'static str, id: &str) -> Self {
    Self {
        entity_id: Some(id.to_string()),          // alloc
        message: format!("{} not found", entity_type), // alloc + format
        ...
    }
}
```

Error paths are not usually "hot," but in OxiCloud many operations pattern-
match on errors to decide control flow (e.g. trash service checks `"not found"`
in error messages via string matching):

```rust
// trash_service.rs
if format!("{}", e).contains("not found") { ... }
```

This is both a performance issue (formatting the error + string search) and a
correctness risk. The `ErrorKind` enum already exists — use it:

```rust
if matches!(e.kind, ErrorKind::NotFound) { ... }
```

**Fix for DomainError allocations:**
- Use `Cow<'static, str>` for `message` (most messages are literals)
- Use `Cow<'_, str>` for `entity_id` (most IDs are passed as `&str`)
- Only allocate when the error crosses an async boundary

---

### 🔴 2b. `compute_relevance` allocates per result

**File:** `src/application/services/search_service.rs`

```rust
// search_service.rs
fn compute_relevance(name: &str, query: &str) -> f64 {
    let name_lower = name.to_lowercase();   // alloc
    let query_lower = query.to_lowercase(); // alloc (same query, every iteration!)
    ...
}
```

For a search returning 100 results, this creates 200 temporary `String`s.

**Fix:** Pre-lowercase the query once before the loop; for file names use
`eq_ignore_ascii_case` / `to_ascii_lowercase` (in-place capable) or
`unicase::UniCase`.

---

### 🟡 2c. `enrich_file`/`enrich_folder` in search service

**File:** `src/application/services/search_service.rs`

```rust
// search_service.rs — enrich_file per result
enriched.formatted_size = format_bytes(file.size as u64);  // format!() alloc
enriched.icon_class = get_icon_class(&file.mime_type);     // returns String
enriched.icon_special_class = get_icon_special_class(&file.mime_type); // String
enriched.category = get_category(&file.mime_type);         // String
```

4 × String allocation per search result. If `get_icon_class` etc. return from
a fixed set, they should return `&'static str`.

---

### 🟡 2d. `target_format.mime_type().to_string()` in transcode service

**File:** `src/infrastructure/services/image_transcode_service.rs`

```rust
// image_transcode_service.rs
Ok((transcoded, target_format.mime_type().to_string(), true))
//                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// mime_type() returns &'static str ("image/webp"), .to_string() allocates
```

**Fix:** Change return type to `&'static str` or `Cow<'static, str>`.

---

### 🟡 2e. `file_id.to_string()` in cache key construction (thumbnails)

**File:** `src/infrastructure/services/thumbnail_service.rs`

```rust
let cache_key = ThumbnailCacheKey {
    file_id: file_id.to_string(), // alloc on each lookup
    size: *size,
};
```

Moka's `get()` takes `&K` and hashes it. If `file_id` is already a `String`,
this clone is unnecessary — store `Arc<str>` as key or borrow via `Borrow`
trait.

---

## 3. Vec allocations

### 🟢 3a. Generally well pre‑sized

Most `Vec` allocations use `with_capacity()` or rely on `collect()` from
known-size iterators. **No major issues found.** Notable good patterns:

```rust
// zip_service.rs
let mut files_by_folder: HashMap<String, Vec<FileDto>> =
    HashMap::with_capacity(all_folders.len());

// batch_operations.rs — uses buffer_unordered, no Vec needed
```

### 🟡 3b. `BatchResult` vectors not pre-sized

```rust
// batch_operations.rs
let mut result = BatchResult {
    successful: Vec::new(), // could be Vec::with_capacity(total)
    failed: Vec::new(),
    ...
};
```

Minor: `Vec::with_capacity(file_ids.len())` for `successful` avoids
reallocations when most operations succeed.

---

## 4. Blocking operations inside async contexts

### 🔴 4a. `std::env::var()` on every request in rate limiter

**File:** `src/interfaces/middleware/rate_limit.rs`

```rust
// rate_limit.rs — extract_client_ip()
fn extract_client_ip(req: &Request<Body>) -> String {
    let trust_proxy = std::env::var("OXICLOUD_TRUST_PROXY_HEADERS")
        .unwrap_or_default();  // BLOCKING SYSCALL per request
    ...
}
```

`std::env::var()` takes a global lock on glibc's environ and is a blocking
syscall. Called on **every single HTTP request**.

**Fix:** Read the env var once at startup into `AppConfig` (it already exists
there as `trust_proxy_headers: bool`). Pass the config to the middleware:

```rust
fn extract_client_ip(req: &Request<Body>, trust_proxy: bool) -> String { ... }
```

---

### 🟡 4b. `ip.to_string()` called twice in rate limiter

**File:** `src/interfaces/middleware/rate_limit.rs`

```rust
// rate_limit.rs — check_and_increment
pub fn check_and_increment(&self, ip: &str) -> bool {
    let current = self.requests.get(ip);  // hashes ip — String lookup OK
    // ... later:
    self.requests.insert(ip.to_string(), ...); // re-allocates String for key
}
```

The `ip` is already a `String` at the call site (`ip.to_string()` in
`extract_client_ip`). This means 2 allocations of the same IP string per
request.

**Fix:** Take `ip: String` by value, reuse it for insertion.

---

### 🟡 4c. `moka::sync::Cache` in JWT service (sync ops on async path)

**File:** `src/infrastructure/services/jwt_service.rs`

```rust
// jwt_service.rs
validation_cache: moka::sync::Cache<String, CachedValidation>,
```

`moka::sync::Cache` performs eviction inline (not background). On hot paths
this can occasionally block the Tokio thread for µs during eviction scans.
For the JWT cache (50k entries, 30s TTL) this is borderline.

**Fix:** Switch to `moka::future::Cache` which performs eviction in a
background async task, or keep `sync` but call `run_pending_tasks()` from a
periodic maintenance future.

---

## 5. HashMap hasher opportunities

### 🟡 5a. `DefaultHasher` in search cache key

**File:** `src/application/services/search_service.rs`

```rust
// search_service.rs
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn cache_key(folder_id: Option<&str>, query: &str, ...) -> u64 {
    let mut hasher = DefaultHasher::new(); // SipHash-2-4
    ...
    hasher.finish()
}
```

SipHash provides HashDoS resistance which is unnecessary for an internal cache
key derived from trusted inputs. Switching to `ahash::AHasher` or `fxhash`
saves ~5 ns per hash (relevant when search results are cached aggressively).

---

### 🟢 5b. Moka caches use their own optimised hasher

Moka internally uses a fast hasher. No action needed for moka-backed caches.

---

## 6. Lock contention patterns

### 🟢 Mostly eliminated

The codebase **correctly** uses:
- `moka` (lock-free segmented map) for all caches
- `tokio::sync::Semaphore` for bounded concurrency (Argon2, thumbnail decode)
- `AtomicU64` for hit/miss counters
- No `RwLock<HashMap<...>>` patterns

**One minor note:** The Argon2 semaphore is set to `MAX_CONCURRENT_HASHES = 2`:

```rust
// share_service.rs
const MAX_CONCURRENT_HASHES: usize = 2;
let hash_semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_HASHES));
```

This is correct for memory safety (~19 MB/hash) but could be a throughput
bottleneck if many users set/verify share passwords concurrently. Consider
making this configurable.

---

## 7. Unnecessary serialization/deserialization

### 🟢 No major issues found

DTOs are converted with hand-written `from_entity()` and `From` impls, not
round-tripped through serde. The only serde usage is at the HTTP boundary
(axum's `Json<T>`) which is unavoidable and correct.

---

## 8. Memory copies that could be zero‑copy

### 🟡 8a. File upload hashes in‑memory content after writing to disk

**File:** `src/application/services/file_upload_service.rs`

```rust
// file_upload_service.rs — create_file
let hash = blake3::hash(content); // hashes full &[u8] in memory
// content is also written to temp file...
```

For files that fit in memory (the `content: &[u8]` path), the content exists
as a slice and is hashed directly — this is fine. But the same content is then
written to a temp file for dedup, meaning the data is traversed twice (hash +
write).

**Fix:** Use `blake3::Hasher` as an `io::Write` adapter — hash while writing
to disk in a single pass:

```rust
let mut hasher = blake3::Hasher::new();
let mut file = File::create(&temp_path)?;
let mut tee = TeeWriter::new(&mut file, &mut hasher);
tee.write_all(content)?;
let hash = hasher.finalize();
```

---

### 🟡 8b. File retrieval accumulates stream into `BytesMut` for cache

**File:** `src/application/services/file_retrieval_service.rs`

```rust
// file_retrieval_service.rs — cache miss for files < 10MB
let mut buf = BytesMut::new();
while let Some(chunk) = stream.next().await {
    buf.extend_from_slice(&chunk?);
}
let content = buf.freeze(); // Bytes (O(1) clone)
```

This is the expected pattern for building a `Bytes` from a stream. The
`BytesMut` will reallocate as it grows. Pre-sizing from the known file size
would avoid reallocations:

```rust
let mut buf = BytesMut::with_capacity(file.size as usize);
```

---

## 9. Database query patterns

### 🟢 9a. No N+1 queries found

All multi-entity operations use:
- JOINs (`get_file` joins `storage.files` with `storage.blobs`)
- `COUNT(*) OVER()` window functions for paginated counts (single query)
- ltree `<@` for subtree operations (single indexed scan)
- Bulk SQL (`DELETE ... WHERE folder_id IN (SELECT ...)` for trash/delete)
- CTEs for atomic read-modify (`swap_blob_hash`, `copy_file`)

This is excellently designed.

---

### 🟡 9b. Dynamic SQL building in search (not prepared)

**File:** `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

```rust
// file_blob_read_repository.rs — search_files_paginated
let mut sql = String::with_capacity(512);
sql.push_str("SELECT ... FROM storage.files f JOIN storage.blobs b ...");
if let Some(_) = criteria.name_contains { sql.push_str(" AND f.name ILIKE ..."); }
if let Some(_) = criteria.mime_type { sql.push_str(" AND f.mime_type = ..."); }
// ... etc
```

Dynamic SQL cannot benefit from PostgreSQL's prepared statement cache (each
unique SQL text is parsed/planned separately). For the ~8 common combinations,
consider pre-building the queries or using PG's `PREPARE`/`EXECUTE`.

---

### 🟡 9c. `hash_cache` uses `String` keys

**File:** `src/infrastructure/repositories/pg/file_blob_read_repository.rs`

```rust
// file_blob_read_repository.rs
hash_cache: Cache<String, String>, // file_id → blob_hash
```

Both file IDs and blob hashes are UUIDs/hex strings (~36 bytes). Using
`Arc<str>` or a 128-bit UUID type as key would reduce per-entry heap overhead.

---

## 10. Inefficient iteration patterns

### 🟡 10a. Search results: map then sort (two passes)

**File:** `src/application/services/search_service.rs`

```rust
// search_service.rs
let enriched: Vec<_> = results.iter().map(|f| enrich_file(f, query)).collect();
enriched.sort_by(|a, b| b.relevance.total_cmp(&a.relevance));
```

Two passes: one to enrich (allocating N `EnrichedFileDto`s), another to sort.
Could be combined into a single pass that computes relevance inline and uses
`sort_unstable_by` (avoids allocation for equal-comparison temporaries):

```rust
let mut enriched: Vec<_> = results.iter().map(|f| enrich_file(f, query)).collect();
enriched.sort_unstable_by(|a, b| b.relevance.total_cmp(&a.relevance));
```

`sort_unstable_by` is ~20% faster than `sort_by` for non-trivial N.

---

### 🟡 10b. `Uuid::parse_str` called multiple times per operation (trash)

**File:** `src/application/services/trash_service.rs`

```rust
// trash_service.rs — restore_from_trash
let trash_uuid = Uuid::parse_str(trash_id)?;
let user_uuid = Uuid::parse_str(user_id)?;
// ... later in delete_permanently, same two parse calls
```

UUIDs are parsed from `&str` in every trash method. If the caller already has
validated UUIDs (e.g., from the auth middleware), accept `Uuid` directly to
skip re-parsing.

---

### 🟡 10c. `generic_batch_operation` clones every item for error reporting

**File:** `src/application/services/batch_operations.rs`

```rust
// batch_operations.rs
items.into_iter().map(|item| {
    let op = operation.clone();
    async move {
        let op_result = op(item.clone()).await; // clone just for the error arm
        (item, op_result)
    }
})
```

`item.clone()` is only needed if the operation fails (to report which item
failed). For success paths this is wasted work. Consider using an index-based
approach or `Arc<T>`.

---

## 11. Dynamic dispatch in hot paths

### 🟡 11a. `Arc<dyn FileUseCaseFactory>` in `ApplicationServices`

**File:** `src/common/di.rs`

```rust
pub struct ApplicationServices {
    pub file_use_case_factory: Arc<dyn FileUseCaseFactory>,
    ...
}
```

Every file operation goes through a `dyn` trait dispatch. The vtable indirect
call costs ~2 ns but — more importantly — prevents inlining and LTO across
the boundary. Since there is only one concrete implementation, using a concrete
type wrapped in `Arc<ConcreteFileUseCaseFactory>` would allow the compiler to
devirtualise and inline.

---

### 🟡 11b. `Box<dyn StdError + Send + Sync>` in every `DomainError`

**File:** `src/domain/errors.rs`

```rust
pub source: Option<Box<dyn StdError + Send + Sync>>,
```

Every error with a source allocates a `Box`. In hot error paths (e.g., "file
not found" during cache-miss-then-load), this adds ~30 ns of heap allocation.

**Fix:** Use a concrete error enum or `anyhow::Error` (which uses a thin
pointer and avoids the double indirection).

---

## 12. Additional findings

### 🔴 12a. `format!("{}", e).contains("not found")` for error matching

**File:** `src/application/services/trash_service.rs`

```rust
// trash_service.rs
Err(e) => {
    if format!("{}", e).contains("not found") { ... }
}
```

This allocates a `String`, formats the error into it, then does a substring
search. Happens on every trash restore/delete for missing items. The
`DomainError` already has `ErrorKind::NotFound`:

```rust
if matches!(e.kind(), ErrorKind::NotFound) { ... }
```

---

### 🟡 12b. Excessive `info!()` logging in trash service

**File:** `src/application/services/trash_service.rs`

The trash service has **14 `info!()` calls** per single `restore_from_trash`
operation and **12** per `delete_permanently`. Each `info!` allocates
`format_args!` and traverses the tracing subscriber pipeline.

**Fix:** Downgrade most to `debug!()` or `trace!()`. Keep one `info!` at the
entry point and one at the exit.

---

### 🟡 12c. `AppConfig::from_env()` reads ~60 env vars sequentially

**File:** `src/common/config.rs`

Each `std::env::var()` call acquires a global lock. At startup this is fine,
but if this function were ever called more than once it would be a bottleneck.
Currently only called once — **no action needed** unless hot-reloading is added.

---

### 🟢 12d. `BatchOperationService` takes `AppConfig` by value

**File:** `src/application/services/batch_operations.rs`

```rust
pub struct BatchOperationService {
    config: AppConfig, // owned, not Arc
    ...
}
```

At construction, the entire `AppConfig` is cloned. Since this happens once at
startup, impact is negligible, but it's inconsistent with other services that
use `Arc<AppConfig>`.

---

## Summary table

| # | Finding | Severity | Per-request cost | Fix complexity |
|---|---------|----------|-----------------|----------------|
| 1a | `CurrentUser` clone per request | 🔴 High | ~200 ns | Low |
| 2a | `DomainError` heap allocs | 🔴 High | ~60 ns × errors | Medium |
| 2b | `compute_relevance` double lowercase | 🔴 High | ~2 µs × N results | Low |
| 4a | `std::env::var()` per request | 🔴 High | ~500 ns | Low |
| 12a | `format!().contains()` error matching | 🔴 High | ~200 ns | Low |
| 1c | `mime_type.clone()` in retrieval | 🟡 Medium | ~30 ns | Low |
| 1d | `file.clone()` in search suggest | 🟡 Medium | ~100 ns × N | Low |
| 1e | `Arc<str>` → `String` in batch ops | 🟡 Medium | ~30 ns × N items | Low |
| 2c | `enrich_file` 4× String allocs | 🟡 Medium | ~120 ns × N | Medium |
| 2d | `.to_string()` on `&'static str` | 🟡 Medium | ~15 ns | Low |
| 2e | `file_id.to_string()` thumbnail key | 🟡 Medium | ~15 ns | Low |
| 4b | IP string double-alloc in rate limiter | 🟡 Medium | ~30 ns | Low |
| 4c | `moka::sync::Cache` in JWT service | 🟡 Medium | occasional µs | Medium |
| 5a | SipHash for search cache key | 🟡 Medium | ~5 ns | Low |
| 8a | Double-traverse in upload hash | 🟡 Medium | ~ms for large files | Medium |
| 8b | `BytesMut` not pre-sized | 🟡 Medium | reallocations | Low |
| 9b | Dynamic SQL not prepared | 🟡 Medium | ~50 µs parse | High |
| 10a | `sort_by` → `sort_unstable_by` | 🟡 Medium | ~20% slower sort | Low |
| 10b | Repeated `Uuid::parse_str` | 🟡 Medium | ~50 ns × calls | Low |
| 10c | `item.clone()` in generic batch | 🟡 Medium | varies | Medium |
| 11a | `dyn FileUseCaseFactory` | 🟡 Medium | ~2 ns + no inline | Medium |
| 11b | `Box<dyn Error>` per error | 🟡 Medium | ~30 ns | High |
| 12b | 14× `info!()` in trash restore | 🟡 Medium | ~1 µs total | Low |
| 3b | `BatchResult` vecs not pre-sized | 🟢 Low | rare realloc | Low |

---

## Recommended priority order

1. **`std::env::var()` in rate limiter** (4a) — 5-minute fix, blocks every request
2. **`CurrentUser` → `Arc<CurrentUser>`** (1a) — 30-minute refactor
3. **`format!().contains()` → `ErrorKind` match** (12a) — 15-minute fix
4. **Pre-lowercase query in search** (2b) — 10-minute fix
5. **`DomainError` use `Cow`** (2a) — 2-hour refactor, touches many files
6. **`BytesMut::with_capacity`** (8b) — 1-line fix
7. **Return `&'static str` from icon/mime helpers** (2c, 2d) — 30-minute refactor
8. **IP string reuse in rate limiter** (4b) — 10-minute fix
9. **`sort_unstable_by` in search** (10a) — 1-line fix
10. **Remaining items** — diminishing returns, schedule as convenient

---

## What's already excellent

The following patterns demonstrate strong performance engineering:

- **Moka lock-free caches** everywhere (file content, JWT, search, thumbnails, transcode, blob hash) — no `RwLock<HashMap>` anywhere
- **BLAKE3** for content-addressable hashing (~5× faster than SHA-256) with `update_mmap_rayon` for large files
- **Dedicated rayon thread pool** for image transcoding (isolated from Tokio's blocking pool)
- **Streaming I/O** for file downloads (64 KB chunks), ZIP creation (256 KB buffer), and database cursors
- **ltree GiST indexes** for O(log N) subtree operations
- **`COUNT(*) OVER()`** window functions — single query for paginated results + total count
- **Content-addressable dedup** with write-first strategy and atomic blob reference counting
- **`HEX_PREFIXES`** compile-time lookup table avoiding `format!()` in dedup hot path
- **Semaphore-bounded** Argon2 hashing (memory safety) and image decode (back-pressure)
- **`Arc<str>`** usage in batch operations for shared string references
- **CTE-based atomic operations** (`swap_blob_hash`, `copy_file`) — zero round-trip waste
- **PG triggers** for `ref_count` management — no Rust-side bookkeeping overhead
