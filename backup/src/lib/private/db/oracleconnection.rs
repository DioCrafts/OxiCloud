// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use async_trait::async_trait;

/// Oracle specific database connection
pub struct OracleConnection {
    inner: Box<dyn Connection>,
}

impl OracleConnection {
    /// Create a new Oracle connection
    pub fn new(connection: Box<dyn Connection>) -> Self {
        Self { inner: connection }
    }

    /// Quote the keys of the hashmap
    fn quote_keys(&self, data: HashMap<String, Value>) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        for (key, value) in data {
            result.insert(self.quote_identifier(&key), value);
        }
        result
    }
}

#[async_trait]
impl Connection for OracleConnection {
    /// Insert data into a table
    async fn insert(
        &self, 
        table_name: &str, 
        data: HashMap<String, Value>, 
        types: Option<HashMap<String, Type>>
    ) -> Result<u64, DbError> {
        let quoted_table = self.quote_identifier(table_name);
        let quoted_data = self.quote_keys(data);
        self.inner.insert(&quoted_table, quoted_data, types).await
    }

    /// Update data in a table
    async fn update(
        &self, 
        table_name: &str, 
        data: HashMap<String, Value>, 
        identifier: HashMap<String, Value>, 
        types: Option<HashMap<String, Type>>
    ) -> Result<u64, DbError> {
        let quoted_table = self.quote_identifier(table_name);
        let quoted_data = self.quote_keys(data);
        let quoted_identifier = self.quote_keys(identifier);
        self.inner.update(&quoted_table, quoted_data, quoted_identifier, types).await
    }

    /// Delete data from a table
    async fn delete(
        &self, 
        table_name: &str, 
        identifier: HashMap<String, Value>
    ) -> Result<u64, DbError> {
        let quoted_table = self.quote_identifier(table_name);
        let quoted_identifier = self.quote_keys(identifier);
        self.inner.delete(&quoted_table, quoted_identifier).await
    }

    /// Quote an identifier
    fn quote_identifier(&self, identifier: &str) -> String {
        self.inner.quote_identifier(identifier)
    }
}

// These types would be defined elsewhere in the actual codebase
#[derive(Debug, Clone)]
pub enum Value {
    // various value types
}

#[derive(Debug, Clone)]
pub enum Type {
    // various column types
}

#[derive(Debug)]
pub enum DbError {
    // various error types
}

#[async_trait]
pub trait Connection: Send + Sync {
    async fn insert(
        &self, 
        table_name: &str, 
        data: HashMap<String, Value>, 
        types: Option<HashMap<String, Type>>
    ) -> Result<u64, DbError>;
    
    async fn update(
        &self, 
        table_name: &str, 
        data: HashMap<String, Value>, 
        identifier: HashMap<String, Value>, 
        types: Option<HashMap<String, Type>>
    ) -> Result<u64, DbError>;
    
    async fn delete(
        &self, 
        table_name: &str, 
        identifier: HashMap<String, Value>
    ) -> Result<u64, DbError>;
    
    fn quote_identifier(&self, identifier: &str) -> String;
}