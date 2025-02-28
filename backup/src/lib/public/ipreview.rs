// Copyright (C) 2013 Frank Karlitschek <frank@owncloud.org>
// Copyright (C) 2013 Georg Ehrke <georg@owncloud.com>
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

// Public interface of ownCloud for apps to use.
// Preview interface

use std::path::Path;

use async_trait::async_trait;

/// This trait provides functions to render and show thumbnails and previews of files
#[async_trait]
pub trait Preview {
    /// The type representing an image in the system
    type Image;

    /// Return a preview of a file
    ///
    /// # Arguments
    /// * `file` - The path to the file where you want a thumbnail from
    /// * `max_x` - The maximum X size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `max_y` - The maximum Y size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `scale_up` - Scale smaller images up to the thumbnail size or not. Might look ugly
    ///
    /// # Returns
    /// A Result containing the image or an error
    async fn create_preview<P: AsRef<Path> + Send>(
        &self,
        file: P,
        max_x: u32,
        max_y: u32,
        scale_up: bool,
    ) -> Result<Self::Image, Box<dyn std::error::Error + Send + Sync>>;

    /// Returns true if the passed mime type is supported
    ///
    /// # Arguments
    /// * `mime_type` - The mime type to check for support
    ///
    /// # Returns
    /// True if the mime type is supported, false otherwise
    fn is_mime_supported(&self, mime_type: &str) -> bool;
}