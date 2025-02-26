use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Provide read only support for the old filecache
pub struct Legacy {
    user: String,
    cache_has_items: Option<bool>,
}

impl Legacy {
    pub fn new(user: String) -> Self {
        Legacy {
            user,
            cache_has_items: None,
        }
    }

    /// get the numbers of items in the legacy cache
    ///
    /// # Returns
    /// * `u64` - The count of items
    pub async fn get_count(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let sql = "SELECT COUNT(`id`) AS `count` FROM `*PREFIX*fscache` WHERE `user` = ?";
        let result = db::execute_audited(sql, &[&self.user]).await?;
        
        if let Some(row) = result.first() {
            let count: u64 = row.get("count")?;
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// check if a legacy cache is present and holds items
    ///
    /// # Returns
    /// * `bool` - Whether the cache has items
    pub async fn has_items(&mut self) -> bool {
        if let Some(has_items) = self.cache_has_items {
            return has_items;
        }

        let query = match db::prepare("SELECT `id` FROM `*PREFIX*fscache` WHERE `user` = ?", 1).await {
            Ok(q) => q,
            Err(_) => {
                self.cache_has_items = Some(false);
                return false;
            }
        };

        let result = match query.execute(&[&self.user]).await {
            Ok(r) => r,
            Err(_) => {
                self.cache_has_items = Some(false);
                return false;
            }
        };

        // Check if result is invalid or has an error
        if result.is_err() || result.has_error() {
            self.cache_has_items = Some(false);
            return false;
        }

        let has_rows = result.rows().len() > 0;
        self.cache_has_items = Some(has_rows);
        has_rows
    }

    /// get an item from the legacy cache
    ///
    /// # Arguments
    /// * `path` - The path or ID to look up
    ///
    /// # Returns
    /// * `Result<CacheEntry, Error>` - The cache entry data
    pub async fn get<T: AsRef<str>>(&self, path: T) -> Result<CacheEntry, Box<dyn std::error::Error>> {
        let path_str = path.as_ref();
        let sql = if path_str.parse::<i64>().is_ok() {
            "SELECT * FROM `*PREFIX*fscache` WHERE `id` = ?"
        } else {
            "SELECT * FROM `*PREFIX*fscache` WHERE `path` = ?"
        };

        let result = db::execute_audited(sql, &[&path_str]).await?;
        
        if let Some(row) = result.first() {
            let path: String = row.get("path")?;
            let user: String = row.get("user")?;
            let etag = self.get_etag(&path, Some(&user)).await?;
            
            let mut entry = CacheEntry::from_row(row)?;
            entry.etag = etag;
            
            Ok(entry)
        } else {
            Err("Entry not found".into())
        }
    }

    /// Get the ETag for the given path
    ///
    /// # Arguments
    /// * `path` - The path to get the etag for
    /// * `user` - Optional user, will be derived from path if not provided
    ///
    /// # Returns
    /// * `String` - The etag value
    pub async fn get_etag(
        &self, 
        path: &str, 
        user: Option<&str>
    ) -> Result<String, Box<dyn std::error::Error>> {
        static QUERY: Lazy<Mutex<Option<db::PreparedStatement>>> = Lazy::new(|| Mutex::new(None));
        
        let path_details: Vec<&str> = path.split('/').collect();
        
        let user = if let Some(u) = user {
            u.to_string()
        } else if path_details.len() < 2 {
            return Ok(String::new());
        } else {
            path_details[1].to_string()
        };
        
        let relative_path = if path_details.len() < 4 || path_details[3].is_empty() {
            String::new()
        } else {
            path_details[3].to_string()
        };

        {
            let mut query_guard = QUERY.lock().unwrap();
            if query_guard.is_none() {
                *query_guard = Some(db::prepare(
                    "SELECT `propertyvalue` FROM `*PREFIX*properties` WHERE `userid` = ? AND `propertypath` = ? AND `propertyname` = '{DAV:}getetag'",
                    2
                ).await?);
            }
        }

        let query = QUERY.lock().unwrap().as_ref().unwrap();
        let property_path = format!("/{}", relative_path);
        
        let result = db::execute_audited_with_stmt(
            query,
            &[&user, &property_path]
        ).await?;
        
        if let Some(row) = result.first() {
            let value: String = row.get("propertyvalue")?;
            Ok(value.trim_matches('"').to_string())
        } else {
            Ok(String::new())
        }
    }

    /// get all child items of an item from the legacy cache
    ///
    /// # Arguments
    /// * `id` - The parent ID
    ///
    /// # Returns
    /// * `Result<Vec<CacheEntry>, Error>` - The child cache entries
    pub async fn get_children(&self, id: i64) -> Result<Vec<CacheEntry>, Box<dyn std::error::Error>> {
        let sql = "SELECT * FROM `*PREFIX*fscache` WHERE `parent` = ?";
        let result = db::execute_audited(sql, &[&id]).await?;
        
        let mut children = Vec::new();
        for row in result.rows() {
            let path: String = row.get("path")?;
            let user: String = row.get("user")?;
            let etag = self.get_etag(&path, Some(&user)).await?;
            
            let mut entry = CacheEntry::from_row(row)?;
            entry.etag = etag;
            
            children.push(entry);
        }
        
        Ok(children)
    }
}

/// Represents an entry in the file cache
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub id: i64,
    pub path: String,
    pub user: String,
    pub parent: i64,
    pub etag: String,
    // Add other fields from the cache table as needed
}

impl CacheEntry {
    fn from_row(row: &db::Row) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(CacheEntry {
            id: row.get("id")?,
            path: row.get("path")?,
            user: row.get("user")?,
            parent: row.get("parent")?,
            etag: String::new(), // Will be filled later
            // Add other fields from the cache table as needed
        })
    }
}

// Mock DB module - In a real implementation, this would be replaced with actual database code
mod db {
    use std::collections::HashMap;
    
    pub struct PreparedStatement;
    
    pub struct Row {
        data: HashMap<String, String>,
    }
    
    impl Row {
        pub fn get<T>(&self, key: &str) -> Result<T, Box<dyn std::error::Error>> 
        where 
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::error::Error + 'static
        {
            self.data.get(key)
                .ok_or_else(|| format!("Column not found: {}", key).into())
                .and_then(|v| v.parse::<T>().map_err(|e| Box::new(e) as Box<dyn std::error::Error>))
        }
    }
    
    pub struct QueryResult {
        rows: Vec<Row>,
        error: Option<String>,
    }
    
    impl QueryResult {
        pub fn first(&self) -> Option<&Row> {
            self.rows.first()
        }
        
        pub fn rows(&self) -> &[Row] {
            &self.rows
        }
        
        pub fn is_err(&self) -> bool {
            self.error.is_some()
        }
        
        pub fn has_error(&self) -> bool {
            self.error.is_some()
        }
    }
    
    pub async fn prepare(
        _query: &str, 
        _limit: usize
    ) -> Result<PreparedStatement, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(PreparedStatement)
    }
    
    pub async fn execute_audited(
        _sql: &str, 
        _params: &[&dyn AsRef<str>]
    ) -> Result<QueryResult, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(QueryResult {
            rows: Vec::new(),
            error: None,
        })
    }
    
    pub async fn execute_audited_with_stmt(
        _stmt: &PreparedStatement,
        _params: &[&dyn AsRef<str>]
    ) -> Result<QueryResult, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(QueryResult {
            rows: Vec::new(),
            error: None,
        })
    }
}