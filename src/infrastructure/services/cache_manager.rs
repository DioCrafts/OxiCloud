use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time;
use futures::future::BoxFuture;
use tokio::sync::RwLock;

/// Representación de metadatos en caché
#[derive(Debug, Clone)]
pub struct CachedMetadata {
    /// Si el archivo o directorio existe
    pub exists: bool,
    /// Tamaño en bytes (para archivos)
    pub size: Option<u64>,
    /// Timestamp de creación
    pub created_at: Option<u64>,
    /// Timestamp de modificación
    pub modified_at: Option<u64>,
    /// Tiempo de expiración de la caché
    expires_at: Instant,
}

/// Estructura para gestionar la caché de metadatos de archivos y directorios
pub struct StorageCacheManager {
    /// Caché de existencia y metadatos
    cache: RwLock<HashMap<PathBuf, CachedMetadata>>,
    /// TTL para entradas de archivos (milisegundos)
    file_ttl_ms: u64,
    /// TTL para entradas de directorios (milisegundos)
    dir_ttl_ms: u64,
    /// Tamaño máximo de caché
    max_entries: usize,
}

impl StorageCacheManager {
    /// Crea una nueva instancia del gestor de caché
    pub fn new(file_ttl_ms: u64, dir_ttl_ms: u64, max_entries: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::with_capacity(max_entries)),
            file_ttl_ms,
            dir_ttl_ms,
            max_entries,
        }
    }
    
    /// Crea una instancia por defecto del gestor de caché
    pub fn default() -> Self {
        Self::new(
            60_000,    // 1 minuto para archivos
            300_000,   // 5 minutos para directorios
            10_000,    // máximo 10,000 entradas
        )
    }
    
    /// Verifica si un archivo o directorio existe en caché
    pub async fn check_exists(&self, path: &PathBuf, _is_dir: bool) -> Result<bool, ()> {
        // Intentar obtener de la caché
        if let Some(metadata) = self.get_cached_metadata(path).await {
            return Ok(metadata.exists);
        }
        
        // No está en caché
        Err(())
    }
    
    /// Obtiene los metadatos de un path desde la caché
    async fn get_cached_metadata(&self, path: &PathBuf) -> Option<CachedMetadata> {
        let cache = self.cache.read().await;
        
        if let Some(metadata) = cache.get(path) {
            // Verificar si la entrada expiró
            if Instant::now() < metadata.expires_at {
                return Some(metadata.clone());
            }
        }
        
        None
    }
    
    /// Actualiza la caché con los metadatos de un path
    pub async fn update_cache(&self, path: &PathBuf, exists: bool, size: Option<u64>, 
                             created_at: Option<u64>, modified_at: Option<u64>, is_dir: bool) {
        let mut cache = self.cache.write().await;
        
        // Si la caché está llena, eliminar entradas aleatorias antes de agregar
        if cache.len() >= self.max_entries {
            self.evict_entries(&mut cache, 100).await;
        }
        
        // Determinar TTL basado en si es archivo o directorio
        let ttl = if is_dir {
            Duration::from_millis(self.dir_ttl_ms)
        } else {
            Duration::from_millis(self.file_ttl_ms)
        };
        
        // Crear metadatos y agregar a la caché
        let metadata = CachedMetadata {
            exists,
            size,
            created_at,
            modified_at,
            expires_at: Instant::now() + ttl,
        };
        
        cache.insert(path.clone(), metadata);
    }
    
    /// Elimina entradas aleatorias de la caché cuando está llena
    async fn evict_entries(&self, cache: &mut HashMap<PathBuf, CachedMetadata>, count: usize) {
        // Obtener las entradas más antiguas para eliminar
        let mut entries: Vec<_> = cache.keys().cloned().collect();
        
        // Limitar el número de entradas a eliminar
        let to_remove = count.min(entries.len() / 10);
        
        if to_remove == 0 {
            return;
        }
        
        // Eliminar las primeras entradas (implementación simple)
        entries.truncate(to_remove);
        
        for path in entries {
            cache.remove(&path);
        }
    }
    
    /// Inicia una tarea de limpieza periódica
    pub fn start_cleanup_task(cache_manager: Arc<Self>) -> BoxFuture<'static, ()> {
        Box::pin(async move {
            let interval = Duration::from_secs(60); // Ejecutar cada minuto
            
            loop {
                time::sleep(interval).await;
                
                // Limpiar entradas expiradas
                let now = Instant::now();
                let mut cache = cache_manager.cache.write().await;
                
                // Encontrar entradas expiradas
                let expired: Vec<_> = cache
                    .iter()
                    .filter(|(_, metadata)| now > metadata.expires_at)
                    .map(|(path, _)| path.clone())
                    .collect();
                
                // Eliminar entradas expiradas
                for path in expired {
                    cache.remove(&path);
                }
                
                // Registrar estadísticas
                let cache_size = cache.len();
                drop(cache);
                
                tracing::debug!("Cache cleanup completed. Entries remaining: {}", cache_size);
            }
        })
    }
    
    /// Invalida una entrada específica de la caché
    pub async fn invalidate(&self, path: &PathBuf) {
        let mut cache = self.cache.write().await;
        cache.remove(path);
    }
    
    /// Invalida todas las entradas de la caché relacionadas con una carpeta
    pub async fn invalidate_folder(&self, folder_path: &PathBuf) {
        let mut cache = self.cache.write().await;
        
        // Eliminar entradas que sean descendientes de la carpeta
        let folder_str = folder_path.to_string_lossy().to_string();
        
        // Encontrar entradas a eliminar
        let to_remove: Vec<_> = cache
            .keys()
            .filter_map(|path| {
                let path_str = path.to_string_lossy().to_string();
                if path_str.starts_with(&folder_str) {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect();
        
        // Eliminar las entradas
        for path in to_remove {
            cache.remove(&path);
        }
    }
    
    /// Obtiene el número actual de entradas en la caché
    pub async fn cache_size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }
}