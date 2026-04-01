# 03 - Storage Safety

OxiCloud ensures data integrity and durability through a combination of PostgreSQL transactional guarantees and atomic blob writes. The goal: writes either complete fully or not at all, data reaches persistent storage, and the system recovers from crashes or power loss.

---

## Storage Model

OxiCloud uses a **100% blob storage model**:

- **Metadata** (file names, folder hierarchy, sizes, MIME types, trash status) lives in **PostgreSQL** — protected by ACID transactions.
- **File content** is stored as content-addressed blobs via **DedupService** at `.blobs/{prefix}/{hash}.blob` — protected by atomic writes and fsync.

---

## PostgreSQL Safety (Metadata)

All file and folder metadata operations use PostgreSQL transactions:

- **Single-row operations** (INSERT, UPDATE, DELETE) are inherently atomic.
- **Multi-step operations** (e.g., move file: UPDATE folder_id + UPDATE path) use explicit transactions via `sqlx`.
- **Foreign key constraints** prevent orphaned records (e.g., files referencing non-existent folders).
- **Unique constraints** prevent duplicate names within the same parent folder.
- **Soft-delete** for trash (`is_trashed = TRUE`) preserves data until explicit permanent deletion.

The `storage.trash_items` VIEW provides a unified read interface over trashed files and folders without duplicating data.

---

## Blob Storage Safety (Content)

### DedupService Atomic Writes

**File:** `src/infrastructure/services/dedup_service.rs`

When storing file content, DedupService uses the following pattern:

1. **Hash computation** — SHA-256 hash of content determines the blob path
2. **Deduplication check** — if a blob with the same hash exists, only increment the reference counter (no write needed)
3. **Atomic write** — if new content:
   - Write to a temporary file (`.blob.tmp`)
   - Call `fsync` to ensure data reaches persistent storage
   - Atomically rename temp file to final path (`.blobs/{prefix}/{hash}.blob`)
4. **Reference counting** — track how many files reference each blob

This ensures that a blob either fully exists or doesn't — no partial writes.

### FileSystemUtils

**File:** `src/infrastructure/services/file_system_utils.rs`

Low-level utilities used internally by DedupService and other infrastructure services:

```rust
/// Atomic write: temp file → fsync → rename
pub async fn atomic_write<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<(), IoError>

/// Directory creation with fsync
pub async fn create_dir_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError>

/// Rename with directory sync
pub async fn rename_with_sync<P, Q>(from: P, to: Q) -> Result<(), IoError>

/// Delete with directory sync
pub async fn remove_file_with_sync<P: AsRef<Path>>(path: P) -> Result<(), IoError>
```

### fsync Guarantees

- `sync_all()` on written files ensures data and metadata reach the physical storage device
- Directory entries are synced after create/rename/delete operations
- Prevents data loss during crashes or power failures between OS buffer flush and disk write

---

## Transaction Flow: File Upload

```
1. DedupService.store_bytes(content)
   → Compute SHA-256 hash
   → Check if blob exists (dedup hit → increment ref, return hash)
   → Write to .blobs/{prefix}/{hash}.blob.tmp
   → fsync + rename → .blobs/{prefix}/{hash}.blob

2. FileBlobWriteRepository.save_file()
   → BEGIN TRANSACTION
   → INSERT INTO storage.files (name, folder_id, blob_hash, size, ...)
   → COMMIT
```

If step 1 fails, no metadata is written. If step 2 fails, the blob exists but is unreferenced (cleaned up by garbage collection). Data is never in an inconsistent state.

## Transaction Flow: File Deletion

```
1. FileBlobWriteRepository.delete_file_permanently()
   → BEGIN TRANSACTION
   → DELETE FROM storage.files WHERE id = $1 (captures blob_hash first)
   → COMMIT

2. DedupService.decrement_ref(blob_hash)
   → Decrement reference counter
   → If counter reaches 0, delete the blob file
```

If step 2 fails, an unreferenced blob may remain on disk (occupies space but is not a correctness issue). Future garbage collection can clean these up.

---

## Benefits

1. **ACID transactions** — metadata operations are atomic, consistent, isolated, and durable
2. **Content-addressable storage** — identical content is stored once, referenced by hash
3. **Crash resilience** — atomic blob writes + PostgreSQL WAL ensure recovery
4. **No partial writes** — temp file + rename pattern guarantees all-or-nothing
5. **Referential integrity** — foreign keys prevent orphaned metadata

---

## Performance Considerations

- PostgreSQL connection pooling (`sqlx::PgPool`) amortizes connection overhead
- Dedup hash computation is CPU-bound but avoids unnecessary disk writes for duplicate content
- Blob fsync adds latency vs. buffered writes, but ensures durability for critical user data
- Content cache (in-memory LRU) serves repeat reads without disk or DB access
