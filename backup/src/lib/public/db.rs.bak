// Copyright (c) 2012-present Frank Karlitschek frank@owncloud.org
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

// Public interface of ownCloud for apps to use.
// DB module

use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Database error type
#[derive(Debug)]
pub struct DbError {
    message: String,
    code: Option<i32>,
}

impl DbError {
    pub fn new(message: &str, code: Option<i32>) -> Self {
        Self {
            message: message.to_string(),
            code,
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.code {
            write!(f, "Database error ({}): {}", code, self.message)
        } else {
            write!(f, "Database error: {}", self.message)
        }
    }
}

impl Error for DbError {}

/// Result type for database operations
pub type DbResult<T> = Result<T, DbError>;

/// Type for database field input
#[derive(Debug, Clone)]
pub struct FieldInput {
    pub value: String,
    pub is_key: bool,
}

impl FieldInput {
    pub fn new<T: ToString>(value: T, is_key: bool) -> Self {
        Self {
            value: value.to_string(),
            is_key,
        }
    }
}

/// Prepared statement trait
#[async_trait]
pub trait PreparedStatement {
    /// Execute the prepared statement
    async fn execute(&self, params: Vec<String>) -> DbResult<()>;
    
    /// Execute the prepared statement and return rows
    async fn execute_query(&self, params: Vec<String>) -> DbResult<Vec<HashMap<String, String>>>;
}

/// Database interface - this provides access to the internal database system
pub struct Db;

impl Db {
    /// Prepare a SQL query
    ///
    /// # Arguments
    ///
    /// * `query` - Query string
    /// * `limit` - Optional limit for number of results
    /// * `offset` - Optional offset for results
    ///
    /// # Returns
    ///
    /// A prepared SQL statement that needs to be executed
    pub async fn prepare(
        query: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> DbResult<Box<dyn PreparedStatement>> {
        // This would call the internal implementation
        // For now we return a placeholder error
        Err(DbError::new("Not implemented", None))
    }

    /// Insert a row if a matching row doesn't exist
    ///
    /// # Arguments
    ///
    /// * `table` - The table name (will replace *PREFIX*) to perform the replace on
    /// * `input` - Key-value map of field name to FieldInput containing value and key status
    ///
    /// # Returns
    ///
    /// Success or failure as a Result
    pub async fn insert_if_not_exist(
        table: &str,
        input: HashMap<String, FieldInput>,
    ) -> DbResult<bool> {
        // This would call the internal implementation
        // For now we return a placeholder error
        Err(DbError::new("Not implemented", None))
    }

    /// Gets last value of autoincrement
    ///
    /// # Arguments
    ///
    /// * `table` - The optional table name (will replace *PREFIX*) and add sequence suffix
    ///
    /// # Returns
    ///
    /// The last inserted ID
    ///
    /// Call this method right after the insert command or other functions may
    /// cause trouble!
    pub async fn insertid(table: Option<&str>) -> DbResult<i64> {
        // This would call the internal implementation
        // For now we return a placeholder error
        Err(DbError::new("Not implemented", None))
    }

    /// Start a transaction
    pub async fn begin_transaction() -> DbResult<()> {
        // This would call the internal implementation
        // For now we return a placeholder error
        Err(DbError::new("Not implemented", None))
    }

    /// Commit the database changes done during a transaction that is in progress
    pub async fn commit() -> DbResult<()> {
        // This would call the internal implementation
        // For now we return a placeholder error
        Err(DbError::new("Not implemented", None))
    }

    /// Check if a result is an error
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    ///
    /// # Returns
    ///
    /// True if the result is an error, false otherwise
    pub fn is_error<T>(result: &DbResult<T>) -> bool {
        result.is_err()
    }

    /// Returns the error code and message as a string for logging
    ///
    /// # Arguments
    ///
    /// * `error` - The error to get message from
    ///
    /// # Returns
    ///
    /// A string with error code and message
    pub fn get_error_message(error: &DbError) -> String {
        error.to_string()
    }
}