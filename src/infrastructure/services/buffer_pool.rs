use std::cmp::min;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use std::time::{Duration, Instant};
use tracing::debug;

/// Tamaño por defecto de los buffers en el pool
pub const DEFAULT_BUFFER_SIZE: usize = 64 * 1024; // 64KB

/// Número máximo por defecto de buffers en el pool
pub const DEFAULT_MAX_BUFFERS: usize = 100;

/// Tiempo de vida por defecto de un buffer inactivo (en segundos)
pub const DEFAULT_BUFFER_TTL: u64 = 60;

/// Buffer pooling para optimizar operaciones de lectura/escritura
pub struct BufferPool {
    /// Pool de buffers disponibles
    pool: Mutex<VecDeque<PooledBuffer>>,
    /// Semáforo para limitar el número máximo de buffers
    limit: Semaphore,
    /// Tamaño de los buffers en el pool
    buffer_size: usize,
    /// Estadísticas del pool
    stats: Mutex<BufferPoolStats>,
    /// Tiempo de vida de un buffer inactivo
    buffer_ttl: Duration,
}

/// Estructura para tracking de estadísticas del pool
#[derive(Debug, Clone, Default)]
pub struct BufferPoolStats {
    /// Número total de operaciones de get
    pub gets: usize,
    /// Número de hits del pool (reutilización exitosa)
    pub hits: usize,
    /// Número de misses (creación de nuevo buffer)
    pub misses: usize,
    /// Número de retornos al pool
    pub returns: usize,
    /// Número de eviction por TTL
    pub evictions: usize,
    /// Número máximo de buffers alcanzado
    pub max_buffers_reached: usize,
    /// Esperas por semáforo
    pub waits: usize,
}

/// Buffer del pool con metadatos para gestión
struct PooledBuffer {
    /// Buffer real de bytes
    buffer: Vec<u8>,
    /// Timestamp de cuándo se añadió/retornó al pool
    last_used: Instant,
}

/// Buffer prestado del pool con cleanup automático
#[derive(Clone)]
pub struct BorrowedBuffer {
    /// Buffer actual
    buffer: Vec<u8>,
    /// Tamaño real utilizado del buffer
    used_size: usize,
    /// Referencia al pool para retornar
    pool: Arc<BufferPool>,
    /// Si el buffer debe o no retornarse al pool
    return_to_pool: bool,
}

impl BufferPool {
    /// Crea un nuevo pool de buffers
    pub fn new(buffer_size: usize, max_buffers: usize, buffer_ttl_secs: u64) -> Arc<Self> {
        Arc::new(Self {
            pool: Mutex::new(VecDeque::with_capacity(max_buffers)),
            limit: Semaphore::new(max_buffers),
            buffer_size,
            stats: Mutex::new(BufferPoolStats::default()),
            buffer_ttl: Duration::from_secs(buffer_ttl_secs),
        })
    }
    
    /// Crea un pool con configuración por defecto
    pub fn default() -> Arc<Self> {
        Self::new(
            DEFAULT_BUFFER_SIZE,
            DEFAULT_MAX_BUFFERS,
            DEFAULT_BUFFER_TTL
        )
    }
    
    /// Obtiene un buffer del pool o crea uno nuevo si es necesario.
    /// This version takes an Arc<Self> to ensure the BorrowedBuffer keeps a proper
    /// reference to the shared pool (not a clone).
    #[allow(unused_variables)]
    pub async fn get_buffer(self: &Arc<Self>) -> BorrowedBuffer {
        // Incrementar contador de gets
        {
            let mut stats = self.stats.lock().await;
            stats.gets += 1;
        }
        
        // Control de concurrencia
        // Acquire a semaphore permit. If none available, wait.
        // We forget() the permit so it doesn't auto-release on drop.
        // Instead, the permit is manually released in return_buffer/Drop via add_permits(1).
        match self.limit.try_acquire() {
            Ok(permit) => permit.forget(),
            Err(_) => {
                // No hay permisos disponibles, esperamos
                {
                    let mut stats = self.stats.lock().await;
                    stats.waits += 1;
                    stats.max_buffers_reached += 1;
                }
                
                debug!("Buffer pool: waiting for available buffer");
                let permit = self.limit.acquire().await.expect("Semaphore should not be closed");
                debug!("Buffer pool: acquired buffer after waiting");
                permit.forget();
            }
        };
        
        // Intentar obtener un buffer existente del pool
        let mut pool_locked = self.pool.lock().await;
        
        let pool_arc = Arc::clone(self);
        
        if let Some(mut pooled_buffer) = pool_locked.pop_front() {
            // Verificar si el buffer ha expirado
            if pooled_buffer.last_used.elapsed() > self.buffer_ttl {
                // Buffer expirado, descartamos y creamos uno nuevo
                let mut stats = self.stats.lock().await;
                stats.evictions += 1;
                stats.misses += 1;
                drop(stats);
                
                debug!("Buffer pool: evicted expired buffer");
                
                // Crear nuevo buffer (reutilizando el permiso)
                drop(pool_locked); // Liberar el lock antes de retornar
                
                BorrowedBuffer {
                    buffer: vec![0; self.buffer_size],
                    used_size: 0,
                    pool: pool_arc,
                    return_to_pool: true,
                }
            } else {
                // Buffer válido, lo reutilizamos
                let mut stats = self.stats.lock().await;
                stats.hits += 1;
                drop(stats);
                
                // Liberar el lock antes de retornar
                drop(pool_locked);
                
                // Limpiar buffer por seguridad
                pooled_buffer.buffer.fill(0);
                
                BorrowedBuffer {
                    buffer: pooled_buffer.buffer,
                    used_size: 0,
                    pool: pool_arc,
                    return_to_pool: true,
                }
            }
        } else {
            // No hay buffers disponibles, creamos uno nuevo
            let mut stats = self.stats.lock().await;
            stats.misses += 1;
            drop(stats);
            
            // Liberar el lock antes de retornar
            drop(pool_locked);
            
            debug!("Buffer pool: creating new buffer");
            
            BorrowedBuffer {
                buffer: vec![0; self.buffer_size],
                used_size: 0,
                pool: pool_arc,
                return_to_pool: true,
            }
        }
    }
    
    /// Retorna un buffer al pool
    async fn return_buffer(&self, mut buffer: Vec<u8>) {
        // Si el buffer es del tamaño incorrecto, lo descartamos
        if buffer.capacity() != self.buffer_size {
            debug!("Buffer pool: discarding buffer of wrong size: {} (expected {})", 
                 buffer.capacity(), self.buffer_size);
            // Release the semaphore permit even if we discard the buffer
            self.limit.add_permits(1);
            return;
        }
        
        // Resize para asegurar capacidad correcta
        buffer.resize(self.buffer_size, 0);
        
        // Añadir al pool
        let mut pool_locked = self.pool.lock().await;
        
        pool_locked.push_back(PooledBuffer {
            buffer,
            last_used: Instant::now(),
        });
        
        // Actualizar estadísticas
        let mut stats = self.stats.lock().await;
        stats.returns += 1;
        
        // Release the semaphore permit so another caller can acquire a buffer
        drop(pool_locked);
        drop(stats);
        self.limit.add_permits(1);
    }
    
    /// Limpia buffers expirados del pool
    pub async fn clean_expired_buffers(&self) {
        let _now = Instant::now();
        let mut pool_locked = self.pool.lock().await;
        
        // Contar expirados
        let count_before = pool_locked.len();
        
        // Filtrar manteniendo solo los no expirados
        pool_locked.retain(|buffer| {
            buffer.last_used.elapsed() <= self.buffer_ttl
        });
        
        // Contar cuántos se eliminaron
        let removed = count_before - pool_locked.len();
        
        if removed > 0 {
            // Actualizar estadísticas
            let mut stats = self.stats.lock().await;
            stats.evictions += removed;
            
            debug!("Buffer pool: cleaned {} expired buffers", removed);
        }
    }
    
    /// Obtiene estadísticas actuales del pool
    pub async fn get_stats(&self) -> BufferPoolStats {
        self.stats.lock().await.clone()
    }
    
    /// Inicia la tarea periódica de limpieza
    pub fn start_cleaner(pool: Arc<Self>) {
        tokio::spawn(async move {
            let interval = Duration::from_secs(30); // Limpiar cada 30 segundos
            
            loop {
                tokio::time::sleep(interval).await;
                pool.clean_expired_buffers().await;
                
                // Loguear estadísticas periódicamente
                let stats = pool.get_stats().await;
                debug!("Buffer pool stats: gets={}, hits={}, misses={}, hit_ratio={:.2}%, returns={}, \
                      evictions={}, max_reached={}, waits={}",
                     stats.gets, 
                     stats.hits, 
                     stats.misses,
                     if stats.gets > 0 { (stats.hits as f64 * 100.0) / stats.gets as f64 } else { 0.0 },
                     stats.returns,
                     stats.evictions,
                     stats.max_buffers_reached,
                     stats.waits);
            }
        });
    }
}

impl Clone for BufferPool {
    fn clone(&self) -> Self {
        Self {
            pool: Mutex::new(VecDeque::new()),
            limit: Semaphore::new(self.limit.available_permits()),
            buffer_size: self.buffer_size,
            stats: Mutex::new(BufferPoolStats::default()),
            buffer_ttl: self.buffer_ttl,
        }
    }
}

impl BorrowedBuffer {
    /// Accede al buffer interno
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    
    /// Obtiene una referencia a los datos utilizados
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.used_size]
    }
    
    /// Establece cuántos bytes se utilizaron realmente
    pub fn set_used(&mut self, size: usize) {
        self.used_size = min(size, self.buffer.len());
    }
    
    /// Convierte en un Vec<u8> que incluye solo los datos utilizados
    pub fn into_vec(mut self) -> Vec<u8> {
        // Marcar para no devolver al pool
        self.return_to_pool = false;
        
        // Crear un nuevo vector solo con los datos utilizados
        self.buffer[..self.used_size].to_vec()
    }
    
    /// Copia datos a este buffer y actualiza el tamaño usado
    pub fn copy_from_slice(&mut self, data: &[u8]) -> usize {
        let copy_size = min(data.len(), self.buffer.len());
        self.buffer[..copy_size].copy_from_slice(&data[..copy_size]);
        self.used_size = copy_size;
        copy_size
    }
    
    /// Impide que el buffer se devuelva al pool al destruirse
    pub fn do_not_return(mut self) -> Self {
        self.return_to_pool = false;
        self
    }
    
    /// Obtiene el tamaño total del buffer
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
    
    /// Obtiene el tamaño usado del buffer
    pub fn used_size(&self) -> usize {
        self.used_size
    }
}

// Cuando se hace drop de un BorrowedBuffer, lo devuelve al pool
impl Drop for BorrowedBuffer {
    fn drop(&mut self) {
        if self.return_to_pool {
            // Tomar posesión del buffer y crear un clone del pool
            let buffer = std::mem::take(&mut self.buffer);
            let pool = self.pool.clone();
            
            // Spawn del return para que el drop no bloquee
            // return_buffer will release the semaphore permit
            tokio::spawn(async move {
                pool.return_buffer(buffer).await;
            });
        } else {
            // Buffer not returned to pool, but we still need to release the semaphore permit
            self.pool.limit.add_permits(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_buffer_pooling() {
        // Crear pool pequeño para testing
        let pool = BufferPool::new(1024, 5, 60);
        
        // Obtener un buffer
        let mut buffer1 = pool.get_buffer().await;
        buffer1.copy_from_slice(b"test data");
        assert_eq!(buffer1.as_slice(), b"test data");
        
        // Obtener otro buffer
        let buffer2 = pool.get_buffer().await;
        
        // Verificar stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 2);
        assert_eq!(stats.hits, 0); // sin hits todavía
        assert_eq!(stats.misses, 2); // todos son misses
        
        // Devolver buffer1 al pool (implícitamente por drop)
        drop(buffer1);
        
        // Permitir que el return asíncrono ocurra
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Obtener otro buffer (debería reutilizar el retornado)
        let buffer3 = pool.get_buffer().await;
        
        // Verificar stats actualizados
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 3);
        assert_eq!(stats.hits, 1); // ahora debería haber un hit
        assert_eq!(stats.returns, 1); // un buffer retornado
        
        // Limpiar
        drop(buffer2);
        drop(buffer3);
    }
    
    #[tokio::test]
    async fn test_buffer_operations() {
        let pool = BufferPool::new(1024, 10, 60);
        
        // Obtener buffer
        let mut buffer = pool.get_buffer().await;
        
        // Escribir datos
        buffer.copy_from_slice(b"Hello, world!");
        assert_eq!(buffer.used_size(), 13);
        assert_eq!(buffer.as_slice(), b"Hello, world!");
        
        // Convertir a vec y verificar
        let vec = buffer.into_vec(); // Esto impide retornar al pool
        assert_eq!(vec, b"Hello, world!");
        
        // Verificar que no se incrementan los returns (buffer no retornado)
        tokio::time::sleep(Duration::from_millis(10)).await;
        let stats = pool.get_stats().await;
        assert_eq!(stats.returns, 0);
    }
    
    #[tokio::test]
    async fn test_pool_limit() {
        // Pool con solo 3 buffers
        let pool = BufferPool::new(1024, 3, 60);
        
        // Obtener 3 buffers (alcanza el límite)
        let buffer1 = pool.get_buffer().await;
        let buffer2 = pool.get_buffer().await;
        let buffer3 = pool.get_buffer().await;
        
        // Verificar stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.gets, 3);
        assert_eq!(stats.waits, 0); // sin esperas todavía
        
        // Intentar obtener un 4º buffer en una tarea separada (debería esperar)
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let _buffer4 = pool_clone.get_buffer().await;
            true
        });
        
        // Dar tiempo para que la tarea intente tomar el buffer
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verificar que hay una espera
        let stats = pool.get_stats().await;
        assert_eq!(stats.waits, 1);
        
        // Liberar un buffer
        drop(buffer1);
        
        // Dar tiempo para el retorno asíncrono y para que la tarea en espera obtenga su buffer
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verificar que la tarea pudo continuar
        assert!(handle.await.unwrap());
        
        // Limpiar
        drop(buffer2);
        drop(buffer3);
    }
    
    #[tokio::test]
    async fn test_ttl_expiration() {
        // Pool con TTL muy corto para testing
        let pool = BufferPool::new(1024, 5, 1); // 1 segundo TTL
        
        // Obtener y devolver un buffer
        let buffer = pool.get_buffer().await;
        drop(buffer);
        
        // Permitir que el return asíncrono ocurra
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Verificar que hay un buffer en el pool
        let stats = pool.get_stats().await;
        assert_eq!(stats.returns, 1);
        
        // Esperar a que expire el TTL
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Limpiar expirados
        pool.clean_expired_buffers().await;
        
        // Obtener otro buffer (debería ser un miss ya que el anterior expiró)
        let _buffer2 = pool.get_buffer().await;
        
        // Verificar stats
        let stats = pool.get_stats().await;
        assert_eq!(stats.evictions, 1); // un buffer expirado
        assert_eq!(stats.hits, 0); // sin hits (el buffer expiró)
        assert_eq!(stats.misses, 2); // dos misses (1er y 3er get)
    }
}