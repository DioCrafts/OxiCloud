// Repositorios PostgreSQL (blob-storage model)
pub mod pg;

// Re-exportar para facilitar acceso
pub use pg::{
    DeviceCodePgRepository, FileBlobReadRepository, FileBlobWriteRepository,
    FolderDbRepository, SessionPgRepository, TrashDbRepository, UserPgRepository,
};
