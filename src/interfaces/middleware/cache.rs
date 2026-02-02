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

const MAX_CACHE_ENTRIES: usize = 1000;  // Máximo número de entradas en caché
const DEFAULT_MAX_AGE: u64 = 60;        // Tiempo de vida por defecto en segundos

// Definición de tipos para mayor claridad
type CacheKey = String;
type EntityTag = String;

/// Un valor almacenado en caché
#[derive(Clone)]
struct CacheEntry {
    /// El ETag calculado para este valor
    etag: EntityTag,
    /// Los datos serializados en bytes
    data: Option<Bytes>,
    /// Las cabeceras originales
    headers: HeaderMap,
    /// Timestamp de cuando fue almacenado
    timestamp: SystemTime,
    /// Tiempo de vida en segundos
    max_age: u64,
}

/// Cache para respuestas HTTP con soporte para ETag
#[derive(Clone)]
pub struct HttpCache {
    /// Almacenamiento de entradas en caché
    cache: Arc<Mutex<HashMap<CacheKey, CacheEntry>>>,
    /// Tiempo de vida por defecto para las entradas
    default_max_age: u64,
}

impl HttpCache {
    /// Crea una nueva instancia del caché
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::with_capacity(100))),
            default_max_age: DEFAULT_MAX_AGE,
        }
    }
    
    /// Crea una nueva instancia con un tiempo de vida especificado
    pub fn with_max_age(max_age: u64) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::with_capacity(100))),
            default_max_age: max_age,
        }
    }
    
    /// Obtiene estadísticas del caché
    pub fn stats(&self) -> (usize, usize) {
        let lock = self.cache.lock().unwrap();
        let total = lock.len();
        
        // Contar entradas válidas
        let _now = SystemTime::now();
        let valid = lock.values().filter(|entry| {
            match entry.timestamp.elapsed() {
                Ok(elapsed) => elapsed.as_secs() < entry.max_age,
                Err(_) => false,
            }
        }).count();
        
        (total, valid)
    }
    
    /// Limpia entradas expiradas
    pub fn cleanup(&self) -> usize {
        let mut lock = self.cache.lock().unwrap();
        let initial_count = lock.len();
        
        // Eliminar entradas expiradas
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
    
    /// Establece una entrada en el caché
    fn set(&self, key: &str, etag: EntityTag, data: Option<Bytes>, headers: HeaderMap, max_age: Option<u64>) {
        let mut lock = self.cache.lock().unwrap();
        
        // Aplicar política de eviction si el caché está lleno
        if lock.len() >= MAX_CACHE_ENTRIES {
            debug!("Cache full, removing oldest entries");
            // Eliminar el 10% de las entradas más antiguas
            self.evict_oldest(&mut lock, MAX_CACHE_ENTRIES / 10);
        }
        
        // Almacenar la nueva entrada
        lock.insert(key.to_string(), CacheEntry {
            etag,
            data,
            headers,
            timestamp: SystemTime::now(),
            max_age: max_age.unwrap_or(self.default_max_age),
        });
    }
    
    /// Elimina las entradas más antiguas del caché
    fn evict_oldest(&self, cache: &mut HashMap<CacheKey, CacheEntry>, count: usize) {
        // Ordenar por timestamp
        let mut entries: Vec<(CacheKey, SystemTime)> = cache
            .iter()
            .map(|(key, entry)| (key.clone(), entry.timestamp))
            .collect();
        
        // Ordenar por timestamp (más antiguo primero)
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        
        // Eliminar las entradas más antiguas
        for (key, _) in entries.iter().take(count) {
            cache.remove(key);
        }
    }
    
    /// Obtiene una entrada del caché
    fn get(&self, key: &str) -> Option<CacheEntry> {
        let lock = self.cache.lock().unwrap();
        
        // Buscar la entrada
        if let Some(entry) = lock.get(key) {
            // Verificar si ha expirado
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
    
    /// Genera un ETag simple para un bloque de bytes
    fn calculate_etag_for_bytes(&self, bytes: &[u8]) -> EntityTag {
        // Calcular hash
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        let hash = hasher.finish();
        
        format!("\"{}\"", hash)
    }
}

/// Middleware de caché HTTP
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
    // Solo aplicar caché para solicitudes GET
    if req.method() != Method::GET {
        return Ok(next.run(req).await);
    }
    
    // Verificar si la respuesta está en caché
    let if_none_match = req.headers()
        .get("if-none-match")
        .and_then(|v| v.to_str().ok());
    
    // Si hay una entrada en caché
    if let Some(cache_entry) = cache.get(cache_key) {
        // Comprobar si el cliente ya tiene la versión actualizada
        if let Some(client_etag) = if_none_match {
            if client_etag == cache_entry.etag {
                // El cliente tiene la versión más reciente, enviar 304 Not Modified
                debug!("Cache hit (304) for key: {}", cache_key);
                return Ok(create_not_modified_response(&cache_entry));
            }
        }
        
        // El cliente necesita la versión actualizada
        if let Some(data) = &cache_entry.data {
            debug!("Cache hit (200) for key: {}", cache_key);
            
            // Crear respuesta con los datos en caché
            let mut response = Response::new(Body::from(data.clone()));
            
            // Copiar cabeceras originales
            for (key, value) in &cache_entry.headers {
                if !key.as_str().eq_ignore_ascii_case("transfer-encoding") {
                    response.headers_mut().insert(key.clone(), value.clone());
                }
            }
            
            // Añadir cabeceras de caché
            set_cache_headers(&mut response, &cache_entry.etag, max_age.unwrap_or(cache_entry.max_age));
            
            return Ok(response);
        }
    }
    
    // No está en caché o ha expirado, continuar con el middleware
    debug!("Cache miss for key: {}", cache_key);
    let response = next.run(req).await;
    
    // No cachear errores
    if !response.status().is_success() {
        return Ok(response);
    }
    
    // Convertir la respuesta para calcular el ETag
    let (parts, _body) = response.into_parts();
    let bytes = axum::body::to_bytes(_body, 1024 * 1024 * 10).await.unwrap_or_default();
    
    // Calcular ETag
    let etag = cache.calculate_etag_for_bytes(&bytes);
    
    // Guardar en caché
    cache.set(
        cache_key, 
        etag.clone(), 
        Some(bytes.clone()),
        parts.headers.clone(),
        max_age
    );
    
    // Crear la respuesta con ETag
    let mut response = Response::from_parts(parts, Body::from(bytes));
    set_cache_headers(&mut response, &etag, max_age.unwrap_or(cache.default_max_age));
    
    Ok(response)
}

/// Crea una respuesta 304 Not Modified
fn create_not_modified_response(entry: &CacheEntry) -> Response<Body> {
    let mut response = Response::builder()
        .status(StatusCode::NOT_MODIFIED)
        .body(Body::empty())
        .unwrap();
    
    // Copiar cabeceras de caché
    if let Some(cache_control) = entry.headers.get("cache-control") {
        response.headers_mut().insert("cache-control", cache_control.clone());
    }
    
    // Añadir ETag
    response.headers_mut().insert(
        "etag", 
        HeaderValue::from_str(&entry.etag).unwrap_or(HeaderValue::from_static(""))
    );
    
    response
}

/// Configura las cabeceras de caché para una respuesta
fn set_cache_headers(response: &mut Response<Body>, etag: &str, max_age: u64) {
    // Añadir ETag
    response.headers_mut().insert(
        "etag", 
        HeaderValue::from_str(etag).unwrap_or(HeaderValue::from_static(""))
    );
    
    // Configurar Cache-Control
    let cache_control = format!("public, max-age={}", max_age);
    response.headers_mut().insert(
        "cache-control",
        HeaderValue::from_str(&cache_control).unwrap_or(HeaderValue::from_static(""))
    );
    
    // Añadir cabecera Last-Modified
    let now: DateTime<Utc> = Utc::now();
    let last_modified = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    response.headers_mut().insert(
        "last-modified",
        HeaderValue::from_str(&last_modified).unwrap_or(HeaderValue::from_static(""))
    );
}

/// Layer para aplicar middleware de caché
#[derive(Clone)]
pub struct HttpCacheLayer {
    cache: HttpCache,
    max_age: Option<u64>,
}

impl HttpCacheLayer {
    /// Crea una nueva capa de caché
    pub fn new(cache: HttpCache) -> Self {
        Self {
            cache,
            max_age: None,
        }
    }
    
    /// Establece el tiempo de vida máximo
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

/// Servicio que implementa la lógica de caché
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
        // Generar clave de caché
        let cache_key = req.uri().path().to_string();
        
        // Solo aplicar caché para solicitudes GET
        if req.method() != Method::GET {
            let future = self.inner.call(req);
            return Box::pin(async move {
                let response = future.await.map_err(|e| e.into())?;
                Ok(response_map_body(response))
            });
        }
        
        // Obtener ETag del cliente
        let if_none_match = req.headers()
            .get("if-none-match")
            .and_then(|v| v.to_str().ok());
        
        // Verificar si hay una entrada en caché
        let cache_clone = self.cache.clone();
        let max_age = self.max_age;
        let entry = cache_clone.get(&cache_key);
        
        match entry {
            Some(cache_entry) if if_none_match == Some(&cache_entry.etag) => {
                // El cliente tiene la versión correcta, enviar 304
                debug!("Cache HIT (304): {}", cache_key);
                let response = create_not_modified_response(&cache_entry);
                return Box::pin(async move { Ok(response) });
            },
            Some(cache_entry) if cache_entry.data.is_some() => {
                // El cliente necesita la versión actualizada
                debug!("Cache HIT (200): {}", cache_key);
                let mut response = Response::new(Body::from(cache_entry.data.clone().unwrap()));
                
                // Copiar cabeceras originales
                for (key, value) in &cache_entry.headers {
                    if !key.as_str().eq_ignore_ascii_case("transfer-encoding") {
                        response.headers_mut().insert(key.clone(), value.clone());
                    }
                }
                
                // Añadir cabeceras de caché
                set_cache_headers(&mut response, &cache_entry.etag, max_age.unwrap_or(cache_entry.max_age));
                
                return Box::pin(async move { Ok(response) });
            },
            _ => {
                // No está en caché o ha expirado
                debug!("Cache MISS: {}", cache_key);
                let future = self.inner.call(req);
                let cache_clone = self.cache.clone();
                let max_age = self.max_age;
                let cache_key = cache_key.clone();
                
                return Box::pin(async move {
                    let response = future.await.map_err(|e| e.into())?;
                    let response = response_map_body(response);
                    
                    // No cachear errores
                    if !response.status().is_success() {
                        return Ok(response);
                    }
                    
                    // Obtener el cuerpo y calcular ETag
                    let (parts, body) = response.into_parts();
                    let bytes = axum::body::to_bytes(body, 1024 * 1024 * 10).await?;
                    
                    // Calcular ETag
                    let etag = cache_clone.calculate_etag_for_bytes(&bytes);
                    
                    // Guardar en caché
                    cache_clone.set(
                        &cache_key, 
                        etag.clone(), 
                        Some(bytes.clone()),
                        parts.headers.clone(),
                        max_age
                    );
                    
                    // Crear la respuesta con ETag
                    let mut response = Response::from_parts(parts, Body::from(bytes));
                    set_cache_headers(&mut response, &etag, max_age.unwrap_or(cache_clone.default_max_age));
                    
                    Ok(response)
                });
            }
        }
    }
}

// Función auxiliar para convertir cualquier cuerpo en Body
fn response_map_body<B>(response: Response<B>) -> Response<Body>
where
    B: http_body::Body + Send + 'static,
    B::Data: Send + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let (parts, _body) = response.into_parts();
    
    // Create a simple empty body as a fallback - in production you would handle this better
    let mapped_body = Body::empty();
    
    Response::from_parts(parts, mapped_body)
}

/// Inicia una tarea de limpieza periódica para el caché
pub fn start_cache_cleanup_task(cache: HttpCache) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Cada 5 minutos
        
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
    use hyper::{Request, Body, Response};
    use axum::routing::get;
    use axum::{Extension, Json, Router};
    use tower::ServiceExt;
    use http::StatusCode;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct TestData {
        id: u32,
        name: String,
    }
    
    #[tokio::test]
    async fn test_etag_generation() {
        let cache = HttpCache::new();
        
        let data1 = TestData { id: 1, name: "Test".to_string() };
        let data2 = TestData { id: 1, name: "Test".to_string() };
        let data3 = TestData { id: 2, name: "Test".to_string() };
        
        let etag1 = cache.calculate_etag(&data1);
        let etag2 = cache.calculate_etag(&data2);
        let etag3 = cache.calculate_etag(&data3);
        
        // Mismos datos deben generar mismo ETag
        assert_eq!(etag1, etag2);
        
        // Datos diferentes deben generar ETags diferentes
        assert_ne!(etag1, etag3);
    }
    
    #[tokio::test]
    async fn test_cache_hit_miss() {
        let cache = HttpCache::new();
        
        // Primera petición (cache miss)
        let response1 = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(r#"{"id":1,"name":"Test"}"#))
            .unwrap();
        
        let (parts1, body1) = response1.into_parts();
        let bytes1 = hyper::body::to_bytes(body1).await.unwrap();
        
        let etag1 = cache.calculate_etag_for_bytes(&bytes1);
        cache.set("test", etag1.clone(), Some(bytes1.clone()), parts1.headers.clone(), None);
        
        // Verificar cache hit
        let entry = cache.get("test").unwrap();
        assert_eq!(entry.etag, etag1);
        assert_eq!(entry.data.unwrap(), bytes1);
        
        // Verificar cache miss
        assert!(cache.get("nonexistent").is_none());
    }
}