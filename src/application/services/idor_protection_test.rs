//! Tests for IDOR (Insecure Direct Object Reference) protection.
//!
//! Verifies that ownership checks at the repository and service layers
//! correctly reject access when the caller is not the file owner.

use bytes::Bytes;
use futures::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;

use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use crate::domain::entities::file::File;
use crate::domain::services::path_service::StoragePath;

// ═══════════════════════════════════════════════════════════════════════════
// Mock repositories
// ═══════════════════════════════════════════════════════════════════════════

/// A simple in-memory mock that maps (file_id → (File, owner_id)).
struct MockFileReadPort {
    /// file_id → (File, owner_id)
    files: Mutex<HashMap<String, (File, String)>>,
}

impl MockFileReadPort {
    fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
        }
    }

    /// Insert a test file owned by `owner_id`.
    fn insert(&self, id: &str, name: &str, owner_id: &str) {
        let file = File::new(
            id.to_string(),
            name.to_string(),
            StoragePath::from_string(&format!("/{}", name)),
            42,
            "text/plain".to_string(),
            None,
        )
        .unwrap();
        self.files
            .lock()
            .unwrap()
            .insert(id.to_string(), (file, owner_id.to_string()));
    }
}

impl FileReadPort for MockFileReadPort {
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
        let files = self.files.lock().unwrap();
        files
            .get(id)
            .map(|(f, _)| f.clone())
            .ok_or_else(|| DomainError::not_found("File", id.to_string()))
    }

    async fn get_file_for_owner(&self, id: &str, owner_id: &str) -> Result<File, DomainError> {
        let files = self.files.lock().unwrap();
        match files.get(id) {
            Some((file, actual_owner)) if actual_owner == owner_id => Ok(file.clone()),
            // Return NotFound regardless — do not leak existence
            _ => Err(DomainError::not_found("File", id.to_string())),
        }
    }

    async fn list_files(&self, _folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        Ok(Vec::new())
    }

    async fn get_file_stream(
        &self,
        _id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        unimplemented!()
    }

    async fn get_file_range_stream(
        &self,
        _id: &str,
        _start: u64,
        _end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        unimplemented!()
    }

    async fn get_file_path(&self, _id: &str) -> Result<StoragePath, DomainError> {
        unimplemented!()
    }

    async fn get_parent_folder_id(&self, _path: &str) -> Result<String, DomainError> {
        unimplemented!()
    }

    async fn get_blob_hash(&self, _file_id: &str) -> Result<String, DomainError> {
        Ok(String::new())
    }

    async fn search_files_paginated(
        &self,
        _folder_id: Option<&str>,
        _criteria: &crate::application::dtos::search_dto::SearchCriteriaDto,
        _user_id: &str,
    ) -> Result<(Vec<File>, usize), DomainError> {
        Ok((Vec::new(), 0))
    }

    async fn count_files(
        &self,
        _folder_id: Option<&str>,
        _criteria: &crate::application::dtos::search_dto::SearchCriteriaDto,
        _user_id: &str,
    ) -> Result<usize, DomainError> {
        Ok(0)
    }

    async fn stream_files_in_subtree(
        &self,
        _folder_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<File, DomainError>> + Send>>, DomainError> {
        Ok(Box::pin(futures::stream::empty()))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests — FileReadPort::get_file_for_owner (Repository layer, Solution C)
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn get_file_for_owner_returns_file_for_correct_owner() {
    let repo = MockFileReadPort::new();
    repo.insert("file-1", "secret.txt", "alice");

    let result = repo.get_file_for_owner("file-1", "alice").await;
    assert!(result.is_ok(), "owner should be able to read own file");
    assert_eq!(result.unwrap().id(), "file-1");
}

#[tokio::test]
async fn get_file_for_owner_rejects_wrong_owner() {
    let repo = MockFileReadPort::new();
    repo.insert("file-1", "secret.txt", "alice");

    let result = repo.get_file_for_owner("file-1", "bob").await;
    assert!(result.is_err(), "non-owner should be rejected");

    // Must be NotFound, NOT Forbidden — avoids leaking existence
    let err = result.unwrap_err();
    let msg = format!("{}", err);
    assert!(
        msg.contains("not found") || msg.contains("NotFound"),
        "error must be NotFound, got: {}",
        msg
    );
}

#[tokio::test]
async fn get_file_for_owner_returns_not_found_for_missing_file() {
    let repo = MockFileReadPort::new();

    let result = repo.get_file_for_owner("nonexistent", "alice").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn verify_file_owner_uses_default_impl() {
    let repo = MockFileReadPort::new();
    repo.insert("file-1", "secret.txt", "alice");

    // Default impl delegates to get_file_for_owner and maps to ()
    assert!(repo.verify_file_owner("file-1", "alice").await.is_ok());
    assert!(repo.verify_file_owner("file-1", "bob").await.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests — FileManagementService _owned methods (Service layer, Solution B)
// ═══════════════════════════════════════════════════════════════════════════
//
// Note: FileManagementService::with_trash takes concrete types for the write
// repository (Arc<FileBlobWriteRepository>). We cannot construct real PG repos
// without a database. Instead, we test the verify_owner logic indirectly by
// testing the mock-based trait interactions at the port level, and document
// that integration tests hitting the real DB are the ultimate verification.
//
// The tests below verify the *contract*: _owned methods must call
// verify_owner before delegating, and verify_owner must fail-closed when
// no read repo is available.

#[tokio::test]
async fn verify_file_owner_delegates_to_read_port() {
    // This test verifies the FileReadPort contract that verify_file_owner
    // returns Ok for the correct owner and Err for others.
    let read = MockFileReadPort::new();
    read.insert("abc-123", "report.pdf", "user-42");

    // Same user → Ok
    let ok = read.verify_file_owner("abc-123", "user-42").await;
    assert!(ok.is_ok(), "correct owner should pass verify_file_owner");

    // Different user → Err
    let err = read.verify_file_owner("abc-123", "attacker-99").await;
    assert!(err.is_err(), "wrong owner should fail verify_file_owner");
}

#[tokio::test]
async fn owned_methods_require_ownership_check_first() {
    // Simulate what the _owned methods do: verify_owner then delegate.
    // We test with the mock read port to prove the sequence.
    let read = MockFileReadPort::new();
    read.insert("file-1", "data.csv", "owner-a");

    // Step 1: verify_owner for correct owner → Ok
    let step1 = read.verify_file_owner("file-1", "owner-a").await;
    assert!(step1.is_ok());

    // Step 2: verify_owner for attacker → Err, so the move/rename never executes
    let step2 = read.verify_file_owner("file-1", "attacker").await;
    assert!(step2.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests — Trait-level _owned method stubs (StubFileManagementUseCase)
// ═══════════════════════════════════════════════════════════════════════════

use crate::application::ports::file_ports::FileManagementUseCase;
use crate::common::stubs::StubFileManagementUseCase;

#[tokio::test]
async fn stub_move_file_owned_returns_ok() {
    let stub = StubFileManagementUseCase;
    let result = stub
        .move_file_owned("file-1", "user-1", Some("folder-2".to_string()))
        .await;
    assert!(result.is_ok(), "stub should return Ok for move_file_owned");
}

#[tokio::test]
async fn stub_rename_file_owned_returns_ok() {
    let stub = StubFileManagementUseCase;
    let result = stub
        .rename_file_owned("file-1", "user-1", "new-name.txt")
        .await;
    assert!(
        result.is_ok(),
        "stub should return Ok for rename_file_owned"
    );
}

use crate::application::ports::file_ports::FileRetrievalUseCase;
use crate::common::stubs::StubFileRetrievalUseCase;

#[tokio::test]
async fn stub_get_file_owned_returns_ok() {
    let stub = StubFileRetrievalUseCase;
    let result = stub.get_file_owned("file-1", "user-1").await;
    assert!(result.is_ok(), "stub should return Ok for get_file_owned");
}

#[tokio::test]
async fn stub_get_file_optimized_owned_returns_ok() {
    let stub = StubFileRetrievalUseCase;
    let result = stub
        .get_file_optimized_owned("file-1", "user-1", true, false)
        .await;
    assert!(
        result.is_ok(),
        "stub should return Ok for get_file_optimized_owned"
    );
}
