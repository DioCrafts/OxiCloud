//! Thumbnail Port - Application layer abstraction for thumbnail generation.
//!
//! This module defines the port (trait) for thumbnail operations,
//! keeping the application and interface layers independent of specific
//! image processing implementations.

use crate::common::errors::DomainError;
use bytes::Bytes;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Thumbnail sizes supported by the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThumbnailSize {
    /// Small icon for file listings (150×150)
    Icon,
    /// Medium preview for gallery view (400×400)
    Preview,
    /// Large preview for detail view (800×800)
    Large,
}

impl ThumbnailSize {
    /// Get the maximum dimension for this size.
    pub fn max_dimension(&self) -> u32 {
        match self {
            ThumbnailSize::Icon => 150,
            ThumbnailSize::Preview => 400,
            ThumbnailSize::Large => 800,
        }
    }

    /// Get the directory name for this size.
    pub fn dir_name(&self) -> &'static str {
        match self {
            ThumbnailSize::Icon => "icon",
            ThumbnailSize::Preview => "preview",
            ThumbnailSize::Large => "large",
        }
    }

    /// Get all thumbnail sizes.
    pub fn all() -> &'static [ThumbnailSize] {
        &[
            ThumbnailSize::Icon,
            ThumbnailSize::Preview,
            ThumbnailSize::Large,
        ]
    }
}

/// Statistics about the thumbnail cache.
#[derive(Debug, Clone)]
pub struct ThumbnailStatsDto {
    pub cached_thumbnails: usize,
    pub cache_size_bytes: usize,
    pub max_cache_bytes: usize,
}

/// Port for thumbnail generation and retrieval.
///
/// Implementations handle the actual image processing, caching,
/// and storage of thumbnails, while the application layer only
/// interacts through this abstraction.
pub trait ThumbnailPort: Send + Sync + 'static {
    /// Check if a file is an image that can have thumbnails.
    fn is_supported_image(&self, mime_type: &str) -> bool;

    /// Get a thumbnail, generating it on-demand if needed.
    ///
    /// Returns the thumbnail bytes in WebP format.
    async fn get_thumbnail(
        &self,
        file_id: &str,
        size: ThumbnailSize,
        original_path: &Path,
    ) -> Result<Bytes, DomainError>;

    /// Generate all thumbnail sizes for a file in the background.
    ///
    /// Called after file upload to pre-generate thumbnails.
    fn generate_all_sizes_background(self: Arc<Self>, file_id: String, original_path: PathBuf);

    /// Delete all thumbnails for a file.
    async fn delete_thumbnails(&self, file_id: &str) -> Result<(), DomainError>;

    /// Try to get a cached thumbnail without generating one.
    ///
    /// Returns `None` if no cached thumbnail exists on disk or in memory.
    /// Used for non-image file types (videos) where thumbnails are
    /// generated client-side and uploaded.
    async fn get_cached_thumbnail(
        &self,
        file_id: &str,
        size: ThumbnailSize,
    ) -> Option<Bytes>;

    /// Store an externally-generated thumbnail (e.g. client-side video frame).
    ///
    /// Validates the image, re-encodes to WebP, and persists to cache.
    async fn store_external_thumbnail(
        &self,
        file_id: &str,
        size: ThumbnailSize,
        data: Bytes,
    ) -> Result<Bytes, DomainError>;

    /// Get cache statistics.
    async fn get_stats(&self) -> ThumbnailStatsDto;
}
