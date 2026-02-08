use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, instrument};

use crate::common::errors::Result;
use crate::domain::repositories::trash_repository::TrashRepository;
use crate::application::ports::trash_ports::TrashUseCase;

/// Servicio para la limpieza automática de elementos expirados en la papelera
pub struct TrashCleanupService {
    trash_service: Arc<dyn TrashUseCase>,
    trash_repository: Arc<dyn TrashRepository>,
    cleanup_interval_hours: u64,
}

impl TrashCleanupService {
    pub fn new(
        trash_service: Arc<dyn TrashUseCase>,
        trash_repository: Arc<dyn TrashRepository>,
        cleanup_interval_hours: u64,
    ) -> Self {
        Self {
            trash_service,
            trash_repository,
            cleanup_interval_hours: cleanup_interval_hours.max(1), // Mínimo 1 hora
        }
    }
    
    /// Inicia el trabajo de limpieza periódica
    #[instrument(skip(self))]
    pub async fn start_cleanup_job(&self) {
        let trash_repository = self.trash_repository.clone();
        let trash_service = self.trash_service.clone();
        let interval_hours = self.cleanup_interval_hours;
        
        info!("Iniciando trabajo de limpieza de papelera con intervalo de {} horas", interval_hours);
        
        tokio::spawn(async move {
            let interval_duration = Duration::from_secs(interval_hours * 60 * 60);
            let mut interval = time::interval(interval_duration);
            
            // Primera ejecución inmediata
            Self::cleanup_expired_items(trash_repository.clone(), trash_service.clone()).await
                .unwrap_or_else(|e| error!("Error en la limpieza inicial de la papelera: {:?}", e));
            
            loop {
                interval.tick().await;
                debug!("Ejecutando tarea programada de limpieza de papelera");
                
                if let Err(e) = Self::cleanup_expired_items(
                    trash_repository.clone(), 
                    trash_service.clone()
                ).await {
                    error!("Error en la limpieza programada de la papelera: {:?}", e);
                }
            }
        });
    }
    
    /// Limpia los elementos expirados en la papelera
    #[instrument(skip(trash_repository, trash_service))]
    async fn cleanup_expired_items(
        trash_repository: Arc<dyn TrashRepository>,
        trash_service: Arc<dyn TrashUseCase>,
    ) -> Result<()> {
        debug!("Comenzando limpieza de elementos expirados en la papelera");
        
        // Obtener todos los elementos expirados
        let expired_items = trash_repository.get_expired_items().await?;
        
        if expired_items.is_empty() {
            debug!("No hay elementos expirados para limpiar");
            return Ok(());
        }
        
        info!("Encontrados {} elementos expirados para eliminar", expired_items.len());
        
        // Eliminar cada elemento expirado
        for item in expired_items {
            let trash_id = item.id().to_string();
            let user_id = item.user_id().to_string();
            
            debug!("Eliminando elemento expirado: id={}, user={}", trash_id, user_id);
            
            // Si falla una eliminación, continuar con las demás
            if let Err(e) = trash_service.delete_permanently(&trash_id, &user_id).await {
                error!("Error eliminando elemento expirado {}: {:?}", trash_id, e);
            } else {
                debug!("Elemento expirado eliminado correctamente: {}", trash_id);
            }
        }
        
        info!("Limpieza de papelera completada");
        Ok(())
    }
}