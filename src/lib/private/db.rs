//! ownCloud
//!
//! @author Frank Karlitschek
//! @copyright 2012 Frank Karlitschek frank@owncloud.org
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex, Once};
use std::path::Path;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, MySql, Sqlite, Any, AnyPool, query};
use sqlx::database::Database;
use sqlx::error::Error as SqlxError;
use sqlx::query::Query;
use once_cell::sync::Lazy;

pub const MDB2_SCHEMA_DUMP_STRUCTURE: &str = "1";

#[derive(Debug)]
pub struct DatabaseException {
    message: String,
    query: Option<String>,
}

impl DatabaseException {
    pub fn new(message: impl Into<String>, query: Option<String>) -> Self {
        Self {
            message: message.into(),
            query,
        }
    }

    pub fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}

impl Display for DatabaseException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl Error for DatabaseException {}

#[derive(Debug, Clone)]
pub enum DbType {
    Sqlite,
    Mysql,
    Postgres,
    Oracle,
    Mssql,
}

pub type DbResult<T> = Result<T, DatabaseException>;

#[derive(Debug, Clone)]
pub struct StatementWrapper {
    sql: String,
    limit: Option<i64>,
    offset: Option<i64>,
    is_manipulation: bool,
}

impl StatementWrapper {
    pub fn new(sql: String, limit: Option<i64>, offset: Option<i64>, is_manipulation: bool) -> Self {
        Self {
            sql,
            limit,
            offset,
            is_manipulation,
        }
    }

    pub async fn execute<'a>(&self, pool: &AnyPool, params: Option<Vec<sqlx::Any>>) -> DbResult<u64> {
        let mut query = sqlx::query(&self.sql);
        
        if let Some(parameters) = params {
            for param in parameters {
                query = query.bind(param);
            }
        }
        
        let result = query.execute(pool).await
            .map_err(|e| DatabaseException::new(e.to_string(), Some(self.sql.clone())))?;
            
        Ok(result.rows_affected())
    }
}

#[async_trait]
pub trait MDB2SchemaManager {
    async fn get_db_structure(&self, file: &str) -> DbResult<bool>;
    async fn create_db_from_structure(&self, file: &str) -> DbResult<bool>;
    async fn update_db_from_structure(&self, file: &str) -> DbResult<bool>;
    async fn drop_table(&self, table_name: &str) -> DbResult<()>;
    async fn remove_db_structure(&self, file: &str) -> DbResult<()>;
    async fn replace_db(&self, file: &str) -> DbResult<()>;
}

pub struct DB {
    connection_pool: Option<AnyPool>,
    prefix: String,
    db_type: DbType,
}

static DB_INSTANCE: Lazy<Mutex<DB>> = Lazy::new(|| {
    Mutex::new(DB {
        connection_pool: None,
        prefix: "oc_".to_string(),
        db_type: DbType::Sqlite,
    })
});

impl DB {
    pub async fn connect() -> DbResult<bool> {
        let mut db = DB_INSTANCE.lock().unwrap();
        
        if db.connection_pool.is_some() {
            return Ok(true);
        }

        // The global data we need
        let name = OC_Config::get_value("dbname", "owncloud");
        let host = OC_Config::get_value("dbhost", "");
        let user = OC_Config::get_value("dbuser", "");
        let pass = OC_Config::get_value("dbpassword", "");
        let db_type_str = OC_Config::get_value("dbtype", "sqlite");
        
        let (host, port) = if host.contains(':') {
            let parts: Vec<&str> = host.split(':').collect();
            (parts[0].to_string(), parts[1].parse::<u16>().unwrap_or(0))
        } else {
            (host, 0)
        };

        let table_prefix = OC_Config::get_value("dbtableprefix", "oc_");
        
        let pool = match db_type_str.as_str() {
            "sqlite" | "sqlite3" => {
                let datadir = OC_Config::get_value("datadirectory", 
                    format!("{}/data", OC::server_root()));
                let db_path = format!("{}/{}.db", datadir, name);
                
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(10)
                    .connect(&format!("sqlite:{}", db_path))
                    .await
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?;
                
                db.db_type = DbType::Sqlite;
                // SQLite connection pool converted to Any
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(10)
                    .connect(&format!("sqlite:{}", db_path))
                    .await
                    .map(|p| sqlx::Any::from(p))
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?
            },
            "mysql" => {
                let connection_string = format!(
                    "mysql://{}:{}@{}:{}/{}?charset=UTF8",
                    user, pass, host, port, name
                );
                
                db.db_type = DbType::Mysql;
                sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(10)
                    .connect(&connection_string)
                    .await
                    .map(|p| sqlx::Any::from(p))
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?
            },
            "pgsql" => {
                let connection_string = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    user, pass, host, port, name
                );
                
                db.db_type = DbType::Postgres;
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(10)
                    .connect(&connection_string)
                    .await
                    .map(|p| sqlx::Any::from(p))
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?
            },
            "oci" => {
                // Oracle support would require a specific driver
                // This is a placeholder as Oracle support isn't available in sqlx
                db.db_type = DbType::Oracle;
                return Err(DatabaseException::new("Oracle support not implemented", None));
            },
            "mssql" => {
                // MSSQL support would require a specific driver
                // This is a placeholder as MSSQL support isn't available in sqlx
                db.db_type = DbType::Mssql;
                return Err(DatabaseException::new("MSSQL support not implemented", None));
            },
            _ => {
                return Err(DatabaseException::new("Unknown database type", None));
            }
        };

        db.connection_pool = Some(pool);
        db.prefix = table_prefix;
        
        Ok(true)
    }
    
    pub fn get_connection() -> DbResult<AnyPool> {
        let db = DB_INSTANCE.lock().unwrap();
        match &db.connection_pool {
            Some(pool) => Ok(pool.clone()),
            None => Err(DatabaseException::new("Database connection not established", None)),
        }
    }
    
    pub fn get_mdb2_schema_manager() -> impl MDB2SchemaManager {
        // Implementation would depend on the concrete schema manager
        DefaultMDB2SchemaManager {}
    }
    
    pub async fn prepare(query: &str, limit: Option<i64>, offset: Option<i64>, is_manipulation: Option<bool>) -> DbResult<StatementWrapper> {
        let is_manipulation = is_manipulation.unwrap_or_else(|| Self::is_manipulation(query));
        
        Ok(StatementWrapper::new(
            query.to_string(),
            limit,
            offset,
            is_manipulation,
        ))
    }
    
    pub fn is_manipulation(sql: &str) -> bool {
        let sql_lower = sql.to_lowercase();
        
        if let Some(pos) = sql_lower.find("select") {
            if pos < 10 {
                return false;
            }
        }
        
        for keyword in &["insert", "update", "delete"] {
            if let Some(pos) = sql_lower.find(keyword) {
                if pos < 10 {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub async fn execute_audited(stmt: impl Into<StmtParam>, parameters: Option<Vec<sqlx::Any>>) -> DbResult<u64> {
        let stmt_param = stmt.into();
        let pool = Self::get_connection()?;
        
        match stmt_param {
            StmtParam::String(sql) => {
                if sql.to_lowercase().contains("limit") {
                    return Err(DatabaseException::new(
                        "LIMIT and OFFSET are forbidden for portability reasons, pass an array with 'limit' and 'offset' instead",
                        Some(sql)
                    ));
                }
                
                let wrapper = Self::prepare(&sql, None, None, None).await?;
                wrapper.execute(&pool, parameters).await
            },
            StmtParam::Array(stmt_array) => {
                if !stmt_array.contains_key("sql") {
                    return Err(DatabaseException::new(
                        "statement array must at least contain key 'sql'",
                        None
                    ));
                }
                
                let sql = stmt_array.get("sql").unwrap().to_string();
                let limit = stmt_array.get("limit").and_then(|l| l.parse::<i64>().ok());
                let offset = stmt_array.get("offset").and_then(|o| o.parse::<i64>().ok());
                
                let wrapper = Self::prepare(&sql, limit, offset, None).await?;
                wrapper.execute(&pool, parameters).await
            },
            StmtParam::StatementWrapper(wrapper) => {
                wrapper.execute(&pool, parameters).await
            },
        }
    }
    
    pub async fn insertid(table: Option<&str>) -> DbResult<i64> {
        let pool = Self::get_connection()?;
        let db = DB_INSTANCE.lock().unwrap();
        
        match db.db_type {
            DbType::Sqlite => {
                let result = sqlx::query("SELECT last_insert_rowid()")
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?;
                
                Ok(result.get::<i64, _>(0))
            },
            DbType::Mysql => {
                let result = sqlx::query("SELECT LAST_INSERT_ID()")
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?;
                
                Ok(result.get::<i64, _>(0))
            },
            DbType::Postgres => {
                let sequence_name = match table {
                    Some(table_name) => format!("{}_id_seq", table_name.replace(&db.prefix, "")),
                    None => "last_value".to_string(),
                };
                
                let result = sqlx::query(&format!("SELECT currval('{}')", sequence_name))
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| DatabaseException::new(e.to_string(), None))?;
                
                Ok(result.get::<i64, _>(0))
            },
            _ => Err(DatabaseException::new("insertid not implemented for this database type", None)),
        }
    }
    
    pub async fn insert_if_not_exist(table: &str, input: std::collections::HashMap<String, String>) -> DbResult<u64> {
        let pool = Self::get_connection()?;
        let db = DB_INSTANCE.lock().unwrap();
        
        let columns: Vec<String> = input.keys().cloned().collect();
        let values: Vec<String> = input.values().cloned().collect();
        
        let column_list = columns.join(", ");
        let placeholder_list = (0..values.len()).map(|_| "?").collect::<Vec<&str>>().join(", ");
        
        let where_conditions = columns.iter()
            .enumerate()
            .map(|(i, col)| format!("{} = ${}", col, i + 1))
            .collect::<Vec<String>>()
            .join(" AND ");
        
        // This is a simplified version - real implementation would vary by DB type
        let sql = format!(
            "INSERT INTO {} ({}) SELECT {} WHERE NOT EXISTS (SELECT 1 FROM {} WHERE {})",
            table, column_list, placeholder_list, table, where_conditions
        );
        
        let mut query = sqlx::query(&sql);
        for value in values {
            query = query.bind(value);
        }
        
        let result = query.execute(&pool)
            .await
            .map_err(|e| DatabaseException::new(e.to_string(), Some(sql)))?;
        
        Ok(result.rows_affected())
    }
    
    pub async fn begin_transaction() -> DbResult<()> {
        let pool = Self::get_connection()?;
        
        sqlx::query("BEGIN")
            .execute(&pool)
            .await
            .map_err(|e| DatabaseException::new(e.to_string(), Some("BEGIN".to_string())))?;
        
        Ok(())
    }
    
    pub async fn commit() -> DbResult<()> {
        let pool = Self::get_connection()?;
        
        sqlx::query("COMMIT")
            .execute(&pool)
            .await
            .map_err(|e| DatabaseException::new(e.to_string(), Some("COMMIT".to_string())))?;
        
        Ok(())
    }
    
    pub async fn get_db_structure(file: &str, mode: i32) -> DbResult<bool> {
        let schema_manager = Self::get_mdb2_schema_manager();
        schema_manager.get_db_structure(file).await
    }
    
    pub async fn create_db_from_structure(file: &str) -> DbResult<bool> {
        let schema_manager = Self::get_mdb2_schema_manager();
        schema_manager.create_db_from_structure(file).await
    }
    
    pub async fn update_db_from_structure(file: &str) -> DbResult<bool> {
        let schema_manager = Self::get_mdb2_schema_manager();
        
        match schema_manager.update_db_from_structure(file).await {
            Ok(result) => Ok(result),
            Err(e) => {
                OC_Log::write("core", &format!("Failed to update database structure ({})", e), OC_Log::FATAL);
                Err(e)
            }
        }
    }
    
    pub async fn drop_table(table_name: &str) -> DbResult<()> {
        let schema_manager = Self::get_mdb2_schema_manager();
        schema_manager.drop_table(table_name).await
    }
    
    pub async fn remove_db_structure(file: &str) -> DbResult<()> {
        let schema_manager = Self::get_mdb2_schema_manager();
        schema_manager.remove_db_structure(file).await
    }
    
    pub async fn replace_db(file: &str) -> DbResult<()> {
        let schema_manager = Self::get_mdb2_schema_manager();
        schema_manager.replace_db(file).await
    }
    
    pub fn is_error(result: &DbResult<u64>) -> bool {
        result.is_err()
    }
    
    pub fn raise_exception_on_error<T>(result: DbResult<T>, message: Option<&str>) -> DbResult<T> {
        match &result {
            Ok(_) => result,
            Err(e) => {
                if let Some(msg) = message {
                    Err(DatabaseException::new(
                        format!("{}, Root cause: {}", msg, e),
                        e.get_query().map(String::from)
                    ))
                } else {
                    Err(DatabaseException::new(e.to_string(), e.get_query().map(String::from)))
                }
            }
        }
    }
    
    pub fn get_error_code<T>(error: &DbResult<T>) -> Option<String> {
        match error {
            Err(_) => {
                let pool = Self::get_connection().ok()?;
                // This would require DB-specific error code extraction
                Some("ERR".to_string())
            },
            Ok(_) => None,
        }
    }
    
    pub fn get_error_message<T>(error: &DbResult<T>) -> String {
        match error {
            Err(e) => e.to_string(),
            Ok(_) => "".to_string(),
        }
    }
    
    pub async fn enable_caching(enabled: bool) -> DbResult<()> {
        // SQLx handles caching differently than Doctrine
        // This is a placeholder implementation
        Ok(())
    }
}

pub enum StmtParam {
    String(String),
    Array(std::collections::HashMap<String, String>),
    StatementWrapper(StatementWrapper),
}

impl From<&str> for StmtParam {
    fn from(s: &str) -> Self {
        StmtParam::String(s.to_string())
    }
}

impl From<String> for StmtParam {
    fn from(s: String) -> Self {
        StmtParam::String(s)
    }
}

impl From<std::collections::HashMap<String, String>> for StmtParam {
    fn from(map: std::collections::HashMap<String, String>) -> Self {
        StmtParam::Array(map)
    }
}

impl From<StatementWrapper> for StmtParam {
    fn from(wrapper: StatementWrapper) -> Self {
        StmtParam::StatementWrapper(wrapper)
    }
}

// Placeholder implementations for dependencies that would be defined elsewhere
struct OC_Config;
impl OC_Config {
    fn get_value(key: &str, default: impl Into<String>) -> String {
        // Placeholder implementation
        default.into()
    }
}

struct OC;
impl OC {
    fn server_root() -> String {
        // Placeholder implementation
        "/var/www/owncloud".to_string()
    }
}

struct OC_Log;
impl OC_Log {
    const FATAL: i32 = 4;
    
    fn write(app: &str, message: &str, level: i32) {
        // Placeholder implementation
        println!("[{}] {}: {}", level, app, message);
    }
}

struct OC_User;
impl OC_User {
    fn set_user_id(user_id: Option<&str>) {
        // Placeholder implementation
    }
}

struct OC_Template;
impl OC_Template {
    fn print_error_page(message: &str) {
        // Placeholder implementation
        println!("Error page: {}", message);
    }
}

// Placeholder for MDB2SchemaManager implementation
struct DefaultMDB2SchemaManager;

#[async_trait]
impl MDB2SchemaManager for DefaultMDB2SchemaManager {
    async fn get_db_structure(&self, file: &str) -> DbResult<bool> {
        // Placeholder implementation
        Ok(true)
    }
    
    async fn create_db_from_structure(&self, file: &str) -> DbResult<bool> {
        // Placeholder implementation
        Ok(true)
    }
    
    async fn update_db_from_structure(&self, file: &str) -> DbResult<bool> {
        // Placeholder implementation
        Ok(true)
    }
    
    async fn drop_table(&self, table_name: &str) -> DbResult<()> {
        // Placeholder implementation
        Ok(())
    }
    
    async fn remove_db_structure(&self, file: &str) -> DbResult<()> {
        // Placeholder implementation
        Ok(())
    }
    
    async fn replace_db(&self, file: &str) -> DbResult<()> {
        // Placeholder implementation
        Ok(())
    }
}