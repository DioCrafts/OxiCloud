// Copyright (c) 2013 Bart Visscher bartv@thisnet.nl
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use async_trait::async_trait;
use std::error::Error;

/// Represents a database statement that can be executed
pub trait Statement {
    // Methods for statement execution would be defined here
}

/// Input field for database operations
pub struct InputField<T> {
    pub value: T,
    pub key: bool,
}

/// Public interface of ownCloud for apps to use.
/// DBConnection interface
#[async_trait]
pub trait IDbConnection {
    type Statement: Statement;
    type Error: Error;

    /// Used to abstract the owncloud database access away
    /// 
    /// # Arguments
    /// * `sql` - the sql query with ? placeholder for params
    /// * `limit` - the maximum number of rows
    /// * `offset` - from which row we want to start
    /// 
    /// # Returns
    /// The prepared statement.
    async fn prepare(&self, sql: &str, limit: Option<usize>, offset: Option<usize>) -> Result<Self::Statement, Self::Error>;

    /// Used to get the id of the just inserted element
    /// 
    /// # Arguments
    /// * `table_name` - the name of the table where we inserted the item
    /// 
    /// # Returns
    /// the id of the inserted element
    async fn last_insert_id(&self, table_name: Option<&str>) -> Result<i64, Self::Error>;

    /// Insert a row if a matching row doesn't exists.
    ///
    /// # Arguments
    /// * `table` - The table name (will replace *PREFIX*) to perform the replace on.
    /// * `input` - The input hashmap in the form:
    ///
    ///   {
    ///     "id": InputField { value: 6, key: true },
    ///     "name": InputField { value: "Stoyan", key: false },
    ///     "family": InputField { value: "Stefanov", key: false },
    ///     "birth_date": InputField { value: "1975-06-20", key: false }
    ///   }
    ///
    /// 
    /// # Returns
    /// `true` if successful, `false` otherwise
    async fn insert_if_not_exist<T>(&self, table: &str, input: &HashMap<&str, InputField<T>>) -> Result<bool, Self::Error>
    where
        T: Send + Sync;

    /// Start a transaction
    /// 
    /// # Returns
    /// `true` on success or `false` on failure
    async fn begin_transaction(&self) -> Result<bool, Self::Error>;

    /// Commit the database changes done during a transaction that is in progress
    /// 
    /// # Returns
    /// `true` on success or `false` on failure
    async fn commit(&self) -> Result<bool, Self::Error>;

    /// Rollback the database changes done during a transaction that is in progress
    /// 
    /// # Returns
    /// `true` on success or `false` on failure
    async fn roll_back(&self) -> Result<bool, Self::Error>;

    /// Gets the error code and message as a string for logging
    /// 
    /// # Returns
    /// The error string
    fn get_error(&self) -> String;
}