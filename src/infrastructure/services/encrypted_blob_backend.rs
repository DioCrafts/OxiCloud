//! `EncryptedBlobBackend` — AES-256-GCM encryption decorator for blob storage.
//!
//! Transparently encrypts blobs before they reach the inner backend and
//! decrypts them on read.  Each blob gets a random 96-bit nonce which is
//! prepended to the ciphertext.  The GCM authentication tag (16 bytes) is
//! appended by the cipher.
//!
//! **IMPORTANT**: BLAKE3 hashing is performed on the *plaintext* by
//! `DedupService` before this layer sees the blob, so content-addressable
//! dedup still works correctly.
//!
//! Layout on disk/S3: `[12-byte nonce][ciphertext + 16-byte GCM tag]`

use std::path::{Path, PathBuf};
use std::pin::Pin;

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Nonce};
use bytes::Bytes;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::application::ports::blob_storage_ports::{
    BlobStorageBackend, BlobStream, StorageHealthStatus,
};
use crate::domain::errors::DomainError;

/// Nonce size for AES-256-GCM (96 bits = 12 bytes).
const NONCE_SIZE: usize = 12;

/// `BlobStorageBackend` decorator that encrypts blobs at rest.
pub struct EncryptedBlobBackend {
    inner: Arc<dyn BlobStorageBackend>,
    cipher: Aes256Gcm,
}

impl EncryptedBlobBackend {
    /// Create a new encryption layer wrapping `inner`.
    ///
    /// `key` must be exactly 32 bytes (AES-256).
    pub fn new(inner: Arc<dyn BlobStorageBackend>, key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key).expect("AES-256 key must be 32 bytes");
        Self { inner, cipher }
    }

    /// Generate a random 32-byte key suitable for AES-256.
    pub fn generate_key() -> [u8; 32] {
        use aes_gcm::aead::rand_core::RngCore;
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }
}

impl BlobStorageBackend for EncryptedBlobBackend {
    fn initialize(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), DomainError>> + Send + '_>> {
        self.inner.initialize()
    }

    fn put_blob(
        &self,
        hash: &str,
        source_path: &Path,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, DomainError>> + Send + '_>> {
        let inner = self.inner.clone();
        let hash = hash.to_string();
        let source = source_path.to_path_buf();
        // Clone cipher key material (Aes256Gcm is not Send-safe to move across await)
        let cipher = self.cipher.clone();
        Box::pin(async move {
            // Read plaintext from source
            let plaintext = fs::read(&source).await.map_err(|e| {
                DomainError::internal_error("Encryption", format!("read source: {e}"))
            })?;

            // Encrypt: nonce || ciphertext (includes GCM tag)
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).map_err(|e| {
                DomainError::internal_error("Encryption", format!("encrypt failed: {e}"))
            })?;

            // Write encrypted blob to a temp file
            let tmp = source.with_extension("enc.tmp");
            let mut file = fs::File::create(&tmp).await.map_err(|e| {
                DomainError::internal_error("Encryption", format!("create tmp: {e}"))
            })?;
            file.write_all(nonce.as_slice()).await.map_err(|e| {
                DomainError::internal_error("Encryption", format!("write nonce: {e}"))
            })?;
            file.write_all(&ciphertext).await.map_err(|e| {
                DomainError::internal_error("Encryption", format!("write ciphertext: {e}"))
            })?;
            file.flush()
                .await
                .map_err(|e| DomainError::internal_error("Encryption", format!("flush: {e}")))?;
            drop(file);

            let result = inner.put_blob(&hash, &tmp).await;
            let _ = fs::remove_file(&tmp).await;
            result
        })
    }

    fn get_blob_stream(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<BlobStream, DomainError>> + Send + '_>>
    {
        let inner = self.inner.clone();
        let hash = hash.to_string();
        let cipher = self.cipher.clone();
        Box::pin(async move {
            // Read entire encrypted blob (nonce + ciphertext) into memory for decryption
            let enc_stream = inner.get_blob_stream(&hash).await?;
            let encrypted = collect_stream(enc_stream).await?;

            if encrypted.len() < NONCE_SIZE {
                return Err(DomainError::internal_error(
                    "Encryption",
                    "encrypted blob too short (missing nonce)",
                ));
            }

            let (nonce_bytes, ciphertext) = encrypted.split_at(NONCE_SIZE);
            let nonce = Nonce::from_slice(nonce_bytes);
            let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
                DomainError::internal_error("Encryption", format!("decrypt failed: {e}"))
            })?;

            let stream: BlobStream =
                Box::pin(futures::stream::once(
                    async move { Ok(Bytes::from(plaintext)) },
                ));
            Ok(stream)
        })
    }

    fn get_blob_range_stream(
        &self,
        hash: &str,
        start: u64,
        end: Option<u64>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<BlobStream, DomainError>> + Send + '_>>
    {
        let inner = self.inner.clone();
        let hash = hash.to_string();
        let cipher = self.cipher.clone();
        Box::pin(async move {
            // Must decrypt the full blob then slice the plaintext range
            let enc_stream = inner.get_blob_stream(&hash).await?;
            let encrypted = collect_stream(enc_stream).await?;

            if encrypted.len() < NONCE_SIZE {
                return Err(DomainError::internal_error(
                    "Encryption",
                    "encrypted blob too short",
                ));
            }

            let (nonce_bytes, ciphertext) = encrypted.split_at(NONCE_SIZE);
            let nonce = Nonce::from_slice(nonce_bytes);
            let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
                DomainError::internal_error("Encryption", format!("decrypt failed: {e}"))
            })?;

            let start = start as usize;
            let end = end.map(|e| (e as usize) + 1).unwrap_or(plaintext.len());
            let end = end.min(plaintext.len());
            let start = start.min(end);

            let slice = Bytes::from(plaintext[start..end].to_vec());
            let stream: BlobStream = Box::pin(futures::stream::once(async move { Ok(slice) }));
            Ok(stream)
        })
    }

    fn delete_blob(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), DomainError>> + Send + '_>> {
        self.inner.delete_blob(hash)
    }

    fn blob_exists(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<bool, DomainError>> + Send + '_>> {
        self.inner.blob_exists(hash)
    }

    fn blob_size(
        &self,
        hash: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<u64, DomainError>> + Send + '_>> {
        // The stored size includes nonce + GCM tag overhead.
        // Return the *plaintext* size by subtracting overhead.
        let inner = self.inner.clone();
        let hash = hash.to_string();
        Box::pin(async move {
            let encrypted_size = inner.blob_size(&hash).await?;
            // overhead = 12 (nonce) + 16 (GCM tag) = 28 bytes
            Ok(encrypted_size.saturating_sub(28))
        })
    }

    fn health_check(
        &self,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<StorageHealthStatus, DomainError>> + Send + '_>,
    > {
        let inner = self.inner.clone();
        Box::pin(async move {
            let mut status = inner.health_check().await?;
            status.backend_type = format!("encrypted({})", status.backend_type);
            status.message = format!("{} | Encryption: AES-256-GCM", status.message);
            Ok(status)
        })
    }

    fn backend_type(&self) -> &'static str {
        "encrypted"
    }

    fn local_blob_path(&self, _hash: &str) -> Option<PathBuf> {
        // Encrypted blobs cannot be served directly from disk
        None
    }
}

/// Collect a byte stream into a single `Vec<u8>`.
async fn collect_stream(stream: BlobStream) -> Result<Vec<u8>, DomainError> {
    use futures::StreamExt;
    let mut stream = stream;
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk
            .map_err(|e| DomainError::internal_error("Encryption", format!("stream read: {e}")))?;
        buf.extend_from_slice(&bytes);
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::services::local_blob_backend::LocalBlobBackend;
    use tempfile::TempDir;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let blob_dir = tmp.path().join("blobs");
        let local = Arc::new(LocalBlobBackend::new(&blob_dir));
        local.initialize().await.unwrap();

        let key = EncryptedBlobBackend::generate_key();
        let encrypted = EncryptedBlobBackend::new(local, &key);

        // Write a test blob
        let data = b"Hello, encrypted world!";
        let source = tmp.path().join("test.tmp");
        let mut f = fs::File::create(&source).await.unwrap();
        f.write_all(data).await.unwrap();
        f.flush().await.unwrap();
        drop(f);

        let hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
        encrypted.put_blob(hash, &source).await.unwrap();

        // Read back via stream
        let stream = encrypted.get_blob_stream(hash).await.unwrap();
        let decrypted = collect_stream(stream).await.unwrap();
        assert_eq!(decrypted, data);

        // Read range
        let range_stream = encrypted
            .get_blob_range_stream(hash, 7, Some(15))
            .await
            .unwrap();
        let range_data = collect_stream(range_stream).await.unwrap();
        assert_eq!(range_data, b"encrypted");

        // Size should reflect plaintext
        let size = encrypted.blob_size(hash).await.unwrap();
        assert_eq!(size, data.len() as u64);

        // Exists
        assert!(encrypted.blob_exists(hash).await.unwrap());

        // Delete
        encrypted.delete_blob(hash).await.unwrap();
        assert!(!encrypted.blob_exists(hash).await.unwrap());
    }
}
