use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::{fs, time};
use tokio::fs::File as TokioFile;
use tokio_util::codec::{BytesCodec, FramedRead};
use futures::{Stream, StreamExt};
use bytes::Bytes;
use tokio::task;
use mime_guess::from_path;

use crate::domain::entities::file::File;
use crate::application::ports::storage_ports::FileReadPort;
use crate::common::errors::DomainError;
use crate::infrastructure::repositories::repository_errors::{FileRepositoryResult, FileRepositoryError};
use crate::infrastructure::repositories::parallel_file_processor::ParallelFileProcessor;
use crate::application::ports::cache_ports::MetadataCachePort;
use crate::application::services::storage_mediator::StorageMediator;
use crate::infrastructure::services::path_service::PathService;
use crate::domain::services::path_service::StoragePath;
use crate::common::config::AppConfig;

/// Implementación de repositorio para operaciones de **lectura** de archivos.
///
/// Implementa `FileReadPort`:
/// get_file, list_files, get_file_content, get_file_stream,
/// get_file_range_stream, get_file_mmap, get_file_path, get_parent_folder_id.
pub struct FileFsReadRepository {
    root_path: PathBuf,
    storage_mediator: Arc<dyn StorageMediator>,
    id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
    path_service: Arc<PathService>,
    metadata_cache: Arc<dyn MetadataCachePort>,
    config: AppConfig,
    parallel_processor: Option<Arc<ParallelFileProcessor>>,
}

impl FileFsReadRepository {
    /// Constructor completo con todas las dependencias de infraestructura.
    pub fn new(
        root_path: PathBuf,
        storage_mediator: Arc<dyn StorageMediator>,
        id_mapping_service: Arc<dyn crate::application::ports::outbound::IdMappingPort>,
        path_service: Arc<PathService>,
        metadata_cache: Arc<dyn MetadataCachePort>,
        config: AppConfig,
        parallel_processor: Option<Arc<ParallelFileProcessor>>,
    ) -> Self {
        Self {
            root_path,
            storage_mediator,
            id_mapping_service,
            path_service,
            metadata_cache,
            config,
            parallel_processor,
        }
    }

    /// Stub para pruebas (no realiza I/O real).
    pub fn default_stub() -> Self {
        Self {
            root_path: PathBuf::from("./storage"),
            storage_mediator: Arc::new(
                crate::application::services::storage_mediator::FileSystemStorageMediator::new_stub(),
            ),
            id_mapping_service: Arc::new(crate::common::stubs::StubIdMappingPort),
            path_service: Arc::new(PathService::new(PathBuf::from("./storage"))),
            metadata_cache: Arc::new(
                crate::infrastructure::services::file_metadata_cache::FileMetadataCache::default()
            ) as Arc<dyn MetadataCachePort>,
            config: AppConfig::default(),
            parallel_processor: None,
        }
    }

    // ─── helpers internos ────────────────────────────────────

    fn resolve_storage_path(&self, storage_path: &StoragePath) -> PathBuf {
        self.path_service.resolve_path(storage_path)
    }

    async fn get_file_metadata_raw(&self, abs_path: &PathBuf) -> FileRepositoryResult<(u64, u64, u64)> {
        // Cache first
        if let Some(cached) = self.metadata_cache.get_metadata(abs_path).await {
            if let (Some(s), Some(c), Some(m)) = (cached.size, cached.created_at, cached.modified_at) {
                return Ok((s, c, m));
            }
        }
        let metadata = time::timeout(self.config.timeouts.file_timeout(), fs::metadata(abs_path))
            .await
            .map_err(|_| FileRepositoryError::StorageError(format!("Timeout metadata: {}", abs_path.display())))?
            .map_err(|e| FileRepositoryError::StorageError(e.to_string()))?;
        let size = metadata.len();
        let created_at = metadata.created()
            .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or(0);
        let modified_at = metadata.modified()
            .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or(0);
        let _ = self.metadata_cache.refresh_metadata(abs_path).await;
        Ok((size, created_at, modified_at))
    }

    async fn get_file_by_id(&self, id: &str) -> FileRepositoryResult<File> {
        let storage_path = self.id_mapping_service.get_path_by_id(id).await
            .map_err(|e| FileRepositoryError::Other(e.to_string()))?;
        let abs_path = self.resolve_storage_path(&storage_path);

        if !abs_path.exists() || !abs_path.is_file() {
            return Err(FileRepositoryError::NotFound(
                format!("File {} not found at {}", id, storage_path.to_string()),
            ));
        }

        let (size, created_at, modified_at) = self.get_file_metadata_raw(&abs_path).await?;
        let name = storage_path
            .file_name()
            .ok_or_else(|| FileRepositoryError::InvalidPath(storage_path.to_string()))?;
        let mime_type = from_path(&abs_path).first_or_octet_stream().to_string();

        File::with_timestamps(
            id.to_string(), name, storage_path, size, mime_type, None,
            created_at, modified_at,
        )
        .map_err(|e| FileRepositoryError::Other(e.to_string()))
    }
}

impl Clone for FileFsReadRepository {
    fn clone(&self) -> Self {
        Self {
            root_path: self.root_path.clone(),
            storage_mediator: self.storage_mediator.clone(),
            id_mapping_service: self.id_mapping_service.clone(),
            path_service: self.path_service.clone(),
            metadata_cache: self.metadata_cache.clone(),
            config: self.config.clone(),
            parallel_processor: self.parallel_processor.clone(),
        }
    }
}

#[async_trait]
impl FileReadPort for FileFsReadRepository {
    async fn get_file(&self, id: &str) -> Result<File, DomainError> {
        self.get_file_by_id(id).await.map_err(|e| match e {
            FileRepositoryError::NotFound(msg) => DomainError::not_found("File", msg),
            FileRepositoryError::StorageError(msg) => DomainError::internal_error("File", msg),
            other => DomainError::internal_error("File", other.to_string()),
        })
    }

    async fn list_files(&self, folder_id: Option<&str>) -> Result<Vec<File>, DomainError> {
        let folder_storage_path = match folder_id {
            Some(id) => {
                match self.storage_mediator.get_folder_path(id).await {
                    Ok(path) => {
                        let lossy = path.to_string_lossy().to_string();
                        let folder_name = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or(&lossy);
                        StoragePath::from_string(folder_name)
                    }
                    Err(_) => return Ok(Vec::new()),
                }
            }
            None => StoragePath::root(),
        };

        let abs_folder_path = self.path_service.resolve_path(&folder_storage_path);
        if !abs_folder_path.exists() || !abs_folder_path.is_dir() {
            return Ok(Vec::new());
        }

        let mut files_result = Vec::new();
        let mut entries = fs::read_dir(&abs_folder_path).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?
        {
            let path = entry.path();
            if !path.is_file() { continue; }
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with('.') || file_name == "folder_ids.json" || file_name == "file_ids.json" {
                continue;
            }
            let metadata = match fs::metadata(&path).await {
                Ok(m) => m,
                Err(_) => continue,
            };
            let file_storage_path = folder_storage_path.join(&file_name);
            let id = match self.id_mapping_service.get_or_create_id(&file_storage_path).await {
                Ok(id) => id,
                Err(_) => continue,
            };
            let size = metadata.len();
            let created_at = metadata.created()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                .unwrap_or(0);
            let modified_at = metadata.modified()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                .unwrap_or(0);
            let mime_type = from_path(&path).first_or_octet_stream().to_string();

            match File::with_timestamps(id, file_name, file_storage_path, size, mime_type, folder_id.map(String::from), created_at, modified_at) {
                Ok(file) => files_result.push(file),
                Err(_) => continue,
            }
        }

        // Persist any new ID mappings
        let _ = self.id_mapping_service.save_changes().await;
        Ok(files_result)
    }

    async fn get_file_content(&self, id: &str) -> Result<Vec<u8>, DomainError> {
        let file = self.get_file_by_id(id).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let abs_path = self.resolve_storage_path(file.storage_path());

        let metadata = time::timeout(self.config.timeouts.file_timeout(), fs::metadata(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", format!("Timeout metadata: {}", abs_path.display())))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let file_size = metadata.len();

        if !self.config.resources.can_load_in_memory(file_size) {
            return Err(DomainError::internal_error("File",
                format!("File too large for memory: {} MB", file_size / (1024 * 1024))));
        }

        // Parallel read for very large files
        if self.config.resources.needs_parallel_processing(file_size, &self.config.concurrency) {
            let content = if let Some(processor) = &self.parallel_processor {
                processor.read_file_parallel(&abs_path).await
            } else {
                let processor = ParallelFileProcessor::new(self.config.clone());
                processor.read_file_parallel(&abs_path).await
            };
            return content.map_err(|e| DomainError::internal_error("File", e.to_string()));
        }

        // spawn_blocking for large-ish files
        if self.config.resources.is_large_file(file_size) {
            let abs_clone = abs_path.clone();
            let chunk_size = self.config.resources.chunk_size_bytes;
            let content = task::spawn_blocking(move || -> std::io::Result<Vec<u8>> {
                use std::io::{Read, BufReader};
                let file = std::fs::File::open(&abs_clone)?;
                let mut reader = BufReader::with_capacity(chunk_size, file);
                let mut buf = Vec::with_capacity(file_size as usize);
                reader.read_to_end(&mut buf)?;
                Ok(buf)
            }).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            return Ok(content);
        }

        // Small files — async read
        time::timeout(self.config.timeouts.file_timeout(), fs::read(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", format!("Timeout reading: {}", abs_path.display())))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))
    }

    async fn get_file_stream(
        &self,
        id: &str,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        let file = self.get_file_by_id(id).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let abs_path = self.resolve_storage_path(file.storage_path());

        let metadata = time::timeout(self.config.timeouts.file_timeout(), fs::metadata(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", "Timeout getting metadata"))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let file_size = metadata.len();
        let is_large = self.config.resources.is_large_file(file_size);

        let fh = time::timeout(self.config.timeouts.file_timeout(), TokioFile::open(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", "Timeout opening file"))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        let chunk_size = if is_large { self.config.resources.chunk_size_bytes } else { 4096 };
        let codec = BytesCodec::new();
        let stream = FramedRead::with_capacity(fh, codec, chunk_size)
            .map(|r| r.map(|bm| bm.freeze()));
        Ok(Box::new(stream))
    }

    async fn get_file_range_stream(
        &self,
        id: &str,
        start: u64,
        end: Option<u64>,
    ) -> Result<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>, DomainError> {
        use tokio::io::AsyncSeekExt;

        let file = self.get_file_by_id(id).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let abs_path = self.resolve_storage_path(file.storage_path());

        let metadata = time::timeout(self.config.timeouts.file_timeout(), fs::metadata(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", "Timeout"))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let file_size = metadata.len();
        if start >= file_size {
            return Err(DomainError::internal_error("File",
                format!("Range start {} beyond file size {}", start, file_size)));
        }
        let actual_end = end.map(|e| e.min(file_size - 1)).unwrap_or(file_size - 1);
        let range_length = actual_end - start + 1;

        let mut fh = time::timeout(self.config.timeouts.file_timeout(), TokioFile::open(&abs_path))
            .await
            .map_err(|_| DomainError::internal_error("File", "Timeout opening file"))?
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        fh.seek(std::io::SeekFrom::Start(start)).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;

        let chunk_size = if range_length > 1024 * 1024 { self.config.resources.chunk_size_bytes } else { 8192 };
        use tokio::io::AsyncReadExt;
        let limited = fh.take(range_length);
        let codec = BytesCodec::new();
        let stream = FramedRead::with_capacity(limited, codec, chunk_size)
            .map(|r| r.map(|bm| bm.freeze()));
        Ok(Box::new(stream))
    }

    async fn get_file_mmap(&self, id: &str) -> Result<Bytes, DomainError> {
        use memmap2::Mmap;
        let file = self.get_file_by_id(id).await
            .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
        let abs_path = self.resolve_storage_path(file.storage_path());
        let path_clone = abs_path.clone();

        task::spawn_blocking(move || -> Result<Bytes, DomainError> {
            let fh = std::fs::File::open(&path_clone)
                .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            let mmap = unsafe { Mmap::map(&fh) }
                .map_err(|e| DomainError::internal_error("File", e.to_string()))?;
            Ok(Bytes::copy_from_slice(&mmap[..]))
        }).await
        .map_err(|e| DomainError::internal_error("File", e.to_string()))?
    }

    async fn get_file_path(&self, id: &str) -> Result<StoragePath, DomainError> {
        self.id_mapping_service.get_path_by_id(id).await
    }

    async fn get_parent_folder_id(&self, path: &str) -> Result<String, DomainError> {
        let storage_path = StoragePath::from_string(path);
        match storage_path.parent() {
            Some(parent) if !parent.is_empty() => {
                self.id_mapping_service.get_or_create_id(&parent).await
            }
            _ => Ok("root".to_string()),
        }
    }
}