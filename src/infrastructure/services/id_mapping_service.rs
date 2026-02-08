use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use tokio::fs;
use tokio::time;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::domain::services::path_service::StoragePath;
use crate::common::errors::{DomainError, ErrorKind};
use crate::application::ports::outbound::IdMappingPort;
use crate::common::config::TimeoutConfig;

/// Error específico para el servicio de mapeo de IDs
#[derive(Debug, thiserror::Error)]
pub enum IdMappingError {
    #[error("ID not found: {0}")]
    NotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

// Implementar conversión de IdMappingError a DomainError
impl From<IdMappingError> for DomainError {
    fn from(err: IdMappingError) -> Self {
        match err {
            IdMappingError::NotFound(id) => DomainError::not_found("IdMapping", id),
            IdMappingError::IoError(e) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("IO error: {}", e)
            ).with_source(e),
            IdMappingError::Timeout(msg) => DomainError::timeout(
                "IdMapping",
                format!("Timeout: {}", msg)
            ),
            IdMappingError::SerializationError(e) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("Serialization error: {}", e)
            ).with_source(e),
            IdMappingError::Other(msg) => DomainError::new(
                ErrorKind::InternalError,
                "IdMapping",
                format!("Other error: {}", msg)
            ),
        }
    }
}

/// Estructura para almacenar IDs mapeados a sus rutas
#[derive(Serialize, Deserialize, Debug, Default)]
struct IdMap {
    path_to_id: HashMap<String, String>,
    id_to_path: HashMap<String, String>, // Campo para búsqueda bidireccional eficiente
    version: u32, // Versión para detectar cambios
}

/// Servicio para gestionar mapeos entre rutas y IDs únicos
pub struct IdMappingService {
    map_path: PathBuf,
    id_map: RwLock<IdMap>,
    save_mutex: Mutex<()>, // Para evitar múltiples guardados concurrentes
    timeouts: TimeoutConfig,
    pending_save: RwLock<bool>, // Indica si hay cambios pendientes
}

impl IdMappingService {
    /// Crea un nuevo servicio de mapeo de IDs
    pub async fn new(map_path: PathBuf) -> Result<Self, DomainError> {
        let timeouts = TimeoutConfig::default();
        let id_map = Self::load_id_map(&map_path, &timeouts).await?;
        
        Ok(Self {
            map_path,
            id_map: RwLock::new(id_map),
            save_mutex: Mutex::new(()),
            timeouts,
            pending_save: RwLock::new(false),
        })
    }
    
    /// Crea un servicio de mapeo de IDs en memoria (para pruebas)
    ///
    /// Similar functionality as new_in_memory but with a simpler signature for dummy use
    pub fn dummy() -> Self {
        Self {
            map_path: PathBuf::from("/tmp/dummy_id_map.json"),
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
    
    /// Crea un servicio de mapeo de IDs en memoria (para pruebas - versión original)
    pub fn new_in_memory() -> Self {
        Self {
            map_path: PathBuf::from("memory"),
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
    
    /// Carga el mapa de IDs desde disco con manejo robusto de errores
    async fn load_id_map(map_path: &PathBuf, timeouts: &TimeoutConfig) -> Result<IdMap, DomainError> {
        if map_path.exists() {
            // Intentar leer con timeout para evitar bloqueos indefinidos
            let read_result = time::timeout(
                timeouts.lock_timeout(),
                fs::read_to_string(map_path)
            ).await
            .map_err(|_| DomainError::timeout("IdMapping", format!("Timeout reading ID map from {}", map_path.display())))?;
            
            let content = read_result.map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to read ID map from {}: {}", map_path.display(), e)))?;
            
            // Parsear el JSON
            match serde_json::from_str::<IdMap>(&content) {
                Ok(mut map) => {
                    // Reconstruir el mapa inverso si es necesario
                    if map.id_to_path.is_empty() && !map.path_to_id.is_empty() {
                        let mut rebuild_count = 0;
                        for (path, id) in &map.path_to_id {
                            map.id_to_path.insert(id.clone(), path.clone());
                            rebuild_count += 1;
                        }
                        tracing::info!("Rebuilt inverse mapping with {} entries", rebuild_count);
                    }
                    
                    tracing::info!("Loaded ID map with {} entries (version: {})", 
                                   map.path_to_id.len(), map.version);
                    return Ok(map);
                },
                Err(e) => {
                    tracing::error!("Error parsing ID map: {}", e);
                    // Intentar hacer un respaldo del archivo corrupto
                    let backup_path = map_path.with_extension("json.bak");
                    if let Err(copy_err) = tokio::fs::copy(map_path, &backup_path).await {
                        tracing::error!("Failed to backup corrupted map file: {}", copy_err);
                    } else {
                        tracing::info!("Backed up corrupted ID map to {}", backup_path.display());
                    }
                    
                    tracing::info!("Creating new empty map after error");
                    return Ok(IdMap {
                        path_to_id: HashMap::new(),
                        id_to_path: HashMap::new(),
                        version: 1, // Iniciar con versión 1
                    });
                }
            }
        }
        
        // Devolver un mapa vacío si el archivo no existe y crear el archivo
        tracing::info!("No existing ID map found, creating new empty map");
        let empty_map = IdMap {
            path_to_id: HashMap::new(),
            id_to_path: HashMap::new(),
            version: 1, // Iniciar con versión 1
        };
        
        // Ensure directory exists
        if let Some(parent) = map_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    tracing::error!("Failed to create directory for ID map: {}", e);
                }
            }
        }
        
        // Write empty map to file
        match serde_json::to_string_pretty(&empty_map) {
            Ok(json) => {
                if let Err(e) = fs::write(map_path, json).await {
                    tracing::error!("Failed to write initial empty ID map: {}", e);
                } else {
                    tracing::info!("Created initial empty ID map at {}", map_path.display());
                }
            },
            Err(e) => {
                tracing::error!("Failed to serialize empty ID map: {}", e);
            }
        }
        
        Ok(empty_map)
    }
    
    /// Guarda el mapa de IDs en disco de manera segura
    async fn save_id_map(&self) -> Result<(), DomainError> {
        // Adquirir bloqueo exclusivo para salvar
        let _lock = time::timeout(
            self.timeouts.lock_timeout(),
            self.save_mutex.lock()
        ).await
        .map_err(|_| DomainError::timeout("IdMapping", "Timeout acquiring save lock for ID mapping"))?;
        
        // Crear JSON con el lock de lectura para minimizar el tiempo de bloqueo
        let json = {
            let mut map = time::timeout(
                self.timeouts.lock_timeout(),
                self.id_map.write()
            ).await
            .map_err(|_| DomainError::timeout("IdMapping", "Timeout acquiring write lock for ID mapping"))?;
            
            // Incrementar versión sólo si hay cambios por guardar
            let pending = *self.pending_save.read().await;
            if pending {
                map.version += 1;
                tracing::debug!("Incrementing ID map version to {}", map.version);
            }
            
            // Use serde with reasonably safe defaults
            serde_json::to_string_pretty(&*map)
                .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to serialize ID map to JSON: {}", e)))?
        };
        
        // Escribir a un archivo temporal primero para evitar corrupción
        let temp_path = self.map_path.with_extension("json.tmp");
        fs::write(&temp_path, &json).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to write temporary ID map to {}: {}", temp_path.display(), e)))?;
        
        // Realizar el rename atómico
        fs::rename(&temp_path, &self.map_path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to rename temporary ID map to {}: {}", self.map_path.display(), e)))?;
        
        // Resetear flag de pendientes
        {
            let mut pending = self.pending_save.write().await;
            *pending = false;
        }
        
        tracing::info!("Saved ID map successfully to {}", self.map_path.display());
        Ok(())
    }
    
    /// Genera un ID único
    fn generate_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Marca cambios como pendientes
    async fn mark_pending(&self) {
        let mut pending = self.pending_save.write().await;
        *pending = true;
    }
    
    /// Obtiene el ID para una ruta o genera uno nuevo si no existe
    pub async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, IdMappingError> {
        let path_str = path.to_string();
        
        // Primer intento con lock de lectura (más eficiente)
        {
            let read_result = match time::timeout(
                self.timeouts.lock_timeout(), 
                self.id_map.read()
            ).await {
                Ok(guard) => guard,
                Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring read lock for ID mapping".to_string())),
            };
            
            if let Some(id) = read_result.path_to_id.get(&path_str) {
                return Ok(id.clone());
            }
        }
        
        // Si no se encuentra, adquirir lock de escritura
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID mapping".to_string())),
        };
        
        let mut map = write_result;
        
        // Verificar nuevamente (podría haberse agregado mientras esperábamos el lock)
        if let Some(id) = map.path_to_id.get(&path_str) {
            return Ok(id.clone());
        }
        
        // Generar un nuevo ID y almacenarlo
        let id = self.generate_id();
        map.path_to_id.insert(path_str.clone(), id.clone());
        map.id_to_path.insert(id.clone(), path_str);
        
        // Marcar como pendiente para guardar
        drop(map); // Liberar el write lock antes de adquirir otro
        self.mark_pending().await;
        
        tracing::debug!("Created new ID mapping: {} -> {}", path.to_string(), id);
        
        Ok(id)
    }
    
    /// Obtiene una ruta por su ID con manejo de timeout
    pub async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, IdMappingError> {
        let read_result = match time::timeout(
            self.timeouts.lock_timeout(), 
            self.id_map.read()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring read lock for ID lookup".to_string())),
        };
        
        if let Some(path_str) = read_result.id_to_path.get(id) {
            return Ok(StoragePath::from_string(path_str));
        }
        
        Err(IdMappingError::NotFound(id.to_string()))
    }
    
    /// Actualiza el mapeo de un ID existente a una nueva ruta
    pub async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), IdMappingError> {
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID update".to_string())),
        };
        
        let mut map = write_result;
        
        // Buscar la ruta anterior para eliminarla
        if let Some(old_path) = map.id_to_path.get(id).cloned() {
            map.path_to_id.remove(&old_path);
            
            // Registrar la nueva ruta
            let new_path_str = new_path.to_string();
            map.path_to_id.insert(new_path_str.clone(), id.to_string());
            map.id_to_path.insert(id.to_string(), new_path_str);
            
            // Marcar como pendiente
            drop(map); // Liberar el write lock antes de adquirir otro
            self.mark_pending().await;
            
            tracing::debug!("Updated path mapping for ID {}: {} -> {}", 
                id, old_path, new_path.to_string());
            
            Ok(())
        } else {
            Err(IdMappingError::NotFound(id.to_string()))
        }
    }
    
    /// Elimina un ID del mapa
    pub async fn remove_id(&self, id: &str) -> Result<(), IdMappingError> {
        let write_result = match time::timeout(
            self.timeouts.lock_timeout(),
            self.id_map.write()
        ).await {
            Ok(guard) => guard,
            Err(_) => return Err(IdMappingError::Timeout("Timeout acquiring write lock for ID removal".to_string())),
        };
        
        let mut map = write_result;
        
        // Buscar la ruta para eliminarla
        if let Some(path) = map.id_to_path.remove(id) {
            map.path_to_id.remove(&path);
            
            // Marcar como pendiente
            drop(map); // Liberar el write lock antes de adquirir otro
            self.mark_pending().await;
            
            tracing::debug!("Removed ID mapping: {} -> {}", id, path);
            Ok(())
        } else {
            Err(IdMappingError::NotFound(id.to_string()))
        }
    }
    
    /// Guarda cambios pendientes al disco inmediatamente, sin debounce
    pub async fn save_pending_changes(&self) -> Result<(), IdMappingError> {
        // Verificar si hay cambios pendientes
        {
            let pending = self.pending_save.read().await;
            if !*pending {
                return Ok(());
            }
        }
        
        // Guardar inmediatamente (sin debounce ni spawn)
        match self.save_id_map().await {
            Ok(_) => {
                tracing::info!("ID mappings saved successfully to disk at {}", self.map_path.display());
                
                // Verificar explícitamente que el archivo existe y tiene tamaño
                match std::fs::metadata(&self.map_path) {
                    Ok(metadata) => {
                        if metadata.len() > 0 {
                            tracing::info!("Verified saved map file exists with size: {} bytes", metadata.len());
                        } else {
                            tracing::warn!("Map file exists but has zero size - this might cause issues");
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to verify saved map file: {}", e);
                        // Intentar un segundo guardado si la verificación falla
                        if let Err(retry_err) = self.save_id_map().await {
                            tracing::error!("Second save attempt also failed: {}", retry_err);
                            return Err(IdMappingError::IoError(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Failed to verify and retry save: {}", retry_err)
                            )));
                        }
                        tracing::info!("Second save attempt succeeded");
                    }
                }
                
                Ok(())
            },
            Err(e) => {
                tracing::error!("Failed to save ID map to {}: {}", self.map_path.display(), e);
                // Intentar un segundo guardado con retraso en caso de error
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                match self.save_id_map().await {
                    Ok(_) => {
                        tracing::info!("Second save attempt succeeded after initial failure");
                        Ok(())
                    },
                    Err(retry_e) => {
                        tracing::error!("Second save attempt also failed: {}", retry_e);
                        Err(IdMappingError::IoError(std::io::Error::new(
                            std::io::ErrorKind::Other, 
                            format!("Failed to save ID mappings after retry: {}", retry_e)
                        )))
                    }
                }
            }
        }
    }
}

#[async_trait]
impl IdMappingPort for IdMappingService {
    /// Obtiene el ID para una ruta o genera uno nuevo si no existe
    async fn get_or_create_id(&self, path: &StoragePath) -> Result<String, DomainError> {
        self.get_or_create_id(path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to get or create ID for path: {}: {}", path.to_string(), e)))
    }
    
    /// Obtiene una ruta por su ID con manejo de timeout
    async fn get_path_by_id(&self, id: &str) -> Result<StoragePath, DomainError> {
        self.get_path_by_id(id).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to get path for ID: {}: {}", id, e)))
    }
    
    /// Actualiza el mapeo de un ID existente a una nueva ruta
    async fn update_path(&self, id: &str, new_path: &StoragePath) -> Result<(), DomainError> {
        self.update_path(id, new_path).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to update path for ID: {} to {}: {}", id, new_path.to_string(), e)))
    }
    
    /// Elimina un ID del mapa
    async fn remove_id(&self, id: &str) -> Result<(), DomainError> {
        self.remove_id(id).await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to remove ID: {}: {}", id, e)))
    }
    
    /// Guarda cambios pendientes al disco
    async fn save_changes(&self) -> Result<(), DomainError> {
        self.save_pending_changes().await
            .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to save pending ID mapping changes: {}", e)))
    }
}

// The extension methods were moved to the IdMappingPort trait as default implementations

// Implementar Clone para poder usar en tokio::spawn
/// Synchronous helper for contexts where we can't use async
impl IdMappingService {
    /// Create a new service synchronously (only for stubs and initialization)
    pub fn new_sync(map_path: PathBuf) -> Self {
        // Create a minimal implementation for initialization purposes
        Self {
            map_path,
            id_map: RwLock::new(IdMap::default()),
            save_mutex: Mutex::new(()),
            timeouts: TimeoutConfig::default(),
            pending_save: RwLock::new(false),
        }
    }
}

impl Clone for IdMappingService {
    fn clone(&self) -> Self {
        // No podemos clonar directamente los RwLock/Mutex,
        // pero podemos crear nuevas instancias que apunten al mismo Arc interno
        // Sin embargo, en este caso simplemente necesitamos la map_path
        Self {
            map_path: self.map_path.clone(),
            id_map: RwLock::new(IdMap::default()), // Esto no se usa en el task asíncrono
            save_mutex: Mutex::new(()),           // Esto tampoco
            timeouts: self.timeouts.clone(),
            pending_save: RwLock::new(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_get_or_create_id() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = IdMappingService::new(map_path).await.unwrap();
        
        let path = StoragePath::from_string("/test/file.txt");
        let id = service.get_or_create_id(&path).await.unwrap();
        
        assert!(!id.is_empty(), "ID should not be empty");
        
        // Verificar que el mismo ID se devuelve para la misma ruta
        let id2 = service.get_or_create_id(&path).await.unwrap();
        assert_eq!(id, id2, "Same path should return same ID");
    }
    
    #[tokio::test]
    async fn test_update_path() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = IdMappingService::new(map_path).await.unwrap();
        
        let old_path = StoragePath::from_string("/test/old.txt");
        let id = service.get_or_create_id(&old_path).await.unwrap();
        
        let new_path = StoragePath::from_string("/test/new.txt");
        service.update_path(&id, &new_path).await.unwrap();
        
        let retrieved_path = service.get_path_by_id(&id).await.unwrap();
        assert_eq!(retrieved_path, new_path, "Path should be updated");
    }
    
    #[tokio::test]
    async fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        // Crear y poblar el servicio
        let service = IdMappingService::new(map_path.clone()).await.unwrap();
        
        let path1 = StoragePath::from_string("/test/file1.txt");
        let path2 = StoragePath::from_string("/test/file2.txt");
        let id1 = service.get_or_create_id(&path1).await.unwrap();
        let id2 = service.get_or_create_id(&path2).await.unwrap();
        
        // Guardar cambios
        service.save_pending_changes().await.unwrap();
        
        // Esperar para asegurar que el guardado asíncrono termine
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Crear un nuevo servicio que debería cargar el mismo mapa
        let service2 = IdMappingService::new(map_path).await.unwrap();
        
        // Verificar que los IDs coinciden
        let loaded_id1 = service2.get_or_create_id(&path1).await.unwrap();
        let loaded_id2 = service2.get_or_create_id(&path2).await.unwrap();
        
        assert_eq!(id1, loaded_id1, "ID1 should be preserved");
        assert_eq!(id2, loaded_id2, "ID2 should be preserved");
    }
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        use futures::future::join_all;
        
        let temp_dir = tempdir().unwrap();
        let map_path = temp_dir.path().join("id_map.json");
        
        let service = std::sync::Arc::new(IdMappingService::new(map_path).await.unwrap());
        
        // Crear múltiples tareas que intentan acceder simultáneamente
        let mut tasks = Vec::new();
        for i in 0..100 {
            let path = StoragePath::from_string(&format!("/test/concurrent/file{}.txt", i));
            let service_clone = service.clone();
            
            tasks.push(tokio::spawn(async move {
                service_clone.get_or_create_id(&path).await
            }));
        }
        
        // Esperar a que todas terminen
        let results = join_all(tasks).await;
        
        // Verificar que todas tuvieron éxito
        for result in results {
            assert!(result.unwrap().is_ok(), "Concurrent operations should succeed");
        }
        
        // Guardar cambios
        service.save_pending_changes().await.unwrap();
    }
}