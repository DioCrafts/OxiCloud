use std::collections::HashMap;
use std::fmt;
use regex::Regex;
use log::debug;
use async_trait::async_trait;

/**
 * Small wrapper around Doctrine\DBAL\Driver\Statement to make it behave, more like an MDB2 Statement
 */
pub struct StatementWrapper<S: Statement> {
    statement: S,
    is_manipulation: bool,
    last_arguments: Vec<Value>,
}

pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "NULL"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "String(\"{}\")", s),
            Value::Integer(i) => write!(f, "Integer({})", i),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::Bool(b) => write!(f, "Bool({})", b),
            Value::Null => write!(f, "Null"),
        }
    }
}

#[async_trait]
pub trait Statement {
    async fn execute(&self, params: &[Value]) -> Result<bool, DbError>;
    async fn row_count(&self) -> Result<usize, DbError>;
    async fn fetch(&self) -> Result<Option<HashMap<String, Value>>, DbError>;
    async fn fetch_column(&self, column: usize) -> Result<Option<Value>, DbError>;
    fn get_query_string(&self) -> &str;
    fn get_wrapped_statement(&self) -> &dyn Statement;
}

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    QueryError(String),
    Other(String),
}

pub struct Config;

impl Config {
    pub fn get_value(key: &str, default: &str) -> String {
        // Implementation would depend on the actual config system
        // This is a placeholder
        default.to_string()
    }
}

pub struct Log;

impl Log {
    pub const DEBUG: u8 = 0;
    pub const FATAL: u8 = 4;

    pub fn write(module: &str, message: &str, level: u8) {
        // Implementation would depend on the actual logging system
        // This is a placeholder
        match level {
            Self::DEBUG => debug!("[{}] {}", module, message),
            _ => eprintln!("[{}] {}", module, message),
        }
    }
}

pub struct DB;

impl DB {
    pub async fn prepare(query: &str) -> Result<StatementWrapper<impl Statement>, DbError> {
        // Implementation would depend on the actual database system
        // This is a placeholder
        Err(DbError::Other("Not implemented".to_string()))
    }
}

impl<S: Statement> StatementWrapper<S> {
    pub fn new(statement: S, is_manipulation: bool) -> Self {
        Self {
            statement,
            is_manipulation,
            last_arguments: Vec::new(),
        }
    }

    /**
     * Provide numRows
     */
    pub async fn num_rows(&self) -> Result<usize, DbError> {
        let db_type = Config::get_value("dbtype", "sqlite");
        
        if db_type == "oci" {
            // OCI doesn't have a queryString, just do a rowCount for now
            return self.statement.row_count().await;
        }
        
        let regex = Regex::new(r"^SELECT\s+(?:ALL\s+|DISTINCT\s+)?(?:.*?)\s+FROM\s+(.*)$").unwrap();
        let query_string = self.statement.get_wrapped_statement().get_query_string();
        
        if let Some(captures) = regex.captures(query_string) {
            if let Some(table) = captures.get(1) {
                let count_query = format!("SELECT COUNT(*) FROM {}", table.as_str());
                let query = DB::prepare(&count_query).await?;
                let result = query.execute(&self.last_arguments).await?;
                match result {
                    ExecuteResult::Query(q) => {
                        let value = q.fetch_column(0).await?;
                        match value {
                            Some(Value::Integer(i)) => Ok(i as usize),
                            _ => Err(DbError::Other("Failed to get count".to_string())),
                        }
                    },
                    _ => Err(DbError::Other("Expected query result".to_string())),
                }
            } else {
                self.statement.row_count().await
            }
        } else {
            self.statement.row_count().await
        }
    }

    /**
     * Make execute return the result instead of a bool
     */
    pub async fn execute(&mut self, input: Vec<Value>) -> Result<ExecuteResult<S>, DbError> {
        if Config::get_value("log_query", "false") == "true" {
            let params_str = format!("{:?}", input);
            Log::write("core", &format!("DB execute with arguments : {}", params_str), Log::DEBUG);
        }
        
        self.last_arguments = input.clone();
        
        let result = if !input.is_empty() {
            let db_type = Config::get_value("dbtype", "sqlite");
            
            let input = if db_type == "mssql" {
                self.try_fix_substring_last_argument_data_for_mssql(input).await?
            } else {
                input
            };
            
            self.statement.execute(&input).await?
        } else {
            self.statement.execute(&[]).await?
        };
        
        if !result {
            return Err(DbError::QueryError("Statement execution failed".to_string()));
        }
        
        if self.is_manipulation {
            let rows = self.statement.row_count().await?;
            Ok(ExecuteResult::RowCount(rows))
        } else {
            Ok(ExecuteResult::Query(QueryResult::new(self)))
        }
    }

    async fn try_fix_substring_last_argument_data_for_mssql(&mut self, mut input: Vec<Value>) -> Result<Vec<Value>, DbError> {
        let query = self.statement.get_wrapped_statement().get_query_string();
        let pos = query.to_lowercase().find("substring");
        
        if pos.is_none() {
            return Ok(input);
        }
        
        let mut new_query = String::new();
        let mut c_arg = 0;
        let mut in_substring = false;
        
        // Create new query
        for i in 0..query.len() {
            if !in_substring {
                // Defines when we should start inserting values
                if query[i..].starts_with("SUBSTRING") || query[i..].starts_with("substring") {
                    in_substring = true;
                }
            } else {
                // Defines when we should stop inserting values
                if query[i..].starts_with(")") {
                    in_substring = false;
                }
            }
            
            if query[i..].starts_with("?") {
                // We found a question mark
                if in_substring {
                    new_query.push_str(&input[c_arg].to_string());
                    
                    // Remove from input array
                    input.remove(c_arg);
                } else {
                    new_query.push('?');
                    c_arg += 1;
                }
            } else {
                new_query.push(query.chars().nth(i).unwrap());
            }
        }
        
        // Here we would rebuild the statement with the new query
        // This implementation would depend on the database driver
        
        // For this example, we'll just return the modified input
        self.last_arguments = input.clone();
        
        Ok(input)
    }
    
    /**
     * Provide an alias for fetch
     */
    pub async fn fetch_row(&self) -> Result<Option<HashMap<String, Value>>, DbError> {
        self.statement.fetch().await
    }

    /**
     * Provide a simple fetchOne.
     * Fetch single column from the next row
     */
    pub async fn fetch_one(&self, colnum: usize) -> Result<Option<Value>, DbError> {
        self.statement.fetch_column(colnum).await
    }
}

pub enum ExecuteResult<'a, S: Statement> {
    RowCount(usize),
    Query(QueryResult<'a, S>),
}

pub struct QueryResult<'a, S: Statement> {
    statement: &'a StatementWrapper<S>,
}

impl<'a, S: Statement> QueryResult<'a, S> {
    fn new(statement: &'a StatementWrapper<S>) -> Self {
        Self { statement }
    }
    
    pub async fn fetch_column(&self, column: usize) -> Result<Option<Value>, DbError> {
        self.statement.statement.fetch_column(column).await
    }
}