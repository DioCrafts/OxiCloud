// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use md5;
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use sqlx::{Pool, Postgres, Row};

/// Storage cache for storage specific data
pub struct Storage {
    storage_id: String,
    numeric_id: i64,
}

#[async_trait]
pub trait StorageTrait {
    fn get_id(&self) -> &str;
}

impl Storage {
    /// Create a new storage cache
    ///
    /// # Arguments
    ///
    /// * `storage` - Either a storage instance or a storage ID string
    pub async fn new<T>(storage: T, pool: &Pool<Postgres>) -> Result<Self> 
    where 
        T: Into<StorageIdentifier>,
    {
        let storage_id = match storage.into() {
            StorageIdentifier::Instance(s) => s.get_id().to_string(),
            StorageIdentifier::Id(id) => id,
        };
        
        let storage_id = if storage_id.len() > 64 {
            format!("{:x}", md5::compute(storage_id))
        } else {
            storage_id
        };

        // Try to find existing numeric ID
        let numeric_id_result = sqlx::query("SELECT numeric_id FROM storages WHERE id = $1")
            .bind(&storage_id)
            .fetch_optional(pool)
            .await?;

        let numeric_id = if let Some(row) = numeric_id_result {
            row.get::<i64, _>("numeric_id")
        } else {
            // Insert new storage
            let result = sqlx::query("INSERT INTO storages (id) VALUES($1) RETURNING numeric_id")
                .bind(&storage_id)
                .fetch_one(pool)
                .await?;
            
            result.get::<i64, _>("numeric_id")
        };

        Ok(Self {
            storage_id,
            numeric_id,
        })
    }

    /// Get the numeric ID for this storage
    pub fn get_numeric_id(&self) -> i64 {
        self.numeric_id
    }

    /// Get the storage ID for a numeric ID
    ///
    /// # Arguments
    ///
    /// * `numeric_id` - The numeric ID to look up
    pub async fn get_storage_id(numeric_id: i64, pool: &Pool<Postgres>) -> Result<Option<String>> {
        let result = sqlx::query("SELECT id FROM storages WHERE numeric_id = $1")
            .bind(numeric_id)
            .fetch_optional(pool)
            .await?;
        
        Ok(result.map(|row| row.get::<String, _>("id")))
    }

    /// Check if a storage exists
    ///
    /// # Arguments
    ///
    /// * `storage_id` - The storage ID to check
    pub async fn exists(storage_id: &str, pool: &Pool<Postgres>) -> Result<bool> {
        let storage_id = if storage_id.len() > 64 {
            format!("{:x}", md5::compute(storage_id))
        } else {
            storage_id.to_string()
        };

        let result = sqlx::query("SELECT numeric_id FROM storages WHERE id = $1")
            .bind(&storage_id)
            .fetch_optional(pool)
            .await?;
        
        Ok(result.is_some())
    }
}

pub enum StorageIdentifier {
    Instance(Box<dyn StorageTrait>),
    Id(String),
}

impl<T: StorageTrait + 'static> From<T> for StorageIdentifier {
    fn from(storage: T) -> Self {
        StorageIdentifier::Instance(Box::new(storage))
    }
}

impl From<String> for StorageIdentifier {
    fn from(id: String) -> Self {
        StorageIdentifier::Id(id)
    }
}

impl From<&str> for StorageIdentifier {
    fn from(id: &str) -> Self {
        StorageIdentifier::Id(id.to_string())
    }
}