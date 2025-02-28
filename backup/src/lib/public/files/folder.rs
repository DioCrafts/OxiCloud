// Copyright (C) 2013 Robin Appelman <icewind@owncloud.com>
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

use std::path::Path;
use async_trait::async_trait;

use crate::ocp::files::{File, Node, NotPermittedException, NotFoundException};

/// Folder interface for OCP Files
#[async_trait]
pub trait Folder: Node {
    /// Get the full path of an item in the folder within ownCloud's filesystem
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path of an item in the folder
    ///
    /// # Returns
    ///
    /// The full path as a String
    ///
    /// # Errors
    ///
    /// Returns NotPermittedException if the operation is not permitted
    async fn get_full_path<P: AsRef<Path> + Send>(&self, path: P) -> Result<String, NotPermittedException>;

    /// Get the path of an item in the folder relative to the folder
    ///
    /// # Arguments
    ///
    /// * `path` - Absolute path of an item in the folder
    ///
    /// # Returns
    ///
    /// The relative path as a String
    ///
    /// # Errors
    ///
    /// Returns NotFoundException if the path doesn't exist
    async fn get_relative_path<P: AsRef<Path> + Send>(&self, path: P) -> Result<String, NotFoundException>;

    /// Check if a node is a (grand-)child of the folder
    ///
    /// # Arguments
    ///
    /// * `node` - The node to check
    ///
    /// # Returns
    ///
    /// `true` if the node is a subnode of this folder, `false` otherwise
    async fn is_sub_node(&self, node: &dyn Node) -> bool;

    /// Get the content of this directory
    ///
    /// # Returns
    ///
    /// A vector of nodes in this directory
    ///
    /// # Errors
    ///
    /// Returns NotFoundException if the directory doesn't exist
    async fn get_directory_listing(&self) -> Result<Vec<Box<dyn Node>>, NotFoundException>;

    /// Get the node at the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path of the file or folder
    ///
    /// # Returns
    ///
    /// The requested node
    ///
    /// # Errors
    ///
    /// Returns NotFoundException if the path doesn't exist
    async fn get<P: AsRef<Path> + Send>(&self, path: P) -> Result<Box<dyn Node>, NotFoundException>;

    /// Check if a file or folder exists in the folder
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path of the file or folder
    ///
    /// # Returns
    ///
    /// `true` if the node exists, `false` otherwise
    async fn node_exists<P: AsRef<Path> + Send>(&self, path: P) -> bool;

    /// Create a new folder
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path of the new folder
    ///
    /// # Returns
    ///
    /// The newly created folder
    ///
    /// # Errors
    ///
    /// Returns NotPermittedException if the operation is not permitted
    async fn new_folder<P: AsRef<Path> + Send>(&self, path: P) -> Result<Box<dyn Folder>, NotPermittedException>;

    /// Create a new file
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path of the new file
    ///
    /// # Returns
    ///
    /// The newly created file
    ///
    /// # Errors
    ///
    /// Returns NotPermittedException if the operation is not permitted
    async fn new_file<P: AsRef<Path> + Send>(&self, path: P) -> Result<Box<dyn File>, NotPermittedException>;

    /// Search for files with the name matching the query
    ///
    /// # Arguments
    ///
    /// * `query` - The search query
    ///
    /// # Returns
    ///
    /// Vector of nodes matching the query
    async fn search(&self, query: &str) -> Vec<Box<dyn Node>>;

    /// Search for files by mimetype
    ///
    /// The mimetype can either be a full mimetype (image/png) or a wildcard mimetype (image)
    ///
    /// # Arguments
    ///
    /// * `mimetype` - The mimetype to search for
    ///
    /// # Returns
    ///
    /// Vector of nodes matching the mimetype
    async fn search_by_mime(&self, mimetype: &str) -> Vec<Box<dyn Node>>;

    /// Get a file or folder inside the folder by its internal id
    ///
    /// # Arguments
    ///
    /// * `id` - The internal id
    ///
    /// # Returns
    ///
    /// Vector of nodes with the matching id
    async fn get_by_id(&self, id: i64) -> Vec<Box<dyn Node>>;

    /// Get the amount of free space inside the folder
    ///
    /// # Returns
    ///
    /// The amount of free space in bytes
    async fn get_free_space(&self) -> u64;

    /// Check if new files or folders can be created within the folder
    ///
    /// # Returns
    ///
    /// `true` if new content can be created, `false` otherwise
    async fn is_creatable(&self) -> bool;
}