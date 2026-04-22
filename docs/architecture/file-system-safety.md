# Storage Safety

OxiCloud protects file integrity with two layers working together:

- PostgreSQL transactions for metadata
- Atomic blob writes for content

The result is a simple guarantee: operations either complete fully or fail cleanly enough to recover without corrupting user data.

## Storage Model

- Metadata such as names, folders, MIME types, quotas, and trash state lives in PostgreSQL
- File content is stored as content-addressed blobs under the storage backend
- Deduplication metadata is tracked separately so multiple files can reference the same content safely

## Metadata Safety

PostgreSQL protects metadata with ACID transactions.

- Single-row writes are atomic by default
- Multi-step operations use explicit transactions
- Foreign keys prevent orphaned references
- Unique constraints prevent illegal duplicates in the same scope
- Trash uses soft-delete semantics until permanent deletion is requested

## Content Safety

Blob writes rely on an atomic write pattern:

1. Write new content to a temporary file
2. `fsync` the file to push data and metadata to durable storage
3. Rename the temp file into its final content-addressed path
4. Sync parent directory metadata when needed

This prevents partially written blobs from appearing as valid stored content.

## Deduplication Notes

OxiCloud's deduplication pipeline uses BLAKE3 hashing and chunk manifest tracking. The storage layer can therefore reuse identical content while still keeping metadata changes transactional.

If content is stored successfully but the later metadata transaction fails, the content may remain as an unreferenced blob. That is a space leak, not a consistency leak, and can be cleaned up later.

## Upload Flow

```text
1. Receive content and spool it safely to storage
2. Finalize the content-addressed blob write
3. Begin metadata transaction
4. Insert or update file metadata in PostgreSQL
5. Commit
```

If step 1 or 2 fails, no metadata is committed. If step 4 or 5 fails, metadata rolls back and the storage layer can clean up unreferenced content later.

## Delete Flow

```text
1. Begin metadata transaction
2. Remove or soft-delete the metadata row
3. Commit
4. Decrement blob references and remove physical content when the refcount reaches zero
```

If the metadata transaction fails, the physical file is not considered deleted. If the refcount cleanup fails, the system may keep extra content on disk, but user-visible metadata remains correct.

## Why This Matters

- Crash resilience during uploads and deletes
- Safe recovery after power loss or host restarts
- Clean separation between metadata correctness and background storage cleanup
- Predictable behavior for trash, deduplication, and shared storage backends

## Related Pages

- [Internal Architecture](/architecture/)
- [Database Transactions](/architecture/database-transactions)
- [Storage Quotas](/architecture/storage-quotas)