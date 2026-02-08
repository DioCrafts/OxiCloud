use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use async_trait::async_trait;
use tokio::time;

use crate::common::errors::Result;
use crate::application::dtos::search_dto::{SearchCriteriaDto, SearchResultsDto};
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::ports::inbound::SearchUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::ports::storage_ports::FileReadPort;

/**
 * Implementación del servicio de búsqueda para archivos y carpetas.
 * 
 * Este servicio implementa la funcionalidad de búsqueda avanzada que permite
 * a los usuarios encontrar archivos y carpetas basados en diversos criterios
 * como nombre, tipo, fecha y tamaño. También incluye una caché para mejorar
 * el rendimiento de búsquedas repetidas.
 */
pub struct SearchService {
    /// Repositorio para operaciones con archivos
    file_repository: Arc<dyn FileReadPort>,
    
    /// Repositorio para operaciones con carpetas
    folder_repository: Arc<dyn FolderStoragePort>,
    
    /// Caché de resultados de búsqueda con tiempo de expiración
    search_cache: Arc<Mutex<HashMap<SearchCacheKey, CachedSearchResult>>>,
    
    /// Duración de validez de la caché en segundos
    cache_ttl: u64,
    
    /// Tamaño máximo de la caché (número de resultados almacenados)
    max_cache_size: usize,
}

/// Clave para la caché de búsqueda
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchCacheKey {
    /// Representación serializada de los criterios de búsqueda
    criteria_hash: String,
    
    /// ID del usuario (para aislar búsquedas entre usuarios)
    user_id: String,
}

/// Resultado de búsqueda en caché con tiempo de expiración
struct CachedSearchResult {
    /// Resultados de la búsqueda
    results: SearchResultsDto,
    
    /// Momento en que se creó la entrada de caché
    timestamp: Instant,
}

impl SearchService {
    /**
     * Crea una nueva instancia del servicio de búsqueda.
     * 
     * @param file_repository Repositorio para operaciones con archivos
     * @param folder_repository Repositorio para operaciones con carpetas
     * @param cache_ttl Tiempo de vida de la caché en segundos (0 para desactivar)
     * @param max_cache_size Tamaño máximo de la caché
     */
    pub fn new(
        file_repository: Arc<dyn FileReadPort>,
        folder_repository: Arc<dyn FolderStoragePort>,
        cache_ttl: u64,
        max_cache_size: usize,
    ) -> Self {
        let search_service = Self {
            file_repository,
            folder_repository,
            search_cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl,
            max_cache_size,
        };
        
        // Iniciar tarea de limpieza de caché si TTL > 0
        if cache_ttl > 0 {
            Self::start_cache_cleanup_task(search_service.search_cache.clone(), cache_ttl);
        }
        
        search_service
    }
    
    /**
     * Inicia una tarea asíncrona para limpiar entradas expiradas de la caché.
     * 
     * @param cache_ref Referencia a la caché compartida
     * @param ttl_seconds TTL en segundos
     */
    fn start_cache_cleanup_task(
        cache_ref: Arc<Mutex<HashMap<SearchCacheKey, CachedSearchResult>>>,
        ttl_seconds: u64,
    ) {
        tokio::spawn(async move {
            let cleanup_interval = Duration::from_secs(ttl_seconds / 2);
            let ttl = Duration::from_secs(ttl_seconds);
            
            loop {
                time::sleep(cleanup_interval).await;
                
                // Obtener lock y limpiar entradas expiradas
                if let Ok(mut cache) = cache_ref.lock() {
                    let now = Instant::now();
                    
                    // Identificar entradas expiradas
                    let expired_keys: Vec<SearchCacheKey> = cache
                        .iter()
                        .filter(|(_, result)| now.duration_since(result.timestamp) > ttl)
                        .map(|(key, _)| key.clone())
                        .collect();
                    
                    // Eliminar entradas expiradas
                    for key in expired_keys {
                        cache.remove(&key);
                    }
                }
            }
        });
    }
    
    /**
     * Crea una clave de caché a partir de los criterios de búsqueda.
     * 
     * @param criteria Criterios de búsqueda
     * @param user_id ID del usuario (para aislar caché entre usuarios)
     * @return Clave para la caché
     */
    fn create_cache_key(&self, criteria: &SearchCriteriaDto, user_id: &str) -> SearchCacheKey {
        // Serializar criterios para generar un hash
        let criteria_str = serde_json::to_string(criteria).unwrap_or_default();
        
        SearchCacheKey {
            criteria_hash: criteria_str,
            user_id: user_id.to_string(),
        }
    }
    
    /**
     * Intenta obtener resultados de la caché.
     * 
     * @param key Clave de caché
     * @return Opcionalmente, los resultados si existen y no han expirado
     */
    fn get_from_cache(&self, key: &SearchCacheKey) -> Option<SearchResultsDto> {
        // Si TTL es 0, la caché está desactivada
        if self.cache_ttl == 0 {
            return None;
        }
        
        if let Ok(cache) = self.search_cache.lock() {
            if let Some(cached_result) = cache.get(key) {
                let now = Instant::now();
                let ttl = Duration::from_secs(self.cache_ttl);
                
                // Comprobar si la entrada ha expirado
                if now.duration_since(cached_result.timestamp) < ttl {
                    return Some(cached_result.results.clone());
                }
            }
        }
        
        None
    }
    
    /**
     * Almacena resultados en la caché.
     * 
     * @param key Clave de caché
     * @param results Resultados a almacenar
     */
    fn store_in_cache(&self, key: SearchCacheKey, results: SearchResultsDto) {
        // Si TTL es 0, la caché está desactivada
        if self.cache_ttl == 0 {
            return;
        }
        
        if let Ok(mut cache) = self.search_cache.lock() {
            // Si la caché está llena, eliminar la entrada más antigua
            if cache.len() >= self.max_cache_size {
                if let Some((oldest_key, _)) = cache
                    .iter()
                    .min_by_key(|(_, result)| result.timestamp) {
                    let key_to_remove = oldest_key.clone();
                    cache.remove(&key_to_remove);
                }
            }
            
            // Almacenar el nuevo resultado
            cache.insert(key, CachedSearchResult {
                results,
                timestamp: Instant::now(),
            });
        }
    }
    
    /**
     * Filtra archivos según los criterios de búsqueda.
     * 
     * @param files Lista de archivos a filtrar
     * @param criteria Criterios de búsqueda
     * @return Archivos que cumplen con los criterios
     */
    fn filter_files(&self, files: Vec<FileDto>, criteria: &SearchCriteriaDto) -> Vec<FileDto> {
        files.into_iter()
            .filter(|file| {
                // Filtrar por nombre
                if let Some(name_query) = &criteria.name_contains {
                    if !file.name.to_lowercase().contains(&name_query.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filtrar por tipo de archivo (extensión)
                if let Some(file_types) = &criteria.file_types {
                    if let Some(extension) = file.name.split('.').last() {
                        if !file_types.iter().any(|ext| ext.eq_ignore_ascii_case(extension)) {
                            return false;
                        }
                    } else {
                        // No tiene extensión
                        return false;
                    }
                }
                
                // Filtrar por fecha de creación
                if let Some(created_after) = criteria.created_after {
                    if file.created_at < created_after {
                        return false;
                    }
                }
                
                if let Some(created_before) = criteria.created_before {
                    if file.created_at > created_before {
                        return false;
                    }
                }
                
                // Filtrar por fecha de modificación
                if let Some(modified_after) = criteria.modified_after {
                    if file.modified_at < modified_after {
                        return false;
                    }
                }
                
                if let Some(modified_before) = criteria.modified_before {
                    if file.modified_at > modified_before {
                        return false;
                    }
                }
                
                // Filtrar por tamaño
                if let Some(min_size) = criteria.min_size {
                    if file.size < min_size {
                        return false;
                    }
                }
                
                if let Some(max_size) = criteria.max_size {
                    if file.size > max_size {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /**
     * Filtra carpetas según los criterios de búsqueda.
     * 
     * @param folders Lista de carpetas a filtrar
     * @param criteria Criterios de búsqueda
     * @return Carpetas que cumplen con los criterios
     */
    fn filter_folders(&self, folders: Vec<FolderDto>, criteria: &SearchCriteriaDto) -> Vec<FolderDto> {
        folders.into_iter()
            .filter(|folder| {
                // Filtrar por nombre
                if let Some(name_query) = &criteria.name_contains {
                    if !folder.name.to_lowercase().contains(&name_query.to_lowercase()) {
                        return false;
                    }
                }
                
                // Filtrar por fecha de creación
                if let Some(created_after) = criteria.created_after {
                    if folder.created_at < created_after {
                        return false;
                    }
                }
                
                if let Some(created_before) = criteria.created_before {
                    if folder.created_at > created_before {
                        return false;
                    }
                }
                
                // Filtrar por fecha de modificación
                if let Some(modified_after) = criteria.modified_after {
                    if folder.modified_at < modified_after {
                        return false;
                    }
                }
                
                if let Some(modified_before) = criteria.modified_before {
                    if folder.modified_at > modified_before {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /**
     * Implementación de la búsqueda recursiva a través de carpetas.
     * 
     * @param current_folder_id ID de la carpeta actual
     * @param criteria Criterios de búsqueda
     * @param found_files Archivos encontrados hasta ahora
     * @param found_folders Carpetas encontradas hasta ahora
     */
    async fn search_recursive(
        &self,
        current_folder_id: Option<&str>,
        criteria: &SearchCriteriaDto,
        found_files: &mut Vec<FileDto>,
        found_folders: &mut Vec<FolderDto>,
    ) -> Result<()> {
        Box::pin(async move {
        // Listar archivos en la carpeta actual
        let files = self.file_repository.list_files(current_folder_id).await?;
        
        // Filtrar archivos según criterios y agregarlos a los resultados
        let filtered_files = self.filter_files(
            files.into_iter().map(FileDto::from).collect(), 
            criteria
        );
        found_files.extend(filtered_files);
        
        // Si la búsqueda es recursiva, procesar subcarpetas
        if criteria.recursive {
            // Listar subcarpetas
            let folders = self.folder_repository.list_folders(current_folder_id).await?;
            
            // Filtrar carpetas según criterios y agregarlas a los resultados
            let filtered_folders: Vec<FolderDto> = self.filter_folders(
                folders.into_iter().map(FolderDto::from).collect(),
                criteria
            );
            
            // Añadir las carpetas filtradas a los resultados
            found_folders.extend(filtered_folders.iter().cloned());
            
            // Buscar recursivamente en cada subcarpeta
            for folder in filtered_folders {
                self.search_recursive(
                    Some(&folder.id),
                    criteria,
                    found_files,
                    found_folders,
                ).await?;
            }
        }
        
        Ok(())
        }).await
    }
}

#[async_trait]
impl SearchUseCase for SearchService {
    /**
     * Realiza una búsqueda basada en los criterios especificados.
     * 
     * @param criteria Criterios de búsqueda
     * @return Resultados de la búsqueda
     */
    async fn search(&self, criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
        // TODO: Obtener ID de usuario del contexto de autenticación
        let user_id = "default-user";
        let cache_key = self.create_cache_key(&criteria, user_id);
        
        // Intentar obtener resultados de la caché
        if let Some(cached_results) = self.get_from_cache(&cache_key) {
            return Ok(cached_results);
        }
        
        // Inicializar colecciones para resultados
        let mut found_files: Vec<FileDto> = Vec::new();
        let mut found_folders: Vec<FolderDto> = Vec::new();
        
        // Realizar búsqueda en la carpeta especificada o en la raíz
        self.search_recursive(
            criteria.folder_id.as_deref(),
            &criteria,
            &mut found_files,
            &mut found_folders,
        ).await?;
        
        // Aplicar paginación
        let total_count = found_files.len() + found_folders.len();
        
        // Ordenar por relevancia o fecha según criterios
        // Por defecto, ordenamos por fecha de modificación (más reciente primero)
        found_files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        found_folders.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        
        // Aplicar límite y offset para paginación
        let start_idx = criteria.offset.min(total_count);
        let end_idx = (criteria.offset + criteria.limit).min(total_count);
        
        let paginated_items: Vec<(bool, usize)> = (start_idx..end_idx)
            .map(|i| {
                if i < found_folders.len() {
                    (true, i) // Es una carpeta
                } else {
                    (false, i - found_folders.len()) // Es un archivo
                }
            })
            .collect();
        
        // Extraer elementos paginados
        let mut paginated_folders = Vec::new();
        let mut paginated_files = Vec::new();
        
        for (is_folder, idx) in paginated_items {
            if is_folder {
                if idx < found_folders.len() {
                    paginated_folders.push(found_folders[idx].clone());
                }
            } else {
                if idx < found_files.len() {
                    paginated_files.push(found_files[idx].clone());
                }
            }
        }
        
        // Crear objeto de resultados
        let search_results = SearchResultsDto::new(
            paginated_files,
            paginated_folders,
            criteria.limit,
            criteria.offset,
            Some(total_count),
        );
        
        // Almacenar en caché
        self.store_in_cache(cache_key, search_results.clone());
        
        Ok(search_results)
    }
    
    /**
     * Limpia la caché de resultados de búsqueda.
     * 
     * @return Resultado indicando éxito
     */
    async fn clear_search_cache(&self) -> Result<()> {
        if let Ok(mut cache) = self.search_cache.lock() {
            cache.clear();
        }
        Ok(())
    }
}

// Implementar el caso de uso de prueba (stub)
impl SearchService {
    /// Crea una versión stub del servicio para pruebas
    pub fn new_stub() -> impl SearchUseCase {
        struct SearchServiceStub;
        
        #[async_trait]
        impl SearchUseCase for SearchServiceStub {
            async fn search(&self, _criteria: SearchCriteriaDto) -> Result<SearchResultsDto> {
                Ok(SearchResultsDto::empty())
            }
            
            async fn clear_search_cache(&self) -> Result<()> {
                Ok(())
            }
        }
        
        SearchServiceStub
    }
}