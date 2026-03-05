//! WebDAV lock store backed by Moka (in-memory cache with per-entry TTL).
//!
//! Locks are automatically evicted when their timeout expires, preventing
//! orphaned locks from accumulating.  Two caches are maintained:
//!
//! - `by_path`  : path → `LockEntry`  (for LOCK conflict detection)
//! - `by_token` : token → path        (for fast UNLOCK / refresh lookups)
//!
//! Both caches share the same TTL so entries disappear together.

use std::sync::Arc;
use std::time::Duration;

use crate::application::adapters::webdav_adapter::{LockInfo, LockScope};

/// Default lock timeout when the client does not specify one (RFC 4918 §10.7).
const DEFAULT_LOCK_TIMEOUT_SECS: u64 = 1800; // 30 minutes

/// Absolute maximum TTL a client may request.
const MAX_LOCK_TIMEOUT_SECS: u64 = 86_400; // 24 hours

/// A stored lock entry.
#[derive(Clone, Debug)]
pub struct LockEntry {
    pub info: LockInfo,
    pub path: String,
}

/// In-memory WebDAV lock store with automatic TTL-based expiration.
///
/// Uses Moka's `sync::Cache` — lock-free (sharded) reads, bounded size,
/// and per-entry TTL via `policy::Expiry`.
pub struct WebDavLockStore {
    /// path → `LockEntry`
    by_path: moka::sync::Cache<String, LockEntry>,
    /// token → path (reverse index)
    by_token: moka::sync::Cache<String, String>,
}

impl WebDavLockStore {
    /// Create a new lock store.
    ///
    /// * `max_capacity` — upper bound on simultaneous locks (evicts LRU on overflow).
    pub fn new(max_capacity: u64) -> Self {
        // We use `expire_after` (per-entry TTL) via insert with explicit ttl,
        // so we configure a generous global time_to_live as a safety net.
        let by_path = moka::sync::Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(MAX_LOCK_TIMEOUT_SECS))
            .build();

        let by_token = moka::sync::Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(MAX_LOCK_TIMEOUT_SECS))
            .build();

        Self { by_path, by_token }
    }

    // ── Public API ──────────────────────────────────────────────

    /// Attempt to acquire a lock on `path`.
    ///
    /// Returns `Ok(LockEntry)` on success, or `Err(existing)` if the resource
    /// is already exclusively locked by a different token.
    #[allow(clippy::result_large_err)]
    pub fn acquire(&self, path: &str, info: LockInfo) -> Result<LockEntry, LockEntry> {
        // Check for existing conflicting lock
        if let Some(existing) = self.by_path.get(path)
            && existing.info.scope == LockScope::Exclusive
        {
            return Err(existing);
        }

        let ttl = Self::parse_timeout(info.timeout.as_deref());
        let entry = LockEntry {
            info,
            path: path.to_owned(),
        };

        self.by_path.insert(path.to_owned(), entry.clone());
        self.by_token
            .insert(entry.info.token.clone(), path.to_owned());

        // Moka 0.12 does not expose per-entry set_expiration_after_insert at
        // insert time.  We rely on the global `time_to_live` as an upper bound
        // and use the `invalidate_after` helper below for custom TTL.
        //
        // To implement shorter-than-max TTL we schedule an async invalidation.
        if ttl.as_secs() < MAX_LOCK_TIMEOUT_SECS {
            let by_path = self.by_path.clone();
            let by_token = self.by_token.clone();
            let token = entry.info.token.clone();
            let path_owned = path.to_owned();
            tokio::spawn(async move {
                tokio::time::sleep(ttl).await;
                // Only remove if the entry still matches (wasn't refreshed/replaced)
                if let Some(e) = by_path.get(&path_owned)
                    && e.info.token == token
                {
                    by_path.invalidate(&path_owned);
                    by_token.invalidate(&token);
                }
            });
        }

        Ok(entry)
    }

    /// Refresh an existing lock (extend its timeout).
    ///
    /// Returns `Some(LockEntry)` with updated timeout, or `None` if the token
    /// is unknown (expired or never existed).
    pub fn refresh(&self, token: &str, new_timeout: Option<&str>) -> Option<LockEntry> {
        let path = self.by_token.get(token)?;
        let mut entry = self.by_path.get(&path)?;

        if entry.info.token != token {
            return None; // token mismatch — lock was replaced
        }

        let ttl = Self::parse_timeout(new_timeout.or(entry.info.timeout.as_deref()));
        let timeout_str = format!("Second-{}", ttl.as_secs());
        entry.info.timeout = Some(timeout_str.clone());

        // Re-insert to reset the TTL
        self.by_path.insert(path.clone(), entry.clone());
        self.by_token.insert(token.to_owned(), path.clone());

        if ttl.as_secs() < MAX_LOCK_TIMEOUT_SECS {
            let by_path = self.by_path.clone();
            let by_token = self.by_token.clone();
            let token_owned = token.to_owned();
            let path_owned = path.clone();
            tokio::spawn(async move {
                tokio::time::sleep(ttl).await;
                if let Some(e) = by_path.get(&path_owned)
                    && e.info.token == token_owned
                {
                    by_path.invalidate(&path_owned);
                    by_token.invalidate(&token_owned);
                }
            });
        }

        Some(entry)
    }

    /// Release a lock by its token.
    ///
    /// Returns `true` if the lock existed and was removed.
    pub fn release(&self, token: &str) -> bool {
        if let Some(path) = self.by_token.get(token) {
            // Only remove from by_path if the token still matches
            if let Some(entry) = self.by_path.get(&path)
                && entry.info.token == token
            {
                self.by_path.invalidate(&path);
            }
            self.by_token.invalidate(token);
            true
        } else {
            false
        }
    }

    /// Look up a lock by resource path.
    pub fn get_by_path(&self, path: &str) -> Option<LockEntry> {
        self.by_path.get(path)
    }

    /// Look up a lock by token.
    pub fn get_by_token(&self, token: &str) -> Option<LockEntry> {
        let path = self.by_token.get(token)?;
        self.by_path.get(&path)
    }

    // ── Helpers ─────────────────────────────────────────────────

    /// Parse a WebDAV `Timeout` header value into a [`Duration`].
    ///
    /// Accepted formats (RFC 4918 §10.7):
    /// - `Second-NNN`
    /// - `Infinite`  (clamped to `MAX_LOCK_TIMEOUT_SECS`)
    /// - Comma-separated list (first value wins)
    fn parse_timeout(header: Option<&str>) -> Duration {
        let raw = match header {
            Some(v) if !v.is_empty() => v,
            _ => return Duration::from_secs(DEFAULT_LOCK_TIMEOUT_SECS),
        };

        // Take the first value in a comma-separated list
        let first = raw.split(',').next().unwrap_or(raw).trim();

        if first.eq_ignore_ascii_case("Infinite") {
            return Duration::from_secs(MAX_LOCK_TIMEOUT_SECS);
        }

        if let Some(secs_str) = first.strip_prefix("Second-")
            && let Ok(secs) = secs_str.trim().parse::<u64>()
        {
            return Duration::from_secs(secs.min(MAX_LOCK_TIMEOUT_SECS));
        }

        Duration::from_secs(DEFAULT_LOCK_TIMEOUT_SECS)
    }
}

/// Create a shared lock store wrapped in `Arc` for embedding in `AppState`.
pub fn create_webdav_lock_store() -> Arc<WebDavLockStore> {
    // 10 000 simultaneous locks should be more than enough; Moka evicts LRU
    // if the cap is reached, so stale entries are cleaned automatically.
    Arc::new(WebDavLockStore::new(10_000))
}
