# 10 - ZIP and Compression

Two compression features exist: **ZIP download** (download folders as ZIP archives) and **Gzip compression** (transparent gzip for file content responses).

---

## ZIP Download

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **ZipPort** trait | `src/application/ports/zip_ports.rs` |
| Infrastructure | **ZipService** | `src/infrastructure/services/zip_service.rs` |
| Interfaces | `FolderHandler::download_folder_zip` | `src/interfaces/api/handlers/folder_handler.rs` |

### REST API

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/folders/{id}/download` | Download folder as ZIP archive |

### Implementation

- Compression method: `Deflated` via the `zip` crate
- Traversal: iterative work-queue pattern (not recursive async) using a **PendingFolder** struct
- Cycle detection: `HashSet<String>` of processed folder IDs
- ZIP is built entirely in-memory (`ZipWriter<Cursor<Vec<u8>>>`)
- UNIX permissions: `0o755` for all entries

### Port Trait

```rust
#[async_trait]
pub trait ZipPort: Send + Sync + 'static {
    async fn create_folder_zip(&self, folder_id: &str, folder_name: &str) -> Result<Vec<u8>, DomainError>;
}
```

### Example

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "https://oxicloud.example.com/api/folders/abc-123/download" \
  -o my-folder.zip
```

---

## Gzip Compression

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **CompressionPort** trait | `src/application/ports/compression_ports.rs` |
| Infrastructure | **GzipCompressionService** | `src/infrastructure/services/compression_service.rs` |

### Configuration

| Constant | Value | Description |
|---|---|---|
| `COMPRESSION_SIZE_THRESHOLD` | 50 KB | Files below this are never compressed |

### Compression Levels

```rust
pub enum CompressionLevel {
    None    = 0,
    Fast    = 1,
    Default = 6,
    Best    = 9,
}
```

### Port Trait

```rust
#[async_trait]
pub trait CompressionPort: Send + Sync + 'static {
    async fn compress_data(&self, data: &[u8], level: CompressionLevel) -> Result<Vec<u8>, DomainError>;
    async fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>, DomainError>;
    fn should_compress(&self, mime_type: &str, size: u64) -> bool;
}
```

### Skip List

These MIME types are **never compressed** (already compressed or binary):
- `image/*` (except `svg`, `bmp`)
- `audio/*`, `video/*`
- `application/zip`, `application/gzip`, `application/x-compressed`
- `application/x-7z-compressed`, `application/x-rar-compressed`
- `application/x-bzip2`, `application/x-xz`

### Implementation Details

- Uses the `flate2` crate (`GzEncoder` / `GzDecoder`)
- Compress/decompress run inside `spawn_blocking` to avoid blocking the async runtime
- Optional **BufferPool** integration for buffer reuse
- Buffer pool estimates: 80% of input size for compression, 5x for decompression
