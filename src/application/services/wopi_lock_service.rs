//! In-memory WOPI lock service.
//!
//! Manages file locks required by the WOPI protocol for concurrent editing.
//! Uses an in-memory HashMap — suitable for single-instance deployments.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// A lock entry for a file.
#[derive(Debug, Clone)]
struct LockEntry {
    lock_id: String,
    expires_at: Instant,
}

/// Error returned when a lock operation conflicts.
#[derive(Debug)]
pub struct LockConflict {
    /// The lock ID currently held on the file
    pub existing_lock_id: String,
}

/// In-memory WOPI lock manager.
#[derive(Clone)]
pub struct WopiLockService {
    locks: Arc<RwLock<HashMap<String, LockEntry>>>,
    lock_duration: Duration,
}

impl WopiLockService {
    pub fn new(lock_ttl_secs: u64) -> Self {
        Self {
            locks: Arc::new(RwLock::new(HashMap::new())),
            lock_duration: Duration::from_secs(lock_ttl_secs),
        }
    }

    /// Lock a file. If already locked with the same lock_id, refreshes the timer.
    pub async fn lock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict> {
        let mut locks = self.locks.write().await;
        if let Some(entry) = locks.get(file_id) {
            if entry.lock_id == lock_id || entry.expires_at <= Instant::now() {
                // Same lock or expired — allow
            } else {
                return Err(LockConflict {
                    existing_lock_id: entry.lock_id.clone(),
                });
            }
        }
        locks.insert(
            file_id.to_string(),
            LockEntry {
                lock_id: lock_id.to_string(),
                expires_at: Instant::now() + self.lock_duration,
            },
        );
        Ok(())
    }

    /// Unlock a file. The lock_id must match.
    pub async fn unlock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict> {
        let mut locks = self.locks.write().await;
        if let Some(entry) = locks.get(file_id)
            && entry.lock_id != lock_id
            && entry.expires_at > Instant::now()
        {
            return Err(LockConflict {
                existing_lock_id: entry.lock_id.clone(),
            });
        }
        locks.remove(file_id);
        Ok(())
    }

    /// Refresh the lock timer. The file must be locked with the given lock_id.
    pub async fn refresh_lock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict> {
        let mut locks = self.locks.write().await;
        match locks.get(file_id) {
            None => {
                // No lock exists — WOPI spec requires 409 with empty lock
                return Err(LockConflict {
                    existing_lock_id: String::new(),
                });
            }
            Some(entry) if entry.expires_at <= Instant::now() => {
                // Lock expired — treat as unlocked
                locks.remove(file_id);
                return Err(LockConflict {
                    existing_lock_id: String::new(),
                });
            }
            Some(entry) if entry.lock_id != lock_id => {
                // Different lock holder
                return Err(LockConflict {
                    existing_lock_id: entry.lock_id.clone(),
                });
            }
            Some(_) => {
                // Matching lock — refresh the timer
            }
        }
        locks.insert(
            file_id.to_string(),
            LockEntry {
                lock_id: lock_id.to_string(),
                expires_at: Instant::now() + self.lock_duration,
            },
        );
        Ok(())
    }

    /// Get the current lock ID for a file, if locked.
    pub async fn get_lock(&self, file_id: &str) -> Option<String> {
        let locks = self.locks.read().await;
        locks.get(file_id).and_then(|entry| {
            if entry.expires_at > Instant::now() {
                Some(entry.lock_id.clone())
            } else {
                None
            }
        })
    }

    /// Remove expired locks. Call this periodically.
    pub async fn cleanup_expired(&self) {
        let mut locks = self.locks.write().await;
        let now = Instant::now();
        locks.retain(|_, entry| entry.expires_at > now);
    }

    /// Start a background task that cleans up expired locks every 60 seconds.
    pub fn start_cleanup_task(self: &Arc<Self>) {
        let service = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                service.cleanup_expired().await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lock_and_unlock() {
        let svc = WopiLockService::new(1800);
        svc.lock("file-1", "lock-abc").await.expect("Should lock");
        assert_eq!(svc.get_lock("file-1").await, Some("lock-abc".to_string()));
        svc.unlock("file-1", "lock-abc")
            .await
            .expect("Should unlock");
        assert_eq!(svc.get_lock("file-1").await, None);
    }

    #[tokio::test]
    async fn test_lock_conflict() {
        let svc = WopiLockService::new(1800);
        svc.lock("file-1", "lock-abc").await.expect("Should lock");
        let result = svc.lock("file-1", "lock-xyz").await;
        assert!(result.is_err());
        let conflict = result.unwrap_err();
        assert_eq!(conflict.existing_lock_id, "lock-abc");
    }

    #[tokio::test]
    async fn test_same_lock_refreshes() {
        let svc = WopiLockService::new(1800);
        svc.lock("file-1", "lock-abc").await.expect("Should lock");
        svc.lock("file-1", "lock-abc")
            .await
            .expect("Same lock should succeed");
    }

    #[tokio::test]
    async fn test_refresh_lock() {
        let svc = WopiLockService::new(1800);
        svc.lock("file-1", "lock-abc").await.expect("Should lock");
        svc.refresh_lock("file-1", "lock-abc")
            .await
            .expect("Should refresh");
        assert_eq!(svc.get_lock("file-1").await, Some("lock-abc".to_string()));
    }

    #[tokio::test]
    async fn test_unlock_conflict() {
        let svc = WopiLockService::new(1800);
        svc.lock("file-1", "lock-abc").await.expect("Should lock");
        let result = svc.unlock("file-1", "wrong-lock").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_lock_returns_none_for_unlocked() {
        let svc = WopiLockService::new(1800);
        assert_eq!(svc.get_lock("file-1").await, None);
    }

    #[tokio::test]
    async fn test_refresh_lock_on_unlocked_file_returns_conflict() {
        let svc = WopiLockService::new(1800);
        let result = svc.refresh_lock("file-1", "lock-abc").await;
        assert!(result.is_err());
        let conflict = result.unwrap_err();
        assert_eq!(conflict.existing_lock_id, "");
    }

    #[tokio::test]
    async fn test_refresh_lock_on_expired_lock_returns_conflict() {
        let svc = WopiLockService::new(0); // 0 seconds = immediate expiry
        svc.lock("file-1", "lock-old").await.expect("Should lock");
        tokio::time::sleep(Duration::from_millis(10)).await;
        let result = svc.refresh_lock("file-1", "lock-old").await;
        assert!(result.is_err());
        let conflict = result.unwrap_err();
        assert_eq!(conflict.existing_lock_id, "");
    }

    #[tokio::test]
    async fn test_expired_lock_allows_new_lock() {
        let svc = WopiLockService::new(0); // 0 seconds = immediate expiry
        svc.lock("file-1", "lock-old").await.expect("Should lock");
        tokio::time::sleep(Duration::from_millis(10)).await;
        // Expired lock should not block a new lock from a different holder
        svc.lock("file-1", "lock-new")
            .await
            .expect("Expired lock should allow new lock");
    }
}
