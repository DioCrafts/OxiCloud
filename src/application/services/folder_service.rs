use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::services::path_service::StoragePath;
use crate::application::dtos::folder_dto::{CreateFolderDto, RenameFolderDto, MoveFolderDto, FolderDto};
use crate::application::ports::inbound::FolderUseCase;
use crate::application::ports::outbound::FolderStoragePort;
use crate::application::transactions::storage_transaction::StorageTransaction;
use crate::common::errors::{DomainError, ErrorKind, ErrorContext};

/// Implementación del caso de uso para operaciones de carpetas
pub struct FolderService {
    folder_storage: Arc<dyn FolderStoragePort>,
}

impl FolderService {
    /// Crea un nuevo servicio de carpetas
    pub fn new(folder_storage: Arc<dyn FolderStoragePort>) -> Self {
        Self { folder_storage }
    }
    
    /// Creates a stub implementation for testing and middleware
    pub fn new_stub() -> impl FolderUseCase {
        struct FolderServiceStub;
        
        #[async_trait]
        impl FolderUseCase for FolderServiceStub {
            async fn create_folder(&self, _dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
            
            async fn get_folder(&self, _id: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
            
            async fn get_folder_by_path(&self, _path: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
            
            async fn list_folders(&self, _parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError> {
                Ok(vec![])
            }
            
            async fn list_folders_paginated(
                &self, 
                _parent_id: Option<&str>,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto
            ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError> {
                Ok(crate::application::dtos::pagination::PaginatedResponseDto::new(
                    vec![],
                    0,
                    10,
                    0
                ))
            }
            
            async fn rename_folder(&self, _id: &str, _dto: RenameFolderDto) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
            
            async fn move_folder(&self, _id: &str, _dto: MoveFolderDto) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
            
            async fn delete_folder(&self, _id: &str) -> Result<(), DomainError> {
                Ok(())
            }
        }
        
        FolderServiceStub
    }
}

#[async_trait]
impl FolderUseCase for FolderService {
    /// Crea una nueva carpeta
    async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
        // Validación de entrada
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "Folder name cannot be empty"
            ));
        }
        
        // Si se proporciona un parent_id, verificar que existe
        if let Some(parent_id) = &dto.parent_id {
            let parent_exists = self.folder_storage.get_folder(parent_id).await.is_ok();
            if !parent_exists {
                return Err(DomainError::not_found("Folder", parent_id));
            }
        }
        
        // Crear la carpeta
        let folder = self.folder_storage.create_folder(dto.name, dto.parent_id, dto.user_id)
            .await
            .with_context(|| "Failed to create folder")?;
        
        // Convertir a DTO
        Ok(FolderDto::from(folder))
    }
    
    /// Obtiene una carpeta por su ID
    async fn get_folder(&self, id: &str) -> Result<FolderDto, DomainError> {
        let folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get folder with ID: {}", id))?;
        
        Ok(FolderDto::from(folder))
    }
    
    /// Obtiene una carpeta por su ruta
    async fn get_folder_by_path(&self, path: &str) -> Result<FolderDto, DomainError> {
        // Convertir la ruta de string a StoragePath
        let storage_path = StoragePath::from_string(path);
        
        let folder = self.folder_storage.get_folder_by_path(&storage_path)
            .await
            .with_context(|| format!("Failed to get folder at path: {}", path))?;
        
        Ok(FolderDto::from(folder))
    }
    
    /// Lista carpetas dentro de una carpeta padre
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self.folder_storage.list_folders(parent_id)
            .await
            .with_context(|| format!("Failed to list folders in parent: {:?}", parent_id))?;
        
        // Convertir a DTOs
        Ok(folders.into_iter().map(FolderDto::from).collect())
    }
    
    /// Lista carpetas con paginación
    async fn list_folders_paginated(
        &self, 
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError> {
        // Validar y ajustar la paginación
        let pagination = pagination.validate_and_adjust();
        
        // Obtener carpetas paginadas y conteo total
        let (folders, total_items) = self.folder_storage.list_folders_paginated(
            parent_id,
            pagination.offset(),
            pagination.limit(),
            true // Siempre incluir total para mejor UX
        )
        .await
        .with_context(|| format!("Failed to list folders with pagination in parent: {:?}", parent_id))?;
        
        // El total es necesario para calcular la paginación
        let total = total_items.unwrap_or(folders.len());
        
        // Convertir a PaginatedResponseDto
        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total
        );
        
        Ok(response)
    }
    
    /// Renombra una carpeta
    async fn rename_folder(&self, id: &str, dto: RenameFolderDto) -> Result<FolderDto, DomainError> {
        // Validación de entrada
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "New folder name cannot be empty"
            ));
        }
        
        // Verificar que la carpeta existe
        let existing_folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get folder with ID: {} for renaming", id))?;
        
        // Crear transacción para renombrar
        let mut transaction = StorageTransaction::new("rename_folder");
        
        // Operación principal: renombrar carpeta
        // Clone all values to avoid lifetime issues
        let folder_storage = self.folder_storage.clone();
        let id_owned = id.to_string();
        let name_owned = dto.name.clone();
        
        // Create future with owned values
        let rename_op = async move {
            folder_storage.rename_folder(&id_owned, name_owned).await?;
            Ok(())
        };
        let rollback_op = {
            let original_name = existing_folder.name().to_string();
            let storage = self.folder_storage.clone();
            let id_clone = id.to_string();
            
            async move {
                // En caso de fallo, restaurar el nombre original
                storage.rename_folder(&id_clone, original_name).await
                    .map(|_| ())
                    .map_err(|e| DomainError::new(
                        ErrorKind::InternalError,
                        "Folder",
                        format!("Failed to rollback folder rename: {}", e)
                    ))
            }
        };
        
        // Añadir a la transacción
        transaction.add_operation(rename_op, rollback_op);
        
        // Ejecutar transacción
        transaction.commit().await?;
        
        // Obtener la carpeta renombrada
        let folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get renamed folder with ID: {}", id))?;
        
        Ok(FolderDto::from(folder))
    }
    
    /// Mueve una carpeta a un nuevo padre
    async fn move_folder(&self, id: &str, dto: MoveFolderDto) -> Result<FolderDto, DomainError> {
        // Verificar que la carpeta origen existe
        let source_folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get folder with ID: {} for moving", id))?;
        
        // Si se especifica un parent_id, verificar que existe
        if let Some(parent_id) = &dto.parent_id {
            // Verificar que no estamos intentando mover la carpeta a sí misma o a uno de sus descendientes
            if parent_id == id {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Folder",
                    "Cannot move a folder into itself"
                ));
            }
            
            // Verificar que el destino existe
            let parent_exists = self.folder_storage.get_folder(parent_id).await.is_ok();
            if !parent_exists {
                return Err(DomainError::not_found("Folder", parent_id));
            }
            
            // TODO: Idealmente deberíamos verificar toda la jerarquía para evitar ciclos
        }
        
        // Crear transacción para mover
        let mut transaction = StorageTransaction::new("move_folder");
        
        // Operación principal: mover carpeta
        // Clone all values to avoid lifetime issues
        let folder_storage = self.folder_storage.clone();
        let id_owned = id.to_string();
        // Get parent ID as owned string or None
        let parent_id_owned = dto.parent_id.as_ref().map(|p| p.to_string());
        
        // Create future with owned values
        let move_op = async move {
            // Convert Option<String> to Option<&str>
            let parent_ref = parent_id_owned.as_deref();
            folder_storage.move_folder(&id_owned, parent_ref).await?;
            Ok(())
        };
        let rollback_op = {
            let original_parent_id = source_folder.parent_id().map(String::from);
            let storage = self.folder_storage.clone();
            let id_clone = id.to_string();
            
            async move {
                // En caso de fallo, restaurar la ubicación original
                storage.move_folder(&id_clone, original_parent_id.as_deref()).await
                    .map(|_| ())
                    .map_err(|e| DomainError::new(
                        ErrorKind::InternalError,
                        "Folder",
                        format!("Failed to rollback folder move: {}", e)
                    ))
            }
        };
        
        // Añadir a la transacción
        transaction.add_operation(move_op, rollback_op);
        
        // Ejecutar transacción
        transaction.commit().await?;
        
        // Obtener la carpeta movida
        let folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get moved folder with ID: {}", id))?;
        
        Ok(FolderDto::from(folder))
    }
    
    /// Elimina una carpeta
    async fn delete_folder(&self, id: &str) -> Result<(), DomainError> {
        // Verificar que la carpeta existe
        let _folder = self.folder_storage.get_folder(id)
            .await
            .with_context(|| format!("Failed to get folder with ID: {} for deletion", id))?;
        
        // En una implementación real, podríamos verificar permisos, dependencias, etc.
        
        // Eliminar la carpeta
        self.folder_storage.delete_folder(id)
            .await
            .with_context(|| format!("Failed to delete folder with ID: {}", id))
    }
}