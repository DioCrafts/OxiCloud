use std::sync::Arc;
use futures::{future::join_all, Future};
use tokio::sync::Semaphore;
use tracing::info;
use thiserror::Error;

use crate::application::services::file_service::FileService;
use crate::application::services::folder_service::FolderService;
use crate::common::errors::DomainError;
use crate::common::config::AppConfig;
use crate::application::ports::inbound::FolderUseCase;
use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;

/// Errores específicos para operaciones por lotes
#[derive(Debug, Error)]
pub enum BatchOperationError {
    #[error("Error de dominio: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Operación cancelada: {0}")]
    Cancelled(String),
    
    #[error("Límite de concurrencia excedido: {0}")]
    ConcurrencyLimit(String),
    
    #[error("Error en operación del lote: {0} ({1} de {2} completadas)")]
    PartialFailure(String, usize, usize),
    
    #[error("Error interno: {0}")]
    Internal(String),
}

/// Resultado de una operación por lotes con estadísticas
#[derive(Debug, Clone)]
pub struct BatchResult<T> {
    /// Resultados exitosos
    pub successful: Vec<T>,
    /// Operaciones fallidas con sus errores
    pub failed: Vec<(String, String)>,
    /// Estadísticas de la operación
    pub stats: BatchStats,
}

/// Estadísticas de una operación por lotes
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Número total de operaciones
    pub total: usize,
    /// Número de operaciones exitosas
    pub successful: usize,
    /// Número de operaciones fallidas
    pub failed: usize,
    /// Tiempo total de ejecución en milisegundos
    pub execution_time_ms: u128,
    /// Concurrencia máxima alcanzada
    pub max_concurrency: usize,
}

/// Servicio de operaciones por lotes
pub struct BatchOperationService {
    file_service: Arc<FileService>,
    folder_service: Arc<FolderService>,
    config: AppConfig,
    semaphore: Arc<Semaphore>,
}

impl BatchOperationService {
    /// Crea una nueva instancia del servicio de operaciones por lotes
    pub fn new(
        file_service: Arc<FileService>, 
        folder_service: Arc<FolderService>,
        config: AppConfig
    ) -> Self {
        // Limitar la concurrencia basada en la configuración
        let max_concurrency = config.concurrency.max_concurrent_files;
        
        Self {
            file_service,
            folder_service,
            config,
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
        }
    }
    
    /// Crea una nueva instancia con la configuración por defecto
    pub fn default(
        file_service: Arc<FileService>, 
        folder_service: Arc<FolderService>
    ) -> Self {
        Self::new(file_service, folder_service, AppConfig::default())
    }
    
    /// Copia múltiples archivos en paralelo
    pub async fn copy_files(
        &self,
        file_ids: Vec<String>,
        target_folder_id: Option<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Iniciando copia en lote de {} archivos", file_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación a realizar para cada archivo
        let operations = file_ids.into_iter().map(|file_id| {
            let file_service = self.file_service.clone();
            let target_folder = target_folder_id.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let copy_result = file_service.move_file(&file_id, target_folder.clone()).await;
                
                // Liberar el permiso explícitamente (también se libera al hacer drop)
                drop(permit);
                
                // Devolver el resultado junto con el ID para identificar éxitos/fallos
                (file_id, copy_result)
            }
        });
        
        // Ejecutar todas las operaciones en paralelo con control de concurrencia
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Copia en lote completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Mueve múltiples archivos en paralelo
    pub async fn move_files(
        &self,
        file_ids: Vec<String>,
        target_folder_id: Option<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Iniciando movimiento en lote de {} archivos", file_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación a realizar para cada archivo
        let operations = file_ids.into_iter().map(|file_id| {
            let file_service = self.file_service.clone();
            let target_folder = target_folder_id.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let move_result = file_service.move_file(&file_id, target_folder.clone()).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado junto con el ID para identificar éxitos/fallos
                (file_id, move_result)
            }
        });
        
        // Ejecutar todas las operaciones en paralelo con control de concurrencia
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Movimiento en lote completado: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Elimina múltiples archivos en paralelo
    pub async fn delete_files(
        &self,
        file_ids: Vec<String>,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        info!("Iniciando eliminación en lote de {} archivos", file_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación a realizar para cada archivo
        let operations = file_ids.into_iter().map(|file_id| {
            let file_service = self.file_service.clone();
            let semaphore = self.semaphore.clone();
            let id_clone = file_id.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let delete_result = file_service.delete_file(&file_id).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado junto con el ID
                (id_clone.clone(), delete_result.map(|_| id_clone))
            }
        });
        
        // Ejecutar todas las operaciones en paralelo con control de concurrencia
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Eliminación en lote completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Carga múltiples archivos en paralelo (datos en memoria)
    pub async fn get_multiple_files(
        &self,
        file_ids: Vec<String>,
    ) -> Result<BatchResult<FileDto>, BatchOperationError> {
        info!("Iniciando carga en lote de {} archivos", file_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: file_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación a realizar para cada archivo
        let operations = file_ids.into_iter().map(|file_id| {
            let file_service = self.file_service.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let get_result = file_service.get_file(&file_id).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado junto con el ID
                (file_id, get_result)
            }
        });
        
        // Ejecutar todas las operaciones en paralelo con control de concurrencia
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (file_id, operation_result) in operation_results {
            match operation_result {
                Ok(file) => {
                    result.successful.push(file);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((file_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Carga en lote completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Elimina múltiples carpetas en paralelo
    pub async fn delete_folders(
        &self,
        folder_ids: Vec<String>,
        _recursive: bool,
    ) -> Result<BatchResult<String>, BatchOperationError> {
        info!("Iniciando eliminación en lote de {} carpetas", folder_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación a realizar para cada carpeta
        let operations = folder_ids.into_iter().map(|folder_id| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();
            let id_clone = folder_id.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                // For both recursive and non-recursive, use the standard delete_folder method
                // since FolderUseCase only has a single delete_folder method
                let delete_result = folder_service.delete_folder(&folder_id).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado junto con el ID
                (id_clone.clone(), delete_result.map(|_| id_clone))
            }
        });
        
        // Ejecutar todas las operaciones en paralelo con control de concurrencia
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(id) => {
                    result.successful.push(id);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Eliminación en lote de carpetas completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Operación genérica de lote para cualquier tipo de función asíncrona
    pub async fn generic_batch_operation<T, F, Fut>(
        &self,
        items: Vec<T>,
        operation: F,
    ) -> Result<BatchResult<T>, BatchOperationError>
    where
        T: Clone + Send + 'static + std::fmt::Debug,
        F: Fn(T, Arc<Semaphore>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = Result<T, DomainError>> + Send + 'static,
    {
        info!("Iniciando operación genérica en lote con {} items", items.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: items.len(),
                ..Default::default()
            },
        };
        
        // Convertir cada item a una tarea
        let tasks = items.iter().map(|item| {
            let item_clone = item.clone();
            let op = operation.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // La función proporcionada debe manejar la adquisición del semáforo
                let op_result = op(item_clone.clone(), semaphore).await;
                
                // Devolver el resultado junto con el item original para identificación
                (item_clone, op_result)
            }
        });
        
        // Ejecutar todas las tareas en paralelo
        let operation_results = join_all(tasks).await;
        
        // Procesar resultados
        for (item, operation_result) in operation_results {
            match operation_result {
                Ok(result_item) => {
                    result.successful.push(result_item);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    // Convertir el item a string para el reporte de error
                    result.failed.push((format!("{:?}", item), e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Operación genérica en lote completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Crear múltiples carpetas en paralelo
    pub async fn create_folders(
        &self,
        folders: Vec<(String, Option<String>)>, // (nombre, padre_id)
    ) -> Result<BatchResult<FolderDto>, BatchOperationError> {
        info!("Iniciando creación en lote de {} carpetas", folders.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folders.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación para cada carpeta
        let operations = folders.into_iter().map(|(name, parent_id)| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let dto = crate::application::dtos::folder_dto::CreateFolderDto {
                    name: name.clone(),
                    parent_id: parent_id.clone()
                };
                let create_result = folder_service.create_folder(dto).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado con un identificador para los errores
                let id = format!("{}:{}", name, parent_id.unwrap_or_default());
                (id, create_result)
            }
        });
        
        // Ejecutar todas las operaciones en paralelo
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (id, operation_result) in operation_results {
            match operation_result {
                Ok(folder) => {
                    result.successful.push(folder);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Creación en lote de carpetas completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
    
    /// Obtener metadatos de múltiples carpetas en paralelo
    pub async fn get_multiple_folders(
        &self,
        folder_ids: Vec<String>,
    ) -> Result<BatchResult<FolderDto>, BatchOperationError> {
        info!("Iniciando carga en lote de {} carpetas", folder_ids.len());
        let start_time = std::time::Instant::now();
        
        // Crear estructura para el resultado
        let mut result = BatchResult {
            successful: Vec::new(),
            failed: Vec::new(),
            stats: BatchStats {
                total: folder_ids.len(),
                ..Default::default()
            },
        };
        
        // Definir la operación para cada carpeta
        let operations = folder_ids.into_iter().map(|folder_id| {
            let folder_service = self.folder_service.clone();
            let semaphore = self.semaphore.clone();
            
            async move {
                // Adquirir permiso del semáforo
                let permit = semaphore.acquire().await.unwrap();
                
                let get_result = folder_service.get_folder(&folder_id).await;
                
                // Liberar el permiso explícitamente
                drop(permit);
                
                // Devolver el resultado con su ID
                (folder_id, get_result)
            }
        });
        
        // Ejecutar todas las operaciones en paralelo
        let operation_results = join_all(operations).await;
        
        // Procesar los resultados
        for (folder_id, operation_result) in operation_results {
            match operation_result {
                Ok(folder) => {
                    result.successful.push(folder);
                    result.stats.successful += 1;
                }
                Err(e) => {
                    result.failed.push((folder_id, e.to_string()));
                    result.stats.failed += 1;
                }
            }
        }
        
        // Completar estadísticas
        result.stats.execution_time_ms = start_time.elapsed().as_millis();
        result.stats.max_concurrency = self.config.concurrency.max_concurrent_files
            .min(result.stats.total);
        
        info!(
            "Carga en lote de carpetas completada: {}/{} exitosas en {}ms",
            result.stats.successful,
            result.stats.total,
            result.stats.execution_time_ms
        );
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use mockall::predicate::*;
    use mockall::mock;
    
    // Crear mocks para los servicios
    mock! {
        FileSvcMock {}
        
        #[async_trait]
        impl FileService for FileSvcMock {
            async fn create_file(&self, name: String, folder_id: Option<String>, content_type: String, content: Vec<u8>) -> Result<File, DomainError>;
            async fn get_file(&self, id: &str) -> Result<File, DomainError>;
            async fn delete_file(&self, id: &str) -> Result<(), DomainError>;
            async fn move_file(&self, id: &str, target_folder_id: Option<String>) -> Result<File, DomainError>;
            async fn copy_file(&self, id: &str, target_folder_id: Option<String>) -> Result<File, DomainError>;
            async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError>;
            async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError>;
        }
    }
    
    mock! {
        FolderSvcMock {}
        
        #[async_trait]
        impl FolderService for FolderSvcMock {
            async fn create_folder(&self, name: String, parent_id: Option<String>) -> Result<Folder, DomainError>;
            async fn get_folder(&self, id: &str) -> Result<Folder, DomainError>;
            async fn delete_folder(&self, id: &str) -> Result<(), DomainError>;
            async fn delete_folder_recursive(&self, id: &str) -> Result<(), DomainError>;
            async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<Folder>, DomainError>;
            async fn move_folder(&self, id: &str, target_parent_id: Option<String>) -> Result<Folder, DomainError>;
        }
    }
    
    #[tokio::test]
    async fn test_batch_delete_files() {
        // Crear mocks
        let mut file_service = MockFileSvcMock::new();
        
        // Configurar comportamiento esperado
        file_service.expect_delete_file()
            .times(3)
            .returning(|id| {
                if id == "error-id" {
                    Err(DomainError::not_found("FileService", "File not found"))
                } else {
                    Ok(())
                }
            });
        
        // Crear el servicio de batch con los mocks
        let batch_service = BatchOperationService::new(
            Arc::new(file_service),
            Arc::new(MockFolderSvcMock::new()),
            AppConfig::default()
        );
        
        // Ejecutar la operación de batch
        let file_ids = vec![
            "id1".to_string(), 
            "id2".to_string(), 
            "error-id".to_string()
        ];
        
        let result = batch_service.delete_files(file_ids).await.unwrap();
        
        // Verificar los resultados
        assert_eq!(result.stats.total, 3);
        assert_eq!(result.stats.successful, 2);
        assert_eq!(result.stats.failed, 1);
        assert_eq!(result.successful.len(), 2);
        assert_eq!(result.failed.len(), 1);
        assert_eq!(result.failed[0].0, "error-id");
    }
    
    #[tokio::test]
    async fn test_generic_batch_operation() {
        // Crear el servicio de batch
        let batch_service = BatchOperationService::new(
            Arc::new(MockFileSvcMock::new()),
            Arc::new(MockFolderSvcMock::new()),
            AppConfig::default()
        );
        
        // Definir una operación genérica de prueba
        let operation = |item: i32, semaphore: Arc<Semaphore>| async move {
            // Adquirir y liberar el semáforo
            let _permit = semaphore.acquire().await.unwrap();
            
            if item % 2 == 0 {
                // Simular éxito para números pares
                Ok(item * 2)
            } else {
                // Simular error para números impares
                Err(DomainError::invalid_input("Test", "Odd number not allowed"))
            }
        };
        
        // Ejecutar la operación de batch
        let items = vec![1, 2, 3, 4, 5];
        
        let result = batch_service.generic_batch_operation(items, operation).await.unwrap();
        
        // Verificar los resultados
        assert_eq!(result.stats.total, 5);
        assert_eq!(result.stats.successful, 2);
        assert_eq!(result.stats.failed, 3);
        
        // Los números pares deberían estar en los éxitos, duplicados
        assert!(result.successful.contains(&4)); // 2*2
        assert!(result.successful.contains(&8)); // 4*2
        
        // Los impares deberían estar en los fallos
        assert_eq!(result.failed.len(), 3);
    }
}