use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use anyhow::Result;
use std::time::Duration;
use crate::common::config::AppConfig;

pub async fn create_database_pool(config: &AppConfig) -> Result<PgPool> {
    tracing::info!("Inicializando conexión a PostgreSQL con URL: {}", 
                  config.database.connection_string.replace("postgres://", "postgres://[user]:[pass]@"));
    
    // Add a more robust connection attempt with retries
    let mut attempt = 0;
    const MAX_ATTEMPTS: usize = 3;
    
    while attempt < MAX_ATTEMPTS {
        attempt += 1;
        tracing::info!("Intento de conexión a PostgreSQL #{}", attempt);
        
        // Crear el pool de conexiones con las opciones de configuración
        match PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .acquire_timeout(Duration::from_secs(config.database.connect_timeout_secs))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.database.max_lifetime_secs))
            .connect(&config.database.connection_string)
            .await {
                Ok(pool) => {
                    // Verificar la conexión
                    match sqlx::query("SELECT 1").execute(&pool).await {
                        Ok(_) => {
                            tracing::info!("Conexión a PostgreSQL establecida correctamente");
                            
                            // Verify if migrations have been applied
                            let migration_check = sqlx::query("SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'auth' AND tablename = 'users')")
                                .fetch_one(&pool)
                                .await;
                                
                            match migration_check {
                                Ok(row) => {
                                    let tables_exist: bool = row.get(0);
                                    if !tables_exist {
                                        tracing::warn!("Las tablas de la base de datos no existen. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
                                    }
                                },
                                Err(_) => {
                                    tracing::warn!("No se pudo verificar el estado de las migraciones. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
                                }
                            }
                            
                            return Ok(pool);
                        },
                        Err(e) => {
                            tracing::error!("Error al verificar conexión: {}", e);
                            tracing::warn!("La base de datos parece no estar configurada. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
                            if attempt >= MAX_ATTEMPTS {
                                return Err(anyhow::anyhow!("Error al verificar la conexión a PostgreSQL: {}", e));
                            }
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Error al conectar a PostgreSQL: {}", e);
                    if attempt >= MAX_ATTEMPTS {
                        return Err(anyhow::anyhow!("Error en la conexión a PostgreSQL: {}", e));
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
    }
    
    Err(anyhow::anyhow!("No se pudo establecer la conexión a PostgreSQL después de {} intentos", MAX_ATTEMPTS))
}