use async_trait::async_trait;
use bytes::Bytes;
use image::{ImageFormat, imageops::FilterType};
/**
 * Thumbnail Generation Service
 *
 * Generates and manages image thumbnails for fast gallery previews.
 *
 * Features:
 * - Background thumbnail generation after upload
 * - Multiple sizes (icon 150x150, preview 800x600)
 * - WebP output for smaller file sizes
 * - Lock-free moka cache with weight-based eviction
 * - Lazy generation on first request if not pre-generated
 */
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

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

/// Thumbnail service for generating and caching image thumbnails
pub struct ThumbnailService {
    /// Root path for thumbnail storage
    thumbnails_root: PathBuf,
    /// Lock-free concurrent cache (moka) with weight-based eviction
    cache: moka::future::Cache<ThumbnailCacheKey, Bytes>,
    /// Configured maximum cache weight (for stats reporting)
    max_cache_bytes: u64,
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

        let cache = moka::future::Cache::builder()
            .max_capacity(max_cache_bytes as u64)
            .weigher(|_key: &ThumbnailCacheKey, value: &Bytes| -> u32 {
                value.len().min(u32::MAX as usize) as u32
            })
            .time_to_live(std::time::Duration::from_secs(600))
            .build();

        Self {
            thumbnails_root,
            cache,
            max_cache_bytes: max_cache_bytes as u64,
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
            .join(format!("{}.webp", file_id))
    }

    /// Get a thumbnail, generating it if needed.
    ///
    /// # Arguments
    /// * `file_id` - ID of the original file
    /// * `size` - Desired thumbnail size
    /// * `original_path` - Path to the original image file
    ///
    /// # Returns
    /// Bytes of the thumbnail image (WebP format)
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
                tracing::info!(
                    "🎨 Generating thumbnail: {} {:?}",
                    file_id_owned,
                    size
                );
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

    /// Generate a thumbnail from an image file
    async fn generate_thumbnail(
        &self,
        original_path: &Path,
        size: ThumbnailSize,
    ) -> Result<Bytes, ThumbnailError> {
        let path = original_path.to_path_buf();
        let max_dim = size.max_dimension();

        // Run image processing in blocking thread pool
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, ThumbnailError> {
            // Load image
            let img = image::open(&path).map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

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
                ThumbnailSize::Icon    => FilterType::Triangle,   // 150px — max speed
                ThumbnailSize::Preview => FilterType::CatmullRom, // 400px — good balance
                ThumbnailSize::Large   => FilterType::CatmullRom, // 800px — sufficient quality
            };
            let thumbnail = img.resize(new_width, new_height, filter);

            // Encode as WebP for smaller file size
            let mut buffer = Vec::new();
            thumbnail
                .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
                .map_err(|e| ThumbnailError::ImageError(e.to_string()))?;

            Ok(buffer)
        })
        .await
        .map_err(|e| ThumbnailError::TaskError(e.to_string()))?;

        result.map(Bytes::from)
    }

    /// Generate all thumbnail sizes for a file in the background
    ///
    /// This is called after file upload to pre-generate thumbnails
    pub fn generate_all_sizes_background(self: Arc<Self>, file_id: String, original_path: PathBuf) {
        tokio::spawn(async move {
            tracing::info!("🖼️ Background thumbnail generation starting: {}", file_id);

            for size in ThumbnailSize::all() {
                match self.generate_thumbnail(&original_path, *size).await {
                    Ok(bytes) => {
                        // Save to disk
                        let thumb_path = self.get_thumbnail_path(&file_id, *size);
                        if let Some(parent) = thumb_path.parent() {
                            let _ = fs::create_dir_all(parent).await;
                        }
                        if let Err(e) = fs::write(&thumb_path, &bytes).await {
                            tracing::warn!("Failed to save thumbnail {}: {}", file_id, e);
                        } else {
                            tracing::debug!("✅ Generated thumbnail: {} {:?}", file_id, size);
                        }
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to generate thumbnail {} {:?}: {}",
                            file_id,
                            size,
                            e
                        );
                    }
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

#[async_trait]
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
