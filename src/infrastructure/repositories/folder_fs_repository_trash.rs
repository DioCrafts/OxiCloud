use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, error};

use crate::infrastructure::repositories::repository_errors::FolderRepositoryResult;
use crate::infrastructure::repositories::folder_fs_repository::FolderFsRepository;

// Este archivo contiene la implementación de los métodos relacionados con la papelera
// para el repositorio de carpetas FolderFsRepository

// Implementación de métodos de papelera para el repositorio de carpetas
impl FolderFsRepository {
    // Obtiene la ruta completa a la papelera
    fn get_trash_dir(&self) -> PathBuf {
        self.get_root_path().join(".trash").join("folders")
    }
    
    // Crea una ruta única en la papelera para la carpeta
    async fn create_trash_folder_path(&self, folder_id: &str) -> FolderRepositoryResult<PathBuf> {
        let trash_dir = self.get_trash_dir();
        
        // Asegurarse que el directorio de la papelera existe
        if !trash_dir.exists() {
            fs::create_dir_all(&trash_dir).await
                .map_err(|e| FolderRepositoryError::StorageError(e.to_string()))?;
        }
        
        // Crear una ruta única para la carpeta en la papelera
        Ok(trash_dir.join(folder_id))
    }
}

// Implementación de los métodos públicos del trait FolderRepository relacionados con la papelera
// Implementation of internal methods for trash functionality
// These will be enabled when the trash feature is re-enabled
impl FolderFsRepository {
    /// Helper method that will be used for trash functionality
    pub(crate) async fn _trash_move_to_trash(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        debug!("Moviendo carpeta a la papelera: {}", folder_id);
        
        // Obtener la ruta física de la carpeta
        let folder_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => path,
            Err(e) => {
                error!("Error obteniendo ruta de la carpeta {}: {:?}", folder_id, e);
                return Err(e);
            }
        };
        
        let folder_path_buf = PathBuf::from(folder_path.to_string());
        
        // Verificamos que la carpeta existe
        if !folder_path_buf.exists() {
            return Err(FolderRepositoryError::NotFound(format!("Folder not found: {}", folder_id)));
        }
        
        // Crear directorio en la papelera
        let trash_folder_path = self.create_trash_folder_path(folder_id).await?;
        
        // Mover la carpeta físicamente a la papelera
        match fs::rename(&folder_path_buf, &trash_folder_path).await {
            Ok(_) => {
                debug!("Carpeta movida a papelera: {} -> {}", folder_path_buf.display(), trash_folder_path.display());
                
                // Actualizar el mapeo al nuevo path en la papelera
                if let Err(e) = self.update_mapped_folder_path(folder_id, &trash_folder_path).await {
                    error!("Error actualizando mapeo de carpeta en papelera: {}", e);
                    return Err(e);
                }
                
                Ok(())
            },
            Err(e) => {
                error!("Error moviendo carpeta a papelera: {}", e);
                Err(FolderRepositoryError::StorageError(e.to_string()))
            }
        }
    }
    
    /// Restaura una carpeta desde la papelera a su ubicación original
    pub(crate) async fn _trash_restore_from_trash(&self, folder_id: &str, original_path: &str) -> FolderRepositoryResult<()> {
        debug!("Restaurando carpeta {} a {}", folder_id, original_path);
        
        // Obtener la ruta actual en la papelera
        let current_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => PathBuf::from(path),
            Err(e) => {
                error!("Error obteniendo ruta actual de la carpeta {}: {:?}", folder_id, e);
                return Err(e);
            }
        };
        
        // Convertir la ruta original a PathBuf
        let original_path_buf = PathBuf::from(original_path);
        
        // Asegurar que el directorio padre de destino existe
        if let Some(parent) = original_path_buf.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .map_err(|e| {
                        error!("Error creando directorio padre para restauración: {}", e);
                        FolderRepositoryError::StorageError(e.to_string())
                    })?;
            }
        }
        
        // Mover la carpeta de la papelera a su ubicación original
        match fs::rename(&current_path, &original_path_buf).await {
            Ok(_) => {
                debug!("Carpeta restaurada: {} -> {}", current_path.display(), original_path_buf.display());
                
                // Actualizar el mapeo a la ruta original
                if let Err(e) = self.update_mapped_folder_path(folder_id, &original_path_buf).await {
                    error!("Error actualizando mapeo de carpeta restaurada: {}", e);
                    return Err(e);
                }
                
                Ok(())
            },
            Err(e) => {
                error!("Error restaurando carpeta: {}", e);
                Err(FolderRepositoryError::StorageError(e.to_string()))
            }
        }
    }
    
    /// Elimina una carpeta permanentemente (usado por la papelera)
    pub(crate) async fn _trash_delete_folder_permanently(&self, folder_id: &str) -> FolderRepositoryResult<()> {
        debug!("Eliminando carpeta permanentemente: {}", folder_id);
        
        // Similar a delete_folder pero sin validaciones adicionales
        let folder_path = match self.get_mapped_folder_path(folder_id).await {
            Ok(path) => PathBuf::from(path),
            Err(e) => {
                error!("Error obteniendo ruta de la carpeta {}: {:?}", folder_id, e);
                return Err(e);
            }
        };
        
        // Eliminar la carpeta recursivamente
        if folder_path.exists() {
            match fs::remove_dir_all(&folder_path).await {
                Ok(_) => {
                    debug!("Carpeta eliminada permanentemente: {}", folder_path.display());
                },
                Err(e) => {
                    error!("Error eliminando carpeta permanentemente: {}", e);
                    // No reportar error si la carpeta ya no existe
                    if e.kind() != std::io::ErrorKind::NotFound {
                        return Err(FolderRepositoryError::StorageError(e.to_string()));
                    }
                }
            }
        }
        
        // Eliminar el mapeo
        if let Err(e) = self.remove_mapped_folder_id(folder_id).await {
            error!("Error eliminando mapeo de la carpeta: {}", e);
            return Err(e);
        }
        
        debug!("Carpeta eliminada permanentemente con éxito: {}", folder_id);
        Ok(())
    }
}

// Re-exportaciones necesarias para el compilador
use crate::infrastructure::repositories::repository_errors::FolderRepositoryError;