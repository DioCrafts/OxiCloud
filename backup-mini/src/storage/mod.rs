//! Capa de almacenamiento de OxiCloud
//! 
//! Este módulo proporciona abstracciones sobre el sistema de almacenamiento físico
//! y permite implementar diferentes backends de almacenamiento.

pub mod disk;
pub mod models;

use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;

/// Definición de un proveedor de almacenamiento
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Inicializa el proveedor de almacenamiento
    async fn initialize(&self) -> Result<()>;
    
    /// Guarda un archivo en el almacenamiento
    async fn save_file<R>(&self, file_id: Uuid, reader: &mut R, size: u64) -> Result<()>
    where
        R: AsyncRead + Unpin + Send;
    
    /// Lee un archivo del almacenamiento
    async fn read_file<W>(&self, file_id: Uuid, writer: &mut W) -> Result<u64>
    where
        W: AsyncWrite + Unpin + Send;
    
    /// Elimina un archivo del almacenamiento
    async fn delete_file(&self, file_id: Uuid) -> Result<()>;
    
    /// Comprueba si un archivo existe en el almacenamiento
    async fn file_exists(&self, file_id: Uuid) -> Result<bool>;
    
    /// Obtiene la ruta física de un archivo (si aplica al proveedor)
    async fn get_file_path(&self, file_id: Uuid) -> Option<String>;
    
    /// Calcula el espacio total ocupado por el almacenamiento
    async fn get_total_size(&self) -> Result<u64>;
}