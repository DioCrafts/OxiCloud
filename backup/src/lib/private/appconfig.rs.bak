// Copyright (C) 2012 Frank Karlitschek <frank@owncloud.org>
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

/*
 * The following SQL statement is just a help for developers and will not be
 * executed!
 *
 * CREATE TABLE `appconfig` (
 * `appid` VARCHAR(255) NOT NULL,
 * `configkey` VARCHAR(255) NOT NULL,
 * `configvalue` VARCHAR(255) NOT NULL
 * )
 */

use crate::db::DB;
use std::collections::HashMap;
use async_trait::async_trait;

/// This struct provides an easy way for apps to store config values in the
/// database.
pub struct AppConfig;

#[async_trait]
pub trait HookEmitter {
    async fn emit(event: &str, params: HashMap<&str, String>);
}

pub struct Hook;

#[async_trait]
impl HookEmitter for Hook {
    async fn emit(event: &str, params: HashMap<&str, String>) {
        // Implementation would go here
    }
}

impl AppConfig {
    /// Get all apps using the config
    ///
    /// This function returns a list of all apps that have at least one
    /// entry in the appconfig table.
    pub async fn get_apps(db: &DB) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // No magic in here!
        let query = "SELECT DISTINCT `appid` FROM `*PREFIX*appconfig`";
        let result = db.prepare(query).execute(&[]).await?;

        let mut apps = Vec::new();
        for row in result.rows {
            if let Some(app_id) = row.get("appid") {
                apps.push(app_id);
            }
        }

        Ok(apps)
    }

    /// Get the available keys for an app
    ///
    /// This function gets all keys of an app. Please note that the values are
    /// not returned.
    pub async fn get_keys(db: &DB, app: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // No magic in here as well
        let query = "SELECT `configkey` FROM `*PREFIX*appconfig` WHERE `appid` = ?";
        let result = db.prepare(query).execute(&[&app]).await?;

        let mut keys = Vec::new();
        for row in result.rows {
            if let Some(config_key) = row.get("configkey") {
                keys.push(config_key);
            }
        }

        Ok(keys)
    }

    /// Gets the config value
    ///
    /// This function gets a value from the appconfig table. If the key does
    /// not exist the default value will be returned
    pub async fn get_value<T: AsRef<str>>(
        db: &DB,
        app: &str,
        key: &str,
        default: Option<T>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // At least some magic in here :-)
        let query = "SELECT `configvalue` FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?";
        let result = db.prepare(query).execute(&[&app, &key]).await?;

        if let Some(row) = result.rows.first() {
            if let Some(value) = row.get("configvalue") {
                return Ok(value);
            }
        }

        match default {
            Some(val) => Ok(val.as_ref().to_string()),
            None => Ok(String::new()),
        }
    }

    /// Check if a key is set in the appconfig
    pub async fn has_key(db: &DB, app: &str, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let exists = Self::get_keys(db, app).await?;
        Ok(exists.contains(&key.to_string()))
    }

    /// Sets a value in the appconfig
    ///
    /// Sets a value. If the key did not exist before it will be created.
    pub async fn set_value(
        db: &DB,
        app: &str,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Does the key exist? yes: update. No: insert
        if !Self::has_key(db, app, key).await? {
            let query = "INSERT INTO `*PREFIX*appconfig` (`appid`, `configkey`, `configvalue`) VALUES(?, ?, ?)";
            db.prepare(query).execute(&[&app, &key, &value]).await?;
        } else {
            let query = "UPDATE `*PREFIX*appconfig` SET `configvalue` = ? WHERE `appid` = ? AND `configkey` = ?";
            db.prepare(query).execute(&[&value, &app, &key]).await?;
        }

        // Emit hook
        let mut params = HashMap::new();
        params.insert("app", app.to_string());
        params.insert("key", key.to_string());
        params.insert("value", value.to_string());
        
        Hook::emit("OC_Appconfig::post_set_value", params).await;
        
        Ok(())
    }

    /// Deletes a key
    ///
    /// Deletes a key.
    pub async fn delete_key(db: &DB, app: &str, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Boring!
        let query = "DELETE FROM `*PREFIX*appconfig` WHERE `appid` = ? AND `configkey` = ?";
        db.prepare(query).execute(&[&app, &key]).await?;

        Ok(true)
    }

    /// Remove app from appconfig
    ///
    /// Removes all keys in appconfig belonging to the app.
    pub async fn delete_app(db: &DB, app: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Nothing special
        let query = "DELETE FROM `*PREFIX*appconfig` WHERE `appid` = ?";
        db.prepare(query).execute(&[&app]).await?;

        Ok(true)
    }

    /// Get multiply values, either the app or key can be used as wildcard by setting it to None
    pub async fn get_values(
        db: &DB,
        app: Option<&str>,
        key: Option<&str>,
    ) -> Result<Option<HashMap<String, String>>, Box<dyn std::error::Error>> {
        if app.is_some() && key.is_some() {
            return Ok(None);
        }

        let mut fields = "`configvalue`".to_string();
        let mut where_clause = "WHERE".to_string();
        let mut params: Vec<&str> = Vec::new();
        let lookup_key: &str;

        if let Some(app_val) = app {
            fields.push_str(", `configkey`");
            where_clause.push_str(" `appid` = ?");
            params.push(app_val);
            lookup_key = "configkey";
        } else if let Some(key_val) = key {
            fields.push_str(", `appid`");
            where_clause.push_str(" `configkey` = ?");
            params.push(key_val);
            lookup_key = "appid";
        } else {
            return Ok(None);
        }

        let query_string = format!("SELECT {} FROM `*PREFIX*appconfig` {}", fields, where_clause);
        let result = db.prepare(&query_string).execute(&params).await?;

        let mut values = HashMap::new();
        for row in result.rows {
            if let (Some(k), Some(v)) = (row.get(lookup_key), row.get("configvalue")) {
                values.insert(k, v);
            }
        }

        Ok(Some(values))
    }
}