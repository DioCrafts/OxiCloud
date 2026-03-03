//! Account lockout service — blocks login for an account after N consecutive
//! failed attempts.
//!
//! Uses a `moka` TTL cache so that:
//! * Failed-attempt counters automatically expire after the lockout window.
//! * No database writes are needed — this is **in-memory** and therefore
//!   per-instance.  If OxiCloud is deployed behind a load balancer with
//!   multiple replicas, a sticky-session or shared Redis store would be
//!   needed for cross-instance coordination (out of scope for v1).
//!
//! Typical flow:
//! 1. **Before password verification** → call [`LoginLockoutService::check`].
//!    If the account is locked, return `403` immediately without touching
//!    Argon2 (saves CPU).
//! 2. **After failed verification** → call [`LoginLockoutService::record_failure`].
//! 3. **After successful login** → call [`LoginLockoutService::record_success`]
//!    to reset the counter.

use moka::sync::Cache;
use std::time::Duration;

/// Tracks consecutive failures for a single username.
#[derive(Clone, Debug)]
struct FailureRecord {
    /// Number of consecutive failed attempts.
    count: u32,
}

/// In-memory account lockout tracker.
#[derive(Clone)]
pub struct LoginLockoutService {
    /// Maps `username -> FailureRecord`.  TTL = lockout window.
    cache: Cache<String, FailureRecord>,
    /// Maximum consecutive failures before the account is temporarily locked.
    max_failures: u32,
    /// How long the lockout lasts (seconds).
    lockout_secs: u64,
}

impl LoginLockoutService {
    /// Create a new lockout service.
    ///
    /// * `max_failures`  — e.g. `5` (lock after 5 bad passwords)
    /// * `lockout_secs`  — e.g. `900` (15-minute lockout)
    /// * `max_accounts`  — upper bound on tracked accounts (evicts LRU)
    pub fn new(max_failures: u32, lockout_secs: u64, max_accounts: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(lockout_secs))
            .max_capacity(max_accounts)
            .build();
        Self {
            cache,
            max_failures,
            lockout_secs,
        }
    }

    /// Check whether the account is currently locked.
    ///
    /// Returns `Ok(())` if the user may attempt login, or
    /// `Err(remaining_secs)` with the *approximate* remaining lockout time.
    pub fn check(&self, username: &str) -> Result<(), u64> {
        if let Some(rec) = self.cache.get(&username.to_lowercase()) {
            if rec.count >= self.max_failures {
                // The entry exists and is over the threshold.  Because moka
                // evicts at TTL we know the lockout window has not yet elapsed.
                return Err(self.lockout_secs);
            }
        }
        Ok(())
    }

    /// Record a failed login attempt.  Returns the new failure count.
    pub fn record_failure(&self, username: &str) -> u32 {
        let key = username.to_lowercase();
        let new_count = self.cache.get(&key).map(|r| r.count + 1).unwrap_or(1);
        self.cache
            .insert(key.clone(), FailureRecord { count: new_count });

        if new_count >= self.max_failures {
            tracing::warn!(
                username = %username,
                attempts = new_count,
                lockout_secs = self.lockout_secs,
                "Account temporarily locked after {} consecutive failed login attempts",
                new_count,
            );
        }
        new_count
    }

    /// Record a successful login — resets the failure counter.
    pub fn record_success(&self, username: &str) {
        self.cache.invalidate(&username.to_lowercase());
    }

    /// Maximum failures before lockout (used to inform callers / error messages).
    pub fn max_failures(&self) -> u32 {
        self.max_failures
    }

    /// Lockout duration in seconds.
    pub fn lockout_secs(&self) -> u64 {
        self.lockout_secs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_login_under_threshold() {
        let svc = LoginLockoutService::new(3, 60, 100);
        assert!(svc.check("alice").is_ok());
        svc.record_failure("alice");
        svc.record_failure("alice");
        // 2 failures — still under threshold
        assert!(svc.check("alice").is_ok());
    }

    #[test]
    fn locks_after_threshold() {
        let svc = LoginLockoutService::new(3, 60, 100);
        svc.record_failure("bob");
        svc.record_failure("bob");
        svc.record_failure("bob");
        assert!(svc.check("bob").is_err());
    }

    #[test]
    fn resets_on_success() {
        let svc = LoginLockoutService::new(3, 60, 100);
        svc.record_failure("carol");
        svc.record_failure("carol");
        svc.record_success("carol");
        // Counter reset — should be allowed again
        assert!(svc.check("carol").is_ok());
        svc.record_failure("carol"); // starts over at 1
        assert!(svc.check("carol").is_ok());
    }

    #[test]
    fn case_insensitive() {
        let svc = LoginLockoutService::new(2, 60, 100);
        svc.record_failure("Dave");
        svc.record_failure("dave");
        assert!(svc.check("DAVE").is_err());
    }
}
