// Copyright (c) 2013 Thomas Tanghus <thomas@tanghus.net>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// Public interface for apps to use.
// Tags interface.

use std::collections::HashMap;
use async_trait::async_trait;

/// Tag representation for returning tag data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagInfo {
    pub id: i64,
    pub name: String,
}

/// Class for easily tagging objects by their id
///
/// A tag can be e.g. 'Family', 'Work', 'Chore', 'Special Occation' or
/// anything else that is either parsed from a vobject or that the user chooses
/// to add.
/// Tag names are not case-sensitive, but will be saved with the case they
/// are entered in. If a user already has a tag 'family' for a type, and
/// tries to add a tag named 'Family' it will be silently ignored.
#[async_trait]
pub trait ITags {
    /// Check if any tags are saved for this type and user.
    ///
    /// # Returns
    /// `true` if no tags are saved, `false` otherwise.
    async fn is_empty(&self) -> bool;

    /// Get the tags for a specific user.
    ///
    /// # Returns
    /// An array of TagInfo containing id and name of tags
    async fn get_tags(&self) -> Result<Vec<TagInfo>, Box<dyn std::error::Error>>;

    /// Get a list of items tagged with `tag`.
    ///
    /// # Arguments
    /// * `tag` - Tag id or name.
    ///
    /// # Returns
    /// An array of object ids or error if tag could not be found.
    async fn get_ids_for_tag<T>(&self, tag: T) -> Result<Vec<i64>, Box<dyn std::error::Error>>
    where
        T: Into<TagIdentifier> + Send;

    /// Checks whether a tag is already saved.
    ///
    /// # Arguments
    /// * `name` - The name to check for.
    ///
    /// # Returns
    /// `true` if the tag exists, `false` otherwise.
    async fn has_tag(&self, name: &str) -> bool;

    /// Add a new tag.
    ///
    /// # Arguments
    /// * `name` - A string with a name of the tag
    ///
    /// # Returns
    /// The id of the added tag or error if it already exists.
    async fn add(&self, name: &str) -> Result<i64, Box<dyn std::error::Error>>;

    /// Rename tag.
    ///
    /// # Arguments
    /// * `from` - The name of the existing tag
    /// * `to` - The new name of the tag.
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn rename(&self, from: &str, to: &str) -> Result<bool, Box<dyn std::error::Error>>;

    /// Add a list of new tags.
    ///
    /// # Arguments
    /// * `names` - A vector of strings containing the name(s) of the tags to add.
    /// * `sync` - When true, save the tags
    /// * `id` - Optional object id to add to these tag(s)
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn add_multiple(
        &self,
        names: Vec<String>,
        sync: bool,
        id: Option<i64>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    /// Delete tag/object relations from the db
    ///
    /// # Arguments
    /// * `ids` - The ids of the objects
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn purge_objects(&self, ids: Vec<i64>) -> Result<bool, Box<dyn std::error::Error>>;

    /// Get favorites for an object type
    ///
    /// # Returns
    /// An array of object ids.
    async fn get_favorites(&self) -> Result<Vec<i64>, Box<dyn std::error::Error>>;

    /// Add an object to favorites
    ///
    /// # Arguments
    /// * `obj_id` - The id of the object
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn add_to_favorites(&self, obj_id: i64) -> Result<bool, Box<dyn std::error::Error>>;

    /// Remove an object from favorites
    ///
    /// # Arguments
    /// * `obj_id` - The id of the object
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn remove_from_favorites(&self, obj_id: i64) -> Result<bool, Box<dyn std::error::Error>>;

    /// Creates a tag/object relation.
    ///
    /// # Arguments
    /// * `obj_id` - The id of the object
    /// * `tag` - The id or name of the tag
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn tag_as<T>(&self, obj_id: i64, tag: T) -> Result<bool, Box<dyn std::error::Error>>
    where
        T: Into<TagIdentifier> + Send;

    /// Delete single tag/object relation from the db
    ///
    /// # Arguments
    /// * `obj_id` - The id of the object
    /// * `tag` - The id or name of the tag
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn untag<T>(&self, obj_id: i64, tag: T) -> Result<bool, Box<dyn std::error::Error>>
    where
        T: Into<TagIdentifier> + Send;

    /// Delete tags
    ///
    /// # Arguments
    /// * `names` - An array of tags to delete
    ///
    /// # Returns
    /// `true` if successful, error otherwise
    async fn delete(&self, names: Vec<String>) -> Result<bool, Box<dyn std::error::Error>>;
}

/// Enum to handle both string and numeric tag identifiers
#[derive(Debug, Clone)]
pub enum TagIdentifier {
    Id(i64),
    Name(String),
}

impl From<i64> for TagIdentifier {
    fn from(id: i64) -> Self {
        TagIdentifier::Id(id)
    }
}

impl From<String> for TagIdentifier {
    fn from(name: String) -> Self {
        TagIdentifier::Name(name)
    }
}

impl From<&str> for TagIdentifier {
    fn from(name: &str) -> Self {
        TagIdentifier::Name(name.to_string())
    }
}