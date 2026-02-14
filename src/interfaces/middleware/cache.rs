use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Method, Request, Response, StatusCode},
    middleware::Next,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tower::{Layer, Service};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use bytes::Bytes;
use tracing::{debug, info};

const MAX_CACHE_ENTRIES: usize = 1000;  // Maximum number of cache entries
const DEFAULT_MAX_AGE: u64 = 60;        // Default time-to-live in seconds

// Type definitions for clarity
type CacheKey = String;
type EntityTag = String;

/// A cached value
#[derive(Clone)]
struct CacheEntry {
    /// The ETag calculated for this value
    etag: EntityTag,
    /// The serialized data in bytes
    data: Option<Bytes>,
    /// The original headers
    headers: HeaderMap,
    /// Timestamp of when it was stored
    timestamp: SystemTime,
    /// Time-to-live in seconds
    max_age: u64,
}

/// Cache for HTTP responses with ETag support
#[derive(Clone)]
pub struct HttpCache {
    /// Cache entry storage
    cache: Arc<Mutex<HashMap<CacheKey, CacheEntry>>>,
    /// Default time-to-live for entries
    default_max_age: u64,
}

impl HttpCache {
    /// Creates a new cache instance
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::with_capacity(100))),
            default_max_age: DEFAULT_MAX_AGE,
        }
    }
    
    /// Creates a new instance with a specified time-to-live
    pub fn with_max_age(max_age: u64) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::with_capacity(100))),
            default_max_age: max_age,
        }
    }
    
    /// Gets cache statistics
    pub fn stats(&self) -> (usize, usize) {
        let lock = self.cache.lock().unwrap();
        let total = lock.len();
        
        // Count valid entries
        let _now = SystemTime::now();
        let valid = lock.values().filter(|entry| {
            match entry.timestamp.elapsed() {
                Ok(elapsed) => elapsed.as_secs() < entry.max_age,
                Err(_) => false,
            }
        }).count();
        
        (total, valid)
    }
    
    /// Cleans up expired entries
    pub fn cleanup(&self) -> usize {
        let mut lock = self.cache.lock().unwrap();
        let initial_count = lock.len();
        
        // Remove expired entries
        let _now = SystemTime::now();
        lock.retain(|_, entry| {
            match entry.timestamp.elapsed() {
                Ok(elapsed) => elapsed.as_secs() < entry.max_age,
                Err(_) => false,
            }
        });
        
        let removed = initial_count - lock.len();
        debug!("HttpCache cleanup: removed {} expired entries", removed);
        
        removed
    }
    
    /// Sets an entry in the cache
    fn set(&self, key: &str, etag: EntityTag, data: Option<Bytes>, headers: HeaderMap, max_age: Option<u64>) {
        let mut lock = self.cache.lock().unwrap();
        
        // Apply eviction policy if the cache is full
        if lock.len() >= MAX_CACHE_ENTRIES {
            debug!("Cache full, removing oldest entries");
            // Remove the oldest 10% of entries
            self.evict_oldest(&mut lock, MAX_CACHE_ENTRIES / 10);
        }
        
        // Store the new entry
        lock.insert(key.to_string(), CacheEntry {
            etag,
            data,
            headers,
            timestamp: SystemTime::now(),
            max_age: max_age.unwrap_or(self.default_max_age),
        });
    }
    
    /// Removes the oldest entries from the cache
    fn evict_oldest(&self, cache: &mut HashMap<CacheKey, CacheEntry>, count: usize) {
        // Sort by timestamp
        let mut entries: Vec<(CacheKey, SystemTime)> = cache
            .iter()
            .map(|(key, entry)| (key.clone(), entry.timestamp))
            .collect();
        
        // Sort by timestamp (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        
        // Remove the oldest entries
        for (key, _) in entries.iter().take(count) {
            cache.remove(key);
        }
    }
    
    /// Gets an entry from the cache
    fn get(&self, key: &str) -> Option<CacheEntry> {
        let lock = self.cache.lock().unwrap();
        
        // Look up the entry
        if let Some(entry) = lock.get(key) {
            // Check if it has expired
            match entry.timestamp.elapsed() {
                Ok(elapsed) if elapsed.as_secs() < entry.max_age => {
                    // Entry is still valid
                    return Some(entry.clone());
                }
                _ => {
                    // Entry has expired
                    return None;
                }
            }
        }
        
        None
    }
    
    /// Generates a simple ETag for a block of bytes
    fn calculate_etag_for_bytes(&self, bytes: &[u8]) -> EntityTag {
        // Calculate hash
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        let hash = hasher.finish();
        
        format!("\"{}\"", hash)
    }
}

/// HTTP cache middleware
pub async fn cache_middleware<T>(
    cache: HttpCache,
    cache_key: &str,
    max_age: Option<u64>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)> 
where 
    T: Serialize
{
    // Only apply cache for GET requests
    if req.method() != Method::GET {
        return Ok(next.run(req).await);
    }
    
    // Check if the response is cached
    let if_none_match = req.headers()
        .get("if-none-match")
        .and_then(|v| v.to_str().ok());
    
    // If there is a cache entry
    if let Some(cache_entry) = cache.get(cache_key) {
        // Check if the client already has the updated version
        if let Some(client_etag) = if_none_match
            && client_etag == cache_entry.etag {
                // The client has the most recent version, send 304 Not Modified
                debug!("Cache hit (304) for key: {}", cache_key);
                return Ok(create_not_modified_response(&cache_entry));
            }
        
        // The client needs the updated version
        if let Some(data) = &cache_entry.data {
            debug!("Cache hit (200) for key: {}", cache_key);
            
            // Create response with cached data
            let mut response = Response::new(Body::from(data.clone()));
            
            // Copy original headers
            for (key, value) in &cache_entry.headers {
                if !key.as_str().eq_ignore_ascii_case("transfer-encoding") {
                    response.headers_mut().insert(key.clone(), value.clone());
                }
            }
            
            // Add cache headers
            set_cache_headers(&mut response, &cache_entry.etag, max_age.unwrap_or(cache_entry.max_age));
            
            return Ok(response);
        }
    }
    
    // Not cached or expired, continue with the middleware
    debug!("Cache miss for key: {}", cache_key);
    let response = next.run(req).await;
    
    // Don't cache errors
    if !response.status().is_success() {
        return Ok(response);
    }
    
    // Convert the response to calculate the ETag
    let (parts, _body) = response.into_parts();
    let bytes = axum::body::to_bytes(_body, 1024 * 1024 * 10).await.unwrap_or_default();
    
    // Calculate ETag
    let etag = cache.calculate_etag_for_bytes(&bytes);
    
    // Save to cache
    cache.set(
        cache_key, 
        etag.clone(), 
        Some(bytes.clone()),
        parts.headers.clone(),
        max_age
    );
    
    // Create the response with ETag
    let mut response = Response::from_parts(parts, Body::from(bytes));
    set_cache_headers(&mut response, &etag, max_age.unwrap_or(cache.default_max_age));
    
    Ok(response)
}

/// Creates a 304 Not Modified response
fn create_not_modified_response(entry: &CacheEntry) -> Response<Body> {
    let mut response = Response::builder()
        .status(StatusCode::NOT_MODIFIED)
        .body(Body::empty())
        .unwrap();
    
    // Copy cache headers
    if let Some(cache_control) = entry.headers.get("cache-control") {
        response.headers_mut().insert("cache-control", cache_control.clone());
    }
    
    // Add ETag
    response.headers_mut().insert(
        "etag", 
        HeaderValue::from_str(&entry.etag).unwrap_or(HeaderValue::from_static(""))
    );
    
    response
}

/// Configures cache headers for a response
fn set_cache_headers(response: &mut Response<Body>, etag: &str, max_age: u64) {
    // Add ETag
    response.headers_mut().insert(
        "etag", 
        HeaderValue::from_str(etag).unwrap_or(HeaderValue::from_static(""))
    );
    
    // Configure Cache-Control
    let cache_control = format!("public, max-age={}", max_age);
    response.headers_mut().insert(
        "cache-control",
        HeaderValue::from_str(&cache_control).unwrap_or(HeaderValue::from_static(""))
    );
    
    // Add Last-Modified header
    let now: DateTime<Utc> = Utc::now();
    let last_modified = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    response.headers_mut().insert(
        "last-modified",
        HeaderValue::from_str(&last_modified).unwrap_or(HeaderValue::from_static(""))
    );
}

/// Layer for applying cache middleware
#[derive(Clone)]
pub struct HttpCacheLayer {
    cache: HttpCache,
    max_age: Option<u64>,
}

impl HttpCacheLayer {
    /// Creates a new cache layer
    pub fn new(cache: HttpCache) -> Self {
        Self {
            cache,
            max_age: None,
        }
    }
    
    /// Sets the maximum time-to-live
    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = Some(max_age);
        self
    }
}

impl<S> Layer<S> for HttpCacheLayer {
    type Service = HttpCacheService<S>;
    
    fn layer(&self, service: S) -> Self::Service {
        HttpCacheService {
            inner: service,
            cache: self.cache.clone(),
            max_age: self.max_age,
        }
    }
}

/// Service that implements cache logic
#[derive(Clone)]
pub struct HttpCacheService<S> {
    inner: S,
    cache: HttpCache,
    max_age: Option<u64>,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for HttpCacheService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    ReqBody: Send + 'static,
    ResBody: http_body::Body + Send + 'static,
    ResBody::Data: Send + 'static,
    ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|e| e.into())
    }
    
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // Generate cache key
        let cache_key = req.uri().path().to_string();
        
        // Only apply cache for GET requests
        if req.method() != Method::GET {
            let future = self.inner.call(req);
            return Box::pin(async move {
                let response = future.await.map_err(|e| e.into())?;
                Ok(response_map_body(response).await)
            });
        }
        
        // Get client ETag
        let if_none_match = req.headers()
            .get("if-none-match")
            .and_then(|v| v.to_str().ok());
        
        // Check if there is a cache entry
        let cache_clone = self.cache.clone();
        let max_age = self.max_age;
        let entry = cache_clone.get(&cache_key);
        
        match entry {
            Some(cache_entry) if if_none_match == Some(&cache_entry.etag) => {
                // The client has the correct version, send 304
                debug!("Cache HIT (304): {}", cache_key);
                let response = create_not_modified_response(&cache_entry);
                Box::pin(async move { Ok(response) })
            },
            Some(cache_entry) if cache_entry.data.is_some() => {
                // The client needs the updated version
                debug!("Cache HIT (200): {}", cache_key);
                let mut response = Response::new(Body::from(cache_entry.data.clone().unwrap()));
                
                // Copy original headers
                for (key, value) in &cache_entry.headers {
                    if !key.as_str().eq_ignore_ascii_case("transfer-encoding") {
                        response.headers_mut().insert(key.clone(), value.clone());
                    }
                }
                
                // Add cache headers
                set_cache_headers(&mut response, &cache_entry.etag, max_age.unwrap_or(cache_entry.max_age));
                
                Box::pin(async move { Ok(response) })
            },
            _ => {
                // Not cached or expired
                debug!("Cache MISS: {}", cache_key);
                let future = self.inner.call(req);
                let cache_clone = self.cache.clone();
                let max_age = self.max_age;
                let cache_key = cache_key.clone();
                
                Box::pin(async move {
                    let response = future.await.map_err(|e| e.into())?;
                    let response = response_map_body(response).await;
                    
                    // Don't cache errors
                    if !response.status().is_success() {
                        return Ok(response);
                    }
                    
                    // Get the body and calculate ETag
                    let (parts, body) = response.into_parts();
                    let bytes = axum::body::to_bytes(body, 1024 * 1024 * 10).await?;
                    
                    // Calculate ETag
                    let etag = cache_clone.calculate_etag_for_bytes(&bytes);
                    
                    // Save to cache
                    cache_clone.set(
                        &cache_key, 
                        etag.clone(), 
                        Some(bytes.clone()),
                        parts.headers.clone(),
                        max_age
                    );
                    
                    // Create the response with ETag
                    let mut response = Response::from_parts(parts, Body::from(bytes));
                    set_cache_headers(&mut response, &etag, max_age.unwrap_or(cache_clone.default_max_age));
                    
                    Ok(response)
                })
            }
        }
    }
}

// Helper function to convert any body into Body preserving its content.
// Previously this function discarded the body with Body::empty(), causing
// data loss in non-cached responses.
async fn response_map_body<B>(response: Response<B>) -> Response<Body>
where
    B: http_body::Body + Send + 'static,
    B::Data: Send + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    use http_body_util::BodyExt;

    let (parts, body) = response.into_parts();

    // Collect the full body into Bytes, preserving all response data
    let collected = body
        .collect()
        .await
        .map(|c| c.to_bytes())
        .unwrap_or_default();

    Response::from_parts(parts, Body::from(collected))
}

/// Starts a periodic cleanup task for the cache
pub fn start_cache_cleanup_task(cache: HttpCache) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes
        
        loop {
            interval.tick().await;
            let removed = cache.cleanup();
            let (total, valid) = cache.stats();
            
            info!("HTTP Cache cleanup: removed {}, current: {}/{}", removed, valid, total);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize, Hash)]
    struct TestData {
        id: u32,
        name: String,
    }
    
    #[tokio::test]
    async fn test_etag_generation() {
        let cache = HttpCache::new();
        
        let data1 = serde_json::to_vec(&TestData { id: 1, name: "Test".to_string() }).unwrap();
        let data2 = serde_json::to_vec(&TestData { id: 1, name: "Test".to_string() }).unwrap();
        let data3 = serde_json::to_vec(&TestData { id: 2, name: "Test".to_string() }).unwrap();
        
        let etag1 = cache.calculate_etag_for_bytes(&data1);
        let etag2 = cache.calculate_etag_for_bytes(&data2);
        let etag3 = cache.calculate_etag_for_bytes(&data3);
        
        // Same data should generate the same ETag
        assert_eq!(etag1, etag2);
        
        // Different data should generate different ETags
        assert_ne!(etag1, etag3);
    }
    
    #[tokio::test]
    async fn test_cache_hit_miss() {
        let cache = HttpCache::new();
        
        // Create test data directly as Bytes
        let bytes1 = Bytes::from(r#"{"id":1,"name":"Test"}"#);
        let headers1 = HeaderMap::new();
        
        let etag1 = cache.calculate_etag_for_bytes(&bytes1);
        cache.set("test", etag1.clone(), Some(bytes1.clone()), headers1, None);
        
        // Verify cache hit
        let entry = cache.get("test").unwrap();
        assert_eq!(entry.etag, etag1);
        assert_eq!(entry.data.unwrap(), bytes1);
        
        // Verify cache miss
        assert!(cache.get("nonexistent").is_none());
    }
}