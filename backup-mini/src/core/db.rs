//! Utilidades para interacción con la base de datos

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::core::config::get_config;

/// Inicializa el pool de conexiones a la base de datos
pub async fn init_db_pool() -> Result<PgPool> {
    let config = get_config();
    let db_url = &config.database.url;
    let max_connections = config.database.max_connections;
    
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await?;
    
    Ok(pool)
}

/// Ejecuta las migraciones de la base de datos
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Comprueba la conexión a la base de datos
pub async fn check_connection(pool: &PgPool) -> Result<()> {
    // Ejecutar una consulta simple para verificar
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

/// Transacción con callback
pub async fn transaction<F, T, E>(pool: &PgPool, f: F) -> Result<T, E>
where
    F: for<'a> FnOnce(&'a mut sqlx::Transaction<'_, sqlx::Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'a>>,
    E: From<sqlx::Error>,
{
    let mut tx = pool.begin().await?;
    let result = f(&mut tx).await;
    
    match result {
        Ok(value) => {
            tx.commit().await?;
            Ok(value)
        }
        Err(error) => {
            // El rollback ocurre automáticamente cuando tx se descarta
            Err(error)
        }
    }
}