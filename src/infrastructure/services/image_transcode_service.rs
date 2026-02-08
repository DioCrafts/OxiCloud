//! Image Transcoding Service - WebP On-Demand Conversion
//! 
//! Automatically transcodes images to WebP format when the browser supports it,
//! reducing bandwidth by 30-50% compared to JPEG/PNG.
//!
//! Features:
//! - Detects browser WebP support via Accept header
//! - Caches transcoded versions to avoid re-conversion
//! - Supports JPEG, PNG, GIF → WebP conversion
//! - Configurable quality settings
//! - Falls back to original if conversion fails

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs;
use bytes::Bytes;
use lru::LruCache;
use std::num::NonZeroUsize;
use image::{ImageFormat, DynamicImage};
use async_trait::async_trait;

use crate::application::ports::transcode_ports::{
    ImageTranscodePort,
    OutputFormat as PortOutputFormat,
    TranscodeStatsDto,
};
use crate::domain::errors::{DomainError, ErrorKind};

/// Maximum file size for transcoding (5MB - larger files stream directly)
pub const MAX_TRANSCODE_SIZE: u64 = 5 * 1024 * 1024;

/// Cache key for transcoded images
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TranscodeKey {
    file_id: String,
    format: OutputFormat,
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
        // WebP has best support currently
        if self.supports_webp {
            Some(OutputFormat::WebP)
        } else {
            None
        }
    }
}

/// Image Transcoding Service
pub struct ImageTranscodeService {
    /// Cache directory for transcoded images
    cache_dir: PathBuf,
    /// In-memory LRU cache for hot transcoded images
    memory_cache: Arc<RwLock<LruCache<TranscodeKey, Bytes>>>,
    /// Maximum memory cache size in bytes
    max_memory_bytes: usize,
    /// Current memory usage
    current_memory_bytes: Arc<RwLock<usize>>,
    /// Statistics
    stats: Arc<RwLock<TranscodeStats>>,
}

/// Transcoding statistics
#[derive(Debug, Default, Clone)]
pub struct TranscodeStats {
    pub cache_hits: u64,
    pub disk_hits: u64,
    pub transcodes: u64,
    pub bytes_saved: u64,
    pub transcode_errors: u64,
}

impl ImageTranscodeService {
    /// Create new transcoding service
    pub fn new(storage_root: &Path, max_cache_entries: usize, max_memory_bytes: usize) -> Self {
        let cache_dir = storage_root.join(".transcoded");
        
        Self {
            cache_dir,
            memory_cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(max_cache_entries).unwrap_or(NonZeroUsize::new(1000).unwrap())
            ))),
            max_memory_bytes,
            current_memory_bytes: Arc::new(RwLock::new(0)),
            stats: Arc::new(RwLock::new(TranscodeStats::default())),
        }
    }
    
    /// Initialize the service (create cache directories)
    pub async fn initialize(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.cache_dir).await?;
        fs::create_dir_all(self.cache_dir.join("webp")).await?;
        tracing::info!("🖼️ Image transcode service initialized at {:?}", self.cache_dir);
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
    
    /// Get transcoded version of an image
    /// Returns (content, mime_type, was_transcoded)
    pub async fn get_transcoded(
        &self,
        file_id: &str,
        original_content: &[u8],
        original_mime: &str,
        target_format: OutputFormat,
    ) -> Result<(Bytes, String, bool), String> {
        let key = TranscodeKey {
            file_id: file_id.to_string(),
            format: target_format,
        };
        
        // Check memory cache first
        {
            let mut cache = self.memory_cache.write().await;
            if let Some(cached) = cache.get(&key) {
                let mut stats = self.stats.write().await;
                stats.cache_hits += 1;
                tracing::debug!("🔥 Transcode memory cache HIT: {}", file_id);
                return Ok((cached.clone(), target_format.mime_type().to_string(), true));
            }
        }
        
        // Check disk cache
        let cache_path = self.get_cache_path(file_id, target_format);
        if cache_path.exists() {
            match fs::read(&cache_path).await {
                Ok(data) => {
                    let content = Bytes::from(data);
                    
                    // Store in memory cache
                    self.cache_in_memory(&key, content.clone()).await;
                    
                    let mut stats = self.stats.write().await;
                    stats.disk_hits += 1;
                    tracing::debug!("💾 Transcode disk cache HIT: {}", file_id);
                    return Ok((content, target_format.mime_type().to_string(), true));
                },
                Err(e) => {
                    tracing::warn!("Failed to read cached transcode: {}", e);
                }
            }
        }
        
        // Need to transcode
        let transcoded = self.transcode_image(original_content, original_mime, target_format)?;
        let transcoded_bytes = Bytes::from(transcoded.clone());
        
        // Calculate savings
        let original_size = original_content.len();
        let transcoded_size = transcoded_bytes.len();
        let saved = if transcoded_size < original_size {
            original_size - transcoded_size
        } else {
            0
        };
        
        // Only use transcoded if it's actually smaller
        if transcoded_size >= original_size {
            tracing::debug!(
                "⚠️ Transcode not beneficial for {}: {} -> {} bytes",
                file_id, original_size, transcoded_size
            );
            return Ok((Bytes::from(original_content.to_vec()), original_mime.to_string(), false));
        }
        
        // Save to disk cache (async, don't wait)
        let cache_path_clone = cache_path.clone();
        let transcoded_clone = transcoded.clone();
        tokio::spawn(async move {
            if let Some(parent) = cache_path_clone.parent() {
                let _ = fs::create_dir_all(parent).await;
            }
            if let Err(e) = fs::write(&cache_path_clone, &transcoded_clone).await {
                tracing::warn!("Failed to cache transcoded image: {}", e);
            }
        });
        
        // Store in memory cache
        self.cache_in_memory(&key, transcoded_bytes.clone()).await;
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.transcodes += 1;
            stats.bytes_saved += saved as u64;
        }
        
        tracing::info!(
            "✨ Transcoded {}: {} -> {} bytes ({:.1}% smaller)",
            file_id,
            original_size,
            transcoded_size,
            (1.0 - transcoded_size as f64 / original_size as f64) * 100.0
        );
        
        Ok((transcoded_bytes, target_format.mime_type().to_string(), true))
    }
    
    /// Perform actual image transcoding
    fn transcode_image(
        &self,
        content: &[u8],
        original_mime: &str,
        target_format: OutputFormat,
    ) -> Result<Vec<u8>, String> {
        // Determine input format
        let input_format = match original_mime {
            "image/jpeg" | "image/jpg" => ImageFormat::Jpeg,
            "image/png" => ImageFormat::Png,
            "image/gif" => ImageFormat::Gif,
            _ => return Err(format!("Unsupported input format: {}", original_mime)),
        };
        
        // Load image
        let img = image::load_from_memory_with_format(content, input_format)
            .map_err(|e| format!("Failed to decode image: {}", e))?;
        
        // Encode to target format
        match target_format {
            OutputFormat::WebP => self.encode_webp(&img),
        }
    }
    
    /// Encode image to WebP
    fn encode_webp(&self, img: &DynamicImage) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        
        // Use image crate's WebP encoder
        img.write_to(&mut cursor, ImageFormat::WebP)
            .map_err(|e| format!("Failed to encode WebP: {}", e))?;
        
        Ok(buffer)
    }
    
    /// Get path for cached transcoded file
    fn get_cache_path(&self, file_id: &str, format: OutputFormat) -> PathBuf {
        self.cache_dir
            .join(format.extension())
            .join(format!("{}.{}", file_id, format.extension()))
    }
    
    /// Store transcoded image in memory cache
    async fn cache_in_memory(&self, key: &TranscodeKey, content: Bytes) {
        let size = content.len();
        
        let mut current = self.current_memory_bytes.write().await;
        
        // Evict if needed
        while *current + size > self.max_memory_bytes {
            let mut cache = self.memory_cache.write().await;
            if let Some((_, evicted)) = cache.pop_lru() {
                *current = current.saturating_sub(evicted.len());
            } else {
                break;
            }
        }
        
        // Add to cache
        if *current + size <= self.max_memory_bytes {
            let mut cache = self.memory_cache.write().await;
            cache.put(key.clone(), content);
            *current += size;
        }
    }
    
    /// Invalidate cached transcodes for a file
    pub async fn invalidate(&self, file_id: &str) {
        // Remove from memory cache
        {
            let mut cache = self.memory_cache.write().await;
            let key = TranscodeKey {
                file_id: file_id.to_string(),
                format: OutputFormat::WebP,
            };
            if let Some(removed) = cache.pop(&key) {
                let mut current = self.current_memory_bytes.write().await;
                *current = current.saturating_sub(removed.len());
            }
        }
        
        // Remove disk cache
        let cache_path = self.get_cache_path(file_id, OutputFormat::WebP);
        let _ = fs::remove_file(&cache_path).await;
    }
    
    /// Get transcoding statistics
    pub async fn get_stats(&self) -> TranscodeStats {
        self.stats.read().await.clone()
    }
    
    /// Clear all caches
    pub async fn clear_cache(&self) -> std::io::Result<()> {
        // Clear memory
        {
            let mut cache = self.memory_cache.write().await;
            cache.clear();
            let mut current = self.current_memory_bytes.write().await;
            *current = 0;
        }
        
        // Clear disk
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir).await?;
            fs::create_dir_all(&self.cache_dir).await?;
            fs::create_dir_all(self.cache_dir.join("webp")).await?;
        }
        
        Ok(())
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
        original_content: &[u8],
        original_mime: &str,
        target_format: PortOutputFormat,
    ) -> Result<(Bytes, String, bool), DomainError> {
        self.get_transcoded(file_id, original_content, original_mime, target_format.into())
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
        let caps = BrowserCapabilities::from_accept_header(
            Some("image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8")
        );
        assert!(caps.supports_webp);
        assert!(caps.supports_avif);
        
        // Safari without WebP (old)
        let caps = BrowserCapabilities::from_accept_header(
            Some("image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5")
        );
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
        assert!(ImageTranscodeService::should_transcode("image/jpeg", 1024 * 1024));
        
        // Large JPEG - no (too big)
        assert!(!ImageTranscodeService::should_transcode("image/jpeg", 10 * 1024 * 1024));
        
        // WebP - no (already optimal)
        assert!(!ImageTranscodeService::should_transcode("image/webp", 1024 * 1024));
    }
}
