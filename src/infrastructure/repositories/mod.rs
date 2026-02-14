pub mod share_fs_repository;

// Repositorios PostgreSQL (blob-storage model)
pub mod pg;

// Re-exportar para facilitar acceso
pub use pg::{
    FileBlobReadRepository, FileBlobWriteRepository, FolderDbRepository, SessionPgRepository,
    TrashDbRepository, UserPgRepository,
};
