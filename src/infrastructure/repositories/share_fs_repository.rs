use std::{path::Path, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{fs, io};

use crate::{
    application::ports::share_ports::ShareStoragePort,
    common::{config::AppConfig, errors::DomainError},
    domain::{
        entities::share::{Share, ShareItemType},
    },
};

// Estructura para almacenar en el sistema de archivos
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShareRecord {
    id: String,
    item_id: String,
    item_type: String,
    token: String,
    password_hash: Option<String>,
    expires_at: Option<u64>,
    permissions_read: bool,
    permissions_write: bool,
    permissions_reshare: bool,
    created_at: u64,
    created_by: String,
    access_count: u64,
}

pub struct ShareFsRepository {
    config: Arc<AppConfig>,
}

impl ShareFsRepository {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    /// Obtiene la ruta del archivo JSON donde se almacenan los enlaces compartidos
    fn get_shares_path(&self) -> String {
        format!("{}/shares.json", self.config.storage_path.display())
    }

    /// Lee todos los enlaces compartidos del archivo JSON
    async fn read_shares(&self) -> Result<Vec<ShareRecord>, io::Error> {
        let path = self.get_shares_path();
        let path = Path::new(&path);

        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(path).await?;
        let shares: Vec<ShareRecord> = serde_json::from_str(&content).unwrap_or_default();

        Ok(shares)
    }

    /// Guarda todos los enlaces compartidos en el archivo JSON
    async fn write_shares(&self, shares: &[ShareRecord]) -> Result<(), io::Error> {
        let path = self.get_shares_path();
        let json = serde_json::to_string_pretty(shares)?;

        // Asegúrate de que el directorio existe
        let dir = Path::new(&path).parent().unwrap();
        if !dir.exists() {
            fs::create_dir_all(dir).await?
        }

        fs::write(path, json).await
    }

    /// Convierte un registro del sistema de archivos a una entidad de dominio
    fn to_entity(&self, record: &ShareRecord) -> Share {
        let item_type = ShareItemType::try_from(record.item_type.as_str())
            .unwrap_or(ShareItemType::File);

        let permissions = crate::domain::entities::share::SharePermissions::new(
            record.permissions_read,
            record.permissions_write,
            record.permissions_reshare,
        );

        Share::from_raw(
            record.id.clone(),
            record.item_id.clone(),
            item_type,
            record.token.clone(),
            record.password_hash.clone(),
            record.expires_at,
            permissions,
            record.created_at,
            record.created_by.clone(),
            record.access_count,
        )
    }

    /// Convierte una entidad de dominio a un registro para el sistema de archivos
    fn to_record(&self, share: &Share) -> ShareRecord {
        ShareRecord {
            id: share.id().to_string(),
            item_id: share.item_id().to_string(),
            item_type: share.item_type().to_string(),
            token: share.token().to_string(),
            password_hash: share.password_hash().map(|s| s.to_string()),
            expires_at: share.expires_at(),
            permissions_read: share.permissions().read(),
            permissions_write: share.permissions().write(),
            permissions_reshare: share.permissions().reshare(),
            created_at: share.created_at(),
            created_by: share.created_by().to_string(),
            access_count: share.access_count(),
        }
    }
}

#[async_trait]
impl ShareStoragePort for ShareFsRepository {
    async fn save_share(&self, share: &Share) -> Result<Share, DomainError> {
        let mut shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        // Verifica si el enlace ya existe
        let existing_index = shares.iter().position(|s| s.id == share.id());

        let record = self.to_record(share);

        if let Some(index) = existing_index {
            // Actualización
            shares[index] = record;
        } else {
            // Inserción
            shares.push(record);
        }

        self.write_shares(&shares).await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        Ok(share.clone())
    }

    async fn find_share_by_id(&self, id: &str) -> Result<Share, DomainError> {
        let shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        let share = shares.iter()
            .find(|s| s.id == id)
            .ok_or_else(|| {
                DomainError::not_found("Share", format!("Share with ID {} not found", id))
            });

        match share {
            Ok(record) => Ok(self.to_entity(record)),
            Err(e) => Err(e),
        }
    }

    async fn find_share_by_token(&self, token: &str) -> Result<Share, DomainError> {
        let shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        let share = shares.iter()
            .find(|s| s.token == token)
            .ok_or_else(|| {
                DomainError::not_found("Share", format!("Share with token {} not found", token))
            });

        match share {
            Ok(record) => Ok(self.to_entity(record)),
            Err(e) => Err(e),
        }
    }

    async fn find_shares_by_item(&self, item_id: &str, item_type: &ShareItemType) -> Result<Vec<Share>, DomainError> {
        let shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        let type_str = item_type.to_string();
        let result: Vec<Share> = shares.iter()
            .filter(|s| s.item_id == item_id && s.item_type == type_str)
            .map(|record| self.to_entity(record))
            .collect();

        Ok(result)
    }

    async fn update_share(&self, share: &Share) -> Result<Share, DomainError> {
        let mut shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        // Busca el índice del enlace a actualizar
        let index = shares.iter().position(|s| s.id == share.id())
            .ok_or_else(|| {
                DomainError::not_found("Share", format!("Share with ID {} not found for update", share.id()))
            })?;

        // Actualiza el registro
        shares[index] = self.to_record(share);

        // Guarda los cambios
        self.write_shares(&shares).await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        Ok(share.clone())
    }

    async fn delete_share(&self, id: &str) -> Result<(), DomainError> {
        let mut shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        // Encuentra el índice del enlace a eliminar
        let initial_len = shares.len();
        shares.retain(|s| s.id != id);

        // Si no se eliminó ningún enlace, significa que no existía
        if shares.len() == initial_len {
            return Err(DomainError::not_found("Share", format!("Share with ID {} not found for deletion", id)));
        }

        // Guarda los cambios
        self.write_shares(&shares).await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        Ok(())
    }

    async fn find_shares_by_user(&self, user_id: &str, offset: usize, limit: usize) -> Result<(Vec<Share>, usize), DomainError> {
        let shares = self.read_shares().await
            .map_err(|e| DomainError::internal_error("Share", e.to_string()))?;

        // Filtra los enlaces del usuario
        let user_shares: Vec<ShareRecord> = shares.into_iter()
            .filter(|s| s.created_by == user_id)
            .collect();

        // Calcula el total
        let total = user_shares.len();

        // Aplica la paginación
        let paginated: Vec<Share> = user_shares.iter()
            .skip(offset)
            .take(limit)
            .map(|record| self.to_entity(record))
            .collect();

        Ok((paginated, total))
    }
}
