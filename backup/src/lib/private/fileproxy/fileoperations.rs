// Copyright (C) 2012 Bjoern Schiessle <schiessle@owncloud.com>
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

use std::sync::Arc;
use std::sync::RwLock;
use once_cell::sync::Lazy;

use crate::files::view::View;

/// Check if standard file operations can be performed
pub struct FileOperationsProxy {
    root_view: Arc<RwLock<Option<View>>>,
}

impl Default for FileOperationsProxy {
    fn default() -> Self {
        Self {
            root_view: Arc::new(RwLock::new(None)),
        }
    }
}

impl FileProxy for FileOperationsProxy {
    fn pre_mkdir(&self, path: &str) -> Result<bool, FileProxyError> {
        let mut view_lock = self.root_view.write().map_err(|_| FileProxyError::LockError)?;
        
        if view_lock.is_none() {
            *view_lock = Some(View::new(""));
        }
        
        if let Some(view) = &*view_lock {
            view.file_exists(path).map(|exists| !exists)
        } else {
            Err(FileProxyError::ViewNotInitialized)
        }
    }
}

/// Trait defining file proxy operations
pub trait FileProxy {
    fn pre_mkdir(&self, path: &str) -> Result<bool, FileProxyError>;
}

/// Errors that can occur during file proxy operations
#[derive(Debug, thiserror::Error)]
pub enum FileProxyError {
    #[error("Failed to acquire lock")]
    LockError,
    
    #[error("View not initialized")]
    ViewNotInitialized,
    
    #[error("File system error: {0}")]
    FileSystemError(String),
}