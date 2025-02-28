/*
 * ownCloud
 *
 * @author Frank Karlitschek
 * @author Jakob Sack
 * @copyright 2012 Frank Karlitschek frank@owncloud.org
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU AFFERO GENERAL PUBLIC LICENSE for more details.
 *
 * You should have received a copy of the GNU Affero General Public
 * License along with this library.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
/*
 *
 * The following SQL statement is just a help for developers and will not be
 * executed!
 *
 * CREATE TABLE  `preferences` (
 * `userid` VARCHAR( 255 ) NOT NULL ,
 * `appid` VARCHAR( 255 ) NOT NULL ,
 * `configkey` VARCHAR( 255 ) NOT NULL ,
 * `configvalue` VARCHAR( 255 ) NOT NULL
 * )
 *
 */

use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};

/// This struct provides an easy way for storing user preferences.
pub struct Preferences {
    pool: Pool<Postgres>,
}

impl Preferences {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Get all users using the preferences
    ///
    /// This function returns a list of all users that have at least one entry
    /// in the preferences table.
    pub async fn get_users(&self) -> Result<Vec<String>, sqlx::Error> {
        let query = "SELECT DISTINCT userid FROM *PREFIX*preferences";
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        let users = rows.iter()
            .map(|row| row.get("userid"))
            .collect();

        Ok(users)
    }

    /// Get all apps of a user
    ///
    /// This function returns a list of all apps of the user that have at least
    /// one entry in the preferences table.
    pub async fn get_apps(&self, user: &str) -> Result<Vec<String>, sqlx::Error> {
        let query = "SELECT DISTINCT appid FROM *PREFIX*preferences WHERE userid = $1";
        let rows = sqlx::query(query)
            .bind(user)
            .fetch_all(&self.pool)
            .await?;

        let apps = rows.iter()
            .map(|row| row.get("appid"))
            .collect();

        Ok(apps)
    }

    /// Get the available keys for an app
    ///
    /// This function gets all keys of an app of a user. Please note that the
    /// values are not returned.
    pub async fn get_keys(&self, user: &str, app: &str) -> Result<Vec<String>, sqlx::Error> {
        let query = "SELECT configkey FROM *PREFIX*preferences WHERE userid = $1 AND appid = $2";
        let rows = sqlx::query(query)
            .bind(user)
            .bind(app)
            .fetch_all(&self.pool)
            .await?;

        let keys = rows.iter()
            .map(|row| row.get("configkey"))
            .collect();

        Ok(keys)
    }

    /// Gets the preference
    ///
    /// This function gets a value from the preferences table. If the key does
    /// not exist the default value will be returned
    pub async fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> Result<String, sqlx::Error> {
        let query = "SELECT configvalue FROM *PREFIX*preferences WHERE userid = $1 AND appid = $2 AND configkey = $3";
        let row = sqlx::query(query)
            .bind(user)
            .bind(app)
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(row.get("configvalue")),
            None => Ok(default.unwrap_or("").to_string()),
        }
    }

    /// Sets a value in the preferences
    ///
    /// Adds a value to the preferences. If the key did not exist before, it
    /// will be added automatically.
    pub async fn set_value(&self, user: &str, app: &str, key: &str, value: &str) -> Result<(), sqlx::Error> {
        // Check if the key does exist
        let query = "SELECT COUNT(*) as count FROM *PREFIX*preferences WHERE userid = $1 AND appid = $2 AND configkey = $3";
        let row = sqlx::query(query)
            .bind(user)
            .bind(app)
            .bind(key)
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        let exists = count > 0;

        if !exists {
            let query = "INSERT INTO *PREFIX*preferences (userid, appid, configkey, configvalue) VALUES ($1, $2, $3, $4)";
            sqlx::query(query)
                .bind(user)
                .bind(app)
                .bind(key)
                .bind(value)
                .execute(&self.pool)
                .await?;
        } else {
            let query = "UPDATE *PREFIX*preferences SET configvalue = $1 WHERE userid = $2 AND appid = $3 AND configkey = $4";
            sqlx::query(query)
                .bind(value)
                .bind(user)
                .bind(app)
                .bind(key)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Deletes a key
    ///
    /// Deletes a key.
    pub async fn delete_key(&self, user: &str, app: &str, key: &str) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM *PREFIX*preferences WHERE userid = $1 AND appid = $2 AND configkey = $3";
        sqlx::query(query)
            .bind(user)
            .bind(app)
            .bind(key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Remove app of user from preferences
    ///
    /// Removes all keys in preferences belonging to the app and the user.
    pub async fn delete_app(&self, user: &str, app: &str) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM *PREFIX*preferences WHERE userid = $1 AND appid = $2";
        sqlx::query(query)
            .bind(user)
            .bind(app)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Remove user from preferences
    ///
    /// Removes all keys in preferences belonging to the user.
    pub async fn delete_user(&self, user: &str) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM *PREFIX*preferences WHERE userid = $1";
        sqlx::query(query)
            .bind(user)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Remove app from all users
    ///
    /// Removes all keys in preferences belonging to the app.
    pub async fn delete_app_from_all_users(&self, app: &str) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM *PREFIX*preferences WHERE appid = $1";
        sqlx::query(query)
            .bind(app)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[async_trait]
pub trait PreferencesInterface {
    async fn get_users(&self) -> Result<Vec<String>, sqlx::Error>;
    async fn get_apps(&self, user: &str) -> Result<Vec<String>, sqlx::Error>;
    async fn get_keys(&self, user: &str, app: &str) -> Result<Vec<String>, sqlx::Error>;
    async fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> Result<String, sqlx::Error>;
    async fn set_value(&self, user: &str, app: &str, key: &str, value: &str) -> Result<(), sqlx::Error>;
    async fn delete_key(&self, user: &str, app: &str, key: &str) -> Result<(), sqlx::Error>;
    async fn delete_app(&self, user: &str, app: &str) -> Result<(), sqlx::Error>;
    async fn delete_user(&self, user: &str) -> Result<(), sqlx::Error>;
    async fn delete_app_from_all_users(&self, app: &str) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl PreferencesInterface for Preferences {
    async fn get_users(&self) -> Result<Vec<String>, sqlx::Error> {
        self.get_users().await
    }

    async fn get_apps(&self, user: &str) -> Result<Vec<String>, sqlx::Error> {
        self.get_apps(user).await
    }

    async fn get_keys(&self, user: &str, app: &str) -> Result<Vec<String>, sqlx::Error> {
        self.get_keys(user, app).await
    }

    async fn get_value(&self, user: &str, app: &str, key: &str, default: Option<&str>) -> Result<String, sqlx::Error> {
        self.get_value(user, app, key, default).await
    }

    async fn set_value(&self, user: &str, app: &str, key: &str, value: &str) -> Result<(), sqlx::Error> {
        self.set_value(user, app, key, value).await
    }

    async fn delete_key(&self, user: &str, app: &str, key: &str) -> Result<(), sqlx::Error> {
        self.delete_key(user, app, key).await
    }

    async fn delete_app(&self, user: &str, app: &str) -> Result<(), sqlx::Error> {
        self.delete_app(user, app).await
    }

    async fn delete_user(&self, user: &str) -> Result<(), sqlx::Error> {
        self.delete_user(user).await
    }

    async fn delete_app_from_all_users(&self, app: &str) -> Result<(), sqlx::Error> {
        self.delete_app_from_all_users(app).await
    }
}