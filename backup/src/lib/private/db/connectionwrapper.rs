use std::sync::Arc;

/// Trait defining database connection operations
pub trait IDBConnection {
    /// Used to prepare SQL statements
    fn prepare(&self, sql: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Statement, DbError>;
    
    /// Used to get the id of the just inserted element
    fn last_insert_id(&self, table: Option<&str>) -> Result<i64, DbError>;
    
    /// Insert a row if a matching row doesn't exist
    fn insert_if_not_exist(&self, table: &str, input: &[(&str, InputValue)]) -> Result<bool, DbError>;
    
    /// Start a transaction
    fn begin_transaction(&self) -> Result<(), DbError>;
    
    /// Commit the database changes done during a transaction that is in progress
    fn commit(&self) -> Result<(), DbError>;
    
    /// Rollback the database changes done during a transaction that is in progress
    fn roll_back(&self) -> Result<(), DbError>;
    
    /// Gets the error code and message as a string for logging
    fn get_error(&self) -> Option<String>;
}

/// Wrapper for database connection implementation
pub struct ConnectionWrapper {
    connection: Arc<dyn Connection>,
}

/// Value type for database input
pub enum InputValue {
    Value(String),
    Key(String, bool),
}

/// Statement representation
pub struct Statement {
    // Implementation details for prepared statements would go here
}

/// Database error type
#[derive(Debug)]
pub struct DbError {
    // Implementation details for database errors would go here
    message: String,
}

/// Connection trait for database implementations
pub trait Connection: Send + Sync {
    fn prepare(&self, sql: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Statement, DbError>;
    fn last_insert_id(&self, table: Option<&str>) -> Result<i64, DbError>;
    fn insert_if_not_exist(&self, table: &str, input: &[(&str, InputValue)]) -> Result<bool, DbError>;
    fn begin_transaction(&self) -> Result<(), DbError>;
    fn commit(&self) -> Result<(), DbError>;
    fn roll_back(&self) -> Result<(), DbError>;
    fn get_error(&self) -> Option<String>;
}

impl ConnectionWrapper {
    /// Create a new connection wrapper
    pub fn new(conn: Arc<dyn Connection>) -> Self {
        Self { connection: conn }
    }
}

impl IDBConnection for ConnectionWrapper {
    /**
     * Used to prepare SQL statements
     * @param sql the sql query with ? placeholder for params
     * @param limit the maximum number of rows
     * @param offset from which row we want to start
     */
    fn prepare(&self, sql: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Statement, DbError> {
        self.connection.prepare(sql, limit, offset)
    }

    /**
     * Used to get the id of the just inserted element
     * @param table the name of the table where we inserted the item
     */
    fn last_insert_id(&self, table: Option<&str>) -> Result<i64, DbError> {
        self.connection.last_insert_id(table)
    }

    /**
     * Insert a row if a matching row doesn't exists.
     * 
     * The input array is in the form:
     * 
     * [
     *   ("id", InputValue::Key("6", true)),
     *   ("name", InputValue::Value("Stoyan")),
     *   ("family", InputValue::Value("Stefanov")),
     *   ("birth_date", InputValue::Value("1975-06-20"))
     * ]
     */
    fn insert_if_not_exist(&self, table: &str, input: &[(&str, InputValue)]) -> Result<bool, DbError> {
        self.connection.insert_if_not_exist(table, input)
    }

    /// Start a transaction
    fn begin_transaction(&self) -> Result<(), DbError> {
        self.connection.begin_transaction()
    }

    /// Commit the database changes done during a transaction that is in progress
    fn commit(&self) -> Result<(), DbError> {
        self.connection.commit()
    }

    /// Rollback the database changes done during a transaction that is in progress
    fn roll_back(&self) -> Result<(), DbError> {
        self.connection.roll_back()
    }

    /// Gets the error code and message as a string for logging
    fn get_error(&self) -> Option<String> {
        self.connection.get_error()
    }
}