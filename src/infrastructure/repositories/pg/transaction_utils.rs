use sqlx::{Error as SqlxError, PgPool, Postgres, Transaction};
use std::sync::Arc;
use tracing::{debug, error, info};

/// Helper function to execute database operations in a transaction
/// Takes a database pool and a closure that will be executed within a transaction
/// The closure receives a transaction object that should be used for all database operations
/// If the closure returns an error, the transaction is rolled back
/// If the closure returns Ok, the transaction is committed
pub async fn with_transaction<F, T, E>(
    pool: &Arc<PgPool>,
    operation_name: &str,
    operation: F,
) -> Result<T, E>
where
    F: for<'c> FnOnce(
        &'c mut Transaction<'_, Postgres>,
    ) -> futures::future::BoxFuture<'c, Result<T, E>>,
    E: From<SqlxError> + std::fmt::Display,
{
    debug!("Starting database transaction for: {}", operation_name);

    // Begin transaction
    let mut tx = pool.begin().await.map_err(|e| {
        error!("Failed to begin transaction for {}: {}", operation_name, e);
        E::from(e)
    })?;

    // Execute the operation within the transaction
    match operation(&mut tx).await {
        Ok(result) => {
            // If operation succeeds, commit the transaction
            match tx.commit().await {
                Ok(_) => {
                    debug!("Transaction committed successfully for: {}", operation_name);
                    Ok(result)
                }
                Err(e) => {
                    error!("Failed to commit transaction for {}: {}", operation_name, e);
                    Err(E::from(e))
                }
            }
        }
        Err(e) => {
            // If operation fails, rollback the transaction
            if let Err(rollback_err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for {}: {}",
                    operation_name, rollback_err
                );
                // Still return the original error
            } else {
                info!("Transaction rolled back for {}: {}", operation_name, e);
            }
            Err(e)
        }
    }
}
