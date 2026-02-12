# 20 - Delta Sync Implementation

Delta sync transfers only the modified parts of a file instead of the whole thing. Based on the rsync algorithm, it can save 90-99% bandwidth in common scenarios.

**Status**: pending implementation
**Priority**: medium
**Estimated savings**: 10-100x less data transfer

## Contents

1. [Problem Statement](#problem-statement)
2. [How It Works](#how-it-works)
3. [Key Algorithms](#key-algorithms)
4. [Proposed Architecture](#proposed-architecture)
5. [Data Structures](#data-structures)
6. [API Endpoints](#api-endpoints)
7. [Step-by-Step Implementation](#step-by-step-implementation)
8. [Integration with Existing System](#integration-with-existing-system)
9. [Use Cases and Effectiveness](#use-cases-and-effectiveness)
10. [Performance Considerations](#performance-considerations)
11. [Testing](#testing)
12. [Required Dependencies](#required-dependencies)

---

## Key Benefits

| Metric | Without Delta Sync | With Delta Sync |
|---------|----------------|----------------|
| Edit 1 line in 100MB | 100MB transferred | ~1KB transferred |
| Sync time (slow connection) | 4+ minutes | <1 second |
| Bandwidth consumption | 100% | 0.1-10% |

---

## Problem Statement

### Current scenario (no delta sync)

```
User has document.docx (50MB) on OxiCloud
    │
    ▼
Downloads full file (50MB) ──────────────────────► 50MB ↓
    │
    ▼
Edits one word
    │
    ▼
Uploads full file again (50MB) ──────────────────► 50MB ↑
    │
    ▼
TOTAL: 100MB transferred to change one word
```

### Target scenario (with delta sync)

```
User has document.docx (50MB) on OxiCloud
    │
    ▼
Downloads full file (50MB) ──────────────────────► 50MB ↓ (first time)
    │
    ▼
Edits one word
    │
    ▼
Uploads ONLY modified blocks ───────────────────► ~50KB ↑
    │
    ▼
TOTAL: 50.05MB (99.9% savings on upload)
```

---

## How It Works

### Block (chunk) concept

The file gets divided into fixed-size blocks (typically 4KB-64KB):

```
Original file (server):
┌────────┬────────┬────────┬────────┬────────┐
│ Bloque │ Bloque │ Bloque │ Bloque │ Bloque │
│   0    │   1    │   2    │   3    │   4    │
│ 4KB    │ 4KB    │ 4KB    │ 4KB    │ 4KB    │
│        │        │        │        │        │
│ weak:A │ weak:B │ weak:C │ weak:D │ weak:E │
│ sha:X1 │ sha:X2 │ sha:X3 │ sha:X4 │ sha:X5 │
└────────┴────────┴────────┴────────┴────────┘

Modified file (client):
┌────────┬────────┬────────┬────────┬────────┐
│ Bloque │ Bloque │ Bloque │ Bloque │ Bloque │
│   0    │   1    │   2    │   3    │   4    │
│ 4KB    │ 4KB    │ 4KB    │ 4KB    │ 4KB    │
│        │        │        │        │        │
│ weak:A │ weak:B │ weak:F │ weak:D │ weak:E │  ← Block 2 changed
│ sha:X1 │ sha:X2 │ sha:Y3 │ sha:X4 │ sha:X5 │
└────────┴────────┴───▲────┴────────┴────────┘
                      │
              ONLY THIS ONE GETS TRANSFERRED
```

### Sync flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                     DELTA SYNC FLOW                                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  CLIENT                               SERVER                        │
│                                                                      │
│  1. Has modified                      1. Has original file           │
│     file                                 + block index               │
│                                                                      │
│  2. Requests signatures ─────────────►                              │
│     GET /files/{id}/signatures                                       │
│                                                                      │
│                       ◄────────────── 3. Returns signature list      │
│                                          [(weak, strong), ...]       │
│                                                                      │
│  4. Compares local blocks                                            │
│     against server                                                   │
│     signatures                                                       │
│                                                                      │
│  5. Generates delta ──────────────────►                              │
│     POST /files/{id}/delta                                           │
│     [References + New data]                                          │
│                                                                      │
│                                       6. Reconstructs file           │
│                                          by applying delta           │
│                                                                      │
│                       ◄────────────── 7. Confirms update             │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Key Algorithms

### 1. Rolling Checksum (modified Adler-32)

The rolling checksum computes the hash of a sliding window in O(1):

```rust
/// Rolling checksum para búsqueda rápida de bloques coincidentes
/// Similar al usado por rsync (Adler-32 modificado)
pub struct RollingChecksum {
    a: u32,  // Suma simple de bytes
    b: u32,  // Suma ponderada
    window_size: usize,
    buffer: VecDeque<u8>,
}

impl RollingChecksum {
    pub fn new(window_size: usize) -> Self {
        Self {
            a: 0,
            b: 0,
            window_size,
            buffer: VecDeque::with_capacity(window_size),
        }
    }
    
    /// Añadir un byte y calcular nuevo checksum
    /// Complejidad: O(1)
    pub fn roll(&mut self, new_byte: u8) -> u32 {
        if self.buffer.len() >= self.window_size {
            // Remover byte antiguo
            let old_byte = self.buffer.pop_front().unwrap() as u32;
            self.a = self.a.wrapping_sub(old_byte).wrapping_add(new_byte as u32);
            self.b = self.b.wrapping_sub(old_byte * self.window_size as u32)
                          .wrapping_add(self.a);
        } else {
            // Ventana no llena todavía
            self.a = self.a.wrapping_add(new_byte as u32);
            self.b = self.b.wrapping_add(self.a);
        }
        
        self.buffer.push_back(new_byte);
        self.checksum()
    }
    
    /// Calcular checksum actual
    pub fn checksum(&self) -> u32 {
        (self.b << 16) | (self.a & 0xFFFF)
    }
    
    /// Reset para nuevo archivo
    pub fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.buffer.clear();
    }
}
```

### 2. Block Signature

Each block carries two signatures:
- **Weak checksum** (32-bit) -- fast O(1) lookup
- **Strong hash** (SHA-256) -- definitive verification

```rust
/// Firma de un bloque para identificación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSignature {
    /// Índice del bloque en el archivo
    pub index: u32,
    /// Offset en bytes desde el inicio del archivo
    pub offset: u64,
    /// Tamaño del bloque (puede ser menor para el último)
    pub size: u32,
    /// Rolling checksum (32-bit) - búsqueda rápida
    pub weak_checksum: u32,
    /// SHA-256 hash (256-bit) - verificación definitiva
    pub strong_hash: [u8; 32],
}

/// Genera firmas para todos los bloques de un archivo
pub fn generate_signatures(data: &[u8], block_size: usize) -> Vec<BlockSignature> {
    let mut signatures = Vec::new();
    let mut offset = 0u64;
    let mut index = 0u32;
    
    for chunk in data.chunks(block_size) {
        // Weak checksum (rolling)
        let weak = adler32_checksum(chunk);
        
        // Strong hash (SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(chunk);
        let strong: [u8; 32] = hasher.finalize().into();
        
        signatures.push(BlockSignature {
            index,
            offset,
            size: chunk.len() as u32,
            weak_checksum: weak,
            strong_hash: strong,
        });
        
        offset += chunk.len() as u64;
        index += 1;
    }
    
    signatures
}

fn adler32_checksum(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    
    (b << 16) | a
}
```

### 3. Delta Generation

```rust
/// Instrucción de delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeltaInstruction {
    /// Copiar bloque existente del archivo original
    Copy {
        /// Índice del bloque en el archivo original
        block_index: u32,
    },
    /// Insertar datos literales nuevos
    Literal {
        /// Datos nuevos a insertar
        data: Vec<u8>,
    },
}

/// Delta completo para actualizar un archivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDelta {
    /// ID del archivo base
    pub base_file_id: String,
    /// Hash del archivo base (para verificación)
    pub base_file_hash: String,
    /// Nuevo tamaño del archivo
    pub new_size: u64,
    /// Instrucciones de delta
    pub instructions: Vec<DeltaInstruction>,
    /// Hash del archivo resultante (para verificación)
    pub result_hash: String,
}

/// Genera delta comparando archivo local con firmas remotas
pub fn generate_delta(
    local_data: &[u8],
    remote_signatures: &[BlockSignature],
    block_size: usize,
) -> FileDelta {
    // Crear índice de weak checksums para búsqueda O(1)
    let mut weak_index: HashMap<u32, Vec<&BlockSignature>> = HashMap::new();
    for sig in remote_signatures {
        weak_index.entry(sig.weak_checksum)
            .or_default()
            .push(sig);
    }
    
    let mut instructions = Vec::new();
    let mut rolling = RollingChecksum::new(block_size);
    let mut pos = 0;
    let mut literal_buffer = Vec::new();
    
    while pos < local_data.len() {
        // Calcular rolling checksum de la ventana actual
        let end = (pos + block_size).min(local_data.len());
        let window = &local_data[pos..end];
        
        let weak = if window.len() == block_size {
            rolling.reset();
            for &b in window {
                rolling.roll(b);
            }
            rolling.checksum()
        } else {
            adler32_checksum(window)
        };
        
        // Buscar coincidencia
        let mut found_match = false;
        
        if let Some(candidates) = weak_index.get(&weak) {
            // Verificar con strong hash
            let mut hasher = Sha256::new();
            hasher.update(window);
            let strong: [u8; 32] = hasher.finalize().into();
            
            for sig in candidates {
                if sig.strong_hash == strong && sig.size as usize == window.len() {
                    // ¡Coincidencia encontrada!
                    
                    // Flush literal buffer si hay datos pendientes
                    if !literal_buffer.is_empty() {
                        instructions.push(DeltaInstruction::Literal {
                            data: std::mem::take(&mut literal_buffer),
                        });
                    }
                    
                    // Añadir instrucción de copia
                    instructions.push(DeltaInstruction::Copy {
                        block_index: sig.index,
                    });
                    
                    pos += window.len();
                    found_match = true;
                    break;
                }
            }
        }
        
        if !found_match {
            // No hay coincidencia, añadir byte a literal buffer
            literal_buffer.push(local_data[pos]);
            pos += 1;
        }
    }
    
    // Flush remaining literal buffer
    if !literal_buffer.is_empty() {
        instructions.push(DeltaInstruction::Literal {
            data: literal_buffer,
        });
    }
    
    // Calcular hash del resultado
    let mut hasher = Sha256::new();
    hasher.update(local_data);
    let result_hash = hex::encode(hasher.finalize());
    
    FileDelta {
        base_file_id: String::new(), // Se llena al enviar
        base_file_hash: String::new(), // Se llena al enviar
        new_size: local_data.len() as u64,
        instructions,
        result_hash,
    }
}
```

### 4. Delta Application

```rust
/// Aplica delta a un archivo base para obtener el nuevo archivo
pub fn apply_delta(
    base_data: &[u8],
    signatures: &[BlockSignature],
    delta: &FileDelta,
    block_size: usize,
) -> Result<Vec<u8>, DeltaSyncError> {
    let mut result = Vec::with_capacity(delta.new_size as usize);
    
    for instruction in &delta.instructions {
        match instruction {
            DeltaInstruction::Copy { block_index } => {
                // Copiar bloque del archivo base
                let sig = signatures.get(*block_index as usize)
                    .ok_or(DeltaSyncError::InvalidBlockIndex(*block_index))?;
                
                let start = sig.offset as usize;
                let end = start + sig.size as usize;
                
                if end > base_data.len() {
                    return Err(DeltaSyncError::InvalidBlockRange);
                }
                
                result.extend_from_slice(&base_data[start..end]);
            }
            DeltaInstruction::Literal { data } => {
                // Insertar datos literales
                result.extend_from_slice(data);
            }
        }
    }
    
    // Verificar hash del resultado
    let mut hasher = Sha256::new();
    hasher.update(&result);
    let actual_hash = hex::encode(hasher.finalize());
    
    if actual_hash != delta.result_hash {
        return Err(DeltaSyncError::HashMismatch {
            expected: delta.result_hash.clone(),
            actual: actual_hash,
        });
    }
    
    Ok(result)
}
```

---

## Proposed Architecture

### File structure

```
src/
├── infrastructure/
│   └── services/
│       ├── mod.rs                      # Añadir: pub mod delta_sync_service;
│       └── delta_sync_service.rs       # NUEVO: Servicio principal
│
├── interfaces/
│   └── api/
│       └── handlers/
│           ├── mod.rs                  # Añadir: pub mod delta_sync_handler;
│           └── delta_sync_handler.rs   # NUEVO: Endpoints API
│
└── common/
    └── di.rs                           # Añadir: delta_sync_service a CoreServices
```

### Main service (delta_sync_service.rs)

```rust
//! Delta Sync Service - Sincronización eficiente por diferencias
//!
//! Implementa algoritmo similar a rsync para transferir solo
//! las partes modificadas de los archivos.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

/// Tamaño de bloque por defecto (16KB - buen balance)
pub const DEFAULT_BLOCK_SIZE: usize = 16 * 1024;

/// Tamaño mínimo de archivo para usar delta sync
pub const MIN_DELTA_SYNC_SIZE: u64 = 64 * 1024; // 64KB

/// Errores del servicio Delta Sync
#[derive(Debug, thiserror::Error)]
pub enum DeltaSyncError {
    #[error("Archivo no encontrado: {0}")]
    FileNotFound(String),
    
    #[error("Firmas no encontradas para archivo: {0}")]
    SignaturesNotFound(String),
    
    #[error("Índice de bloque inválido: {0}")]
    InvalidBlockIndex(u32),
    
    #[error("Rango de bloque inválido")]
    InvalidBlockRange,
    
    #[error("Hash no coincide: esperado {expected}, actual {actual}")]
    HashMismatch { expected: String, actual: String },
    
    #[error("Error de I/O: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Error de serialización: {0}")]
    SerializationError(String),
}

/// Servicio de Delta Sync
pub struct DeltaSyncService {
    /// Directorio para almacenar índices de firmas
    signatures_dir: PathBuf,
    /// Cache en memoria de firmas recientes
    signature_cache: Arc<RwLock<HashMap<String, Vec<BlockSignature>>>>,
    /// Tamaño de bloque configurado
    block_size: usize,
    /// Máximo de entradas en cache
    max_cache_entries: usize,
}

impl DeltaSyncService {
    pub fn new(storage_root: &Path) -> Self {
        Self {
            signatures_dir: storage_root.join(".delta_signatures"),
            signature_cache: Arc::new(RwLock::new(HashMap::new())),
            block_size: DEFAULT_BLOCK_SIZE,
            max_cache_entries: 1000,
        }
    }
    
    pub fn with_block_size(mut self, block_size: usize) -> Self {
        self.block_size = block_size;
        self
    }
    
    /// Inicializar servicio (crear directorios)
    pub async fn initialize(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.signatures_dir).await?;
        tracing::info!("Delta Sync service initialized with block size: {}KB", 
                       self.block_size / 1024);
        Ok(())
    }
    
    /// Generar y almacenar firmas para un archivo
    pub async fn index_file(
        &self, 
        file_id: &str, 
        file_path: &Path
    ) -> Result<Vec<BlockSignature>, DeltaSyncError> {
        let data = fs::read(file_path).await?;
        
        // No indexar archivos pequeños
        if data.len() < MIN_DELTA_SYNC_SIZE as usize {
            return Ok(Vec::new());
        }
        
        let signatures = generate_signatures(&data, self.block_size);
        
        // Guardar en disco
        let sig_path = self.signature_path(file_id);
        let sig_json = serde_json::to_vec(&signatures)
            .map_err(|e| DeltaSyncError::SerializationError(e.to_string()))?;
        fs::write(&sig_path, sig_json).await?;
        
        // Actualizar cache
        {
            let mut cache = self.signature_cache.write().await;
            if cache.len() >= self.max_cache_entries {
                // LRU simple: eliminar primera entrada
                if let Some(key) = cache.keys().next().cloned() {
                    cache.remove(&key);
                }
            }
            cache.insert(file_id.to_string(), signatures.clone());
        }
        
        tracing::debug!("Indexed file {} with {} blocks", file_id, signatures.len());
        Ok(signatures)
    }
    
    /// Obtener firmas de un archivo
    pub async fn get_signatures(
        &self, 
        file_id: &str
    ) -> Result<Vec<BlockSignature>, DeltaSyncError> {
        // Buscar en cache primero
        {
            let cache = self.signature_cache.read().await;
            if let Some(sigs) = cache.get(file_id) {
                return Ok(sigs.clone());
            }
        }
        
        // Cargar de disco
        let sig_path = self.signature_path(file_id);
        if !sig_path.exists() {
            return Err(DeltaSyncError::SignaturesNotFound(file_id.to_string()));
        }
        
        let sig_json = fs::read(&sig_path).await?;
        let signatures: Vec<BlockSignature> = serde_json::from_slice(&sig_json)
            .map_err(|e| DeltaSyncError::SerializationError(e.to_string()))?;
        
        // Actualizar cache
        {
            let mut cache = self.signature_cache.write().await;
            cache.insert(file_id.to_string(), signatures.clone());
        }
        
        Ok(signatures)
    }
    
    /// Aplicar delta a un archivo
    pub async fn apply_delta(
        &self,
        file_id: &str,
        base_path: &Path,
        delta: &FileDelta,
    ) -> Result<Vec<u8>, DeltaSyncError> {
        let base_data = fs::read(base_path).await?;
        let signatures = self.get_signatures(file_id).await?;
        
        apply_delta(&base_data, &signatures, delta, self.block_size)
    }
    
    /// Eliminar firmas de un archivo (cuando se borra)
    pub async fn remove_signatures(&self, file_id: &str) -> Result<(), DeltaSyncError> {
        // Eliminar de cache
        {
            let mut cache = self.signature_cache.write().await;
            cache.remove(file_id);
        }
        
        // Eliminar de disco
        let sig_path = self.signature_path(file_id);
        if sig_path.exists() {
            fs::remove_file(&sig_path).await?;
        }
        
        Ok(())
    }
    
    /// Estadísticas del servicio
    pub async fn get_stats(&self) -> DeltaSyncStats {
        let cache = self.signature_cache.read().await;
        DeltaSyncStats {
            cached_files: cache.len() as u64,
            block_size: self.block_size,
        }
    }
    
    fn signature_path(&self, file_id: &str) -> PathBuf {
        // Usar primeros 2 chars del ID para subdirectorio
        let prefix = &file_id[..2.min(file_id.len())];
        self.signatures_dir.join(prefix).join(format!("{}.sig", file_id))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeltaSyncStats {
    pub cached_files: u64,
    pub block_size: usize,
}
```

---

## API Endpoints

### Handler (delta_sync_handler.rs)

```rust
use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::common::di::AppState;
use crate::infrastructure::services::delta_sync_service::*;

pub struct DeltaSyncHandler;

impl DeltaSyncHandler {
    /// GET /api/files/{id}/signatures
    /// 
    /// Obtiene las firmas de bloques de un archivo para calcular delta
    pub async fn get_signatures(
        State(state): State<AppState>,
        Path(file_id): Path<String>,
    ) -> impl IntoResponse {
        let delta_service = &state.core.delta_sync_service;
        
        match delta_service.get_signatures(&file_id).await {
            Ok(signatures) => {
                Json(SignaturesResponse {
                    file_id,
                    block_size: delta_service.block_size,
                    block_count: signatures.len() as u32,
                    signatures,
                }).into_response()
            }
            Err(DeltaSyncError::SignaturesNotFound(_)) => {
                // Archivo no indexado - cliente debe hacer upload completo
                (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "Signatures not found",
                    "hint": "File not indexed for delta sync, use full upload"
                }))).into_response()
            }
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": e.to_string()
                }))).into_response()
            }
        }
    }
    
    /// POST /api/files/{id}/delta
    /// 
    /// Aplica un delta para actualizar un archivo
    pub async fn apply_delta(
        State(state): State<AppState>,
        Path(file_id): Path<String>,
        Json(delta): Json<FileDelta>,
    ) -> impl IntoResponse {
        let delta_service = &state.core.delta_sync_service;
        let file_service = &state.applications.file_service;
        
        // Obtener path del archivo actual
        let file = match file_service.get_file(&file_id).await {
            Ok(f) => f,
            Err(_) => {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "File not found"
                }))).into_response();
            }
        };
        
        // Aplicar delta
        let file_path = state.core.path_service.resolve_path(file.path());
        match delta_service.apply_delta(&file_id, &file_path, &delta).await {
            Ok(new_data) => {
                // Guardar nuevo contenido
                if let Err(e) = tokio::fs::write(&file_path, &new_data).await {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "error": format!("Failed to write file: {}", e)
                    }))).into_response();
                }
                
                // Re-indexar archivo
                if let Err(e) = delta_service.index_file(&file_id, &file_path).await {
                    tracing::warn!("Failed to re-index file after delta: {}", e);
                }
                
                // Calcular estadísticas
                let delta_size: usize = delta.instructions.iter()
                    .filter_map(|i| match i {
                        DeltaInstruction::Literal { data } => Some(data.len()),
                        _ => None,
                    })
                    .sum();
                
                Json(DeltaApplyResponse {
                    success: true,
                    new_size: new_data.len() as u64,
                    delta_size: delta_size as u64,
                    savings_percent: if new_data.len() > 0 {
                        ((1.0 - (delta_size as f64 / new_data.len() as f64)) * 100.0) as u32
                    } else { 0 },
                }).into_response()
            }
            Err(e) => {
                (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": e.to_string()
                }))).into_response()
            }
        }
    }
    
    /// POST /api/files/{id}/index
    /// 
    /// Fuerza la indexación de un archivo para delta sync
    pub async fn index_file(
        State(state): State<AppState>,
        Path(file_id): Path<String>,
    ) -> impl IntoResponse {
        let delta_service = &state.core.delta_sync_service;
        let file_service = &state.applications.file_service;
        
        // Obtener path del archivo
        let file = match file_service.get_file(&file_id).await {
            Ok(f) => f,
            Err(_) => {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "File not found"
                }))).into_response();
            }
        };
        
        let file_path = state.core.path_service.resolve_path(file.path());
        match delta_service.index_file(&file_id, &file_path).await {
            Ok(signatures) => {
                Json(serde_json::json!({
                    "success": true,
                    "blocks_indexed": signatures.len()
                })).into_response()
            }
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": e.to_string()
                }))).into_response()
            }
        }
    }
    
    /// GET /api/delta/stats
    /// 
    /// Estadísticas del servicio delta sync
    pub async fn get_stats(
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let delta_service = &state.core.delta_sync_service;
        Json(delta_service.get_stats().await)
    }
}

#[derive(Serialize)]
struct SignaturesResponse {
    file_id: String,
    block_size: usize,
    block_count: u32,
    signatures: Vec<BlockSignature>,
}

#[derive(Serialize)]
struct DeltaApplyResponse {
    success: bool,
    new_size: u64,
    delta_size: u64,
    savings_percent: u32,
}
```

### Routes to add in routes.rs

```rust
// Delta Sync routes
let delta_sync_router = Router::new()
    .route("/files/:id/signatures", get(DeltaSyncHandler::get_signatures))
    .route("/files/:id/delta", post(DeltaSyncHandler::apply_delta))
    .route("/files/:id/index", post(DeltaSyncHandler::index_file))
    .route("/delta/stats", get(DeltaSyncHandler::get_stats))
    .with_state(app_state.clone());

// Añadir a router principal
router = router.nest("/api", delta_sync_router);
```

---

## Integration with Existing System

### 1. Auto-index on upload

In `file_handler.rs`, after a successful upload:

```rust
// Después de guardar el archivo...

// Indexar para delta sync (archivos >64KB)
if total_size >= 64 * 1024 {
    let delta_service = &state.core.delta_sync_service;
    if let Err(e) = delta_service.index_file(&file.id, &file_path).await {
        tracing::warn!("Failed to index file for delta sync: {}", e);
        // No es error fatal, el archivo se subió correctamente
    }
}
```

### 2. Clean up signatures on delete

In `file_handler.rs`, when deleting a file:

```rust
// Limpiar firmas de delta sync
let delta_service = &state.core.delta_sync_service;
if let Err(e) = delta_service.remove_signatures(&id).await {
    tracing::warn!("Failed to remove delta signatures: {}", e);
}
```

### 3. Integration with Dedup Service

Delta sync and dedup are complementary:

```
┌──────────────────────────────────────────────────────────────┐
│                    UPLOAD WITH DELTA + DEDUP                  │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  1. Client has modified file.txt                              │
│                                                               │
│  2. GET /files/{id}/signatures                                │
│     → Server returns block signatures                         │
│                                                               │
│  3. Client computes delta locally                             │
│     → Only 3 of 100 blocks changed                            │
│                                                               │
│  4. POST /files/{id}/delta                                    │
│     → Sends only the 3 new blocks                             │
│                                                               │
│  5. Server applies delta                                      │
│     → Reconstructs complete file                              │
│                                                               │
│  6. Server runs dedup on resulting file                        │
│     → If another user has same content, it deduplicates        │
│                                                               │
│  RESULT:                                                      │
│  ├── Delta sync: 97% less transfer                            │
│  └── Dedup: 30-50% less storage                               │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## Use Cases and Effectiveness

| File type | Scenario | Without Delta | With Delta | Savings |
|-----------------|-----------|-----------|-----------|--------|
| `.txt` / `.md` | Edit paragraph | 1MB | ~4KB | **99.6%** |
| `.json` / `.xml` | Change value | 500KB | ~1KB | **99.8%** |
| `.rs` / `.js` | Modify function | 100KB | ~2KB | **98%** |
| `.docx` | Edit page | 5MB | ~100KB | **98%** |
| `.xlsx` | Change cells | 2MB | ~50KB | **97.5%** |
| `.pdf` | Edit text | 10MB | ~2MB | **80%** |
| `.psd` | Edit layer | 100MB | ~5MB | **95%** |
| `.zip` | Add file | 50MB | ~5MB | **90%** |
| `.mp4` | Re-encode | 500MB | 450MB | **10%** |
| `.jpg` | Edit image | 5MB | 4MB | **20%** |

For highly compressed or re-encoded files, delta sync is less effective.

---

## Performance Considerations

### Optimal block size

| Size | Pros | Cons | Best for |
|--------|------|------|------------|
| 4KB | More granular, better savings | More signature overhead | Small files |
| 16KB | Good balance | - | **General use** |
| 64KB | Less overhead | Less granular | Large files |
| 256KB | Minimal overhead | Little savings | Very large files |

### Memory

```rust
// Estimación de memoria por archivo indexado
// 
// BlockSignature size ≈ 48 bytes (4 + 8 + 4 + 4 + 32 - con padding)
// 
// Archivo 100MB con bloques de 16KB:
// - 100MB / 16KB = 6,400 bloques
// - 6,400 × 48 bytes = ~300KB de firmas
// 
// Cache de 1000 archivos ≈ 300MB máximo
```

### CPU

```rust
// Operaciones costosas:
// 
// 1. generate_signatures():  O(n) donde n = tamaño archivo
//    - SHA-256: ~500MB/s en CPU moderna
//    - Adler32: ~2GB/s
//    
// 2. generate_delta(): O(n × m) peor caso, O(n) típico
//    - n = tamaño archivo nuevo
//    - m = número de bloques originales
//    - HashMap lookup: O(1) promedio
//
// 3. apply_delta(): O(n) donde n = tamaño resultado
//    - Mayormente copias de memoria
```

---

## Testing

### Unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rolling_checksum() {
        let mut rc = RollingChecksum::new(4);
        
        // Alimentar bytes
        for b in b"test" {
            rc.roll(*b);
        }
        let checksum1 = rc.checksum();
        
        // Rolling: quitar 't', añadir 'X'
        rc.roll(b'X');
        let checksum2 = rc.checksum();
        
        // Checksums deben ser diferentes
        assert_ne!(checksum1, checksum2);
    }
    
    #[test]
    fn test_generate_signatures() {
        let data = b"Hello, World! This is a test file for delta sync.";
        let sigs = generate_signatures(data, 16);
        
        assert_eq!(sigs.len(), 4); // 50 bytes / 16 = 3.125 → 4 bloques
        assert_eq!(sigs[0].offset, 0);
        assert_eq!(sigs[1].offset, 16);
    }
    
    #[test]
    fn test_delta_identical_files() {
        let data = b"Hello, World!";
        let sigs = generate_signatures(data, 8);
        let delta = generate_delta(data, &sigs, 8);
        
        // Solo instrucciones Copy, sin Literal
        for instr in &delta.instructions {
            assert!(matches!(instr, DeltaInstruction::Copy { .. }));
        }
    }
    
    #[test]
    fn test_delta_small_change() {
        let original = b"Hello, World! This is original.";
        let modified = b"Hello, World! This is MODIFIED.";
        
        let sigs = generate_signatures(original, 8);
        let delta = generate_delta(modified, &sigs, 8);
        
        // Debería haber algunas instrucciones Copy y algunas Literal
        let copies = delta.instructions.iter()
            .filter(|i| matches!(i, DeltaInstruction::Copy { .. }))
            .count();
        let literals = delta.instructions.iter()
            .filter(|i| matches!(i, DeltaInstruction::Literal { .. }))
            .count();
        
        assert!(copies > 0, "Should reuse some blocks");
        assert!(literals > 0, "Should have some new data");
    }
    
    #[test]
    fn test_apply_delta_roundtrip() {
        let original = b"The quick brown fox jumps over the lazy dog.";
        let modified = b"The quick brown cat jumps over the lazy dog.";
        
        let sigs = generate_signatures(original, 8);
        let delta = generate_delta(modified, &sigs, 8);
        let reconstructed = apply_delta(original, &sigs, &delta, 8).unwrap();
        
        assert_eq!(reconstructed, modified);
    }
}
```

### Integration tests

```rust
#[tokio::test]
async fn test_delta_sync_service_workflow() {
    let temp_dir = tempfile::tempdir().unwrap();
    let service = DeltaSyncService::new(temp_dir.path());
    service.initialize().await.unwrap();
    
    // Crear archivo original
    let file_path = temp_dir.path().join("test.txt");
    tokio::fs::write(&file_path, b"Original content here").await.unwrap();
    
    // Indexar
    let sigs = service.index_file("file123", &file_path).await.unwrap();
    assert!(!sigs.is_empty());
    
    // Recuperar firmas
    let retrieved = service.get_signatures("file123").await.unwrap();
    assert_eq!(sigs.len(), retrieved.len());
    
    // Simular modificación y delta
    let modified = b"Modified content here!";
    let delta = generate_delta(modified, &sigs, service.block_size);
    
    // Aplicar delta
    let result = service.apply_delta("file123", &file_path, &delta).await.unwrap();
    assert_eq!(result, modified);
}
```

---

## Required Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Ya existentes - verificar versiones
sha2 = "0.10"
hex = "0.4"

# Nuevas dependencias para delta sync
thiserror = "1.0"     # Para errores tipados (probablemente ya existe)
```

---

## Implementation Checklist

- [ ] Create `delta_sync_service.rs` with basic structures
- [ ] Implement **RollingChecksum**
- [ ] Implement **generate_signatures()**
- [ ] Implement **generate_delta()**
- [ ] Implement **apply_delta()**
- [ ] Create handler and API endpoints
- [ ] Integrate into DI (**CoreServices**)
- [ ] Add routes in `routes.rs`
- [ ] Integrate with upload (automatic indexing)
- [ ] Integrate with delete (signature cleanup)
- [ ] Unit tests
- [ ] Integration tests
- [ ] Document API endpoints
- [ ] Metrics and logging

---

## References

- [rsync algorithm](https://rsync.samba.org/tech_report/)
- [Rolling hash - Wikipedia](https://en.wikipedia.org/wiki/Rolling_hash)
- [Adler-32 checksum](https://en.wikipedia.org/wiki/Adler-32)
- [librsync](https://github.com/librsync/librsync)
