use async_trait::async_trait;
use crate::common::errors::Result;
use crate::application::dtos::recent_dto::RecentItemDto;

/// Define operaciones para gestionar elementos recientes del usuario
#[async_trait]
pub trait RecentItemsUseCase: Send + Sync {
    /// Obtener todos los elementos recientes de un usuario
    async fn get_recent_items(&self, user_id: &str, limit: Option<i32>) -> Result<Vec<RecentItemDto>>;
    
    /// Registrar acceso a un elemento
    async fn record_item_access(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;
    
    /// Eliminar un elemento de recientes
    async fn remove_from_recent(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;
    
    /// Limpiar toda la lista de elementos recientes
    async fn clear_recent_items(&self, user_id: &str) -> Result<()>;
}

// ─────────────────────────────────────────────────────
// Outbound port — persistence abstraction
// ─────────────────────────────────────────────────────

/// Puerto secundario (outbound) para persistencia de elementos recientes.
///
/// Abstrae el acceso a la tabla `auth.user_recent_files` para que
/// `RecentService` no dependa directamente de `PgPool`.
#[async_trait]
pub trait RecentItemsRepositoryPort: Send + Sync + 'static {
    /// Obtiene los últimos elementos recientes de un usuario (ordenados por fecha desc).
    async fn get_recent_items(&self, user_id: &str, limit: i32) -> Result<Vec<RecentItemDto>>;

    /// Registra/actualiza el acceso a un ítem (upsert por user+item+type).
    async fn upsert_access(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<()>;

    /// Elimina un ítem de recientes. Devuelve `true` si existía.
    async fn remove_item(&self, user_id: &str, item_id: &str, item_type: &str) -> Result<bool>;

    /// Elimina todos los elementos recientes de un usuario.
    async fn clear_all(&self, user_id: &str) -> Result<()>;

    /// Elimina elementos que excedan `max_items` (los más antiguos).
    async fn prune(&self, user_id: &str, max_items: i32) -> Result<()>;
}