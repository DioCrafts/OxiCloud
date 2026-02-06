use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

use crate::{
    application::{
        dtos::{
            pagination::PaginatedResponseDto,
            share_dto::{CreateShareDto, ShareDto, UpdateShareDto},
        },
        ports::{
            outbound::{FileStoragePort, FolderStoragePort},
            share_ports::{ShareStoragePort, ShareUseCase},
        },
    },
    common::{config::AppConfig, errors::DomainError},
    domain::entities::share::{Share, ShareItemType, SharePermissions},
};

#[derive(Debug, Error)]
pub enum ShareServiceError {
    #[error("Share not found: {0}")]
    NotFound(String),
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Share expired")]
    Expired,
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Invalid item type: {0}")]
    InvalidItemType(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<ShareServiceError> for DomainError {
    fn from(error: ShareServiceError) -> Self {
        match error {
            ShareServiceError::NotFound(s) => DomainError::not_found("Share", s),
            ShareServiceError::ItemNotFound(s) => DomainError::not_found("Item", s),
            ShareServiceError::AccessDenied(s) => DomainError::access_denied("Share", s),
            ShareServiceError::InvalidPassword(s) => DomainError::access_denied("Share", s),
            ShareServiceError::Expired => DomainError::access_denied("Share", "Share has expired".to_string()),
            ShareServiceError::Repository(s) => DomainError::internal_error("Share", s),
            ShareServiceError::InvalidItemType(s) => DomainError::validation_error(s),
            ShareServiceError::Validation(s) => DomainError::validation_error(s),
        }
    }
}

pub struct ShareService {
    config: Arc<AppConfig>,
    share_repository: Arc<dyn ShareStoragePort>,
    file_repository: Arc<dyn FileStoragePort>,
    folder_repository: Arc<dyn FolderStoragePort>,
}

impl ShareService {
    pub fn new(
        config: Arc<AppConfig>,
        share_repository: Arc<dyn ShareStoragePort>,
        file_repository: Arc<dyn FileStoragePort>,
        folder_repository: Arc<dyn FolderStoragePort>,
    ) -> Self {
        Self {
            config,
            share_repository,
            file_repository,
            folder_repository,
        }
    }

    /// Verifica que el elemento a compartir existe
    async fn verify_item_exists(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<(), ShareServiceError> {
        match item_type {
            ShareItemType::File => {
                self.file_repository
                    .get_file(item_id) // Usando el método correcto del trait FileStoragePort
                    .await
                    .map_err(|_| ShareServiceError::ItemNotFound(format!("File with ID {} not found", item_id)))?;
            }
            ShareItemType::Folder => {
                self.folder_repository
                    .get_folder(item_id) // Usando el método correcto del trait FolderStoragePort
                    .await
                    .map_err(|_| ShareServiceError::ItemNotFound(format!("Folder with ID {} not found", item_id)))?;
            }
        }
        Ok(())
    }

    /// Hash de contraseña
    fn hash_password(&self, password: &str) -> String {
        // En una implementación real, usar un algoritmo seguro como bcrypt
        // Para simplificar, solo devolvemos la misma contraseña
        password.to_string()
    }
}

#[async_trait]
impl ShareUseCase for ShareService {
    async fn create_shared_link(
        &self,
        user_id: &str,
        dto: CreateShareDto,
    ) -> Result<ShareDto, DomainError> {
        // Convertir el tipo de elemento
        let item_type = ShareItemType::try_from(dto.item_type.as_str())
            .map_err(|e| ShareServiceError::InvalidItemType(e.to_string()))?;

        // Verificar que el elemento existe
        self.verify_item_exists(&dto.item_id, &item_type).await?;

        // Convertir el DTO de permisos si existe
        let permissions = dto.permissions.map(|p| p.to_entity());

        // Hash de contraseña si existe
        let password_hash = dto.password.map(|p| self.hash_password(&p));

        // Crear la entidad Share
        let share = Share::new(
            dto.item_id.clone(),
            item_type,
            user_id.to_string(),
            permissions,
            password_hash,
            dto.expires_at,
        )
        .map_err(|e| ShareServiceError::Validation(e.to_string()))?;

        // Guardar en el repositorio
        let saved_share = self
            .share_repository
            .save_share(&share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convertir la entidad a DTO para la respuesta
        Ok(ShareDto::from_entity(&saved_share, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
    }

    async fn get_shared_link(&self, id: &str) -> Result<ShareDto, DomainError> {
        // Buscar el enlace compartido por su ID
        let share = self
            .share_repository
            .find_share_by_id(id)
            .await
            .map_err(|e| ShareServiceError::NotFound(format!("Share with ID {} not found: {}", id, e)))?;

        // Verificar si ha expirado
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Convertir la entidad a DTO para la respuesta
        Ok(ShareDto::from_entity(&share, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
    }

    async fn get_shared_link_by_token(&self, token: &str) -> Result<ShareDto, DomainError> {
        // Buscar el enlace compartido por su token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e)))?;

        // Verificar si ha expirado
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Convertir la entidad a DTO para la respuesta
        Ok(ShareDto::from_entity(&share, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
    }

    async fn get_shared_links_for_item(
        &self,
        item_id: &str,
        item_type: &ShareItemType,
    ) -> Result<Vec<ShareDto>, DomainError> {
        // Buscar todos los enlaces compartidos para el elemento
        let shares = self
            .share_repository
            .find_shares_by_item(item_id, item_type)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Filtrar los enlaces expirados
        let active_shares: Vec<Share> = shares.into_iter().filter(|s| !s.is_expired()).collect();

        // Convertir las entidades a DTOs para la respuesta
        let share_dtos = active_shares
            .iter()
            .map(|s| ShareDto::from_entity(s, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
            .collect();

        Ok(share_dtos)
    }

    async fn update_shared_link(
        &self,
        id: &str,
        dto: UpdateShareDto,
    ) -> Result<ShareDto, DomainError> {
        // Buscar el enlace compartido existente
        let mut share = self
            .share_repository
            .find_share_by_id(id)
            .await
            .map_err(|e| ShareServiceError::NotFound(format!("Share with ID {} not found: {}", id, e)))?;

        // Actualizar permisos si se proporcionan
        if let Some(permissions_dto) = dto.permissions {
            let permissions = SharePermissions::new(
                permissions_dto.read,
                permissions_dto.write,
                permissions_dto.reshare,
            );
            share = share.with_permissions(permissions);
        }

        // Actualizar contraseña si se proporciona
        if let Some(password) = dto.password {
            let password_hash = if password.is_empty() {
                None
            } else {
                Some(self.hash_password(&password))
            };
            share = share.with_password(password_hash);
        }

        // Actualizar fecha de expiración si se proporciona
        if dto.expires_at.is_some() {
            share = share.with_expiration(dto.expires_at);
        }

        // Guardar los cambios
        let updated_share = self
            .share_repository
            .update_share(&share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convertir la entidad a DTO para la respuesta
        Ok(ShareDto::from_entity(&updated_share, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
    }

    async fn delete_shared_link(&self, id: &str) -> Result<(), DomainError> {
        // Eliminar el enlace compartido
        self.share_repository
            .delete_share(id)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn get_user_shared_links(
        &self,
        user_id: &str,
        page: usize,
        per_page: usize,
    ) -> Result<PaginatedResponseDto<ShareDto>, DomainError> {
        // Calcular offset para paginación
        let offset = (page - 1) * per_page;

        // Buscar los enlaces compartidos del usuario
        let (shares, total) = self
            .share_repository
            .find_shares_by_user(user_id, offset, per_page)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        // Convertir las entidades a DTOs
        let share_dtos: Vec<ShareDto> = shares
            .iter()
            .map(|s| ShareDto::from_entity(s, &format!("http://{}:{}", self.config.server_host, self.config.server_port)))
            .collect();

        // Crear el resultado paginado
        let paginated = PaginatedResponseDto::new(
            share_dtos,
            page,
            per_page,
            total
        );

        Ok(paginated)
    }

    async fn verify_shared_link_password(
        &self,
        token: &str,
        password: &str,
    ) -> Result<bool, DomainError> {
        // Buscar el enlace compartido por su token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e)))?;

        // Verificar si ha expirado
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Verificar la contraseña
        Ok(share.verify_password(password))
    }

    async fn register_shared_link_access(&self, token: &str) -> Result<(), DomainError> {
        // Buscar el enlace compartido por su token
        let share = self
            .share_repository
            .find_share_by_token(token)
            .await
            .map_err(|e| ShareServiceError::NotFound(format!("Share with token {} not found: {}", token, e)))?;

        // Verificar si ha expirado
        if share.is_expired() {
            return Err(ShareServiceError::Expired.into());
        }

        // Incrementar el contador de accesos
        let updated_share = share.increment_access_count();

        // Guardar los cambios
        self.share_repository
            .update_share(&updated_share)
            .await
            .map_err(|e| ShareServiceError::Repository(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::share_ports::ShareStoragePort;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct MockFileRepository;
    struct MockFolderRepository;

    #[async_trait]
    impl FileStoragePort for MockFileRepository {
        async fn find_file_by_id(&self, id: &str) -> Result<crate::domain::entities::file::File, DomainError> {
            if id == "test_file_id" {
                let file = crate::domain::entities::file::File::new(
                    id.to_string(),
                    "test.txt".to_string(),
                    "/path/to/test.txt".to_string(),
                    "/test.txt".to_string(),
                    123,
                    "text/plain".to_string(),
                    None,
                    None,
                    None,
                )
                .unwrap();
                Ok(file)
            } else {
                Err(DomainError::NotFound(format!("File {} not found", id)))
            }
        }
        
        // Implementación dummy para el resto de métodos requeridos
        async fn find_files_in_folder(&self, _folder_id: &str) -> Result<Vec<crate::domain::entities::file::File>, DomainError> {
            unimplemented!()
        }
        
        async fn save_file(&self, _file: &crate::domain::entities::file::File) -> Result<crate::domain::entities::file::File, DomainError> {
            unimplemented!()
        }
        
        async fn delete_file(&self, _id: &str) -> Result<(), DomainError> {
            unimplemented!()
        }
        
        async fn find_all_files(&self) -> Result<Vec<crate::domain::entities::file::File>, DomainError> {
            unimplemented!()
        }
    }

    #[async_trait]
    impl FolderStoragePort for MockFolderRepository {
        async fn find_folder_by_id(&self, id: &str) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            if id == "test_folder_id" {
                let folder = crate::domain::entities::folder::Folder::new(
                    id.to_string(),
                    "test".to_string(),
                    "/path/to/test".to_string(),
                    "/test".to_string(),
                    None,
                    None,
                    None,
                )
                .unwrap();
                Ok(folder)
            } else {
                Err(DomainError::NotFound(format!("Folder {} not found", id)))
            }
        }
        
        // Implementación dummy para el resto de métodos requeridos
        async fn find_folders_in_folder(&self, _folder_id: &str) -> Result<Vec<crate::domain::entities::folder::Folder>, DomainError> {
            unimplemented!()
        }
        
        async fn save_folder(&self, _folder: &crate::domain::entities::folder::Folder) -> Result<crate::domain::entities::folder::Folder, DomainError> {
            unimplemented!()
        }
        
        async fn delete_folder(&self, _id: &str) -> Result<(), DomainError> {
            unimplemented!()
        }
        
        async fn find_all_folders(&self) -> Result<Vec<crate::domain::entities::folder::Folder>, DomainError> {
            unimplemented!()
        }
    }

    struct MockShareRepository {
        shares: Mutex<HashMap<String, Share>>,
        tokens: Mutex<HashMap<String, String>>, // token -> id mapping
    }

    impl MockShareRepository {
        fn new() -> Self {
            Self {
                shares: Mutex::new(HashMap::new()),
                tokens: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl ShareStoragePort for MockShareRepository {
        async fn save_share(&self, share: &Share) -> Result<Share, DomainError> {
            let mut shares = self.shares.lock().unwrap();
            let mut tokens = self.tokens.lock().unwrap();
            
            shares.insert(share.id.clone(), share.clone());
            tokens.insert(share.token.clone(), share.id.clone());
            
            Ok(share.clone())
        }
        
        async fn find_share_by_id(&self, id: &str) -> Result<Share, DomainError> {
            let shares = self.shares.lock().unwrap();
            
            shares.get(id)
                .cloned()
                .ok_or_else(|| DomainError::NotFound(format!("Share with ID {} not found", id)))
        }
        
        async fn find_share_by_token(&self, token: &str) -> Result<Share, DomainError> {
            let tokens = self.tokens.lock().unwrap();
            let shares = self.shares.lock().unwrap();
            
            let id = tokens.get(token)
                .ok_or_else(|| DomainError::NotFound(format!("Share with token {} not found", token)))?;
            
            shares.get(id)
                .cloned()
                .ok_or_else(|| DomainError::NotFound(format!("Share with ID {} not found", id)))
        }
        
        async fn find_shares_by_item(&self, item_id: &str, item_type: &ShareItemType) -> Result<Vec<Share>, DomainError> {
            let shares = self.shares.lock().unwrap();
            
            let type_str = item_type.to_string();
            let result: Vec<Share> = shares.values()
                .filter(|s| s.item_id == item_id && s.item_type.to_string() == type_str)
                .cloned()
                .collect();
            
            Ok(result)
        }
        
        async fn update_share(&self, share: &Share) -> Result<Share, DomainError> {
            let mut shares = self.shares.lock().unwrap();
            
            if !shares.contains_key(&share.id) {
                return Err(DomainError::not_found("Share", &share.id));
            }
            
            shares.insert(share.id.clone(), share.clone());
            
            Ok(share.clone())
        }
        
        async fn delete_share(&self, id: &str) -> Result<(), DomainError> {
            let mut shares = self.shares.lock().unwrap();
            let mut tokens = self.tokens.lock().unwrap();
            
            // Find the share to get the token
            let share = shares.get(id)
                .ok_or_else(|| DomainError::not_found("Share", id))?;
            
            // Remove token mapping
            tokens.remove(&share.token);
            
            // Remove the share
            shares.remove(id);
            
            Ok(())
        }
        
        async fn find_shares_by_user(&self, user_id: &str, offset: usize, limit: usize) -> Result<(Vec<Share>, usize), DomainError> {
            let shares = self.shares.lock().unwrap();
            
            let user_shares: Vec<Share> = shares.values()
                .filter(|s| s.created_by == user_id)
                .cloned()
                .collect();
            
            let total = user_shares.len();
            
            // Apply pagination
            let paginated = user_shares.into_iter()
                .skip(offset)
                .take(limit)
                .collect();
            
            Ok((paginated, total))
        }
    }

    #[tokio::test]
    async fn test_create_shared_link() {
        let config = Arc::new(Config {
            base_url: "http://localhost:8085".to_string(),
            storage_path: "/tmp/storage".to_string(),
            log_level: "info".to_string(),
            port: 8085,
            database_url: "".to_string(),
            jwt_secret: "test_secret".to_string(),
            jwt_expiration: 3600,
            enable_cors: false,
            cors_origins: vec![],
        });
        
        let share_repo = Arc::new(MockShareRepository::new());
        let file_repo = Arc::new(MockFileRepository);
        let folder_repo = Arc::new(MockFolderRepository);
        
        let service = ShareService::new(config, share_repo, file_repo, folder_repo);
        
        // Test creating a file share
        let dto = CreateShareDto {
            item_id: "test_file_id".to_string(),
            item_type: "file".to_string(),
            password: Some("secret".to_string()),
            expires_at: None,
            permissions: Some(SharePermissionsDto {
                read: true,
                write: false,
                reshare: false,
            }),
        };
        
        let result = service.create_shared_link("user123", dto).await;
        assert!(result.is_ok());
        
        let share_dto = result.unwrap();
        assert_eq!(share_dto.item_id, "test_file_id");
        assert_eq!(share_dto.item_type, "file");
        assert!(share_dto.has_password);
        assert!(share_dto.url.starts_with("http://localhost:8085/s/"));
    }
}