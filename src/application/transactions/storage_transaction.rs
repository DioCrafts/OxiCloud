use std::future::Future;
use std::pin::Pin;
use crate::common::errors::{DomainError, ErrorKind};

/// Tipo para operaciones y rollbacks asíncronos
type TransactionOp = Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send>>;

/// Transacción para operaciones de almacenamiento
/// Permite definir un conjunto de operaciones y sus rollbacks correspondientes
pub struct StorageTransaction {
    /// Operaciones a ejecutar
    operations: Vec<Box<dyn FnOnce() -> TransactionOp + Send>>,
    /// Operaciones de rollback para revertir cambios en caso de error
    rollbacks: Vec<Box<dyn FnOnce() -> TransactionOp + Send>>,
    /// Nombre de la transacción para logging
    name: String,
}

impl StorageTransaction {
    /// Crea una nueva transacción
    pub fn new(name: &str) -> Self {
        Self {
            operations: Vec::new(),
            rollbacks: Vec::new(),
            name: name.to_string(),
        }
    }
    
    /// Añade una operación a la transacción con su correspondiente rollback
    pub fn add_operation<F, R>(&mut self, operation: F, rollback: R)
    where
        F: Future<Output = Result<(), DomainError>> + Send + 'static,
        R: Future<Output = Result<(), DomainError>> + Send + 'static,
    {
        self.operations.push(Box::new(move || Box::pin(operation)));
        self.rollbacks.push(Box::new(move || Box::pin(rollback)));
    }
    
    /// Añade una operación sin rollback (para limpieza o logging)
    pub fn add_finalizer<F>(&mut self, finalizer: F)
    where
        F: Future<Output = Result<(), DomainError>> + Send + 'static,
    {
        // El rollback es una operación nula
        let noop = async { Ok(()) };
        
        self.operations.push(Box::new(move || Box::pin(finalizer)));
        self.rollbacks.push(Box::new(move || Box::pin(noop)));
    }
    
    /// Ejecuta la transacción aplicando todas las operaciones en orden
    /// Si alguna falla, ejecuta los rollbacks en orden inverso
    pub async fn commit(mut self) -> Result<(), DomainError> {
        tracing::debug!("Iniciando transacción: {}", self.name);
        
        let mut completed_ops = Vec::new();
        
        // Extraer operaciones para evitar problemas de propiedad
        let operations = std::mem::take(&mut self.operations);
        let transaction_name = self.name.clone();
        
        // Ejecutar operaciones
        for (i, op) in operations.into_iter().enumerate() {
            match op().await {
                Ok(()) => {
                    completed_ops.push(i);
                    tracing::trace!("Operación {} completada en transacción: {}", i, transaction_name);
                }
                Err(e) => {
                    tracing::error!("Error en operación {} de transacción {}: {}", i, transaction_name, e);
                    
                    // Ejecutar rollbacks para las operaciones completadas en orden inverso
                    self.rollback(completed_ops).await?;
                    
                    return Err(DomainError::new(
                        ErrorKind::InternalError,
                        "Transaction",
                        format!("Falló la transacción '{}': {}", transaction_name, e)
                    ).with_source(e));
                }
            }
        }
        
        tracing::debug!("Transacción completada exitosamente: {}", transaction_name);
        Ok(())
    }
    
    /// Ejecuta rollbacks para las operaciones completadas
    async fn rollback(mut self, completed_ops: Vec<usize>) -> Result<(), DomainError> {
        tracing::warn!("Iniciando rollback para transacción: {}", self.name);
        
        let mut rollback_errors = Vec::new();
        
        // Extraer rollbacks para evitar problemas de propiedad
        let mut rollbacks = Vec::new();
        std::mem::swap(&mut rollbacks, &mut self.rollbacks);
        
        // Ejecutar rollbacks en orden inverso
        for i in completed_ops.into_iter().rev() {
            if i < rollbacks.len() {
                // Tomar propiedad del rollback (obtener una referencia mutable)
                if let Some(rb) = rollbacks.get_mut(i) {
                    // Intercambiar con una función vacía
                    let rollback = std::mem::replace(rb, Box::new(|| Box::pin(async { Ok(()) })));
                    if let Err(e) = rollback().await {
                        tracing::error!("Error en rollback de operación {} en transacción {}: {}", 
                                      i, self.name, e);
                        rollback_errors.push(e);
                    }
                }
            }
        }
        
        // Si hubo errores en el rollback, reportarlos
        if !rollback_errors.is_empty() {
            tracing::error!("Errores durante rollback de transacción {}: {} errores", 
                           self.name, rollback_errors.len());
            
            return Err(DomainError::new(
                ErrorKind::InternalError,
                "Transaction",
                format!("Errores durante rollback de transacción '{}': {} errores", 
                       self.name, rollback_errors.len())
            ));
        }
        
        tracing::info!("Rollback de transacción completado: {}", self.name);
        Ok(())
    }
}