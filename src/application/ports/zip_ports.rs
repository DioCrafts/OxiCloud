//! ZIP Port - Application layer abstraction for ZIP archive creation.
//!
//! This module defines the port (trait) for ZIP operations,
//! keeping the interface layer independent of specific ZIP
//! implementation details.

use crate::common::errors::DomainError;
use async_trait::async_trait;
use tempfile::NamedTempFile;

/// Port for ZIP archive operations.
///
/// Implementations handle the actual ZIP file creation, compression,
/// and recursive folder traversal.
#[async_trait]
pub trait ZipPort: Send + Sync + 'static {
    /// Create a ZIP archive containing the contents of a folder (recursively).
    ///
    /// Returns a temporary file containing the ZIP archive. The caller streams
    /// it to the client and the file is automatically deleted when dropped.
    async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> Result<NamedTempFile, DomainError>;
}
