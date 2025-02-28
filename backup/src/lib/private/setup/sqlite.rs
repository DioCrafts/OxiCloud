use std::fs;
use std::path::Path;

use async_trait::async_trait;
use log::error;

use crate::config;
use crate::db;
use crate::setup::abstract_database::{AbstractDatabase, DatabaseResult};

pub struct Sqlite {
    pub db_pretty_name: &'static str,
    pub db_definition_file: String,
}

impl Sqlite {
    pub fn new(db_definition_file: String) -> Self {
        Self {
            db_pretty_name: "Sqlite",
            db_definition_file,
        }
    }
}

#[async_trait]
impl AbstractDatabase for Sqlite {
    async fn validate(&self, _config: &config::Config) -> DatabaseResult<Vec<String>> {
        // No validation needed for SQLite
        Ok(vec![])
    }

    async fn initialize(&self, _config: &config::Config) -> DatabaseResult<()> {
        // No initialization needed for SQLite
        Ok(())
    }

    async fn setup_database(&self, _username: &str) -> DatabaseResult<()> {
        let datadir = config::get_value("datadirectory")?;
        let db_path = Path::new(&datadir).join("owncloud.db");

        // Delete the old sqlite database first, might cause infinite loops otherwise
        if db_path.exists() {
            fs::remove_file(&db_path)?;
        }

        // In case of sqlite, we can always fill the database
        error!("creating sqlite db");
        db::create_db_from_structure(&self.db_definition_file).await?;

        Ok(())
    }
}