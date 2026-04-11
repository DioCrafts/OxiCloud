# File Deduplication

OxiCloud uses **SHA-256 content-addressable storage** to avoid storing duplicate files. If two users upload the same file, only one copy is stored on disk.

## How It Works

1. When a file is uploaded, its SHA-256 hash is computed
2. The hash is checked against the blob store (`.blobs/{prefix}/{hash}.blob`)
3. If a blob with that hash already exists, the file metadata points to the existing blob (no extra disk usage)
4. If not, the content is saved as a new blob
5. A reference counter tracks how many files point to each blob

## Automatic Cleanup

When a file is permanently deleted:

1. The blob's reference count is decremented
2. If the reference count reaches zero, the blob is removed from disk

This means disk space is only freed when the **last** reference to a blob is removed.

## Storage Layout

```
storage/
├── .blobs/
│   ├── a1/
│   │   └── a1b2c3d4...sha256.blob
│   ├── f8/
│   │   └── f8e7d6c5...sha256.blob
│   └── ...
```

The first two hex characters of the hash are used as a directory prefix to avoid having millions of files in a single directory.

## Benefits

- **Disk savings** — identical files across users consume storage only once
- **Instant uploads** — if the blob already exists, the upload completes immediately
- **Integrity** — SHA-256 ensures bit-for-bit correctness

## Limitations

- Deduplication is based on exact content match (byte-identical files)
- Near-duplicate files (e.g., a JPEG re-saved at slightly different quality) are stored separately
- Encryption at rest would require per-user keys, which breaks deduplication (planned as opt-in)
