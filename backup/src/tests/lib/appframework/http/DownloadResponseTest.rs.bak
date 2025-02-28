/**
 * ownCloud - App Framework
 *
 * @author Bernhard Posselt
 * @copyright 2012 Bernhard Posselt nukeawhale@gmail.com
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

use std::collections::HashMap;
use mockall::{automock, predicate::*};

#[derive(Debug, Clone)]
pub struct DownloadResponse {
    filename: String,
    content_type: String,
}

impl DownloadResponse {
    pub fn new(filename: impl Into<String>, content_type: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content_type: content_type.into(),
        }
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Disposition".to_string(),
            format!("attachment; filename=\"{}\"", self.filename),
        );
        headers.insert("Content-Type".to_string(), self.content_type.clone());
        headers
    }
}

// Simple child class for testing inheritance
#[derive(Debug, Clone)]
pub struct ChildDownloadResponse(DownloadResponse);

impl ChildDownloadResponse {
    pub fn new(filename: impl Into<String>, content_type: impl Into<String>) -> Self {
        Self(DownloadResponse::new(filename, content_type))
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.0.get_headers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_headers() {
        let response = ChildDownloadResponse::new("file", "content");
        let headers = response.get_headers();

        assert!(headers
            .get("Content-Disposition")
            .unwrap()
            .contains("attachment; filename=\"file\""));
        assert_eq!(headers.get("Content-Type").unwrap(), "content");
    }
}