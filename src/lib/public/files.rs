// lib/public/files.rs

/**
 * ownCloud
 *
 * @author Frank Karlitschek
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

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs;
use tempfile::{NamedTempFile, TempDir};
use async_trait::async_trait;

use crate::oc_helper;
use crate::oc_files::filesystem;
use crate::oc_app;

/**
 * Public interface of ownCloud for apps to use.
 * Files Class
 */

/**
 * This class provides access to the internal filesystem abstraction layer. Use
 * this class exlusively if you want to access files
 */
pub struct Files;

#[async_trait]
impl Files {
    /**
     * Recusive deletion of folders
     * @param path to the folder
     * @return bool
     */
    pub async fn rmdir_r<P: AsRef<Path>>(dir: P) -> io::Result<bool> {
        oc_helper::rmdir_r(dir).await
    }

    /**
     * Get the mimetype form a local file
     * @param path
     * @return string
     * does NOT work for ownClouds filesystem, use OC_FileSystem::getMimeType instead
     */
    pub async fn get_mime_type<P: AsRef<Path>>(path: P) -> io::Result<String> {
        oc_helper::get_mime_type(path).await
    }

    /**
     * Search for files by mimetype
     * @param mimetype
     * @return array
     */
    pub async fn search_by_mime(mimetype: &str) -> io::Result<Vec<PathBuf>> {
        filesystem::search_by_mime(mimetype).await
    }

    /**
     * Copy the contents of one stream to another
     * @param source
     * @param target
     * @return the number of bytes copied
     */
    pub async fn stream_copy<R: Read, W: Write>(source: &mut R, target: &mut W) -> io::Result<u64> {
        let (count, _) = oc_helper::stream_copy(source, target).await?;
        Ok(count)
    }

    /**
     * Create a temporary file with an unique filename
     * @param postfix
     * @return string
     *
     * temporary files are automatically cleaned up after the script is finished
     */
    pub fn tmp_file(postfix: &str) -> io::Result<NamedTempFile> {
        oc_helper::tmp_file(postfix)
    }

    /**
     * Create a temporary folder with an unique filename
     * @return string
     *
     * temporary files are automatically cleaned up after the script is finished
     */
    pub fn tmp_folder() -> io::Result<TempDir> {
        oc_helper::tmp_folder()
    }

    /**
     * Adds a suffix to the name in case the file exists
     * @param path
     * @param filename
     * @return string
     */
    pub fn build_not_existing_filename<P: AsRef<Path>>(path: P, filename: &str) -> PathBuf {
        oc_helper::build_not_existing_filename(path, filename)
    }

    /**
     * Gets the Storage for an app - creates the needed folder if they are not
     * existant
     * @param appid
     * @return \OC\Files\View
     */
    pub async fn get_storage(app: &str) -> io::Result<filesystem::View> {
        oc_app::get_storage(app).await
    }
}