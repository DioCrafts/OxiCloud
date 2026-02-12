use std::future::Future;
use std::pin::Pin;
use crate::common::errors::{DomainError, ErrorKind};

/// Type for async operations and rollbacks
type TransactionOp = Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send>>;

/// Transaction for storage operations
/// Allows defining a set of operations and their corresponding rollbacks
pub struct StorageTransaction {
    /// Operations to execute
    operations: Vec<Box<dyn FnOnce() -> TransactionOp + Send>>,
    /// Rollback operations to revert changes in case of error
    rollbacks: Vec<Box<dyn FnOnce() -> TransactionOp + Send>>,
    /// Transaction name for logging
    name: String,
}

impl StorageTransaction {
    /// Creates a new transaction
    pub fn new(name: &str) -> Self {
        Self {
            operations: Vec::new(),
            rollbacks: Vec::new(),
            name: name.to_string(),
        }
    }
    
    /// Adds an operation to the transaction with its corresponding rollback
    pub fn add_operation<F, R>(&mut self, operation: F, rollback: R)
    where
        F: Future<Output = Result<(), DomainError>> + Send + 'static,
        R: Future<Output = Result<(), DomainError>> + Send + 'static,
    {
        self.operations.push(Box::new(move || Box::pin(operation)));
        self.rollbacks.push(Box::new(move || Box::pin(rollback)));
    }
    
    /// Adds an operation without rollback (for cleanup or logging)
    pub fn add_finalizer<F>(&mut self, finalizer: F)
    where
        F: Future<Output = Result<(), DomainError>> + Send + 'static,
    {
        // The rollback is a no-op
        let noop = async { Ok(()) };
        
        self.operations.push(Box::new(move || Box::pin(finalizer)));
        self.rollbacks.push(Box::new(move || Box::pin(noop)));
    }
    
    /// Executes the transaction by applying all operations in order
    /// If any fails, executes rollbacks in reverse order
    pub async fn commit(mut self) -> Result<(), DomainError> {
        tracing::debug!("Starting transaction: {}", self.name);
        
        let mut completed_ops = Vec::new();
        
        // Extract operations to avoid ownership issues
        let operations = std::mem::take(&mut self.operations);
        let transaction_name = self.name.clone();
        
        // Execute operations
        for (i, op) in operations.into_iter().enumerate() {
            match op().await {
                Ok(()) => {
                    completed_ops.push(i);
                    tracing::trace!("Operation {} completed in transaction: {}", i, transaction_name);
                }
                Err(e) => {
                    tracing::error!("Error in operation {} of transaction {}: {}", i, transaction_name, e);
                    
                    // Execute rollbacks for completed operations in reverse order
                    self.rollback(completed_ops).await?;
                    
                    return Err(DomainError::new(
                        ErrorKind::InternalError,
                        "Transaction",
                        format!("Transaction '{}' failed: {}", transaction_name, e)
                    ).with_source(e));
                }
            }
        }
        
        tracing::debug!("Transaction completed successfully: {}", transaction_name);
        Ok(())
    }
    
    /// Executes rollbacks for completed operations
    async fn rollback(mut self, completed_ops: Vec<usize>) -> Result<(), DomainError> {
        tracing::warn!("Starting rollback for transaction: {}", self.name);
        
        let mut rollback_errors = Vec::new();
        
        // Extract rollbacks to avoid ownership issues
        let mut rollbacks = Vec::new();
        std::mem::swap(&mut rollbacks, &mut self.rollbacks);
        
        // Execute rollbacks in reverse order
        for i in completed_ops.into_iter().rev() {
            if i < rollbacks.len() {
                // Take ownership of the rollback (get a mutable reference)
                if let Some(rb) = rollbacks.get_mut(i) {
                    // Swap with an empty function
                    let rollback = std::mem::replace(rb, Box::new(|| Box::pin(async { Ok(()) })));
                    if let Err(e) = rollback().await {
                        tracing::error!("Error in rollback of operation {} in transaction {}: {}", 
                                      i, self.name, e);
                        rollback_errors.push(e);
                    }
                }
            }
        }
        
        // If there were errors during rollback, report them
        if !rollback_errors.is_empty() {
            tracing::error!("Errors during transaction rollback {}: {} errors", 
                           self.name, rollback_errors.len());
            
            return Err(DomainError::new(
                ErrorKind::InternalError,
                "Transaction",
                format!("Errors during transaction '{}' rollback: {} errors", 
                       self.name, rollback_errors.len())
            ));
        }
        
        tracing::info!("Transaction rollback completed: {}", self.name);
        Ok(())
    }
}