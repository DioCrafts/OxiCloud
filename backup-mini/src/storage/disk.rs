//! Implementación de almacenamiento en disco local
//!
//! Este proveedor almacena los archivos en el sistema de archivos local
//! con una estructura organizada.

use anyhow::{Result, Context};
use async_trait::async_trait;
use log::{info, warn, error};
use std::path::{Path, PathBuf};
use tokio::{
    fs,
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};
use uuid::Uuid;

use crate::core::config::get_config;
use super::StorageProvider;

/// Proveedor de almacenamiento en disco local
pub struct DiskStorage {
    /// Directorio raíz donde se almacenan los archivos
    root_dir: PathBuf,
    /// Tamaño máximo de buffer para operaciones de lectura/escritura
    buffer_size: usize,
}

impl DiskStorage {
    /// Crea una nueva instancia del almacenamiento en disco
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
            buffer_size: 64 * 1024, // 64 KB por defecto
        }
    }
    
    /// Crea una instancia usando la configuración global
    pub fn from_config() -> Self {
        let config = get_config();
        Self::new(Path::new(&config.storage.data_dir).join("files"))
    }
    
    /// Establece el tamaño del buffer
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }
    
    /// Obtiene la ruta completa para un archivo
    fn get_file_path(&self, file_id: Uuid) -> PathBuf {
        self.root_dir.join(file_id.to_string())
    }
    
    /// Obtiene la ruta para un archivo temporal durante la carga
    fn get_temp_path(&self, file_id: Uuid) -> PathBuf {
        self.root_dir.join(format!("{}.tmp", file_id))
    }
}

#[async_trait]
impl StorageProvider for DiskStorage {
    async fn initialize(&self) -> Result<()> {
        // Asegurarse de que el directorio raíz existe
        if !self.root_dir.exists() {
            fs::create_dir_all(&self.root_dir).await
                .context("Error al crear directorio de almacenamiento")?;
            info!("Directorio de almacenamiento creado: {}", self.root_dir.display());
        }
        Ok(())
    }
    
    async fn save_file<R>(&self, file_id: Uuid, reader: &mut R, size: u64) -> Result<()>
    where
        R: AsyncRead + Unpin + Send,
    {
        // Asegurarse de que el almacenamiento está inicializado
        self.initialize().await?;
        
        // Obtener rutas
        let file_path = self.get_file_path(file_id);
        let temp_path = self.get_temp_path(file_id);
        
        // Crear archivo temporal
        let mut file = fs::File::create(&temp_path).await
            .context("Error al crear archivo temporal")?;
        
        // Copiar datos al archivo
        let mut buffer = vec![0u8; self.buffer_size];
        let mut bytes_written = 0u64;
        
        loop {
            let bytes_read = reader.read(&mut buffer).await
                .context("Error al leer datos del origen")?;
            
            if bytes_read == 0 {
                break;
            }
            
            file.write_all(&buffer[..bytes_read]).await
                .context("Error al escribir datos al archivo")?;
            
            bytes_written += bytes_read as u64;
        }
        
        // Cerrar archivo
        file.flush().await.context("Error al finalizar escritura")?;
        drop(file);
        
        // Verificar tamaño
        if size > 0 && bytes_written != size {
            // Si hay discrepancia, eliminar el archivo y lanzar error
            let _ = fs::remove_file(&temp_path).await;
            anyhow::bail!("El tamaño del archivo no coincide: esperado {} bytes, escrito {} bytes", size, bytes_written);
        }
        
        // Mover archivo temporal a ubicación final
        fs::rename(&temp_path, &file_path).await
            .context("Error al mover archivo temporal a ubicación final")?;
        
        info!("Archivo guardado: {} ({} bytes)", file_id, bytes_written);
        Ok(())
    }
    
    async fn read_file<W>(&self, file_id: Uuid, writer: &mut W) -> Result<u64>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let file_path = self.get_file_path(file_id);
        
        // Verificar que el archivo existe
        if !file_path.exists() {
            anyhow::bail!("Archivo no encontrado: {}", file_id);
        }
        
        // Abrir archivo
        let mut file = fs::File::open(&file_path).await
            .context("Error al abrir archivo para lectura")?;
        
        // Copiar datos al writer
        let mut buffer = vec![0u8; self.buffer_size];
        let mut bytes_read_total = 0u64;
        
        loop {
            let bytes_read = file.read(&mut buffer).await
                .context("Error al leer datos del archivo")?;
            
            if bytes_read == 0 {
                break;
            }
            
            writer.write_all(&buffer[..bytes_read]).await
                .context("Error al escribir datos al destino")?;
            
            bytes_read_total += bytes_read as u64;
        }
        
        writer.flush().await.context("Error al finalizar escritura")?;
        
        Ok(bytes_read_total)
    }
    
    async fn delete_file(&self, file_id: Uuid) -> Result<()> {
        let file_path = self.get_file_path(file_id);
        
        // Verificar que el archivo existe
        if !file_path.exists() {
            // No es un error si el archivo no existe
            return Ok(());
        }
        
        // Eliminar archivo
        fs::remove_file(&file_path).await
            .context("Error al eliminar archivo")?;
        
        info!("Archivo eliminado: {}", file_id);
        Ok(())
    }
    
    async fn file_exists(&self, file_id: Uuid) -> Result<bool> {
        let file_path = self.get_file_path(file_id);
        Ok(file_path.exists())
    }
    
    async fn get_file_path(&self, file_id: Uuid) -> Option<String> {
        let file_path = self.get_file_path(file_id);
        file_path.to_str().map(|s| s.to_string())
    }
    
    async fn get_total_size(&self) -> Result<u64> {
        // Esta operación puede ser costosa para directorios grandes
        let mut total_size = 0u64;
        
        let mut entries = fs::read_dir(&self.root_dir).await
            .context("Error al leer directorio de almacenamiento")?;
        
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                total_size += metadata.len();
            }
        }
        
        Ok(total_size)
    }
}

/// Crea e inicializa un proveedor de almacenamiento en disco
pub async fn initialize_disk_storage() -> Result<DiskStorage> {
    let storage = DiskStorage::from_config();
    storage.initialize().await?;
    Ok(storage)
}