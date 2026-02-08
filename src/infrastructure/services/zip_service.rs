use std::io::{Cursor, Read, Write};
use zip::{ZipWriter, write::SimpleFileOptions};
use thiserror::Error;
use tracing::*;
use async_trait::async_trait;
use crate::{
    application::dtos::file_dto::FileDto,
    application::dtos::folder_dto::FolderDto,
    application::ports::inbound::FolderUseCase,
    application::ports::file_ports::FileRetrievalUseCase,
    application::ports::zip_ports::ZipPort,
    common::errors::{Result, DomainError, ErrorKind},
};
use std::sync::Arc;

/// Error relacionado con la creación de archivos ZIP
#[derive(Debug, Error)]
pub enum ZipError {
    #[error("Error de IO: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Error de ZIP: {0}")]
    ZipError(#[from] zip::result::ZipError),
    
    #[error("Error al leer el archivo: {0}")]
    FileReadError(String),
    
    #[error("Error al obtener contenido de carpeta: {0}")]
    FolderContentsError(String),
    
    #[error("Carpeta no encontrada: {0}")]
    FolderNotFound(String),
}

// Implementar From<ZipError> para DomainError para permitir el uso de ?
impl From<ZipError> for DomainError {
    fn from(err: ZipError) -> Self {
        DomainError::new(ErrorKind::InternalError, "zip_service", err.to_string())
    }
}

// Implementar From<zip::result::ZipError> para DomainError directamente
impl From<zip::result::ZipError> for DomainError {
    fn from(err: zip::result::ZipError) -> Self {
        DomainError::new(ErrorKind::InternalError, "zip_service", err.to_string())
    }
}

/// Servicio para crear archivos ZIP
pub struct ZipService {
    file_service: Arc<dyn FileRetrievalUseCase>,
    folder_service: Arc<dyn FolderUseCase>,
}

impl ZipService {
    /// Crea una nueva instancia del servicio ZIP con una referencia al servicio de archivos
    pub fn new(file_service: Arc<dyn FileRetrievalUseCase>, folder_service: Arc<dyn FolderUseCase>) -> Self {
        Self {
            file_service,
            folder_service,
        }
    }
    
    /// Crea un archivo ZIP con el contenido de una carpeta y todas sus subcarpetas
    /// Retorna los bytes del ZIP
    pub async fn create_folder_zip(&self, folder_id: &str, folder_name: &str) -> Result<Vec<u8>> {
        info!("Creando ZIP para carpeta: {} (ID: {})", folder_name, folder_id);
        
        // Verificar si la carpeta existe
        let folder = match self.folder_service.get_folder(folder_id).await {
            Ok(folder) => folder,
            Err(e) => {
                error!("Error al obtener carpeta {}: {}", folder_id, e);
                return Err(ZipError::FolderNotFound(folder_id.to_string()).into());
            }
        };
        
        // Crear un buffer en memoria para el ZIP
        let buf = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);
        
        // Establecer opciones de compresión
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        
        // Objeto para seguir las carpetas procesadas y evitar ciclos
        let mut processed_folders = std::collections::HashSet::new();
        
        // Procesamos la carpeta raíz y construimos el ZIP
        self.process_folder_recursively(
            &mut zip,
            &folder,
            folder_name,
            &options,
            &mut processed_folders
        ).await?;
        
        // Finalizar el ZIP y obtener los bytes
        let mut zip_buf = zip.finish()?;
        
        let mut bytes = Vec::new();
        match zip_buf.read_to_end(&mut bytes) {
            Ok(_) => Ok(bytes),
            Err(e) => {
                error!("Error al leer ZIP finalizado: {}", e);
                Err(ZipError::IoError(e).into())
            }
        }
    }
    
    // Implementación alternativa para evitar recursión en async
    async fn process_folder_recursively(
        &self,
        zip: &mut ZipWriter<Cursor<Vec<u8>>>,
        folder: &FolderDto,
        path: &str,
        options: &SimpleFileOptions,
        processed_folders: &mut std::collections::HashSet<String>
    ) -> Result<()> {
        // Estructura para representar el trabajo pendiente
        struct PendingFolder {
            folder: FolderDto,
            path: String,
        }
        
        // Cola de trabajo para procesamiento iterativo
        let mut work_queue = vec![PendingFolder {
            folder: folder.clone(),
            path: path.to_string(),
        }];
        
        // Procesar la cola mientras haya elementos
        while let Some(current) = work_queue.pop() {
            let folder_id = current.folder.id.to_string();
            
            // Evitar ciclos
            if processed_folders.contains(&folder_id) {
                continue;
            }
            
            processed_folders.insert(folder_id.clone());
            
            // Crear la entrada de directorio en el ZIP
            let folder_path = format!("{}/", current.path);
            match zip.add_directory(&folder_path, *options) {
                Ok(_) => debug!("Carpeta agregada al ZIP: {}", folder_path),
                Err(e) => {
                    warn!("No se pudo agregar carpeta al ZIP (puede que ya exista): {}", e);
                    // Continuamos aunque falle crear el directorio (podría estar duplicado)
                }
            }
            
            // Agregar archivos de la carpeta al ZIP
            let files = match self.file_service.list_files(Some(&folder_id)).await {
                Ok(files) => files,
                Err(e) => {
                    error!("Error al listar archivos en carpeta {}: {}", folder_id, e);
                    return Err(ZipError::FolderContentsError(format!("Error al listar archivos: {}", e)).into());
                }
            };
            
            // Agregar cada archivo al ZIP
            for file in files {
                self.add_file_to_zip(zip, &file, &folder_path, options).await?;
            }
            
            // Procesar subcarpetas
            let subfolders = match self.folder_service.list_folders(Some(&folder_id)).await {
                Ok(folders) => folders,
                Err(e) => {
                    error!("Error al listar subcarpetas en {}: {}", folder_id, e);
                    return Err(ZipError::FolderContentsError(format!("Error al listar subcarpetas: {}", e)).into());
                }
            };
            
            // Agregar subcarpetas a la cola
            for subfolder in subfolders {
                let subfolder_path = format!("{}/{}", current.path, subfolder.name);
                work_queue.push(PendingFolder {
                    folder: subfolder,
                    path: subfolder_path,
                });
            }
        }
        
        Ok(())
    }
    
    // Agrega un archivo al ZIP
    async fn add_file_to_zip(
        &self,
        zip: &mut ZipWriter<Cursor<Vec<u8>>>,
        file: &FileDto,
        folder_path: &str,
        options: &SimpleFileOptions,
    ) -> Result<()> {
        let file_path = format!("{}{}", folder_path, file.name);
        info!("Agregando archivo al ZIP: {}", file_path);
        
        // Obtener el contenido del archivo
        let file_id = file.id.to_string();
        let content = match self.file_service.get_file_content(&file_id).await {
            Ok(content) => content,
            Err(e) => {
                error!("Error al leer contenido del archivo {}: {}", file_id, e);
                return Err(ZipError::FileReadError(format!("Error al leer archivo {}: {}", file_id, e)).into());
            }
        };
        
        // Escribir archivo al ZIP
        match zip.start_file_from_path(std::path::Path::new(&file_path), *options) {
            Ok(_) => {
                match zip.write_all(&content) {
                    Ok(_) => {
                        debug!("Archivo agregado al ZIP: {}", file_path);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Error al escribir contenido del archivo {}: {}", file_path, e);
                        Err(ZipError::IoError(e).into())
                    }
                }
            },
            Err(e) => {
                error!("Error al iniciar archivo en ZIP {}: {}", file_path, e);
                Err(ZipError::ZipError(e).into())
            }
        }
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

#[async_trait]
impl ZipPort for ZipService {
    async fn create_folder_zip(
        &self,
        folder_id: &str,
        folder_name: &str,
    ) -> std::result::Result<Vec<u8>, DomainError> {
        self.create_folder_zip(folder_id, folder_name).await
    }
}