// Copyright 2013 Arthur Schiwon blizzz@owncloud.com
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

/// Access trait for LDAP interactions
pub trait Access {}

/// BackendUtility serves as a base for LDAP backend implementations
pub struct BackendUtility {
    access: Arc<dyn Access>,
}

impl BackendUtility {
    /// Creates a new BackendUtility instance
    ///
    /// # Arguments
    ///
    /// * `access` - An implementation of the Access trait for LDAP interaction
    pub fn new(access: Arc<dyn Access>) -> Self {
        Self { access }
    }
}