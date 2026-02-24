use crate::common::config::AppConfig;
use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

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
/// The schema is applied once via the primary pool.  The maintenance pool
/// shares the same connection string but has its own, smaller budget.
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

    // Apply schema through the primary pool (idempotent)
    tracing::info!("Applying database schema...");
    if let Err(e) = apply_schema(&primary).await {
        return Err(anyhow::anyhow!(
            "Database schema could not be applied: {}. \
             Run manually: psql -f db/schema.sql",
            e
        ));
    }
    tracing::info!("Database schema applied successfully");

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
            Ok(pool) => {
                match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => {
                        tracing::info!("PostgreSQL {} pool established successfully", label);
                        return Ok(pool);
                    }
                    Err(e) => {
                        tracing::error!("Error verifying {} pool connection: {}", label, e);
                        if attempt >= MAX_ATTEMPTS {
                            return Err(anyhow::anyhow!(
                                "Error verifying PostgreSQL {} pool connection: {}",
                                label,
                                e
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!(
                    "Error connecting to PostgreSQL {} pool (attempt {}/{}): {}",
                    label,
                    attempt,
                    MAX_ATTEMPTS,
                    e
                );
                if attempt >= MAX_ATTEMPTS {
                    return Err(anyhow::anyhow!(
                        "Error in PostgreSQL {} pool connection: {}",
                        label,
                        e
                    ));
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not establish PostgreSQL {} pool connection after {} attempts",
        label,
        MAX_ATTEMPTS
    ))
}

/// Apply the embedded schema.sql to the database.
/// First tries `raw_sql` (simple query protocol). If that fails, falls back
/// to splitting the SQL into individual statements and executing them one by one.
async fn apply_schema(pool: &PgPool) -> Result<()> {
    let schema_sql = include_str!("../../db/schema.sql");

    // Attempt 1: raw_sql sends the entire script via the simple query protocol
    match sqlx::raw_sql(schema_sql).execute(pool).await {
        Ok(_) => return Ok(()),
        Err(e) => {
            tracing::warn!(
                "raw_sql failed ({}), falling back to statement-by-statement execution",
                e
            );
        }
    }

    // Attempt 2: split into individual statements respecting dollar-quoting
    let statements = split_sql_statements(schema_sql);
    for (i, stmt) in statements.iter().enumerate() {
        let trimmed = stmt.trim();
        if trimmed.is_empty() || trimmed == ";" {
            continue;
        }
        if let Err(e) = sqlx::raw_sql(trimmed).execute(pool).await {
            let preview = if trimmed.len() > 200 {
                &trimmed[..200]
            } else {
                trimmed
            };
            tracing::error!(
                "Schema statement {} failed: {}\n--- SQL ---\n{}\n-----------",
                i + 1,
                e,
                preview
            );
            return Err(anyhow::anyhow!("Schema statement {} failed: {}", i + 1, e));
        }
    }

    Ok(())
}

/// Split a SQL script into individual statements, correctly handling:
/// - Dollar-quoted blocks (`$BODY$...$BODY$`, `$$...$$`)
/// - Single-quoted strings (`'...'`)
/// - Line comments (`-- ...`)
/// - Block comments (`/* ... */`)
fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Line comment
        if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
            while i < len && chars[i] != '\n' {
                current.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // Block comment
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            current.push(chars[i]);
            current.push(chars[i + 1]);
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                current.push(chars[i]);
                i += 1;
            }
            if i + 1 < len {
                current.push(chars[i]);
                current.push(chars[i + 1]);
                i += 2;
            }
            continue;
        }

        // Single-quoted string
        if chars[i] == '\'' {
            current.push(chars[i]);
            i += 1;
            while i < len {
                current.push(chars[i]);
                if chars[i] == '\'' {
                    if i + 1 < len && chars[i + 1] == '\'' {
                        current.push(chars[i + 1]);
                        i += 2;
                    } else {
                        i += 1;
                        break;
                    }
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // Dollar-quoted string ($tag$...$tag$ or $$...$$)
        if chars[i] == '$' {
            let _start = i;
            i += 1;
            let mut tag = String::from("$");
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                tag.push(chars[i]);
                i += 1;
            }
            if i < len && chars[i] == '$' {
                tag.push('$');
                i += 1;
                // We have a dollar-quote tag, find the closing tag
                current.push_str(&tag);
                loop {
                    if i >= len {
                        break;
                    }
                    if chars[i] == '$' {
                        let remaining = &sql[i..];
                        if remaining.starts_with(&tag) {
                            current.push_str(&tag);
                            i += tag.len();
                            break;
                        }
                    }
                    current.push(chars[i]);
                    i += 1;
                }
            } else {
                // Not a valid dollar-quote, push what we consumed
                current.push_str(&tag);
            }
            continue;
        }

        // Statement separator
        if chars[i] == ';' {
            current.push(';');
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() && trimmed != ";" {
                statements.push(trimmed);
            }
            current.clear();
            i += 1;
            continue;
        }

        current.push(chars[i]);
        i += 1;
    }

    // Trailing statement without semicolon
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() && trimmed != ";" {
        statements.push(trimmed);
    }

    statements
}
