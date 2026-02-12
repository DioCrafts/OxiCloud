use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use anyhow::Result;
use std::time::Duration;
use crate::common::config::AppConfig;

pub async fn create_database_pool(config: &AppConfig) -> Result<PgPool> {
    tracing::info!("Initializing PostgreSQL connection with URL: {}", 
                  config.database.connection_string.replace("postgres://", "postgres://[user]:[pass]@"));
    
    // Add a more robust connection attempt with retries
    let mut attempt = 0;
    const MAX_ATTEMPTS: usize = 3;
    
    while attempt < MAX_ATTEMPTS {
        attempt += 1;
        tracing::info!("PostgreSQL connection attempt #{}", attempt);
        
        // Create the connection pool with configuration options
        match PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .acquire_timeout(Duration::from_secs(config.database.connect_timeout_secs))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.database.max_lifetime_secs))
            .connect(&config.database.connection_string)
            .await {
                Ok(pool) => {
                    // Verify the connection
                    match sqlx::query("SELECT 1").execute(&pool).await {
                        Ok(_) => {
                            tracing::info!("PostgreSQL connection established successfully");
                            
                            // Verify if migrations have been applied
                            let migration_check = sqlx::query("SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'auth' AND tablename = 'users')")
                                .fetch_one(&pool)
                                .await;
                                
                            match migration_check {
                                Ok(row) => {
                                    let tables_exist: bool = row.get(0);
                                    if !tables_exist {
                                        tracing::warn!("Database tables do not exist. Please run migrations with: cargo run --bin migrate --features migrations");
                                    }
                                },
                                Err(_) => {
                                    tracing::warn!("Could not verify migration status. Please run migrations with: cargo run --bin migrate --features migrations");
                                }
                            }
                            
                            return Ok(pool);
                        },
                        Err(e) => {
                            tracing::error!("Error verifying connection: {}", e);
                            tracing::warn!("The database appears to not be configured. Please run migrations with: cargo run --bin migrate --features migrations");
                            if attempt >= MAX_ATTEMPTS {
                                return Err(anyhow::anyhow!("Error verifying PostgreSQL connection: {}", e));
                            }
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Error connecting to PostgreSQL: {}", e);
                    if attempt >= MAX_ATTEMPTS {
                        return Err(anyhow::anyhow!("Error in PostgreSQL connection: {}", e));
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
    }
    
    Err(anyhow::anyhow!("Could not establish PostgreSQL connection after {} attempts", MAX_ATTEMPTS))
}