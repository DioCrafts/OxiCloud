# Caching Architecture

OxiCloud uses **moka** (a lock-free, concurrent cache) for write-behind caching that delivers sub-millisecond hot reads.

## Cache Layers

| Cache | TTL | Max Entries | Purpose |
|---|---|---|---|
| File metadata | 60 s | 10 000 | Avoid re-querying PostgreSQL for file info |
| Directory listings | 120 s | 10 000 | Frequently accessed folder contents |
| Thumbnail cache | configurable | 1 000 | Generated WebP/AVIF thumbnails |
| Image transcode | configurable | 500 | On-the-fly image transcoding results |
| Blob hash | 30 s TTI | 5 000 | SHA-256 hashes for dedup lookups |
| Audio metadata | — | 2 000 | ID3 tags and duration |

## How It Works

1. **Read path:** check cache → if hit, return immediately (sub-ms); if miss, query PostgreSQL, populate cache, return
2. **Write path:** update PostgreSQL → invalidate relevant cache entries
3. **TTL expiry:** entries are evicted after their time-to-live, ensuring eventual consistency

## Why moka?

- **Lock-free** — no mutex contention under concurrent access
- **Bounded memory** — max entries prevent unbounded growth
- **TTL + TTI** — supports both time-to-live and time-to-idle eviction
- **Async-ready** — works natively with Tokio

## Configuration

Cache parameters are currently hardcoded in `src/common/config.rs`. Key defaults:

```rust
file_cache_ttl_ms: 60_000,       // 1 minute
directory_cache_ttl_ms: 120_000,  // 2 minutes
max_cache_entries: 10_000,
```
