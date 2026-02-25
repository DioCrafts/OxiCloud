//! Image Transcoding Service - WebP On-Demand Conversion
//!
//! Automatically transcodes images to WebP format when the browser supports it,
//! reducing bandwidth by 30-50% compared to JPEG/PNG.
//!
//! Architecture:
//! - **Dedicated `rayon` thread pool** for CPU-bound transcoding (never blocks Tokio)
//! - **`moka` lock-free cache** for hot transcoded images (no write-lock on reads)
//! - Disk cache for persistence across restarts
//! - Supports JPEG, PNG, GIF → WebP conversion
//! - Falls back to original if conversion fails or result is larger

use async_trait::async_trait;
use bytes::Bytes;
use image::ImageFormat;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::fs;

use crate::application::ports::transcode_ports::{
    ImageTranscodePort, OutputFormat as PortOutputFormat, TranscodeStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Maximum file size for transcoding (5MB - larger files stream directly)
pub const MAX_TRANSCODE_SIZE: u64 = 5 * 1024 * 1024;

/// Number of threads in the dedicated transcoding pool
const TRANSCODE_POOL_THREADS: usize = 2;

/// Dedicated rayon thread pool for CPU-bound image transcoding.
/// Isolated from Tokio's blocking pool to prevent starvation of other I/O.
fn transcode_pool() -> &'static rayon::ThreadPool {
    static POOL: OnceLock<rayon::ThreadPool> = OnceLock::new();
    POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(TRANSCODE_POOL_THREADS)
            .thread_name(|idx| format!("transcode-{idx}"))
            .build()
            .expect("Failed to create transcode thread pool")
    })
}

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    WebP,
    // Future: AVIF, JPEG-XL
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::WebP => "webp",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            OutputFormat::WebP => "image/webp",
        }
    }
}

/// Result of checking browser support
#[derive(Debug)]
pub struct BrowserCapabilities {
    pub supports_webp: bool,
    pub supports_avif: bool,
}

impl BrowserCapabilities {
    /// Parse Accept header to determine browser image format support
    pub fn from_accept_header(accept: Option<&str>) -> Self {
        let accept = accept.unwrap_or("");
        Self {
            supports_webp: accept.contains("image/webp"),
            supports_avif: accept.contains("image/avif"),
        }
    }

    /// Get the best output format for this browser
    pub fn best_format(&self) -> Option<OutputFormat> {
        if self.supports_webp {
            Some(OutputFormat::WebP)
        } else {
            None
        }
    }
}

/// Lock-free transcoding statistics using atomics (no RwLock needed)
#[derive(Debug, Default)]
struct AtomicTranscodeStats {
    cache_hits: AtomicU64,
    disk_hits: AtomicU64,
    transcodes: AtomicU64,
    bytes_saved: AtomicU64,
    transcode_errors: AtomicU64,
}

/// Snapshot of transcoding statistics
#[derive(Debug, Default, Clone)]
pub struct TranscodeStats {
    pub cache_hits: u64,
    pub disk_hits: u64,
    pub transcodes: u64,
    pub bytes_saved: u64,
    pub transcode_errors: u64,
}

impl AtomicTranscodeStats {
    fn snapshot(&self) -> TranscodeStats {
        TranscodeStats {
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            disk_hits: self.disk_hits.load(Ordering::Relaxed),
            transcodes: self.transcodes.load(Ordering::Relaxed),
            bytes_saved: self.bytes_saved.load(Ordering::Relaxed),
            transcode_errors: self.transcode_errors.load(Ordering::Relaxed),
        }
    }
}

/// Image Transcoding Service
///
/// Uses a dedicated `rayon` thread pool for CPU-bound work and `moka` for
/// lock-free concurrent caching with automatic weight-based eviction.
pub struct ImageTranscodeService {
    /// Cache directory for transcoded images on disk
    cache_dir: PathBuf,
    /// Lock-free concurrent cache (moka) — no write-lock on reads
    memory_cache: moka::future::Cache<String, Bytes>,
    /// Lock-free statistics
    stats: Arc<AtomicTranscodeStats>,
}

impl ImageTranscodeService {
    /// Create new transcoding service
    ///
    /// - `storage_root`: base path for disk cache
    /// - `max_cache_entries`: max number of transcoded images in memory
    /// - `max_memory_bytes`: max total bytes for in-memory cache
    pub fn new(storage_root: &Path, max_cache_entries: usize, max_memory_bytes: usize) -> Self {
        let cache_dir = storage_root.join(".transcoded");

        // Build moka cache with weight-based eviction (by content size)
        let memory_cache = moka::future::Cache::builder()
            .max_capacity(max_memory_bytes as u64)
            .weigher(|_key: &String, value: &Bytes| -> u32 {
                // Weight = byte size, capped to u32::MAX
                value.len().min(u32::MAX as usize) as u32
            })
            .time_to_live(std::time::Duration::from_secs(600)) // 10 min TTL for freshness
            .build();

        // Ignore max_cache_entries — moka uses weight-based eviction, which is
        // more accurate than entry-count limits for variable-size images.
        let _ = max_cache_entries;

        Self {
            cache_dir,
            memory_cache,
            stats: Arc::new(AtomicTranscodeStats::default()),
        }
    }

    /// Initialize the service (create cache directories)
    pub async fn initialize(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.cache_dir).await?;
        fs::create_dir_all(self.cache_dir.join("webp")).await?;
        tracing::info!(
            "🖼️ Image transcode service initialized (rayon pool: {} threads, cache dir: {:?})",
            TRANSCODE_POOL_THREADS,
            self.cache_dir
        );
        Ok(())
    }

    /// Check if a mime type can be transcoded
    pub fn can_transcode(mime_type: &str) -> bool {
        matches!(
            mime_type,
            "image/jpeg" | "image/jpg" | "image/png" | "image/gif"
        )
    }

    /// Check if transcoding should be attempted based on file size and type
    pub fn should_transcode(mime_type: &str, file_size: u64) -> bool {
        Self::can_transcode(mime_type) && file_size <= MAX_TRANSCODE_SIZE
    }

    /// Get transcoded version of an image.
    /// Returns `(content, mime_type, was_transcoded)`.
    ///
    /// Accepts `Bytes` (ref-counted) so callers avoid copying the buffer.
    /// Cloning `Bytes` is O(1) — only an atomic increment.
    pub async fn get_transcoded(
        &self,
        file_id: &str,
        original_content: Bytes,
        original_mime: &str,
        target_format: OutputFormat,
    ) -> Result<(Bytes, String, bool), String> {
        let cache_key = format!("{}:{}", file_id, target_format.extension());

        // ── 1. Check moka memory cache (lock-free read) ──
        if let Some(cached) = self.memory_cache.get(&cache_key).await {
            self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
            tracing::debug!("🔥 Transcode memory cache HIT: {}", file_id);
            return Ok((cached, target_format.mime_type().to_string(), true));
        }

        // ── 2. Check disk cache (async fs) ──
        let cache_path = self.get_cache_path(file_id, target_format);
        if tokio::fs::try_exists(&cache_path).await.unwrap_or(false) {
            match fs::read(&cache_path).await {
                Ok(data) => {
                    let content = Bytes::from(data);
                    self.memory_cache
                        .insert(cache_key.clone(), content.clone())
                        .await;
                    self.stats.disk_hits.fetch_add(1, Ordering::Relaxed);
                    tracing::debug!("💾 Transcode disk cache HIT: {}", file_id);
                    return Ok((content, target_format.mime_type().to_string(), true));
                }
                Err(e) => {
                    tracing::warn!("Failed to read cached transcode: {}", e);
                }
            }
        }

        // ── 3. Transcode on dedicated rayon pool (never blocks Tokio) ──
        let content_for_rayon = original_content.clone(); // O(1) ref-count bump
        let mime_owned = original_mime.to_string();

        let (tx, rx) = tokio::sync::oneshot::channel();

        transcode_pool().spawn(move || {
            let result = transcode_image_blocking(&content_for_rayon, &mime_owned, target_format);
            let _ = tx.send(result);
        });

        let transcoded = rx
            .await
            .map_err(|_| "Transcode task was cancelled".to_string())??;

        let transcoded_bytes = Bytes::from(transcoded);

        // ── 4. Evaluate savings ──
        let original_size = original_content.len();
        let transcoded_size = transcoded_bytes.len();

        if transcoded_size >= original_size {
            tracing::debug!(
                "⚠️ Transcode not beneficial for {}: {} -> {} bytes",
                file_id,
                original_size,
                transcoded_size
            );
            return Ok((original_content, original_mime.to_string(), false));
        }

        let saved = original_size - transcoded_size;

        // ── 5. Persist to disk cache (fire-and-forget) ──
        let cache_path_clone = cache_path.clone();
        let transcoded_for_disk = transcoded_bytes.clone();
        tokio::spawn(async move {
            if let Some(parent) = cache_path_clone.parent() {
                let _ = fs::create_dir_all(parent).await;
            }
            if let Err(e) = fs::write(&cache_path_clone, &transcoded_for_disk).await {
                tracing::warn!("Failed to cache transcoded image: {}", e);
            }
        });

        // ── 6. Store in moka memory cache (lock-free) ──
        self.memory_cache
            .insert(cache_key, transcoded_bytes.clone())
            .await;

        // ── 7. Update stats (lock-free atomics) ──
        self.stats.transcodes.fetch_add(1, Ordering::Relaxed);
        self.stats
            .bytes_saved
            .fetch_add(saved as u64, Ordering::Relaxed);

        tracing::info!(
            "✨ Transcoded {}: {} -> {} bytes ({:.1}% smaller)",
            file_id,
            original_size,
            transcoded_size,
            (1.0 - transcoded_size as f64 / original_size as f64) * 100.0
        );

        Ok((
            transcoded_bytes,
            target_format.mime_type().to_string(),
            true,
        ))
    }

    /// Get path for cached transcoded file
    fn get_cache_path(&self, file_id: &str, format: OutputFormat) -> PathBuf {
        self.cache_dir
            .join(format.extension())
            .join(format!("{}.{}", file_id, format.extension()))
    }

    /// Invalidate cached transcodes for a file
    pub async fn invalidate(&self, file_id: &str) {
        let cache_key = format!("{}:{}", file_id, OutputFormat::WebP.extension());
        self.memory_cache.invalidate(&cache_key).await;

        let cache_path = self.get_cache_path(file_id, OutputFormat::WebP);
        let _ = fs::remove_file(&cache_path).await;
    }

    /// Get transcoding statistics
    pub async fn get_stats(&self) -> TranscodeStats {
        self.stats.snapshot()
    }

    /// Clear all caches
    pub async fn clear_cache(&self) -> std::io::Result<()> {
        self.memory_cache.invalidate_all();

        if tokio::fs::try_exists(&self.cache_dir)
            .await
            .unwrap_or(false)
        {
            fs::remove_dir_all(&self.cache_dir).await?;
            fs::create_dir_all(&self.cache_dir).await?;
            fs::create_dir_all(self.cache_dir.join("webp")).await?;
        }

        Ok(())
    }
}

// ─── CPU-bound transcoding (runs on rayon, never on Tokio) ───────────────────

/// Perform actual image transcoding. This is a pure CPU function — safe to call
/// from `rayon::spawn` or `spawn_blocking`.
fn transcode_image_blocking(
    content: &[u8],
    original_mime: &str,
    target_format: OutputFormat,
) -> Result<Vec<u8>, String> {
    let input_format = match original_mime {
        "image/jpeg" | "image/jpg" => ImageFormat::Jpeg,
        "image/png" => ImageFormat::Png,
        "image/gif" => ImageFormat::Gif,
        _ => return Err(format!("Unsupported input format: {}", original_mime)),
    };

    let img = image::load_from_memory_with_format(content, input_format)
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    match target_format {
        OutputFormat::WebP => {
            let mut buffer = Vec::new();
            let mut cursor = std::io::Cursor::new(&mut buffer);
            img.write_to(&mut cursor, ImageFormat::WebP)
                .map_err(|e| format!("Failed to encode WebP: {}", e))?;
            Ok(buffer)
        }
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

/// Convert port OutputFormat to infra OutputFormat.
impl From<PortOutputFormat> for OutputFormat {
    fn from(fmt: PortOutputFormat) -> Self {
        match fmt {
            PortOutputFormat::WebP => OutputFormat::WebP,
        }
    }
}

#[async_trait]
impl ImageTranscodePort for ImageTranscodeService {
    fn can_transcode(&self, mime_type: &str) -> bool {
        ImageTranscodeService::can_transcode(mime_type)
    }

    fn should_transcode(&self, mime_type: &str, file_size: u64) -> bool {
        ImageTranscodeService::should_transcode(mime_type, file_size)
    }

    async fn get_transcoded(
        &self,
        file_id: &str,
        original_content: Bytes,
        original_mime: &str,
        target_format: PortOutputFormat,
    ) -> Result<(Bytes, String, bool), DomainError> {
        self.get_transcoded(
            file_id,
            original_content,
            original_mime,
            target_format.into(),
        )
        .await
        .map_err(|e| DomainError::new(ErrorKind::InternalError, "ImageTranscode", e))
    }

    async fn invalidate(&self, file_id: &str) {
        self.invalidate(file_id).await
    }

    async fn get_stats(&self) -> TranscodeStatsDto {
        let stats = self.get_stats().await;
        TranscodeStatsDto {
            cache_hits: stats.cache_hits,
            disk_hits: stats.disk_hits,
            transcodes: stats.transcodes,
            bytes_saved: stats.bytes_saved,
            transcode_errors: stats.transcode_errors,
        }
    }

    async fn clear_cache(&self) -> Result<(), DomainError> {
        self.clear_cache().await.map_err(DomainError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_capabilities() {
        // Chrome/Firefox with WebP support
        let caps = BrowserCapabilities::from_accept_header(Some(
            "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
        ));
        assert!(caps.supports_webp);
        assert!(caps.supports_avif);

        // Safari without WebP (old)
        let caps = BrowserCapabilities::from_accept_header(Some(
            "image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5",
        ));
        assert!(!caps.supports_webp);

        // No header
        let caps = BrowserCapabilities::from_accept_header(None);
        assert!(!caps.supports_webp);
    }

    #[test]
    fn test_can_transcode() {
        assert!(ImageTranscodeService::can_transcode("image/jpeg"));
        assert!(ImageTranscodeService::can_transcode("image/png"));
        assert!(ImageTranscodeService::can_transcode("image/gif"));
        assert!(!ImageTranscodeService::can_transcode("image/webp"));
        assert!(!ImageTranscodeService::can_transcode("image/svg+xml"));
        assert!(!ImageTranscodeService::can_transcode("application/pdf"));
    }

    #[test]
    fn test_should_transcode() {
        // Small JPEG - yes
        assert!(ImageTranscodeService::should_transcode(
            "image/jpeg",
            1024 * 1024
        ));

        // Large JPEG - no (too big)
        assert!(!ImageTranscodeService::should_transcode(
            "image/jpeg",
            10 * 1024 * 1024
        ));

        // WebP - no (already optimal)
        assert!(!ImageTranscodeService::should_transcode(
            "image/webp",
            1024 * 1024
        ));
    }

    #[test]
    fn test_transcode_pool_initializes() {
        // Verify the pool can be created without panic
        let pool = transcode_pool();
        assert_eq!(pool.current_num_threads(), TRANSCODE_POOL_THREADS);
    }
}
