use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use anyhow::Result;
use std::time::Duration;
use crate::common::config::AppConfig;

pub async fn create_database_pool(config: &AppConfig) -> Result<PgPool> {
    tracing::info!("Initializing PostgreSQL connection with URL: {}", 
                  config.database.connection_string.replace("postgres://", "postgres://[user]:[pass]@"));
    
    let mut attempt = 0;
    const MAX_ATTEMPTS: usize = 5;
    
    while attempt < MAX_ATTEMPTS {
        attempt += 1;
        tracing::info!("PostgreSQL connection attempt #{}/{}", attempt, MAX_ATTEMPTS);
        
        match PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .acquire_timeout(Duration::from_secs(config.database.connect_timeout_secs))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.database.max_lifetime_secs))
            .connect(&config.database.connection_string)
            .await {
                Ok(pool) => {
                    match sqlx::query("SELECT 1").execute(&pool).await {
                        Ok(_) => {
                            tracing::info!("PostgreSQL connection established successfully");
                            
                            if !tables_exist(&pool).await {
                                tracing::warn!("Database tables do not exist. Auto-applying schema...");
                                if let Err(e) = apply_schema(&pool).await {
                                    return Err(anyhow::anyhow!(
                                        "Database schema could not be applied: {}. \
                                         Run manually: psql -f db/schema.sql", e
                                    ));
                                }
                                
                                // Verify tables were actually created
                                if !tables_exist(&pool).await {
                                    return Err(anyhow::anyhow!(
                                        "Database schema was applied but tables still missing. \
                                         Check db/schema.sql for errors."
                                    ));
                                }
                                tracing::info!("Database schema applied and verified successfully");
                            }
                            
                            return Ok(pool);
                        },
                        Err(e) => {
                            tracing::error!("Error verifying connection: {}", e);
                            if attempt >= MAX_ATTEMPTS {
                                return Err(anyhow::anyhow!("Error verifying PostgreSQL connection: {}", e));
                            }
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Error connecting to PostgreSQL (attempt {}/{}): {}", attempt, MAX_ATTEMPTS, e);
                    if attempt >= MAX_ATTEMPTS {
                        return Err(anyhow::anyhow!("Error in PostgreSQL connection: {}", e));
                    }
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
    }
    
    Err(anyhow::anyhow!("Could not establish PostgreSQL connection after {} attempts", MAX_ATTEMPTS))
}

/// Check whether the core auth tables exist in the database.
async fn tables_exist(pool: &PgPool) -> bool {
    sqlx::query("SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'auth' AND tablename = 'users')")
        .fetch_one(pool)
        .await.map(|row| row.get::<bool, _>(0))
        .unwrap_or(false)
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
            tracing::warn!("raw_sql failed ({}), falling back to statement-by-statement execution", e);
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
            let preview = if trimmed.len() > 200 { &trimmed[..200] } else { trimmed };
            tracing::error!("Schema statement {} failed: {}\n--- SQL ---\n{}\n-----------", i + 1, e, preview);
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