use std::fmt;
use log::{warn, log_enabled, Level};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseSetupError {
    #[error("MS SQL username and/or password not valid: {0}")]
    InvalidCredentials(String),
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Database query error: {0}")]
    QueryError(String),
    #[error("Database error: {0}")]
    GenericError(String),
}

pub struct MssqlConnection {
    conn: tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
}

impl MssqlConnection {
    pub async fn connect(host: &str, database: &str, user: &str, password: &str) -> Result<Self, DatabaseSetupError> {
        let mut config = tiberius::Config::new();
        config.host(host);
        config.database(database);
        config.authentication(tiberius::AuthMethod::sql_server(user, password));

        let tcp = tokio::net::TcpStream::connect(host)
            .await
            .map_err(|e| DatabaseSetupError::ConnectionError(e.to_string()))?;
        tcp.set_nodelay(true)
            .map_err(|e| DatabaseSetupError::ConnectionError(e.to_string()))?;

        let client = tiberius::Client::connect(config, tcp.compat())
            .await
            .map_err(|e| DatabaseSetupError::ConnectionError(e.to_string()))?;

        Ok(Self { conn: client })
    }

    pub async fn execute(&mut self, query: &str) -> Result<(), DatabaseSetupError> {
        self.conn.execute(query, &[])
            .await
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e, query);
                if log_enabled!(Level::Warn) {
                    warn!("{}", entry);
                }
                DatabaseSetupError::QueryError(entry)
            })?;
        
        Ok(())
    }

    pub async fn query_exists(&mut self, query: &str) -> Result<bool, DatabaseSetupError> {
        let result = self.conn.query(query, &[])
            .await
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e, query);
                if log_enabled!(Level::Warn) {
                    warn!("{}", entry);
                }
                DatabaseSetupError::QueryError(entry)
            })?;

        let row = result.into_row()
            .await
            .map_err(|e| {
                let entry = format!("DB Error: \"{}\"\nOffending command was: {}", e, query);
                if log_enabled!(Level::Warn) {
                    warn!("{}", entry);
                }
                DatabaseSetupError::QueryError(entry)
            })?;

        Ok(row.is_some())
    }
}

pub trait Translator {
    fn t(&self, text: &str, params: Vec<String>) -> String;
}

pub trait Config {
    fn set_value(key: &str, value: &str);
}

pub struct OcConfig;

impl Config for OcConfig {
    fn set_value(key: &str, value: &str) {
        // Implementation would depend on the actual OC_Config implementation
        // This is a placeholder
    }
}

pub trait DatabaseStructure {
    fn create_db_from_structure(file_path: &str) -> Result<(), DatabaseSetupError>;
}

pub struct OcDb;

impl DatabaseStructure for OcDb {
    fn create_db_from_structure(file_path: &str) -> Result<(), DatabaseSetupError> {
        // Implementation would depend on the actual OC_DB implementation
        // This is a placeholder
        Ok(())
    }
}

pub struct AbstractDatabase {
    db_host: String,
    db_user: String,
    db_password: String,
    db_name: String,
    table_prefix: String,
    db_definition_file: String,
    translator: Box<dyn Translator>,
}

pub struct MsSql {
    db_pretty_name: String,
    base: AbstractDatabase,
}

impl MsSql {
    pub fn new(
        db_host: String, 
        db_user: String, 
        db_password: String, 
        db_name: String, 
        table_prefix: String, 
        db_definition_file: String,
        translator: Box<dyn Translator>
    ) -> Self {
        Self {
            db_pretty_name: "MS SQL Server".to_string(),
            base: AbstractDatabase {
                db_host,
                db_user,
                db_password,
                db_name,
                table_prefix,
                db_definition_file,
                translator,
            },
        }
    }

    pub async fn setup_database(&self) -> Result<(), DatabaseSetupError> {
        // Check if the database user has admin right
        let mut master_connection = MssqlConnection::connect(
            &self.base.db_host, 
            "master", 
            &self.base.db_user, 
            &self.base.db_password
        ).await.map_err(|e| {
            DatabaseSetupError::InvalidCredentials(
                self.base.translator.t(
                    "You need to enter either an existing account or the administrator.",
                    vec![]
                )
            )
        })?;

        OcConfig::set_value("dbuser", &self.base.db_user);
        OcConfig::set_value("dbpassword", &self.base.db_password);

        self.create_db_login(&mut master_connection).await?;
        self.create_database(&mut master_connection).await?;
        self.create_db_user(&mut master_connection).await?;

        // Drop connection explicitly
        drop(master_connection);

        self.create_database_structure().await?;

        Ok(())
    }

    async fn create_db_login(&self, connection: &mut MssqlConnection) -> Result<(), DatabaseSetupError> {
        let query = format!(
            "SELECT * FROM master.sys.server_principals WHERE name = '{}';", 
            self.base.db_user
        );
        
        let user_exists = connection.query_exists(&query).await?;
        
        if !user_exists {
            let create_query = format!(
                "CREATE LOGIN [{}] WITH PASSWORD = '{}';", 
                self.base.db_user, 
                self.base.db_password
            );
            
            connection.execute(&create_query).await?;
        }
        
        Ok(())
    }

    async fn create_db_user(&self, connection: &mut MssqlConnection) -> Result<(), DatabaseSetupError> {
        let query = format!(
            "SELECT * FROM [{}].sys.database_principals WHERE name = '{}';", 
            self.base.db_name, 
            self.base.db_user
        );
        
        let user_exists = connection.query_exists(&query).await?;
        
        if !user_exists {
            let create_query = format!(
                "USE [{}]; CREATE USER [{}] FOR LOGIN [{}];", 
                self.base.db_name, 
                self.base.db_user, 
                self.base.db_user
            );
            
            connection.execute(&create_query).await?;
        }
        
        let role_query = format!(
            "USE [{}]; EXEC sp_addrolemember 'db_owner', '{}';", 
            self.base.db_name, 
            self.base.db_user
        );
        
        connection.execute(&role_query).await?;
        
        Ok(())
    }

    async fn create_database(&self, connection: &mut MssqlConnection) -> Result<(), DatabaseSetupError> {
        let query = format!("CREATE DATABASE [{}];", self.base.db_name);
        
        connection.execute(&query).await?;
        
        Ok(())
    }

    async fn create_database_structure(&self) -> Result<(), DatabaseSetupError> {
        let mut connection = MssqlConnection::connect(
            &self.base.db_host, 
            &self.base.db_name, 
            &self.base.db_user, 
            &self.base.db_password
        ).await?;

        // Fill the database if needed
        let query = format!(
            "SELECT * FROM INFORMATION_SCHEMA.TABLES \
             WHERE TABLE_SCHEMA = '{}' \
             AND TABLE_NAME = '{}users'", 
            self.base.db_name, 
            self.base.table_prefix
        );

        let table_exists = connection.query_exists(&query).await?;
        
        if !table_exists {
            OcDb::create_db_from_structure(&self.base.db_definition_file)?;
        }
        
        // Drop connection explicitly
        drop(connection);
        
        Ok(())
    }
}