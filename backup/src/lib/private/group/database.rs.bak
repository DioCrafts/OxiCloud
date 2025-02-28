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
 * CREATE TABLE `groups` (
 *   `gid` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
 *   PRIMARY KEY (`gid`)
 * ) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
 *
 * CREATE TABLE `group_user` (
 *   `gid` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
 *   `uid` varchar(64) COLLATE utf8_unicode_ci NOT NULL
 * ) ENGINE=MyISAM DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
 */

use async_trait::async_trait;
use sqlx::{Pool, Row, SqlitePool};
use std::collections::HashMap;

/// Trait for group backend implementations
#[async_trait]
pub trait GroupBackend {
    async fn create_group(&self, gid: &str) -> Result<bool, sqlx::Error>;
    async fn delete_group(&self, gid: &str) -> Result<bool, sqlx::Error>;
    async fn in_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error>;
    async fn add_to_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error>;
    async fn remove_from_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error>;
    async fn get_user_groups(&self, uid: &str) -> Result<Vec<String>, sqlx::Error>;
    async fn get_groups(&self, search: &str, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<String>, sqlx::Error>;
    async fn group_exists(&self, gid: &str) -> Result<bool, sqlx::Error>;
    async fn users_in_group(&self, gid: &str, search: &str, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<String>, sqlx::Error>;
    async fn display_names_in_group(&self, gid: &str, search: &str, limit: i64, offset: i64) -> Result<HashMap<String, String>, sqlx::Error>;
}

/// Class for group management in a SQL Database (e.g. MySQL, SQLite)
pub struct GroupDatabase {
    pool: Pool<sqlx::Sqlite>,
}

impl GroupDatabase {
    pub fn new(pool: Pool<sqlx::Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GroupBackend for GroupDatabase {
    /// Try to create a new group
    ///
    /// Tries to create a new group. If the group name already exists, false will
    /// be returned.
    async fn create_group(&self, gid: &str) -> Result<bool, sqlx::Error> {
        // Check for existence
        let exists = sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` = ?")
            .bind(gid)
            .fetch_optional(&self.pool)
            .await?;

        if exists.is_some() {
            // Can not add an existing group
            Ok(false)
        } else {
            // Add group and exit
            let result = sqlx::query("INSERT INTO `*PREFIX*groups` (`gid`) VALUES(?)")
                .bind(gid)
                .execute(&self.pool)
                .await?;
            
            Ok(result.rows_affected() > 0)
        }
    }

    /// Delete a group
    ///
    /// Deletes a group and removes it from the group_user-table
    async fn delete_group(&self, gid: &str) -> Result<bool, sqlx::Error> {
        // Delete the group
        sqlx::query("DELETE FROM `*PREFIX*groups` WHERE `gid` = ?")
            .bind(gid)
            .execute(&self.pool)
            .await?;

        // Delete the group-user relation
        sqlx::query("DELETE FROM `*PREFIX*group_user` WHERE `gid` = ?")
            .bind(gid)
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    /// Is user in group?
    ///
    /// Checks whether the user is member of a group or not.
    async fn in_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT `uid` FROM `*PREFIX*group_user` WHERE `gid` = ? AND `uid` = ?")
            .bind(gid)
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    /// Add a user to a group
    ///
    /// Adds a user to a group.
    async fn add_to_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error> {
        // No duplicate entries!
        if !self.in_group(uid, gid).await? {
            sqlx::query("INSERT INTO `*PREFIX*group_user` (`uid`, `gid`) VALUES(?, ?)")
                .bind(uid)
                .bind(gid)
                .execute(&self.pool)
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Removes a user from a group
    ///
    /// Removes the user from a group.
    async fn remove_from_group(&self, uid: &str, gid: &str) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM `*PREFIX*group_user` WHERE `uid` = ? AND `gid` = ?")
            .bind(uid)
            .bind(gid)
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    /// Get all groups a user belongs to
    ///
    /// This function fetches all groups a user belongs to. It does not check
    /// if the user exists at all.
    async fn get_user_groups(&self, uid: &str) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query("SELECT `gid` FROM `*PREFIX*group_user` WHERE `uid` = ?")
            .bind(uid)
            .fetch_all(&self.pool)
            .await?;

        let mut groups = Vec::with_capacity(rows.len());
        for row in rows {
            groups.push(row.get("gid"));
        }

        Ok(groups)
    }

    /// Get a list of all groups
    ///
    /// Returns a list with all groups
    async fn get_groups(&self, search: &str, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<String>, sqlx::Error> {
        let search_pattern = format!("{}%", search);
        
        let query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` LIKE ? LIMIT ? OFFSET ?")
                    .bind(&search_pattern)
                    .bind(limit)
                    .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` LIKE ? LIMIT ?")
                    .bind(&search_pattern)
                    .bind(limit)
            },
            (None, Some(offset)) => {
                sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` LIKE ? OFFSET ?")
                    .bind(&search_pattern)
                    .bind(offset)
            },
            (None, None) => {
                sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` LIKE ?")
                    .bind(&search_pattern)
            }
        };

        let rows = query.fetch_all(&self.pool).await?;
        
        let mut groups = Vec::with_capacity(rows.len());
        for row in rows {
            groups.push(row.get("gid"));
        }
        
        Ok(groups)
    }

    /// Check if a group exists
    async fn group_exists(&self, gid: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT `gid` FROM `*PREFIX*groups` WHERE `gid` = ?")
            .bind(gid)
            .fetch_optional(&self.pool)
            .await?;
            
        Ok(result.is_some())
    }

    /// Get a list of all users in a group
    async fn users_in_group(&self, gid: &str, search: &str, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<String>, sqlx::Error> {
        let search_pattern = format!("{}%", search);
        
        let query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query("SELECT `uid` FROM `*PREFIX*group_user` WHERE `gid` = ? AND `uid` LIKE ? LIMIT ? OFFSET ?")
                    .bind(gid)
                    .bind(&search_pattern)
                    .bind(limit)
                    .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query("SELECT `uid` FROM `*PREFIX*group_user` WHERE `gid` = ? AND `uid` LIKE ? LIMIT ?")
                    .bind(gid)
                    .bind(&search_pattern)
                    .bind(limit)
            },
            (None, Some(offset)) => {
                sqlx::query("SELECT `uid` FROM `*PREFIX*group_user` WHERE `gid` = ? AND `uid` LIKE ? OFFSET ?")
                    .bind(gid)
                    .bind(&search_pattern)
                    .bind(offset)
            },
            (None, None) => {
                sqlx::query("SELECT `uid` FROM `*PREFIX*group_user` WHERE `gid` = ? AND `uid` LIKE ?")
                    .bind(gid)
                    .bind(&search_pattern)
            }
        };

        let rows = query.fetch_all(&self.pool).await?;
        
        let mut users = Vec::with_capacity(rows.len());
        for row in rows {
            users.push(row.get("uid"));
        }
        
        Ok(users)
    }

    /// Get a list of all display names in a group
    async fn display_names_in_group(&self, gid: &str, search: &str, limit: i64, offset: i64) -> Result<HashMap<String, String>, sqlx::Error> {
        let search_pattern = format!("{}%", search);
        
        let query_string = "SELECT `*PREFIX*users`.`uid`, `*PREFIX*users`.`displayname` \
            FROM `*PREFIX*users` \
            INNER JOIN `*PREFIX*group_user` ON `*PREFIX*group_user`.`uid` = `*PREFIX*users`.`uid` \
            WHERE `gid` = ? AND `*PREFIX*group_user`.`uid` LIKE ?";
            
        let query = if limit >= 0 {
            format!("{} LIMIT ? OFFSET ?", query_string)
        } else {
            query_string.to_string()
        };
        
        let rows = if limit >= 0 {
            sqlx::query(&query)
                .bind(gid)
                .bind(&search_pattern)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query_string)
                .bind(gid)
                .bind(&search_pattern)
                .fetch_all(&self.pool)
                .await?
        };
        
        let mut display_names = HashMap::new();
        for row in rows {
            let uid: String = row.get("uid");
            let display_name: String = row.get("displayname");
            let display_name = display_name.trim().to_string();
            
            display_names.insert(uid.clone(), if display_name.is_empty() { uid } else { display_name });
        }
        
        Ok(display_names)
    }
}