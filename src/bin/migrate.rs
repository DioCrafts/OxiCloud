use sqlx::postgres::PgPoolOptions;
use std::env;
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logging
    tracing_subscriber::fmt::init();

    // Load environment variables (.env.local first, then .env)
    if let Ok(path) = env::var("DOTENV_PATH") {
        dotenv::from_path(Path::new(&path)).ok();
    } else {
        dotenv::from_filename(".env.local").ok();
        dotenv::dotenv().ok();
    }

    // Get DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be configured");

    println!("Connecting to the database...");

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;

    // Run migrations
    println!("Running migrations...");

    // Get the directory from an environment variable or use a default value
    let migrations_dir = env::var("MIGRATIONS_DIR").unwrap_or_else(|_| "./migrations".to_string());
    println!("Migrations directory: {}", migrations_dir);

    // Create a migrator
    let migrator = sqlx::migrate::Migrator::new(Path::new(&migrations_dir))
        .await
        .expect("Could not create the migrator");

    // Run all pending migrations
    migrator.run(&pool).await?;

    println!("Migrations applied successfully");

    Ok(())
}
