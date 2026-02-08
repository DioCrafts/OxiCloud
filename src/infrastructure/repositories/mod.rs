pub mod folder_fs_repository;
pub mod parallel_file_processor;
pub mod repository_errors;

// Repositorios CQRS (Read/Write) + composite
pub mod file_fs_read_repository;
pub mod file_fs_write_repository;
pub mod composite_file_repository;

pub mod trash_fs_repository;
pub mod folder_fs_repository_trash;
pub mod share_fs_repository;

// Repositorios PostgreSQL
pub mod pg;

// Re-exportar para facilitar acceso
pub use file_fs_read_repository::FileFsReadRepository;
pub use file_fs_write_repository::FileFsWriteRepository;
pub use composite_file_repository::CompositeFileRepository;
pub use pg::{UserPgRepository, SessionPgRepository};
