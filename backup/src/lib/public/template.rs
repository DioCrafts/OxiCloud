// ownCloud
//
// @author Frank Karlitschek
// @copyright 2012 Frank Karlitschek frank@owncloud.org
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
// Template module

use crate::internal;
use std::collections::HashMap;

/// Make image_path available as a simple function
/// 
/// # Arguments
/// * `app` - Application name
/// * `image` - Image name
/// 
/// # Returns
/// Link to the image
///
/// # See also
/// `internal::helpers::image_path`
pub fn image_path(app: &str, image: &str) -> String {
    internal::helpers::image_path(app, image)
}

/// Make mimetype_icon available as a simple function
/// 
/// # Arguments
/// * `mimetype` - MIME type
/// 
/// # Returns
/// Path to the image of this file type
pub fn mimetype_icon(mimetype: &str) -> String {
    internal::helpers::mimetype_icon(mimetype)
}

/// Make preview_icon available as a simple function
/// 
/// # Arguments
/// * `path` - Path of file
/// 
/// # Returns
/// Path to the preview of the image
pub fn preview_icon(path: &str) -> String {
    internal::helpers::preview_icon(path)
}

/// Make public_preview_icon available as a simple function
/// Returns the path to the preview of the image.
/// 
/// # Arguments
/// * `path` - Path of file
/// * `token` - Token for accessing the file
/// 
/// # Returns
/// Link to the preview
pub fn public_preview_icon(path: &str, token: &str) -> String {
    internal::helpers::public_preview_icon(path, token)
}

/// Make human_file_size available as a simple function
/// Example: 2048 to 2 kB.
/// 
/// # Arguments
/// * `bytes` - Size in bytes
/// 
/// # Returns
/// Size as string
pub fn human_file_size(bytes: u64) -> String {
    internal::helpers::human_file_size(bytes)
}

/// Return the relative date in relation to today. Returns something like "last hour" or "two month ago"
/// 
/// # Arguments
/// * `timestamp` - Unix timestamp
/// * `date_only` - Date only flag
/// 
/// # Returns
/// Human readable interpretation of the timestamp
pub fn relative_modified_date(timestamp: i64, date_only: bool) -> String {
    internal::helpers::relative_modified_date(timestamp, None, date_only)
}

/// Return a human readable output for a file size.
/// 
/// # Deprecated
/// Use `human_file_size()` instead
/// 
/// # Arguments
/// * `bytes` - Size of a file in bytes
/// 
/// # Returns
/// Human readable interpretation of a file size
#[deprecated(since = "0.1.0", note = "Please use `human_file_size` instead")]
pub fn simple_file_size(bytes: u64) -> String {
    human_file_size(bytes)
}

/// Generate html code for an options block.
/// 
/// # Arguments
/// * `options` - The options
/// * `selected` - Which one is selected?
/// * `params` - The parameters
/// 
/// # Returns
/// HTML options
pub fn html_select_options(
    options: &HashMap<String, String>,
    selected: &str,
    params: Option<&HashMap<String, String>>
) -> String {
    internal::helpers::html_select_options(options, selected, params)
}

/// This struct provides the template system for owncloud.
/// You can use it to load specific templates, add data and generate the html code
pub struct Template {
    inner: internal::template::Template,
}

impl Template {
    /// Create a new template
    ///
    /// # Arguments
    /// * `app` - App name
    /// * `name` - Template name
    /// * `render_as` - Render as option
    pub fn new(app: &str, name: &str, render_as: Option<&str>) -> Self {
        Template {
            inner: internal::template::Template::new(app, name, render_as),
        }
    }

    /// Assign a variable to the template
    ///
    /// # Arguments
    /// * `key` - Variable name
    /// * `value` - Variable value
    pub fn assign<T: serde::Serialize>(&mut self, key: &str, value: T) {
        self.inner.assign(key, value);
    }

    /// Append a template
    ///
    /// # Arguments
    /// * `template` - Template to append
    pub fn append_template(&mut self, template: &Template) {
        self.inner.append_template(&template.inner);
    }

    /// Fetch and return the template
    ///
    /// # Returns
    /// Rendered template as string
    pub fn fetch(&self) -> String {
        self.inner.fetch()
    }

    /// Print the template
    pub fn print_output(&self) {
        self.inner.print_output();
    }
}