use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;
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
    #[allow(dead_code)]
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

/// Constantes para configuración
const SAVE_DEBOUNCE_MS: u64 = 300; // Tiempo para agrupar operaciones de guardado

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
        
        // Si estamos cargando el mapa de carpetas, asegurarnos de que la carpeta de usuario esté mapeada
        let is_folder_map = map_path.to_string_lossy().contains("folder_ids.json");
        let mut empty_map = IdMap {
            path_to_id: HashMap::new(),
            id_to_path: HashMap::new(),
            version: 1, // Iniciar con versión 1
        };
        
        // Inicializar automáticamente con la carpeta del usuario si es el mapa de carpetas
        if is_folder_map {
            // Obtener el usuario actual del sistema
            tracing::info!("Initializing folder map for user");
            
            // Usar el hostname o un nombre predeterminado, evitando referencias específicas a usuarios
            let username = match std::env::var("HOSTNAME") {
                Ok(name) => name,
                Err(_) => {
                    // Generar un nombre basado en la fecha/hora para ser único pero consistente
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    
                    format!("oxicloud_{}", current_time % 10000)
                }
            };
            
            tracing::info!("Detected username: {}", username);
            
            // Crear una carpeta de usuario predeterminada con un ID generado con uuid
            let user_folder_name = format!("Mi Carpeta - {}", username);
            let user_folder_path = format!("/{}", user_folder_name);
            let user_folder_id = Uuid::new_v4().to_string();
            
            // Añadir el mapeo para la carpeta del usuario
            empty_map.path_to_id.insert(user_folder_path.clone(), user_folder_id.clone());
            empty_map.id_to_path.insert(user_folder_id, user_folder_path);
            
            tracing::info!("Added default user folder mapping for '{}' to ID map", user_folder_name);
            
            // Asegurar que la carpeta física exista en el sistema de archivos
            if let Some(parent_dir) = map_path.parent() {
                let storage_path = parent_dir.to_path_buf();
                let user_folder_path = storage_path.join(user_folder_name);
                
                tracing::info!("Creating user folder at {}", user_folder_path.display());
                if let Err(e) = fs::create_dir_all(&user_folder_path).await {
                    tracing::error!("Failed to create user folder: {}", e);
                }
            }
        }
        
        tracing::info!("No existing ID map found, creating new map with {} entries", empty_map.path_to_id.len());
        
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
                tracing::info!("Incrementing ID map version to {} for {}", map.version, self.map_path.display());
            }
            
            // Use serde with reasonably safe defaults
            serde_json::to_string_pretty(&*map)
                .map_err(|e| DomainError::internal_error("IdMapping", format!("Failed to serialize ID map to JSON: {}", e)))?
        };
        
        tracing::info!("Saving ID map with length {} bytes to {}", json.len(), self.map_path.display());
        
        // Enfoque más robusto: intentar guardar directamente si hay problemas con archivos temporales
        // Intentar primero con archivo temporal
        let temp_filename = format!("{}.{}.tmp", Uuid::new_v4(), std::process::id());
        let temp_path = if let Some(parent) = self.map_path.parent() {
            parent.join(temp_filename)
        } else {
            PathBuf::from(temp_filename)
        };
        
        // Asegurarse de que el directorio padre exista
        if let Some(parent) = self.map_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    tracing::warn!("Failed to create parent directory for ID map: {}", e);
                }
            }
        }
        
        // Intentar escribir al archivo temporal
        tracing::debug!("Writing temporary ID map to {}", temp_path.display());
        let temp_write_result = fs::write(&temp_path, &json).await;
        
        if let Err(e) = &temp_write_result {
            tracing::warn!("Failed to write temporary ID map to {}: {}. Will try direct write.", temp_path.display(), e);
            
            // Si falla, intentar escritura directa
            let direct_write_result = fs::write(&self.map_path, &json).await
                .map_err(|e| DomainError::internal_error("IdMapping", 
                    format!("Failed to write ID map directly to {}: {}", self.map_path.display(), e)));
                    
            // Explícitamente sincronizamos el directorio para garantizar que los cambios se escriban en disco
            if direct_write_result.is_ok() {
                // Intentar sincronizar el directorio si es posible
                if let Some(parent) = self.map_path.parent() {
                    // Clonamos el path para evitar problemas de borrowing
                    let parent_path = parent.to_path_buf();
                    
                    // Solo podemos hacer esto en sistemas Unix/Linux
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileExt;
                        // Esto debe ejecutarse en un bloque blocking para evitar bloquear el runtime de tokio
                        match tokio::task::spawn_blocking(move || {
                            if let Ok(dir) = std::fs::File::open(&parent_path) {
                                if let Err(e) = dir.sync_all() {
                                    tracing::warn!("Failed to fsync directory after writing ID map: {}", e);
                                } else {
                                    tracing::debug!("Successfully synced directory after direct write");
                                }
                            }
                        }).await {
                            Ok(_) => {},
                            Err(e) => tracing::warn!("Failed to run fsync task: {}", e),
                        }
                    }
                }
            }
            
            return direct_write_result;
        }
        
        // Intentar el rename atómico
        tracing::debug!("Renaming temporary file {} to {}", temp_path.display(), self.map_path.display());
        let result = match fs::rename(&temp_path, &self.map_path).await {
            Ok(_) => {
                // Éxito con el método atómico
                tracing::info!("Successfully renamed temporary ID map to {}", self.map_path.display());
                
                // Intentar sincronizar el directorio si es posible
                if let Some(parent) = self.map_path.parent() {
                    // Clonamos el path para evitar problemas de borrowing
                    let parent_path = parent.to_path_buf();
                    
                    // Solo podemos hacer esto en sistemas Unix/Linux
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileExt;
                        // Esto debe ejecutarse en un bloque blocking para evitar bloquear el runtime de tokio
                        match tokio::task::spawn_blocking(move || {
                            if let Ok(dir) = std::fs::File::open(&parent_path) {
                                if let Err(e) = dir.sync_all() {
                                    tracing::warn!("Failed to fsync directory after renaming ID map: {}", e);
                                } else {
                                    tracing::debug!("Successfully synced directory after rename");
                                }
                            }
                        }).await {
                            Ok(_) => {},
                            Err(e) => tracing::warn!("Failed to run fsync task: {}", e),
                        }
                    }
                }
                
                Ok(())
            },
            Err(e) => {
                tracing::warn!("Failed to rename temporary ID map to {}: {}. Will try direct write.", self.map_path.display(), e);
                
                // Intentar eliminar el archivo temporal fallido
                let _ = fs::remove_file(&temp_path).await;
                
                // Si falla el rename, intentar escritura directa
                let direct_write_result = fs::write(&self.map_path, &json).await
                    .map_err(|e| DomainError::internal_error("IdMapping", 
                        format!("Failed to write ID map directly to {}: {}", self.map_path.display(), e)));
                
                // Intentar sincronizar el directorio si la escritura directa tuvo éxito    
                if direct_write_result.is_ok() {
                    // Intentar sincronizar el directorio si es posible
                    if let Some(parent) = self.map_path.parent() {
                        // Clonamos el path para evitar problemas de borrowing
                        let parent_path = parent.to_path_buf();
                        
                        // Solo podemos hacer esto en sistemas Unix/Linux
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::FileExt;
                            // Esto debe ejecutarse en un bloque blocking para evitar bloquear el runtime de tokio
                            match tokio::task::spawn_blocking(move || {
                                if let Ok(dir) = std::fs::File::open(&parent_path) {
                                    if let Err(e) = dir.sync_all() {
                                        tracing::warn!("Failed to fsync directory after direct write of ID map: {}", e);
                                    } else {
                                        tracing::debug!("Successfully synced directory after direct write fallback");
                                    }
                                }
                            }).await {
                                Ok(_) => {},
                                Err(e) => tracing::warn!("Failed to run fsync task: {}", e),
                            }
                        }
                    }
                }
                
                direct_write_result
            }
        };
        
        // Resetear flag de pendientes independientemente del resultado
        if result.is_ok() {
            let mut pending = self.pending_save.write().await;
            *pending = false;
            tracing::info!("Reset pending save flag after successful save to {}", self.map_path.display());
        }
        
        result
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
    
    /// Guarda cambios pendientes al disco
    pub async fn save_pending_changes(&self) -> Result<(), IdMappingError> {
        // Verificar si hay cambios pendientes - rápido check sin esperar
        {
            let pending = self.pending_save.read().await;
            if !*pending {
                tracing::debug!("No pending changes to save for ID map: {}", self.map_path.display());
                return Ok(());
            }
        }
        
        // Guardar inmediatamente de forma síncrona
        tracing::info!("Saving ID map changes immediately for: {}", self.map_path.display());
        
        // Guardar directamente con manejo rápido de errores
        let save_result = self.save_id_map().await;
        if let Err(ref e) = save_result {
            tracing::error!("Failed to save ID map to {}: {}", self.map_path.display(), e);
        } else {
            tracing::info!("ID map changes saved successfully for: {}", self.map_path.display());
        }
        
        // Lanzar guardado asíncrono en background sin esperar
        let map_path = self.map_path.clone();
        let self_clone = self.clone();
        
        tokio::spawn(async move {
            // Verificar si hay nuevos cambios pendientes periodicamente sin bloquear
            time::sleep(Duration::from_millis(SAVE_DEBOUNCE_MS/2)).await;
            
            // Verificar si todavía hay cambios pendientes
            let pending = self_clone.pending_save.read().await;
            if *pending {
                tracing::debug!("Background check: still have pending changes, saving in background: {}", map_path.display());
                if let Err(e) = self_clone.save_id_map().await {
                    tracing::warn!("Background save failed for ID map {}: {}", map_path.display(), e);
                } else {
                    tracing::debug!("Background save completed successfully for {}", map_path.display());
                }
            }
        });
        
        // Devolver el resultado del guardado principal sin esperar por el asíncrono
        // Convertir el tipo de resultado para coincidir con la firma del método
        match save_result {
            Ok(()) => Ok(()),
            Err(e) => Err(IdMappingError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("Failed to save ID map: {}", e)
            )))
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
    #[allow(dead_code)]
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