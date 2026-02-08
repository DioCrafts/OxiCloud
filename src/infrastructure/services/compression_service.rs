use std::io::{Read};
use std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tracing::error;
use std::io;
use flate2::Compression;
use flate2::read::GzEncoder as GzEncoderRead;
use flate2::bufread::GzDecoder;

use crate::application::ports::compression_ports::{
    CompressionPort,
    CompressionLevel as PortCompressionLevel,
};
use crate::domain::errors::DomainError;
use crate::infrastructure::services::buffer_pool::BufferPool;

/// Nivel de compresión para ficheros
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// Sin compresión (solo para transferencia)
    None = 0,
    /// Compresión rápida con menor ratio
    Fast = 1,
    /// Compresión balanceada (por defecto)
    Default = 6,
    /// Compresión máxima (más lenta)
    Best = 9,
}

impl From<CompressionLevel> for Compression {
    fn from(level: CompressionLevel) -> Self {
        match level {
            CompressionLevel::None => Compression::none(),
            CompressionLevel::Fast => Compression::fast(),
            CompressionLevel::Default => Compression::default(),
            CompressionLevel::Best => Compression::best(),
        }
    }
}

/// Umbral de tamaño para decidir si se comprime o no
const COMPRESSION_SIZE_THRESHOLD: u64 = 1024 * 50; // 50KB

/// Interfaz para servicios de compresión
#[async_trait]
pub trait CompressionService: Send + Sync {
    /// Comprime datos en memoria
    async fn compress_data(&self, data: &[u8], level: CompressionLevel) -> io::Result<Vec<u8>>;
    
    /// Descomprime datos en memoria
    async fn decompress_data(&self, compressed_data: &[u8]) -> io::Result<Vec<u8>>;
    
    /// Comprime un stream de datos
    fn compress_stream<S>(&self, stream: S, level: CompressionLevel) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin;
    
    /// Descomprime un stream de datos
    fn decompress_stream<S>(&self, compressed_stream: S) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin;
    
    /// Determina si un archivo debe ser comprimido basado en su tipo MIME y tamaño
    fn should_compress(&self, mime_type: &str, size: u64) -> bool;
}

/// Implementación de servicios de compresión usando Gzip
pub struct GzipCompressionService {
    /// Pool de buffers para optimización de memoria
    buffer_pool: Option<Arc<BufferPool>>,
}

impl GzipCompressionService {
    /// Crea una nueva instancia del servicio
    pub fn new() -> Self {
        Self {
            buffer_pool: None,
        }
    }
    
    /// Crea una nueva instancia del servicio con buffer pool
    pub fn new_with_buffer_pool(buffer_pool: Arc<BufferPool>) -> Self {
        Self {
            buffer_pool: Some(buffer_pool),
        }
    }
}

#[async_trait]
impl CompressionService for GzipCompressionService {
    /// Comprime datos en memoria usando Gzip
    async fn compress_data(&self, data: &[u8], level: CompressionLevel) -> io::Result<Vec<u8>> {
        // Si tenemos un buffer pool, usar un buffer prestado para la compresión
        if let Some(pool) = &self.buffer_pool {
            // Estimar el tamaño de la compresión (aproximadamente 80% del original para casos típicos)
            let estimated_size = (data.len() as f64 * 0.8) as usize;
            
            // Obtener un buffer del pool
            let buffer = pool.get_buffer().await;
            
            // Comprobar si el buffer es suficientemente grande
            if buffer.capacity() >= estimated_size {
                // Ejecutar la compresión en un worker thread usando el buffer
                let buffer_ptr = Arc::new(tokio::sync::Mutex::new(buffer));
                let buffer_clone = buffer_ptr.clone();
                
                // Comprimir datos
                // Clonar los datos para evitar problemas de lifetime
                let data_owned = data.to_vec();
                
                let result = tokio::task::spawn_blocking(move || {
                    let mut encoder = GzEncoderRead::new(&data_owned[..], level.into());
                    
                    // Intentar bloquear el mutex (no debería fallar ya que estamos en un hilo separado)
                    let mut buffer_guard = match futures::executor::block_on(buffer_clone.lock()) {
                        buffer => buffer,
                    };
                    
                    // Leer directamente en el buffer
                    let read_bytes = encoder.read(buffer_guard.as_mut_slice())?;
                    buffer_guard.set_used(read_bytes);
                    
                    Ok(()) as io::Result<()>
                }).await;
                
                // Verificar resultado
                match result {
                    Ok(Ok(())) => {
                        // Obtener el buffer y convertirlo a Vec<u8>
                        let buffer = buffer_ptr.lock().await;
                        let cloned_buffer = buffer.clone();
                        drop(buffer); // Liberar el mutex primero
                        return Ok(cloned_buffer.into_vec());
                    },
                    Ok(Err(e)) => {
                        error!("Error en compresión con buffer pool: {}", e);
                        // Continuar con implementación estándar
                    },
                    Err(e) => {
                        error!("Error en task de compresión con buffer pool: {}", e);
                        // Continuar con implementación estándar
                    }
                }
            }
        }
        
        // Implementación estándar si no hay buffer pool o el buffer es insuficiente
        // Clonar los datos para evitar problemas de lifetime
        let data_owned = data.to_vec();
        
        tokio::task::spawn_blocking(move || {
            let mut encoder = GzEncoderRead::new(&data_owned[..], level.into());
            let mut compressed = Vec::new();
            encoder.read_to_end(&mut compressed)?;
            Ok(compressed)
        }).await.unwrap_or_else(|e| {
            error!("Error en task de compresión: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
    }
    
    /// Descomprime datos en memoria
    async fn decompress_data(&self, compressed_data: &[u8]) -> io::Result<Vec<u8>> {
        // Si tenemos un buffer pool, usar un buffer prestado para la descompresión
        if let Some(pool) = &self.buffer_pool {
            // Estimar el tamaño de la descompresión (aproximadamente 5x del comprimido para casos típicos)
            let estimated_size = compressed_data.len() * 5;
            
            // Obtener un buffer del pool
            let buffer = pool.get_buffer().await;
            
            // Comprobar si el buffer es suficientemente grande
            if buffer.capacity() >= estimated_size {
                // Clonar datos comprimidos para mover al worker
                let data = compressed_data.to_vec();
                let buffer_ptr = Arc::new(tokio::sync::Mutex::new(buffer));
                let buffer_clone = buffer_ptr.clone();
                
                // Descomprimir datos
                let result = tokio::task::spawn_blocking(move || {
                    let mut decoder = GzDecoder::new(&data[..]);
                    
                    // Intentar bloquear el mutex
                    let mut buffer_guard = match futures::executor::block_on(buffer_clone.lock()) {
                        buffer => buffer,
                    };
                    
                    // Leer directamente en el buffer
                    let read_bytes = decoder.read(buffer_guard.as_mut_slice())?;
                    buffer_guard.set_used(read_bytes);
                    
                    Ok(()) as io::Result<()>
                }).await;
                
                // Verificar resultado
                match result {
                    Ok(Ok(())) => {
                        // Obtener el buffer y convertirlo a Vec<u8>
                        let buffer = buffer_ptr.lock().await;
                        let cloned_buffer = buffer.clone();
                        drop(buffer); // Liberar el mutex primero
                        return Ok(cloned_buffer.into_vec());
                    },
                    Ok(Err(e)) => {
                        error!("Error en descompresión con buffer pool: {}", e);
                        // Continuar con implementación estándar
                    },
                    Err(e) => {
                        error!("Error en task de descompresión con buffer pool: {}", e);
                        // Continuar con implementación estándar
                    }
                }
            }
        }
        
        // Implementación estándar si no hay buffer pool o el buffer es insuficiente
        let data = compressed_data.to_vec(); // Clonar para mover al worker
        tokio::task::spawn_blocking(move || {
            let mut decoder = GzDecoder::new(&data[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }).await.unwrap_or_else(|e| {
            error!("Error en task de descompresión: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
    }
    
    /// Comprime un stream de bytes
    fn compress_stream<S>(&self, stream: S, level: CompressionLevel) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin
    {
        // For now, simplify the implementation to avoid complex pinning issues
        // This implementation collects all stream data and then compresses it at once
        // Future optimization would be to implement true streaming compression
        let compression_level = level;
        
        Box::pin(async_stream::stream! {
            let mut data = Vec::new();
            
            // Collect all bytes from the stream
            let mut stream = Box::pin(stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        data.extend_from_slice(&bytes);
                    },
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }
            
            // Compress collected data
            match CompressionService::compress_data(self, &data, compression_level).await {
                Ok(compressed) => {
                    // Return compressed data as a single chunk
                    yield Ok(Bytes::from(compressed));
                },
                Err(e) => {
                    yield Err(e);
                }
            }
        })
    }
    
    /// Descomprime un stream de bytes
    fn decompress_stream<S>(&self, compressed_stream: S) 
        -> impl Stream<Item = io::Result<Bytes>> + Send
    where
        S: Stream<Item = io::Result<Bytes>> + Send + 'static + Unpin
    {
        // For now, simplify the implementation to avoid complex pinning issues
        // This implementation collects all stream data and then decompresses it at once
        // Future optimization would be to implement streaming decompression correctly
        Box::pin(async_stream::stream! {
            let mut compressed_data = Vec::new();
            
            // Collect all bytes from the stream
            let mut stream = Box::pin(compressed_stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        compressed_data.extend_from_slice(&bytes);
                    },
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }
            
            // Decompress collected data
            match CompressionService::decompress_data(self, &compressed_data).await {
                Ok(decompressed) => {
                    // Return decompressed data as a single chunk
                    yield Ok(Bytes::from(decompressed));
                },
                Err(e) => {
                    yield Err(e);
                }
            }
        })
    }
    
    /// Determina si un archivo debe ser comprimido basado en su tipo MIME y tamaño
    fn should_compress(&self, mime_type: &str, size: u64) -> bool {
        // No comprimir archivos muy pequeños (overhead)
        if size < COMPRESSION_SIZE_THRESHOLD {
            return false;
        }
        
        // No comprimir archivos ya comprimidos
        if mime_type.starts_with("image/")
            && !mime_type.contains("svg")
            && !mime_type.contains("bmp") {
            return false;
        }
        
        if mime_type.starts_with("audio/") 
            || mime_type.starts_with("video/") 
            || mime_type.contains("zip")
            || mime_type.contains("gzip")
            || mime_type.contains("compressed")
            || mime_type.contains("7z")
            || mime_type.contains("rar")
            || mime_type.contains("bz2")
            || mime_type.contains("xz")
            || mime_type.contains("jpg")
            || mime_type.contains("jpeg")
            || mime_type.contains("png")
            || mime_type.contains("gif")
            || mime_type.contains("webp")
            || mime_type.contains("mp3")
            || mime_type.contains("mp4")
            || mime_type.contains("ogg")
            || mime_type.contains("webm") {
            return false;
        }
        
        // Comprimir archivos de texto, documentos, y otros tipos compresibles
        true
    }
}

// ─── Port implementation ─────────────────────────────────────────────────────

/// Convert application-layer CompressionLevel to infrastructure CompressionLevel.
impl From<PortCompressionLevel> for CompressionLevel {
    fn from(level: PortCompressionLevel) -> Self {
        match level {
            PortCompressionLevel::None => CompressionLevel::None,
            PortCompressionLevel::Fast => CompressionLevel::Fast,
            PortCompressionLevel::Default => CompressionLevel::Default,
            PortCompressionLevel::Best => CompressionLevel::Best,
        }
    }
}

#[async_trait]
impl CompressionPort for GzipCompressionService {
    async fn compress_data(&self, data: &[u8], level: PortCompressionLevel) -> Result<Vec<u8>, DomainError> {
        CompressionService::compress_data(self, data, level.into()).await.map_err(DomainError::from)
    }

    async fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>, DomainError> {
        CompressionService::decompress_data(self, compressed_data).await.map_err(DomainError::from)
    }

    fn should_compress(&self, mime_type: &str, size: u64) -> bool {
        CompressionService::should_compress(self, mime_type, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;
    use futures::TryStreamExt;
    
    #[tokio::test]
    async fn test_compress_decompress_data() {
        let service = GzipCompressionService::new();
        
        // Datos de prueba
        let data = "Hello, world! ".repeat(1000).into_bytes();
        
        // Comprimir
        let compressed = CompressionService::compress_data(&service, &data, CompressionLevel::Default).await.unwrap();
        
        // Verificar que la compresión reduce el tamaño
        assert!(compressed.len() < data.len());
        
        // Descomprimir
        let decompressed = CompressionService::decompress_data(&service, &compressed).await.unwrap();
        
        // Verificar que los datos originales se recuperan correctamente
        assert_eq!(decompressed, data);
    }
    
    #[tokio::test]
    async fn test_compress_decompress_stream() {
        let service = GzipCompressionService::new();
        
        // Crear datos de prueba
        let chunks = vec![
            Ok(Bytes::from("Hello, ")),
            Ok(Bytes::from("world! ")),
            Ok(Bytes::from("This is a test of streaming compression.")),
        ];
        
        // Convertir a stream
        let input_stream = futures::stream::iter(chunks);
        
        // Comprimir el stream
        let compressed_stream = service.compress_stream(input_stream, CompressionLevel::Default);
        
        // Recolectar los bytes comprimidos
        let compressed_bytes = compressed_stream
            .try_fold(Vec::new(), |mut acc, chunk| async move {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            }).await.unwrap();
        
        // Descomprimir los datos
        let decompressed = CompressionService::decompress_data(&service, &compressed_bytes).await.unwrap();
        
        // Verificar resultado
        let expected = "Hello, world! This is a test of streaming compression.";
        assert_eq!(String::from_utf8(decompressed).unwrap(), expected);
    }
    
    #[test]
    fn test_should_compress() {
        let service = GzipCompressionService::new();
        
        // Casos que no deberían comprimirse
        assert!(!CompressionService::should_compress(&service, "image/jpeg", 100 * 1024));
        assert!(!CompressionService::should_compress(&service, "video/mp4", 10 * 1024 * 1024));
        assert!(!CompressionService::should_compress(&service, "application/zip", 5 * 1024 * 1024));
        
        // Casos que sí deberían comprimirse
        assert!(CompressionService::should_compress(&service, "text/html", 100 * 1024));
        assert!(CompressionService::should_compress(&service, "application/json", 200 * 1024));
        assert!(CompressionService::should_compress(&service, "text/plain", 1024 * 1024));
        
        // Archivos pequeños no deberían comprimirse independientemente del tipo
        assert!(!CompressionService::should_compress(&service, "text/html", 10 * 1024));
    }
}