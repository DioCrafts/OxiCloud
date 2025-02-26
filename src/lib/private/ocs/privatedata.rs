use crate::db;
use crate::ocs::OcsResult;
use crate::user;
use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

/// ownCloud
///
/// @author Frank Karlitschek
/// @author Tom Needham
/// @copyright 2012 Frank Karlitschek frank@owncloud.org
/// @copyright 2012 Tom Needham tom@owncloud.com
///
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

#[derive(Debug, Clone)]
pub struct PrivateData {
    db_pool: Pool<Sqlite>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PrivateDataEntry {
    key: String,
    app: String,
    value: String,
}

impl PrivateData {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }

    /// read keys
    /// test: curl http://login:passwd@oc/core/ocs/v1.php/privatedata/getattribute/testy/123
    /// test: curl http://login:passwd@oc/core/ocs/v1.php/privatedata/getattribute/testy
    pub async fn get(&self, parameters: HashMap<String, String>) -> OcsResult<Vec<PrivateDataEntry>> {
        let user = user::get_user()?;
        let app = self.sanitize_input(&parameters.get("app").ok_or("App parameter required")?);
        let key = parameters.get("key").map(|k| self.sanitize_input(k));

        let entries = match key {
            Some(key) => {
                sqlx::query_as!(
                    PrivateDataEntry,
                    "SELECT `key`, `app`, `value` FROM `prefix_privatedata` WHERE `user` = ? AND `app` = ? AND `key` = ?",
                    user, app, key
                )
                .fetch_all(&self.db_pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    PrivateDataEntry,
                    "SELECT `key`, `app`, `value` FROM `prefix_privatedata` WHERE `user` = ? AND `app` = ?",
                    user, app
                )
                .fetch_all(&self.db_pool)
                .await?
            },
        };

        Ok(OcsResult::success(entries))
    }

    /// set a key
    /// test: curl http://login:passwd@oc/core/ocs/v1.php/privatedata/setattribute/testy/123  --data "value=foobar"
    pub async fn set(&self, parameters: HashMap<String, String>, post_data: HashMap<String, String>) -> OcsResult<()> {
        let user = user::get_user()?;
        let app = self.sanitize_input(&parameters.get("app").ok_or("App parameter required")?);
        let key = self.sanitize_input(&parameters.get("key").ok_or("Key parameter required")?);
        let value = post_data.get("value").cloned().unwrap_or_default();

        // Try to update an existing record first
        let result = sqlx::query!(
            "UPDATE `prefix_privatedata` SET `value` = ? WHERE `user` = ? AND `app` = ? AND `key` = ?",
            value, user, app, key
        )
        .execute(&self.db_pool)
        .await?;

        // If no rows were affected, insert a new record instead
        if result.rows_affected() == 0 {
            sqlx::query!(
                "INSERT INTO `prefix_privatedata` (`user`, `app`, `key`, `value`) VALUES(?, ?, ?, ?)",
                user, app, key, value
            )
            .execute(&self.db_pool)
            .await?;
        }

        Ok(OcsResult::success(()))
    }

    /// delete a key
    /// test: curl http://login:passwd@oc/core/ocs/v1.php/privatedata/deleteattribute/testy/123 --data "post=1"
    pub async fn delete(&self, parameters: HashMap<String, String>) -> OcsResult<()> {
        let user = user::get_user()?;
        
        // Key and app are NOT optional here
        if !parameters.contains_key("app") || !parameters.contains_key("key") {
            return Ok(OcsResult::error(101, "App and key parameters are required"));
        }

        let app = self.sanitize_input(&parameters.get("app").unwrap());
        let key = self.sanitize_input(&parameters.get("key").unwrap());

        // Delete the record
        sqlx::query!(
            "DELETE FROM `prefix_privatedata` WHERE `user` = ? AND `app` = ? AND `key` = ?",
            user, app, key
        )
        .execute(&self.db_pool)
        .await?;

        Ok(OcsResult::success(()))
    }

    // Helper method to sanitize input (similar to addslashes + strip_tags in PHP)
    fn sanitize_input(&self, input: &str) -> String {
        // Simple implementation - in a real app, use proper HTML escaping and SQL parameter binding
        input.replace(['\'', '"', '\\', '<', '>'], "")
    }
}