# 08 - Thumbnails and Transcoding

OxiCloud provides two image optimization features:

- **Thumbnails**: on-demand generation of WebP thumbnails in 3 sizes, with background pre-generation on upload
- **Image Transcoding**: automatic JPEG/PNG/GIF → WebP conversion based on browser `Accept` header

Both features use multi-level caching (memory LRU + disk) and are non-blocking.

---

## Thumbnails

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **ThumbnailPort** trait, **ThumbnailSize** enum | `src/application/ports/thumbnail_ports.rs` |
| Infrastructure | **ThumbnailService** | `src/infrastructure/services/thumbnail_service.rs` |
| Interfaces | Integrated in **FileHandler** | `src/interfaces/api/handlers/file_handler.rs` |

### Thumbnail Sizes

| Size | Dimensions | Directory Name |
|---|---|---|
| `Icon` | 150x150 | `icon` |
| `Preview` | 400x400 | `preview` |
| `Large` | 800x800 | `large` |

### Supported Formats

Input: `image/jpeg`, `image/jpg`, `image/png`, `image/gif`, `image/webp`

Output: always **WebP** (using Lanczos3 resize filter)

### Storage Layout

```
<storage_path>/
  .thumbnails/
    icon/
      <file_id>.webp
    preview/
      <file_id>.webp
    large/
      <file_id>.webp
```

### REST API

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/files/{id}/thumbnail/{size}` | Get thumbnail (`size` = `icon` / `preview` / `large`) |
| `POST` | `/api/files/upload` | Upload file + auto-generate thumbnails for images |

Response headers for thumbnail GET:
- `Content-Type: image/webp`
- `Cache-Control: public, max-age=31536000, immutable`
- `ETag: "thumb-{id}-{size}"`

### Generation Flow

1. **On upload** (`upload_file_with_thumbnails`): after successful file upload, if the MIME type is a supported image, thumbnails for all 3 sizes are generated in a background `tokio::spawn` task
2. **On GET** (lazy): if a thumbnail doesn't exist on disk, it's generated on-demand, cached in memory and on disk

### Caching

- **Memory LRU cache**: configurable max entries and max bytes
- **Disk cache**: persistent WebP files in `.thumbnails/`
- Cache lookup order: memory → disk → generate

### Port Trait

```rust
#[async_trait]
pub trait ThumbnailPort: Send + Sync + 'static {
    fn is_supported_image(&self, mime_type: &str) -> bool;
    async fn get_thumbnail(&self, file_id: &str, size: ThumbnailSize, original_path: &Path) -> Result<Bytes, DomainError>;
    fn generate_all_sizes_background(self: Arc<Self>, file_id: String, original_path: PathBuf);
    async fn delete_thumbnails(&self, file_id: &str) -> Result<(), DomainError>;
    async fn get_stats(&self) -> ThumbnailStatsDto;
}
```

---

## Image Transcoding

### Architecture

| Layer | Component | File |
|---|---|---|
| Application Port | **ImageTranscodePort** trait, **OutputFormat**, **BrowserCapabilities** | `src/application/ports/transcode_ports.rs` |
| Infrastructure | **ImageTranscodeService** | `src/infrastructure/services/image_transcode_service.rs` |
| Interfaces | Integrated in download handler | `src/interfaces/api/handlers/file_handler.rs` |

### How It Works

1. Download handler reads the `Accept` header from the HTTP request
2. `BrowserCapabilities::from_accept_header()` determines if the browser supports WebP
3. If the file is a transcodable image (JPEG/PNG/GIF, ≤5 MB) and the browser supports WebP: check memory cache → check disk cache → transcode on miss
4. If WebP output is **larger** than the original, serve the original instead (smart skip)

### Constants

| Constant | Value | Description |
|---|---|---|
| `MAX_TRANSCODE_SIZE` | 5 MB | Files above this skip transcoding |

### Supported Input Formats

`image/jpeg`, `image/jpg`, `image/png`, `image/gif`

Not transcoded: `image/webp` (already optimal), `image/svg+xml`, `image/bmp`

### Storage Layout

```
<storage_path>/
  .transcoded/
    webp/
      <file_id>.webp
```

### Caching

- **Memory LRU cache**: configurable max entries and max bytes
- **Disk cache**: persistent WebP files in `.transcoded/webp/`
- Disk writes are fire-and-forget via `tokio::spawn` (non-blocking)
- `invalidate(file_id)` evicts from both memory and disk

### Port Trait

```rust
#[async_trait]
pub trait ImageTranscodePort: Send + Sync + 'static {
    fn can_transcode(&self, mime_type: &str) -> bool;
    fn should_transcode(&self, mime_type: &str, file_size: u64) -> bool;
    async fn get_transcoded(&self, file_id: &str, original_content: &[u8], original_mime: &str, target_format: OutputFormat)
        -> Result<(Bytes, String, bool), DomainError>;
    async fn invalidate(&self, file_id: &str);
    async fn get_stats(&self) -> TranscodeStatsDto;
    async fn clear_cache(&self) -> Result<(), DomainError>;
}
```

### Browser Detection

```rust
pub struct BrowserCapabilities {
    pub supports_webp: bool,
    pub supports_avif: bool,  // reserved for future
}

impl BrowserCapabilities {
    pub fn from_accept_header(accept: Option<&str>) -> Self;
    pub fn best_format(&self) -> Option<OutputFormat>;
}
```

### Statistics

```rust
pub struct TranscodeStatsDto {
    pub cache_hits: u64,
    pub disk_hits: u64,
    pub transcodes: u64,
    pub bytes_saved: u64,
    pub transcode_errors: u64,
}
```

### Example Flow

```
Client: GET /api/files/abc-123/download
        Accept: image/webp, image/png, */*

Server: 1. File is "photo.jpg" (800KB) → can transcode yes, should transcode yes
        2. Check memory cache → miss
        3. Check disk cache (.transcoded/webp/abc-123.webp) → miss
        4. Transcode JPEG → WebP (600KB) → smaller yes
        5. Cache in memory + async write to disk
        6. Respond with WebP (saves 200KB, 25% reduction)
```

## Dependencies

- `image = "0.25"` (with `jpeg`, `png`, `gif`, `webp` features) -- used by both thumbnail and transcode
- Thumbnail resize: `imageops::resize` with `FilterType::Lanczos3`
- WebP encoding: via `image` crate's WebP encoder
