use crate::common::config::AppConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Database initialization error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct DbError(String);

type Result<T> = std::result::Result<T, DbError>;

/// Segmented database pools.
///
/// `primary` is used for all user-facing request paths (REST, WebDAV, CalDAV,
/// CardDAV).  `maintenance` is a smaller, isolated pool reserved for
/// background / batch operations (verify_integrity, garbage_collect,
/// update_all_users_storage_usage, trash cleanup) so they can never starve
/// interactive requests.
pub struct DbPools {
    /// Pool for user-facing request paths.
    pub primary: PgPool,
    /// Pool for background / batch maintenance tasks.
    pub maintenance: PgPool,
}

/// Create both the primary and maintenance database pools.
///
/// Pending migrations are applied via the primary pool on startup.
/// The maintenance pool shares the same connection string but has its
/// own, smaller budget.
pub async fn create_database_pools(config: &AppConfig) -> Result<DbPools> {
    tracing::info!(
        "Initializing PostgreSQL connections with URL: {}",
        config
            .database
            .connection_string
            .replace("postgres://", "postgres://[user]:[pass]@")
    );

    // --- primary pool ---
    let primary = create_pool_with_retries(
        &config.database.connection_string,
        config.database.max_connections,
        config.database.min_connections,
        config.database.connect_timeout_secs,
        config.database.idle_timeout_secs,
        config.database.max_lifetime_secs,
        "primary",
    )
    .await?;

    // Run pending migrations (idempotent, tracked in _sqlx_migrations table)
    tracing::info!("Running database migrations...");
    if let Err(e) = run_migrations(&primary).await {
        return Err(DbError(format!(
            "Database migrations failed: {}. \
             Check the migrations/ directory for issues.",
            e
        )));
    }
    tracing::info!("Database migrations complete");

    // --- maintenance pool ---
    let maintenance = create_pool_with_retries(
        &config.database.connection_string,
        config.database.maintenance_max_connections,
        config.database.maintenance_min_connections,
        config.database.connect_timeout_secs,
        config.database.idle_timeout_secs,
        config.database.max_lifetime_secs,
        "maintenance",
    )
    .await?;

    tracing::info!(
        "Database pools ready — primary: {} max / {} min, maintenance: {} max / {} min",
        config.database.max_connections,
        config.database.min_connections,
        config.database.maintenance_max_connections,
        config.database.maintenance_min_connections,
    );

    Ok(DbPools {
        primary,
        maintenance,
    })
}

/// Internal helper: create a single pool with retry logic.
async fn create_pool_with_retries(
    connection_string: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout_secs: u64,
    idle_timeout_secs: u64,
    max_lifetime_secs: u64,
    label: &str,
) -> Result<PgPool> {
    let mut attempt = 0;
    const MAX_ATTEMPTS: usize = 5;

    while attempt < MAX_ATTEMPTS {
        attempt += 1;
        tracing::info!(
            "PostgreSQL {} pool connection attempt #{}/{}",
            label,
            attempt,
            MAX_ATTEMPTS
        );

        match PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .acquire_timeout(Duration::from_secs(connect_timeout_secs))
            .idle_timeout(Duration::from_secs(idle_timeout_secs))
            .max_lifetime(Duration::from_secs(max_lifetime_secs))
            .connect(connection_string)
            .await
        {
            Ok(pool) => match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => {
                    tracing::info!("PostgreSQL {} pool established successfully", label);
                    return Ok(pool);
                }
                Err(e) => {
                    tracing::error!("Error verifying {} pool connection: {}", label, e);
                    if attempt >= MAX_ATTEMPTS {
                        return Err(DbError(format!(
                            "Error verifying PostgreSQL {} pool connection: {}",
                            label, e
                        )));
                    }
                }
            },
            Err(e) => {
                tracing::error!(
                    "Error connecting to PostgreSQL {} pool (attempt {}/{}): {}",
                    label,
                    attempt,
                    MAX_ATTEMPTS,
                    e
                );
                if attempt >= MAX_ATTEMPTS {
                    return Err(DbError(format!(
                        "Error in PostgreSQL {} pool connection: {}",
                        label, e
                    )));
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Err(DbError(format!(
        "Could not establish PostgreSQL {} pool connection after {} attempts",
        label, MAX_ATTEMPTS
    )))
}

/// Run pending migrations from the `migrations/` directory.
///
/// Uses sqlx's built-in migration system which tracks applied migrations
/// in a `_sqlx_migrations` table. Each migration runs in its own transaction.
/// Migration files are embedded at compile time via `sqlx::migrate!()`.
async fn run_migrations(pool: &PgPool) -> Result<()> {
    match sqlx::migrate!().run(pool).await {
        Ok(()) => Ok(()),
        Err(e) => Err(DbError(format!("Migration error: {}", e))),
    }
}
