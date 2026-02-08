use std::sync::Arc;
use async_trait::async_trait;
use tracing::info;
use crate::common::errors::{Result, DomainError, ErrorKind};
use crate::application::ports::recent_ports::{RecentItemsUseCase, RecentItemsRepositoryPort};
use crate::application::dtos::recent_dto::RecentItemDto;

/// Implementación del caso de uso para gestionar elementos recientes.
///
/// Depende de `RecentItemsRepositoryPort` (outbound port) en lugar
/// de acceder directamente a `PgPool`, siguiendo la arquitectura hexagonal.
pub struct RecentService {
    repo: Arc<dyn RecentItemsRepositoryPort>,
    max_recent_items: i32,
}

impl RecentService {
    /// Crear un nuevo servicio de elementos recientes
    pub fn new(repo: Arc<dyn RecentItemsRepositoryPort>, max_recent_items: i32) -> Self {
        Self {
            repo,
            max_recent_items: max_recent_items.max(1).min(100),
        }
    }
}

#[async_trait]
impl RecentItemsUseCase for RecentService {
    /// Obtener elementos recientes de un usuario
    async fn get_recent_items(&self, user_id: &str, limit: Option<i32>) -> Result<Vec<RecentItemDto>> {
        info!("Obteniendo elementos recientes para usuario: {}", user_id);
        let limit_value = limit.unwrap_or(self.max_recent_items).min(self.max_recent_items);
        let items = self.repo.get_recent_items(user_id, limit_value).await?;
        info!("Recuperados {} elementos recientes para usuario {}", items.len(), user_id);
        Ok(items)
    }

    /// Registrar acceso a un elemento
    async fn record_item_access(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()> {
        info!("Registrando acceso a {} '{}' para usuario {}", item_type, item_id, user_id);

        if item_type != "file" && item_type != "folder" {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "RecentItems",
                "El tipo de elemento debe ser 'file' o 'folder'",
            ));
        }

        self.repo.upsert_access(user_id, item_id, item_type).await?;
        self.repo.prune(user_id, self.max_recent_items).await?;

        info!("Registrado correctamente acceso a {} '{}' para usuario {}", item_type, item_id, user_id);
        Ok(())
    }

    /// Eliminar un elemento de recientes
    async fn remove_from_recent(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool> {
        info!("Eliminando {} '{}' de recientes para usuario {}", item_type, item_id, user_id);
        let removed = self.repo.remove_item(user_id, item_id, item_type).await?;
        info!(
            "{} {} '{}' de recientes para usuario {}",
            if removed { "Eliminado correctamente" } else { "No se encontró" },
            item_type, item_id, user_id
        );
        Ok(removed)
    }

    /// Limpiar todos los elementos recientes
    async fn clear_recent_items(&self, user_id: &str) -> Result<()> {
        info!("Limpiando todos los elementos recientes para usuario {}", user_id);
        self.repo.clear_all(user_id).await?;
        info!("Limpiados todos los elementos recientes para usuario {}", user_id);
        Ok(())
    }
}