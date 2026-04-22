# File Deduplication

OxiCloud uses **content-defined chunking (CDC)** with **FastCDC** and **BLAKE3** to deduplicate files at the sub-file level. Instead of storing only whole-file blobs, OxiCloud can split a file into variable-size chunks, reuse unchanged chunks across versions, and track the ordered chunk list in PostgreSQL.

## How It Works

1. OxiCloud analyzes the uploaded file with FastCDC
2. The file is split into variable-size chunks from **64 KB** to **1 MB**, targeting an average of **256 KB**
3. Each chunk is hashed with **BLAKE3** and checked against the blob index
4. Only new chunks are written to the blob backend
5. A manifest in PostgreSQL maps the whole-file hash to the ordered chunk hash list
6. Reference counts are updated so identical chunks are stored once even across multiple files or edited versions

## Storage Model

```text
storage.files      -> metadata rows that reference content
chunk_manifests    -> file_hash -> [chunk_hashes] + chunk_sizes + ref_count
storage.blobs      -> per-chunk blob metadata and reference counts
blob backend       -> actual chunk bytes on disk or remote storage
```

The manifest table is created in `migrations/20260414000000_chunk_manifests.sql` and keeps:

- `file_hash`
- ordered `chunk_hashes`
- `chunk_sizes`
- `total_size`
- `chunk_count`
- `ref_count`

## Why CDC Matters

Whole-file dedup only helps when two files are byte-for-byte identical. CDC helps when files are similar but not identical, for example:

- edited office documents
- versioned project archives
- large media files with partial changes

In those cases, unchanged chunks can be reused and only the modified portions need new storage.

## Backward Compatibility

Older uploads stored before CDC are still readable. When OxiCloud does not find a matching manifest row, it falls back to legacy whole-file blob reads.

## Cleanup Behavior

When a file is permanently deleted:

1. OxiCloud decrements the manifest reference count
2. If the last manifest reference disappears, chunk refcounts are decremented
3. Chunks with `ref_count = 0` are removed from the blob index and then deleted from the backend

This keeps storage correct even when multiple files share the same chunk set.

## Benefits

- Better storage savings for edited and versioned files
- Faster repeat uploads when many chunks already exist
- BLAKE3 hashing for fast content verification
- PostgreSQL-backed manifests for durable indexing and cleanup

## Related Endpoints

The dedup subsystem is also exposed through helper endpoints under `/api/dedup` for hash checks, deduplicated uploads, statistics, and maintenance operations.
