use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::common::errors::{DomainError, Result};

#[derive(Clone)]
pub struct NextcloudChunkedUploadService {
    pub base_dir: PathBuf,
}

impl NextcloudChunkedUploadService {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn new_stub() -> Self {
        Self {
            base_dir: PathBuf::from("./storage/.uploads/nextcloud"),
        }
    }

    /// Validate that a path component contains no traversal characters.
    fn validate_path_component(name: &str, label: &str) -> Result<()> {
        if name.is_empty()
            || name.contains('/')
            || name.contains('\\')
            || name.contains("..")
            || name == "."
        {
            return Err(DomainError::validation_error(format!(
                "ChunkedUpload: invalid {}: contains path traversal characters",
                label
            )));
        }
        Ok(())
    }

    /// Build a session directory path and verify it's inside base_dir.
    fn safe_session_dir(&self, user: &str, upload_id: &str) -> Result<PathBuf> {
        Self::validate_path_component(user, "username")?;
        Self::validate_path_component(upload_id, "upload_id")?;
        Ok(self.base_dir.join(user).join(upload_id))
    }

    /// Create a new upload session directory.
    pub async fn create_session(&self, user: &str, upload_id: &str) -> Result<()> {
        let session_dir = self.safe_session_dir(user, upload_id)?;
        fs::create_dir_all(&session_dir)
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
        Ok(())
    }

    /// Store a chunk in the session directory.
    pub async fn store_chunk(
        &self,
        user: &str,
        upload_id: &str,
        chunk_name: &str,
        data: &[u8],
    ) -> Result<()> {
        Self::validate_path_component(chunk_name, "chunk_name")?;
        let chunk_path = self.safe_session_dir(user, upload_id)?.join(chunk_name);
        let mut file = fs::File::create(&chunk_path)
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
        file.write_all(data)
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
        Ok(())
    }

    /// Assemble all chunks in numeric order into a temp file.
    ///
    /// Returns `(temp_path, total_size)`. The caller is responsible for
    /// cleaning up the temp file after use.
    pub async fn assemble(&self, user: &str, upload_id: &str) -> Result<(PathBuf, u64)> {
        let session_dir = self.safe_session_dir(user, upload_id)?;
        let mut entries: Vec<String> = Vec::new();

        let mut dir = fs::read_dir(&session_dir)
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?
        {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == ".file" {
                continue; // Skip the assembly marker.
            }
            entries.push(name);
        }

        // Sort chunks numerically (Nextcloud sends them as "00001", "00002", ...).
        entries.sort();

        // Stream chunks to a temp file instead of buffering in memory.
        let temp_path = session_dir.join(".assembled");
        let mut out = fs::File::create(&temp_path)
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;

        let mut total_size: u64 = 0;
        for chunk_name in &entries {
            let mut chunk_file = fs::File::open(session_dir.join(chunk_name))
                .await
                .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
            let copied = tokio::io::copy(&mut chunk_file, &mut out)
                .await
                .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
            total_size += copied;
        }

        out.flush()
            .await
            .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;

        Ok((temp_path, total_size))
    }

    /// Delete the upload session directory.
    pub async fn cleanup(&self, user: &str, upload_id: &str) -> Result<()> {
        let session_dir = self.safe_session_dir(user, upload_id)?;
        if session_dir.exists() {
            fs::remove_dir_all(&session_dir)
                .await
                .map_err(|e| DomainError::internal_error("ChunkedUpload", e.to_string()))?;
        }
        Ok(())
    }

    /// Check if a session directory exists.
    pub async fn session_exists(&self, user: &str, upload_id: &str) -> bool {
        self.safe_session_dir(user, upload_id)
            .map(|p| p.exists())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_service() -> (NextcloudChunkedUploadService, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let svc = NextcloudChunkedUploadService::new(dir.path().to_path_buf());
        (svc, dir)
    }

    #[tokio::test]
    async fn test_create_session() {
        let (svc, _dir) = test_service();
        svc.create_session("alice", "upload-001").await.unwrap();
        assert!(svc.session_exists("alice", "upload-001").await);
    }

    #[tokio::test]
    async fn test_session_not_exists_before_create() {
        let (svc, _dir) = test_service();
        assert!(!svc.session_exists("alice", "upload-999").await);
    }

    #[tokio::test]
    async fn test_store_and_assemble_chunks() {
        let (svc, _dir) = test_service();
        svc.create_session("alice", "upload-002").await.unwrap();

        svc.store_chunk("alice", "upload-002", "00001", b"Hello, ")
            .await
            .unwrap();
        svc.store_chunk("alice", "upload-002", "00002", b"World!")
            .await
            .unwrap();

        let (temp_path, size) = svc.assemble("alice", "upload-002").await.unwrap();
        let assembled = fs::read(&temp_path).await.unwrap();
        assert_eq!(assembled, b"Hello, World!");
        assert_eq!(size, 13);
    }

    #[tokio::test]
    async fn test_assemble_chunks_in_sorted_order() {
        let (svc, _dir) = test_service();
        svc.create_session("alice", "upload-003").await.unwrap();

        // Store out of order.
        svc.store_chunk("alice", "upload-003", "00003", b"C")
            .await
            .unwrap();
        svc.store_chunk("alice", "upload-003", "00001", b"A")
            .await
            .unwrap();
        svc.store_chunk("alice", "upload-003", "00002", b"B")
            .await
            .unwrap();

        let (temp_path, size) = svc.assemble("alice", "upload-003").await.unwrap();
        let assembled = fs::read(&temp_path).await.unwrap();
        assert_eq!(assembled, b"ABC");
        assert_eq!(size, 3);
    }

    #[tokio::test]
    async fn test_cleanup_removes_session() {
        let (svc, _dir) = test_service();
        svc.create_session("alice", "upload-004").await.unwrap();
        assert!(svc.session_exists("alice", "upload-004").await);

        svc.cleanup("alice", "upload-004").await.unwrap();
        assert!(!svc.session_exists("alice", "upload-004").await);
    }

    #[tokio::test]
    async fn test_cleanup_nonexistent_session_is_ok() {
        let (svc, _dir) = test_service();
        // Should not error.
        svc.cleanup("alice", "nonexistent").await.unwrap();
    }
}
