//! Modelos de datos para la capa de almacenamiento

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Metadatos de un archivo almacenado
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileMetadata {
    /// Identificador único del archivo
    pub id: Uuid,
    
    /// ID del usuario propietario
    pub user_id: Uuid,
    
    /// Nombre original del archivo
    pub filename: String,
    
    /// Ruta dentro del sistema de archivos virtual
    pub path: String,
    
    /// Tamaño del archivo en bytes
    pub size: i64,
    
    /// Tipo MIME del archivo
    pub mime_type: String,
    
    /// Hash del contenido del archivo (opcional)
    pub content_hash: Option<String>,
    
    /// Indica si es un directorio
    pub is_directory: bool,
    
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última modificación
    pub updated_at: DateTime<Utc>,
}

/// Información sobre una versión de un archivo
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    /// Identificador único de la versión
    pub id: Uuid,
    
    /// ID del archivo al que pertenece esta versión
    pub file_id: Uuid,
    
    /// Número de versión (secuencial)
    pub version_number: i32,
    
    /// Tamaño de esta versión en bytes
    pub size: i64,
    
    /// Hash del contenido de esta versión
    pub content_hash: Option<String>,
    
    /// Fecha de creación de esta versión
    pub created_at: DateTime<Utc>,
    
    /// ID del usuario que creó esta versión
    pub created_by: Uuid,
}

/// Información sobre un enlace compartido
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharedLink {
    /// Identificador único del enlace
    pub id: Uuid,
    
    /// ID del archivo compartido
    pub file_id: Uuid,
    
    /// ID del usuario que compartió el archivo
    pub user_id: Uuid,
    
    /// Token de acceso para el enlace
    pub token: String,
    
    /// Nombre para el enlace (opcional)
    pub name: Option<String>,
    
    /// Fecha de expiración (opcional)
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Protección con contraseña (opcional)
    pub password_hash: Option<String>,
    
    /// Número máximo de descargas (opcional)
    pub max_downloads: Option<i32>,
    
    /// Número actual de descargas
    pub download_count: i32,
    
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última modificación
    pub updated_at: DateTime<Utc>,
}

/// Estadísticas de almacenamiento para un usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// ID del usuario
    pub user_id: Uuid,
    
    /// Espacio total utilizado en bytes
    pub used_space: i64,
    
    /// Cuota asignada en bytes (None = sin límite)
    pub quota: Option<i64>,
    
    /// Número total de archivos
    pub file_count: i32,
    
    /// Fecha de la última actualización de estas estadísticas
    pub updated_at: DateTime<Utc>,
}

/// Tipos de operaciones en archivos para el registro
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "file_operation", rename_all = "snake_case")]
pub enum FileOperation {
    Create,
    Read,
    Update,
    Delete,
    Download,
    Share,
    Rename,
    Move,
}

/// Registro de actividad para un archivo
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileActivity {
    /// Identificador único del registro
    pub id: Uuid,
    
    /// ID del archivo
    pub file_id: Uuid,
    
    /// ID del usuario que realizó la operación
    pub user_id: Uuid,
    
    /// Tipo de operación realizada
    pub operation: FileOperation,
    
    /// Detalles adicionales (formato JSON)
    pub details: Option<serde_json::Value>,
    
    /// Dirección IP desde donde se realizó la operación
    pub ip_address: Option<String>,
    
    /// Agente de usuario (navegador/cliente)
    pub user_agent: Option<String>,
    
    /// Fecha y hora de la operación
    pub created_at: DateTime<Utc>,
}

/// Esquema de la tabla files para migraciones
pub const FILES_TABLE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    size BIGINT NOT NULL DEFAULT 0,
    mime_type TEXT NOT NULL,
    content_hash TEXT,
    is_directory BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Índices para optimizar búsquedas
CREATE INDEX IF NOT EXISTS idx_files_user_id ON files(user_id);
CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
CREATE INDEX IF NOT EXISTS idx_files_user_path ON files(user_id, path);
"#;

/// Esquema de la tabla file_versions para migraciones
pub const FILE_VERSIONS_TABLE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS file_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL,
    version_number INTEGER NOT NULL,
    size BIGINT NOT NULL DEFAULT 0,
    content_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    CONSTRAINT fk_file FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL,
    CONSTRAINT uq_file_version UNIQUE (file_id, version_number)
);

-- Índices para optimizar búsquedas
CREATE INDEX IF NOT EXISTS idx_file_versions_file_id ON file_versions(file_id);
"#;

/// Esquema de la tabla shared_links para migraciones
pub const SHARED_LINKS_TABLE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS shared_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL,
    user_id UUID NOT NULL,
    token TEXT NOT NULL UNIQUE,
    name TEXT,
    expires_at TIMESTAMPTZ,
    password_hash TEXT,
    max_downloads INTEGER,
    download_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_file FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Índices para optimizar búsquedas
CREATE INDEX IF NOT EXISTS idx_shared_links_token ON shared_links(token);
CREATE INDEX IF NOT EXISTS idx_shared_links_user_id ON shared_links(user_id);
CREATE INDEX IF NOT EXISTS idx_shared_links_file_id ON shared_links(file_id);
"#;

/// Esquema de la tabla file_activity para migraciones
pub const FILE_ACTIVITY_TABLE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS file_activity (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL,
    user_id UUID NOT NULL,
    operation TEXT NOT NULL,
    details JSONB,
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_file FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Índices para optimizar búsquedas
CREATE INDEX IF NOT EXISTS idx_file_activity_file_id ON file_activity(file_id);
CREATE INDEX IF NOT EXISTS idx_file_activity_user_id ON file_activity(user_id);
CREATE INDEX IF NOT EXISTS idx_file_activity_created_at ON file_activity(created_at);
"#;