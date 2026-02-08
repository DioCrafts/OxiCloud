use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, UNIX_EPOCH};
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time;
use futures::future::BoxFuture;
use tracing::debug;
use mime_guess::from_path;

use crate::domain::entities::file::File;

use crate::common::config::AppConfig;

/// Tipos de entradas en caché
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEntryType {
    /// Archivo
    File,
    /// Directorio
    Directory,
    /// Tipo desconocido
    Unknown,
}

/// Estadísticas de caché para monitoreo
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Número de hits en caché
    pub hits: usize,
    /// Número de misses en caché
    pub misses: usize,
    /// Número de invalidaciones manuales
    pub invalidations: usize,
    /// Número de expiraciones automáticas
    pub expirations: usize,
    /// Número de inserciones en caché
    pub inserts: usize,
    /// Tiempo total ahorrado (milisegundos)
    pub time_saved_ms: u64,
}

/// Metadatos completos de archivo en caché
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// Ruta absoluta del archivo
    pub path: PathBuf,
    /// Si el archivo existe físicamente
    pub exists: bool,
    /// Tipo de entrada (archivo, directorio)
    pub entry_type: CacheEntryType,
    /// Tamaño en bytes (para archivos)
    pub size: Option<u64>,
    /// Tipo MIME (para archivos)
    pub mime_type: Option<String>,
    /// Timestamp de creación (UNIX epoch seconds)
    pub created_at: Option<u64>,
    /// Timestamp de modificación (UNIX epoch seconds)
    pub modified_at: Option<u64>,
    /// Acceso previo (usado para LRU)
    pub last_access: Instant,
    /// Tiempo de expiración de la caché
    pub expires_at: Instant,
    /// Número de accesos a esta entrada
    pub access_count: usize,
}

impl FileMetadata {
    /// Crea una nueva entrada de metadatos
    pub fn new(
        path: PathBuf,
        exists: bool,
        entry_type: CacheEntryType,
        size: Option<u64>,
        mime_type: Option<String>,
        created_at: Option<u64>,
        modified_at: Option<u64>,
        ttl: Duration,
    ) -> Self {
        let now = Instant::now();
        
        Self {
            path,
            exists,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            last_access: now,
            expires_at: now + ttl,
            access_count: 1,
        }
    }
    
    /// Actualiza el tiempo de último acceso
    pub fn touch(&mut self) {
        self.last_access = Instant::now();
        self.access_count += 1;
    }
    
    /// Verifica si la entrada ha expirado
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
    
    /// Actualiza el tiempo de expiración con un nuevo TTL
    pub fn update_expiry(&mut self, ttl: Duration) {
        self.expires_at = Instant::now() + ttl;
    }
}

/// Caché avanzada de metadatos de archivos
pub struct FileMetadataCache {
    /// Caché principal de metadatos
    metadata_cache: RwLock<HashMap<PathBuf, FileMetadata>>,
    /// Cola LRU para administración de caché
    lru_queue: RwLock<VecDeque<PathBuf>>,
    /// Estadísticas de uso del caché
    stats: RwLock<CacheStats>,
    /// Configuración global de la aplicación
    config: AppConfig,
    /// TTL adaptativo para entradas populares
    ttl_multiplier: f64,
    /// Umbral de popularidad para TTL extendido
    popularity_threshold: usize,
    /// Tamaño máximo de caché
    max_entries: usize,
}

impl FileMetadataCache {
    /// Crea una nueva instancia de caché de metadatos
    pub fn new(config: AppConfig, max_entries: usize) -> Self {
        Self {
            metadata_cache: RwLock::new(HashMap::with_capacity(max_entries)),
            lru_queue: RwLock::new(VecDeque::with_capacity(max_entries)),
            stats: RwLock::new(CacheStats::default()),
            config,
            ttl_multiplier: 5.0, // Entradas populares tienen 5x TTL
            popularity_threshold: 10, // Después de 10 accesos se considera popular
            max_entries,
        }
    }
    
    /// Crea un objeto FileMetadata a partir de un objeto File
    pub fn create_metadata_from_file(file: &File, abs_path: PathBuf) -> FileMetadata {
        let entry_type = CacheEntryType::File;
        let size = Some(file.size());
        let mime_type = Some(file.mime_type().to_string());
        let created_at = Some(file.created_at());
        let modified_at = Some(file.modified_at());
        
        // Usar un TTL estándar
        let ttl = Duration::from_secs(60); // 1 minuto
        
        FileMetadata::new(
            abs_path,
            true,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            ttl,
        )
    }
    
    /// Crea una instancia por defecto
    pub fn default() -> Self {
        Self::new(AppConfig::default(), 10_000)
    }
    
    /// Crea una instancia de caché con configuración por defecto
    pub fn default_with_config(config: AppConfig) -> Self {
        Self::new(config, 50_000) // Caché más grande para sistema en producción
    }
    
    /// Obtiene los metadatos de un archivo si están en caché
    pub async fn get_metadata(&self, path: &Path) -> Option<FileMetadata> {
        let start_time = Instant::now();
        let mut cache = self.metadata_cache.write().await;
        
        if let Some(metadata) = cache.get_mut(path) {
            // Verificar si ha expirado
            if metadata.is_expired() {
                // Eliminar de caché si expiró
                cache.remove(path);
                
                // Actualizar estadísticas
                let mut stats = self.stats.write().await;
                stats.misses += 1;
                stats.expirations += 1;
                
                debug!("Cache entry expired for: {}", path.display());
                
                return None;
            }
            
            // Actualizar tiempo de acceso
            metadata.touch();
            
            // Para entradas populares, extender TTL
            if metadata.access_count >= self.popularity_threshold {
                let new_ttl = match metadata.entry_type {
                    CacheEntryType::File => Duration::from_millis(
                        (self.config.timeouts.file_operation_ms as f64 * self.ttl_multiplier) as u64
                    ),
                    CacheEntryType::Directory => Duration::from_millis(
                        (self.config.timeouts.dir_operation_ms as f64 * self.ttl_multiplier) as u64
                    ),
                    _ => Duration::from_secs(60), // 1 minuto por defecto
                };
                
                metadata.update_expiry(new_ttl);
                debug!("Extended TTL for popular entry: {}", path.display());
            }
            
            // Calcular tiempo ahorrado aproximado
            let elapsed = start_time.elapsed().as_millis() as u64;
            let estimated_io_time: u64 = 10; // Asumimos 10ms mínimo para operación de IO
            let time_saved = estimated_io_time.saturating_sub(elapsed);
            
            // Actualizar estadísticas
            let mut stats = self.stats.write().await;
            stats.hits += 1;
            stats.time_saved_ms += time_saved;
            
            debug!("Cache hit for: {}", path.display());
            
            // Mantener también la cola LRU actualizada
            self.update_lru(path.to_path_buf()).await;
            
            // Clonar para retornar
            return Some(metadata.clone());
        }
        
        // No encontrado en caché
        let mut stats = self.stats.write().await;
        stats.misses += 1;
        
        debug!("Cache miss for: {}", path.display());
        None
    }
    
    /// Actualiza la cola LRU
    async fn update_lru(&self, path: PathBuf) {
        let mut lru = self.lru_queue.write().await;
        
        // Eliminar si ya existe
        if let Some(pos) = lru.iter().position(|p| p == &path) {
            lru.remove(pos);
        }
        
        // Agregar al final (más reciente)
        lru.push_back(path);
    }
    
    /// Verifica si un archivo existe
    pub async fn exists(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.exists);
        }
        
        None
    }
    
    /// Verifica si un path es un directorio
    pub async fn is_dir(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.entry_type == CacheEntryType::Directory);
        }
        
        None
    }
    
    /// Verifica si un path es un archivo
    pub async fn is_file(&self, path: &Path) -> Option<bool> {
        if let Some(metadata) = self.get_metadata(path).await {
            return Some(metadata.entry_type == CacheEntryType::File);
        }
        
        None
    }
    
    /// Obtiene el tamaño de un archivo
    pub async fn get_size(&self, path: &Path) -> Option<u64> {
        if let Some(metadata) = self.get_metadata(path).await {
            return metadata.size;
        }
        
        None
    }
    
    /// Obtiene el tipo MIME de un archivo
    pub async fn get_mime_type(&self, path: &Path) -> Option<String> {
        if let Some(metadata) = self.get_metadata(path).await {
            return metadata.mime_type;
        }
        
        None
    }
    
    /// Refresca los metadatos de un path
    pub async fn refresh_metadata(&self, path: &Path) -> Result<FileMetadata, std::io::Error> {
        // Realizar lectura real del sistema de archivos
        let metadata = fs::metadata(path).await?;
        
        // Determinar tipo de entrada
        let entry_type = if metadata.is_dir() {
            CacheEntryType::Directory
        } else if metadata.is_file() {
            CacheEntryType::File
        } else {
            CacheEntryType::Unknown
        };
        
        // Obtener tamaño para archivos
        let size = if metadata.is_file() {
            Some(metadata.len())
        } else {
            None
        };
        
        // Obtener tipo MIME para archivos
        let mime_type = if metadata.is_file() {
            Some(from_path(path).first_or_octet_stream().to_string())
        } else {
            None
        };
        
        // Obtener timestamps
        let created_at = metadata.created()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .ok();
            
        let modified_at = metadata.modified()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .ok();
        
        // Determinar TTL apropiado
        let ttl = if metadata.is_dir() {
            Duration::from_millis(self.config.timeouts.dir_operation_ms)
        } else {
            Duration::from_millis(self.config.timeouts.file_operation_ms)
        };
        
        // Crear entrada de metadatos
        let file_metadata = FileMetadata::new(
            path.to_path_buf(),
            true,
            entry_type,
            size,
            mime_type,
            created_at,
            modified_at,
            ttl,
        );
        
        // Actualizar caché
        self.update_cache(file_metadata.clone()).await;
        
        Ok(file_metadata)
    }
    
    /// Actualiza la caché con nuevos metadatos
    pub async fn update_cache(&self, metadata: FileMetadata) {
        // Evitar caché llena antes de insertar
        self.ensure_capacity().await;
        
        let path = metadata.path.clone();
        
        // Insertar en caché
        {
            let mut cache = self.metadata_cache.write().await;
            cache.insert(path.clone(), metadata);
            
            // Actualizar estadísticas
            let mut stats = self.stats.write().await;
            stats.inserts += 1;
        }
        
        // Actualizar la cola LRU
        self.update_lru(path).await;
    }
    
    /// Asegura que hay espacio en la caché
    async fn ensure_capacity(&self) {
        let cache_size = {
            let cache = self.metadata_cache.read().await;
            cache.len()
        };
        
        if cache_size >= self.max_entries {
            self.evict_lru_entries(cache_size / 10).await; // Liberar 10%
        }
    }
    
    /// Elimina entradas menos recientemente usadas
    async fn evict_lru_entries(&self, count: usize) {
        let mut paths_to_remove = Vec::with_capacity(count);
        
        // Obtener entries a eliminar de la cola LRU
        {
            let mut lru = self.lru_queue.write().await;
            for _ in 0..count {
                if let Some(path) = lru.pop_front() {
                    paths_to_remove.push(path);
                } else {
                    break;
                }
            }
        }
        
        // Eliminar de la caché principal
        {
            let mut cache = self.metadata_cache.write().await;
            for path in paths_to_remove {
                cache.remove(&path);
            }
        }
        
        debug!("Evicted {} LRU entries from cache", count);
    }
    
    /// Invalidar una entrada específica de caché
    pub async fn invalidate(&self, path: &Path) {
        // Eliminar de la caché principal
        {
            let mut cache = self.metadata_cache.write().await;
            cache.remove(path);
            
            // Actualizar estadísticas
            let mut stats = self.stats.write().await;
            stats.invalidations += 1;
        }
        
        // Eliminar de la cola LRU
        let path_buf = path.to_path_buf();
        {
            let mut lru = self.lru_queue.write().await;
            if let Some(pos) = lru.iter().position(|p| p == &path_buf) {
                lru.remove(pos);
            }
        }
        
        debug!("Invalidated cache entry for: {}", path.display());
    }
    
    /// Invalidar recursivamente entradas bajo un directorio
    pub async fn invalidate_directory(&self, dir_path: &Path) {
        let dir_str = dir_path.to_string_lossy().to_string();
        let mut paths_to_remove = Vec::new();
        
        // Encontrar todos los paths que comienzan con el directorio
        {
            let cache = self.metadata_cache.read().await;
            for path in cache.keys() {
                let path_str = path.to_string_lossy().to_string();
                if path_str.starts_with(&dir_str) {
                    paths_to_remove.push(path.clone());
                }
            }
        }
        
        // Actualizar estadísticas
        {
            let mut stats = self.stats.write().await;
            stats.invalidations += paths_to_remove.len();
        }
        
        // Eliminar cada path encontrado
        for path in paths_to_remove {
            self.invalidate(&path).await;
        }
        
        debug!("Invalidated directory and contents: {}", dir_path.display());
    }
    
    /// Obtener estadísticas actuales de la caché
    pub async fn get_stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        stats.clone()
    }
    
    /// Limpia todas las entradas expiradas de la caché
    pub async fn clear_expired(&self) {
        let now = Instant::now();
        let mut paths_to_remove = Vec::new();
        
        // Encontrar entradas expiradas
        {
            let cache = self.metadata_cache.read().await;
            for (path, metadata) in cache.iter() {
                if now > metadata.expires_at {
                    paths_to_remove.push(path.clone());
                }
            }
        }
        
        // Actualizar estadísticas
        {
            let mut stats = self.stats.write().await;
            stats.expirations += paths_to_remove.len();
        }
        
        // Guardar la cantidad de entradas para el logging
        let num_paths = paths_to_remove.len();
        
        // Eliminar entradas expiradas
        for path in paths_to_remove {
            self.invalidate(&path).await;
        }
        
        debug!("Cleared {} expired entries from cache", num_paths);
    }
    
    /// Inicia el proceso de limpieza periódica
    pub fn start_cleanup_task(cache: Arc<Self>) -> BoxFuture<'static, ()> {
        Box::pin(async move {
            let cleanup_interval = Duration::from_secs(60); // Cada minuto
            
            loop {
                // Esperar el intervalo
                time::sleep(cleanup_interval).await;
                
                // Limpiar entradas expiradas
                cache.clear_expired().await;
                
                // Registrar estadísticas
                let stats = cache.get_stats().await;
                let cache_size = {
                    let cache_map = cache.metadata_cache.read().await;
                    cache_map.len()
                };
                
                debug!(
                    "Cache stats: size={}, hits={}, misses={}, hit_ratio={:.2}%, time_saved={}ms",
                    cache_size,
                    stats.hits,
                    stats.misses,
                    if stats.hits + stats.misses > 0 { 
                        (stats.hits as f64 * 100.0) / (stats.hits + stats.misses) as f64
                    } else { 
                        0.0 
                    },
                    stats.time_saved_ms
                );
            }
        })
    }
    
    /// Precarga metadatos de directorios completos (útil para inicialización)
    pub async fn preload_directory(&self, dir_path: &Path, recursive: bool, max_depth: usize) -> Result<usize, std::io::Error> {
        self._preload_directory_internal(dir_path, recursive, max_depth, 0).await
    }
    
    /// Implementación interna de precarga con seguimiento de profundidad
    async fn _preload_directory_internal(
        &self, 
        dir_path: &Path, 
        recursive: bool, 
        max_depth: usize, 
        current_depth: usize
    ) -> Result<usize, std::io::Error> {
        Box::pin(async move {
        if current_depth > max_depth {
            return Ok(0);
        }
        
        // Obtener entradas del directorio
        let mut entries = fs::read_dir(dir_path).await?;
        let mut count = 0;
        
        // Procesar cada entrada
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = fs::metadata(&path).await?;
            
            // Refrescar metadatos de esta entrada
            self.refresh_metadata(&path).await?;
            count += 1;
            
            // Recursivamente procesar subdirectorios si es necesario
            if recursive && metadata.is_dir() {
                // Box to break recursion
                count += self._preload_directory_internal(
                    &path, 
                    recursive, 
                    max_depth, 
                    current_depth + 1
                ).await?;
            }
        }
        
        Ok(count)
    }).await
    }
}

// ─── MetadataCachePort implementation ────────────────────────

use async_trait::async_trait;
use crate::application::ports::cache_ports::{MetadataCachePort, CachedMetadataDto};
use crate::common::errors::DomainError;

#[async_trait]
impl MetadataCachePort for FileMetadataCache {
    async fn get_metadata(&self, path: &Path) -> Option<CachedMetadataDto> {
        // Delegate to the existing rich get_metadata, then project into the DTO.
        let fm = FileMetadataCache::get_metadata(self, path).await?;
        Some(CachedMetadataDto {
            path: fm.path,
            exists: fm.exists,
            is_file: fm.entry_type == CacheEntryType::File,
            size: fm.size,
            mime_type: fm.mime_type,
            created_at: fm.created_at,
            modified_at: fm.modified_at,
        })
    }

    async fn is_file(&self, path: &Path) -> Option<bool> {
        FileMetadataCache::is_file(self, path).await
    }

    async fn refresh_metadata(&self, path: &Path) -> Result<CachedMetadataDto, DomainError> {
        let fm = FileMetadataCache::refresh_metadata(self, path)
            .await
            .map_err(|e| DomainError::internal_error("MetadataCache", e.to_string()))?;
        Ok(CachedMetadataDto {
            path: fm.path,
            exists: fm.exists,
            is_file: fm.entry_type == CacheEntryType::File,
            size: fm.size,
            mime_type: fm.mime_type,
            created_at: fm.created_at,
            modified_at: fm.modified_at,
        })
    }

    async fn invalidate(&self, path: &Path) {
        FileMetadataCache::invalidate(self, path).await
    }

    async fn invalidate_directory(&self, dir_path: &Path) {
        FileMetadataCache::invalidate_directory(self, dir_path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    
    #[tokio::test]
    async fn test_cache_operations() {
        // Crear directorio temporal para pruebas
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        
        // Crear un archivo de prueba
        let mut file = File::create(&file_path).await.unwrap();
        file.write_all(b"test content").await.unwrap();
        file.flush().await.unwrap();
        drop(file);
        
        // Crear caché
        let config = AppConfig::default();
        let cache = FileMetadataCache::new(config, 1000);
        
        // Verificar miss inicial
        assert!(cache.exists(&file_path).await.is_none());
        
        // Refrescar y verificar hit
        let metadata = cache.refresh_metadata(&file_path).await.unwrap();
        assert_eq!(metadata.entry_type, CacheEntryType::File);
        assert_eq!(metadata.size, Some(12)); // "test content" = 12 bytes
        
        // Verificar que ahora existe en caché
        assert_eq!(cache.exists(&file_path).await, Some(true));
        assert_eq!(cache.is_file(&file_path).await, Some(true));
        
        // Invalidar y verificar que ya no existe en caché
        cache.invalidate(&file_path).await;
        assert!(cache.exists(&file_path).await.is_none());
        
        // Verificar estadísticas
        let stats = cache.get_stats().await;
        assert_eq!(stats.inserts, 1);
        assert_eq!(stats.invalidations, 1);
        assert!(stats.hits > 0);
    }
    
    #[tokio::test]
    async fn test_directory_operations() {
        // Crear estructura de directorios para pruebas
        let temp_dir = tempdir().unwrap();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let base_path = temp_dir.path().canonicalize().unwrap();
        let sub_dir = base_path.join("subdir");
        fs::create_dir(&sub_dir).await.unwrap();
        
        let file1 = base_path.join("file1.txt");
        let file2 = sub_dir.join("file2.txt");
        
        File::create(&file1).await.unwrap();
        File::create(&file2).await.unwrap();
        
        // Crear caché
        let config = AppConfig::default();
        let cache = FileMetadataCache::new(config, 1000);
        
        // Precargar directorio recursivamente
        // preload_directory caches the *contents* of the directory, not the root itself
        let count = cache.preload_directory(&base_path, true, 2).await.unwrap();
        assert_eq!(count, 3); // subdir, file1, file2
        
        // Verificar existencia en caché (solo contenido, no la raíz)
        assert_eq!(cache.is_dir(&sub_dir).await, Some(true));
        assert_eq!(cache.is_file(&file1).await, Some(true));
        assert_eq!(cache.is_file(&file2).await, Some(true));
        
        // Invalidar directorio y contenido
        cache.invalidate_directory(&base_path).await;
        
        // Verificar que nada existe en caché
        assert!(cache.exists(&sub_dir).await.is_none());
        assert!(cache.exists(&file1).await.is_none());
        assert!(cache.exists(&file2).await.is_none());
    }
}