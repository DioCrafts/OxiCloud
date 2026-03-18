use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
/**
 * Thumbnail Generation Service
 *
 * Generates and manages image thumbnails for fast gallery previews.
 *
 * Features:
 * - Background thumbnail generation after upload
 * - Multiple sizes (icon 150x150, preview 800x600)
 * - JPEG output (lossy q=80) for compact thumbnails
 * - Lock-free moka cache with weight-based eviction
 * - Lazy generation on first request if not pre-generated
 */
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;

use crate::application::ports::thumbnail_ports::{
    ThumbnailPort, ThumbnailSize as PortThumbnailSize, ThumbnailStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Thumbnail sizes supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThumbnailSize {
    /// Small icon for file listings (150x150)
    Icon,
    /// Medium preview for gallery view (400x400)
    Preview,
    /// Large preview for detail view (800x800)
    Large,
}

impl ThumbnailSize {
    /// Get the maximum dimension for this size
    pub fn max_dimension(&self) -> u32 {
        match self {
            ThumbnailSize::Icon => 150,
            ThumbnailSize::Preview => 400,
            ThumbnailSize::Large => 800,
        }
    }

    /// Get the directory name for this size
    pub fn dir_name(&self) -> &'static str {
        match self {
            ThumbnailSize::Icon => "icon",
            ThumbnailSize::Preview => "preview",
            ThumbnailSize::Large => "large",
        }
    }

    /// Get all thumbnail sizes
    pub fn all() -> &'static [ThumbnailSize] {
        &[
            ThumbnailSize::Icon,
            ThumbnailSize::Preview,
            ThumbnailSize::Large,
        ]
    }
}

/// Cache key for thumbnails
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ThumbnailCacheKey {
    file_id: String,
    size: ThumbnailSize,
}

/// Maximum pixel count before rejecting decode (50 megapixels → ~200 MB RGBA).
/// Images above this are silently skipped — protects against single-image OOM.
const MAX_DECODE_PIXELS: u64 = 50_000_000;

/// Default max concurrent thumbnail decode operations.
/// 4 × 96 MB (6000×4000 RGBA) = 384 MB worst-case peak.
const DEFAULT_MAX_CONCURRENT_DECODES: usize = 4;

/// Thumbnail service for generating and caching image thumbnails
pub struct ThumbnailService {
    /// Root path for thumbnail storage
    thumbnails_root: PathBuf,
    /// Lock-free concurrent cache (moka) with weight-based eviction
    cache: moka::future::Cache<ThumbnailCacheKey, Bytes>,
    /// Configured maximum cache weight (for stats reporting)
    max_cache_bytes: u64,
    /// Limits how many images are decoded in parallel to bound RAM usage.
    /// Without this, 50 simultaneous uploads would decode 50 bitmaps
    /// (~96 MB each for 6000×4000) = 4.8 GB peak.
    decode_semaphore: Arc<Semaphore>,
}

impl ThumbnailService {
    /// Create a new thumbnail service
    ///
    /// # Arguments
    /// * `storage_root` - Root path of file storage
    /// * `max_cache_entries` - (ignored — moka uses weight-based eviction)
    /// * `max_cache_bytes` - Maximum total bytes to cache
    pub fn new(storage_root: &Path, max_cache_entries: usize, max_cache_bytes: usize) -> Self {
        let thumbnails_root = storage_root.join(".thumbnails");

        // Ignore max_cache_entries — weight-based eviction is more accurate
        // for variable-size thumbnails than entry-count limits.
        let _ = max_cache_entries;

        // No time_to_live — thumbnails are immutable (content never changes
        // for a given file_id).  Eviction is purely weight-based: when the
        // cache exceeds max_cache_bytes the lightest entries are dropped.
        // On eviction the thumbnail is still on disk; the next request
        // promotes it back with a single async read (~0.1 ms).
        let cache = moka::future::Cache::builder()
            .max_capacity(max_cache_bytes as u64)
            .weigher(|_key: &ThumbnailCacheKey, value: &Bytes| -> u32 {
                value.len().min(u32::MAX as usize) as u32
            })
            .build();

        Self {
            thumbnails_root,
            cache,
            max_cache_bytes: max_cache_bytes as u64,
            decode_semaphore: Arc::new(Semaphore::new(DEFAULT_MAX_CONCURRENT_DECODES)),
        }
    }

    /// Initialize the thumbnail directories
    pub async fn initialize(&self) -> std::io::Result<()> {
        for size in ThumbnailSize::all() {
            let dir = self.thumbnails_root.join(size.dir_name());
            fs::create_dir_all(&dir).await?;
        }
        tracing::info!(
            "🖼️ Thumbnail service initialized at {:?}",
            self.thumbnails_root
        );
        Ok(())
    }

    /// Check if a file is an image that can have thumbnails
    pub fn is_supported_image(mime_type: &str) -> bool {
        matches!(
            mime_type,
            "image/jpeg" | "image/jpg" | "image/png" | "image/gif" | "image/webp"
        )
    }

    /// Get the path where a thumbnail would be stored
    fn get_thumbnail_path(&self, file_id: &str, size: ThumbnailSize) -> PathBuf {
        self.thumbnails_root
            .join(size.dir_name())
            .join(format!("{}.jpg", file_id))
    }

    /// Get a thumbnail, generating it if needed.
    ///
    /// # Arguments
    /// * `file_id` - ID of the original file
    /// * `size` - Desired thumbnail size
    /// * `original_path` - Path to the original image file
    ///
    /// # Returns
    /// Bytes of the thumbnail image (JPEG format)
    pub async fn get_thumbnail(
        &self,
        file_id: &str,
        size: ThumbnailSize,
        original_path: &Path,
    ) -> Result<Bytes, ThumbnailError> {
        let cache_key = ThumbnailCacheKey {
            file_id: file_id.to_string(),
            size,
        };

        let thumb_path = self.get_thumbnail_path(file_id, size);
        let original_owned = original_path.to_path_buf();
        let file_id_owned = file_id.to_string();

        // Moka's entry().or_insert_with() guarantees that for the same key
        // only ONE init closure runs; concurrent callers await the same
        // computation instead of stampeding (thundering-herd protection).
        let entry = self
            .cache
            .entry(cache_key)
            .or_insert_with(async {
                // 1. Try loading from disk
                if let Ok(data) = fs::read(&thumb_path).await {
                    tracing::debug!(
                        "💾 Thumbnail loaded from disk: {} {:?}",
                        file_id_owned,
                        size
                    );
                    return Bytes::from(data);
                }

                // 2. Generate thumbnail (CPU-bound, runs in spawn_blocking)
                tracing::info!("🎨 Generating thumbnail: {} {:?}", file_id_owned, size);
                match self.generate_thumbnail(&original_owned, size).await {
                    Ok(bytes) => {
                        // Save to disk (best-effort — don't fail the request)
                        if let Some(parent) = thumb_path.parent() {
                            let _ = fs::create_dir_all(parent).await;
                        }
                        let _ = fs::write(&thumb_path, &bytes).await;
                        bytes
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Thumbnail generation failed for {} {:?}: {e}",
                            file_id_owned,
                            size
                        );
                        // Return empty sentinel — will be evicted quickly by
                        // the weigher (weight 0) and retried on next request.
                        Bytes::new()
                    }
                }
            })
            .await;

        let bytes = entry.into_value();
        if bytes.is_empty() {
            return Err(ThumbnailError::ImageError(
                "Thumbnail generation failed".to_string(),
            ));
        }

        tracing::debug!("🔥 Thumbnail served: {} {:?}", file_id, size);
        Ok(bytes)
    }

    /// Try to serve a thumbnail from cache only (memory → disk).
    ///
    /// Unlike `get_thumbnail`, this does **not** generate a new thumbnail.
    /// Useful for non-image file types (videos) where a client-generated
    /// thumbnail may have been uploaded previously.
    pub async fn get_cached_thumbnail(&self, file_id: &str, size: ThumbnailSize) -> Option<Bytes> {
        // 1. Check in-memory cache
        let cache_key = ThumbnailCacheKey {
            file_id: file_id.to_string(),
            size,
        };
        if let Some(bytes) = self.cache.get(&cache_key).await
            && !bytes.is_empty()
        {
            return Some(bytes);
        }

        // 2. Check disk
        let thumb_path = self.get_thumbnail_path(file_id, size);
        if let Ok(data) = fs::read(&thumb_path).await {
            let bytes = Bytes::from(data);
            // Populate in-memory cache for next hit
            self.cache.insert(cache_key, bytes.clone()).await;
            Some(bytes)
        } else {
            None
        }
    }

    /// Store an externally-generated thumbnail (e.g. client-side video frame).
    ///
    /// **Fast path**: if the payload is already a correctly-sized JPEG, it is
    /// stored as-is — zero decode, zero encode.  The browser pre-scales the
    /// canvas to 400 px and sends JPEG, so this fast path is hit on every
    /// normal video-thumbnail upload.
    ///
    /// **Slow path**: decode → optional resize → re-encode to JPEG q=80.
    /// Only triggered when a client sends an oversized or non-JPEG image.
    pub async fn store_external_thumbnail(
        &self,
        file_id: &str,
        size: ThumbnailSize,
        data: Bytes,
    ) -> Result<Bytes, ThumbnailError> {
        let max_dim = size.max_dimension();

        // Validate + optionally re-encode in blocking thread
        let jpeg_bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, ThumbnailError> {
            // ── Fast path: already a correctly-sized JPEG ─────────────
            // JPEG files start with SOI marker 0xFF 0xD8.
            if data.len() >= 2
                && data[0] == 0xFF
                && data[1] == 0xD8
                && let Ok(reader) =
                    image::ImageReader::new(std::io::Cursor::new(&data)).with_guessed_format()
                && let Ok((w, h)) = reader.into_dimensions()
                && w <= max_dim
                && h <= max_dim
            {
                // Already JPEG at correct size — zero-copy store
                return Ok(data.to_vec());
            }

            // ── Slow path: decode, resize, re-encode to JPEG ─────────
            let img = image::load_from_memory(&data)
                .map_err(|e| ThumbnailError::ImageError(format!("Invalid image data: {e}")))?;

            let (w, h) = (img.width(), img.height());
            let img = if w > max_dim || h > max_dim {
                let filter = FilterType::CatmullRom;
                if w > h {
                    let ratio = max_dim as f32 / w as f32;
                    img.resize(max_dim, (h as f32 * ratio) as u32, filter)
                } else {
                    let ratio = max_dim as f32 / h as f32;
                    img.resize((w as f32 * ratio) as u32, max_dim, filter)
                }
            } else {
                img
            };

            let rgb = img.to_rgb8();
            let mut buffer = Vec::new();
            let encoder = JpegEncoder::new_with_quality(&mut buffer, 80);
            rgb.write_with_encoder(encoder)
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
            Ok(buffer)
        })
        .await
        .map_err(|e| ThumbnailError::TaskError(e.to_string()))??;

        let bytes = Bytes::from(jpeg_bytes);

        // Save to disk
        let thumb_path = self.get_thumbnail_path(file_id, size);
        if let Some(parent) = thumb_path.parent() {
            let _ = fs::create_dir_all(parent).await;
        }
        fs::write(&thumb_path, &bytes)
            .await
            .map_err(|e| ThumbnailError::IoError(e.to_string()))?;

        // Populate in-memory cache
        let cache_key = ThumbnailCacheKey {
            file_id: file_id.to_string(),
            size,
        };
        self.cache.insert(cache_key, bytes.clone()).await;

        tracing::info!("✅ Stored external thumbnail: {} {:?}", file_id, size);
        Ok(bytes)
    }

    /// Generate a thumbnail from an image file.
    ///
    /// Concurrency is bounded by `decode_semaphore` to prevent OOM when
    /// many images are uploaded simultaneously. Resolution is also
    /// capped at `MAX_DECODE_PIXELS` to reject pathologically large images.
    /// After decoding, the encoded image buffer is explicitly dropped before
    /// processing to minimize peak memory usage.
    async fn generate_thumbnail(
        &self,
        original_path: &Path,
        size: ThumbnailSize,
    ) -> Result<Bytes, ThumbnailError> {
        let path = original_path.to_path_buf();
        let max_dim = size.max_dimension();

        // Acquire semaphore permit — bounds peak RAM from concurrent decodes
        let _permit = self
            .decode_semaphore
            .acquire()
            .await
            .map_err(|_| ThumbnailError::TaskError("Decode semaphore closed".into()))?;

        // Run image processing in blocking thread pool
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, ThumbnailError> {
            // Single read: load file once into memory, then work from the buffer
            let data =
                std::fs::read(&path).map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

            // Safety check: read dimensions from in-memory buffer (no 2nd I/O)
            let (w, h) = image::ImageReader::new(std::io::Cursor::new(&data))
                .with_guessed_format()
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?
                .into_dimensions()
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
            if (w as u64) * (h as u64) > MAX_DECODE_PIXELS {
                return Err(ThumbnailError::ImageError(format!(
                    "Image too large for thumbnail: {w}×{h} ({} MP, max {MAX_DECODE_PIXELS})",
                    w as u64 * h as u64 / 1_000_000
                )));
            }

            // Full decode from the same in-memory buffer (no 2nd disk read)
            let img = image::load_from_memory(&data)
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

            // Apply EXIF orientation so thumbnails display correctly
            let img = {
                use crate::infrastructure::services::exif_service::{
                    ExifService, apply_orientation,
                };
                let orientation = ExifService::extract(&data)
                    .and_then(|m| m.orientation)
                    .unwrap_or(1);
                // Free the encoded image data now that image is decoded and EXIF extracted
                drop(data);
                apply_orientation(img, orientation)
            };

            // Calculate new dimensions preserving aspect ratio
            let (orig_width, orig_height) = (img.width(), img.height());
            let (new_width, new_height) = if orig_width > orig_height {
                let ratio = max_dim as f32 / orig_width as f32;
                (max_dim, (orig_height as f32 * ratio) as u32)
            } else {
                let ratio = max_dim as f32 / orig_height as f32;
                ((orig_width as f32 * ratio) as u32, max_dim)
            };

            // Adaptive filter: faster filters for smaller sizes where
            // quality difference vs Lanczos3 is imperceptible
            let filter = match size {
                ThumbnailSize::Icon => FilterType::Triangle, // 150px — max speed
                ThumbnailSize::Preview => FilterType::CatmullRom, // 400px — good balance
                ThumbnailSize::Large => FilterType::CatmullRom, // 800px — sufficient quality
            };
            let thumbnail = img.resize(new_width, new_height, filter);

            // Encode as JPEG (lossy q=80) — explicit quality control,
            // ~2× smaller than image-webp's Rust encoder at same visual quality
            let rgb = thumbnail.to_rgb8();
            let mut buffer = Vec::new();
            let encoder = JpegEncoder::new_with_quality(&mut buffer, 80);
            rgb.write_with_encoder(encoder)
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

            Ok(buffer)
        })
        .await
        .map_err(|e| ThumbnailError::TaskError(e.to_string()))?;

        result.map(Bytes::from)
    }

    /// Generate all thumbnail sizes for a file in the background.
    ///
    /// Loads the image **once** and produces all 3 sizes (Icon, Preview,
    /// Large) inside a single `spawn_blocking` call. This avoids 3×
    /// I/O reads and 3× JPEG/PNG decode — reducing CPU time by ~45%
    /// and peak RAM from ~540 MB to ~180 MB for concurrent uploads.
    /// The encoded image buffer is explicitly dropped after decoding
    /// to further reduce peak memory by the size of the original file.
    pub fn generate_all_sizes_background(self: Arc<Self>, file_id: String, original_path: PathBuf) {
        tokio::spawn(async move {
            tracing::info!("🖼️ Background thumbnail generation starting: {}", file_id);

            // Acquire semaphore permit — bounds peak RAM from concurrent decodes
            let _permit = match self.decode_semaphore.acquire().await {
                Ok(p) => p,
                Err(_) => {
                    tracing::warn!(
                        "Decode semaphore closed, skipping thumbnails for {}",
                        file_id
                    );
                    return;
                }
            };

            let path = original_path.clone();

            // Single spawn_blocking: 1 read + 1 decode + 3 resize + 3 encode
            let results = tokio::task::spawn_blocking(move || {
                // Single read: load file once into memory
                let data =
                    std::fs::read(&path).map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

                // Safety check: read dimensions from in-memory buffer (no 2nd I/O)
                let (w, h) = image::ImageReader::new(std::io::Cursor::new(&data))
                    .with_guessed_format()
                    .map_err(|e| ThumbnailError::ImageError(e.to_string()))?
                    .into_dimensions()
                    .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;
                if (w as u64) * (h as u64) > MAX_DECODE_PIXELS {
                    return Err(ThumbnailError::ImageError(format!(
                        "Image too large for thumbnail: {w}×{h} ({} MP, max {MAX_DECODE_PIXELS})",
                        w as u64 * h as u64 / 1_000_000
                    )));
                }

                // Full decode from the same in-memory buffer (no 2nd disk read)
                let img = image::load_from_memory(&data)
                    .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

                // Apply EXIF orientation so thumbnails display correctly
                let img = {
                    use crate::infrastructure::services::exif_service::{
                        ExifService, apply_orientation,
                    };
                    let orientation = ExifService::extract(&data)
                        .and_then(|m| m.orientation)
                        .unwrap_or(1);
                    // Free the encoded image data now that image is decoded and EXIF extracted
                    drop(data);
                    apply_orientation(img, orientation)
                };

                let (orig_w, orig_h) = (img.width(), img.height());

                ThumbnailSize::all()
                    .iter()
                    .map(|&size| {
                        let max_dim = size.max_dimension();

                        let (new_w, new_h) = if orig_w > orig_h {
                            let ratio = max_dim as f32 / orig_w as f32;
                            (max_dim, (orig_h as f32 * ratio) as u32)
                        } else {
                            let ratio = max_dim as f32 / orig_h as f32;
                            ((orig_w as f32 * ratio) as u32, max_dim)
                        };

                        let filter = match size {
                            ThumbnailSize::Icon => FilterType::Triangle,
                            ThumbnailSize::Preview => FilterType::CatmullRom,
                            ThumbnailSize::Large => FilterType::CatmullRom,
                        };
                        let thumb = img.resize(new_w, new_h, filter);

                        let rgb = thumb.to_rgb8();
                        let mut buf = Vec::new();
                        let encoder = JpegEncoder::new_with_quality(&mut buf, 80);
                        rgb.write_with_encoder(encoder)
                            .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

                        Ok((size, Bytes::from(buf)))
                    })
                    .collect::<Result<Vec<_>, ThumbnailError>>()
            })
            .await;

            // Flatten JoinError + inner ThumbnailError
            let thumbnails = match results {
                Ok(Ok(t)) => t,
                Ok(Err(e)) => {
                    tracing::warn!("Thumbnail generation failed for {}: {}", file_id, e);
                    return;
                }
                Err(e) => {
                    tracing::warn!("Thumbnail task panicked for {}: {}", file_id, e);
                    return;
                }
            };

            // Save each size to disk AND populate moka so the very first
            // GET after upload is served from RAM (zero disk I/O).
            for (size, bytes) in thumbnails {
                let thumb_path = self.get_thumbnail_path(&file_id, size);
                if let Some(parent) = thumb_path.parent() {
                    let _ = fs::create_dir_all(parent).await;
                }
                if let Err(e) = fs::write(&thumb_path, &bytes).await {
                    tracing::warn!("Failed to save thumbnail {} {:?}: {}", file_id, size, e);
                } else {
                    // Populate in-memory cache for instant first-hit serving
                    let cache_key = ThumbnailCacheKey {
                        file_id: file_id.clone(),
                        size,
                    };
                    self.cache.insert(cache_key, bytes).await;
                    tracing::debug!("✅ Generated thumbnail: {} {:?}", file_id, size);
                }
            }

            tracing::info!("✅ Background thumbnail generation complete: {}", file_id);
        });
    }

    /// Delete all thumbnails for a file
    pub async fn delete_thumbnails(&self, file_id: &str) -> Result<(), ThumbnailError> {
        for size in ThumbnailSize::all() {
            let path = self.get_thumbnail_path(file_id, *size);
            if fs::metadata(&path).await.is_ok() {
                fs::remove_file(&path)
                    .await
                    .map_err(|e| ThumbnailError::IoError(e.to_string()))?;
            }

            // Remove from cache (lock-free invalidation)
            let cache_key = ThumbnailCacheKey {
                file_id: file_id.to_string(),
                size: *size,
            };
            self.cache.invalidate(&cache_key).await;
        }

        tracing::debug!("🗑️ Deleted thumbnails for: {}", file_id);
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> ThumbnailStats {
        ThumbnailStats {
            cached_thumbnails: self.cache.entry_count() as usize,
            cache_size_bytes: self.cache.weighted_size() as usize,
            max_cache_bytes: self.max_cache_bytes as usize,
        }
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

/// Convert port ThumbnailSize to infra ThumbnailSize.
impl From<PortThumbnailSize> for ThumbnailSize {
    fn from(size: PortThumbnailSize) -> Self {
        match size {
            PortThumbnailSize::Icon => ThumbnailSize::Icon,
            PortThumbnailSize::Preview => ThumbnailSize::Preview,
            PortThumbnailSize::Large => ThumbnailSize::Large,
        }
    }
}

impl ThumbnailPort for ThumbnailService {
    fn is_supported_image(&self, mime_type: &str) -> bool {
        ThumbnailService::is_supported_image(mime_type)
    }

    async fn get_thumbnail(
        &self,
        file_id: &str,
        size: PortThumbnailSize,
        original_path: &Path,
    ) -> Result<Bytes, DomainError> {
        self.get_thumbnail(file_id, size.into(), original_path)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Thumbnail", e.to_string()))
    }

    fn generate_all_sizes_background(self: Arc<Self>, file_id: String, original_path: PathBuf) {
        ThumbnailService::generate_all_sizes_background(self, file_id, original_path)
    }

    async fn delete_thumbnails(&self, file_id: &str) -> Result<(), DomainError> {
        self.delete_thumbnails(file_id)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Thumbnail", e.to_string()))
    }

    async fn get_cached_thumbnail(&self, file_id: &str, size: PortThumbnailSize) -> Option<Bytes> {
        self.get_cached_thumbnail(file_id, size.into()).await
    }

    async fn store_external_thumbnail(
        &self,
        file_id: &str,
        size: PortThumbnailSize,
        data: Bytes,
    ) -> Result<Bytes, DomainError> {
        self.store_external_thumbnail(file_id, size.into(), data)
            .await
            .map_err(|e| DomainError::new(ErrorKind::InternalError, "Thumbnail", e.to_string()))
    }

    async fn get_stats(&self) -> ThumbnailStatsDto {
        let stats = self.get_stats().await;
        ThumbnailStatsDto {
            cached_thumbnails: stats.cached_thumbnails,
            cache_size_bytes: stats.cache_size_bytes,
            max_cache_bytes: stats.max_cache_bytes,
        }
    }
}

/// Thumbnail service errors
#[derive(Debug, thiserror::Error)]
pub enum ThumbnailError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Image processing error: {0}")]
    ImageError(String),

    #[error("Task error: {0}")]
    TaskError(String),

    #[error("Unsupported image format")]
    UnsupportedFormat,
}

/// Statistics about the thumbnail cache
#[derive(Debug, Clone)]
pub struct ThumbnailStats {
    pub cached_thumbnails: usize,
    pub cache_size_bytes: usize,
    pub max_cache_bytes: usize,
}
