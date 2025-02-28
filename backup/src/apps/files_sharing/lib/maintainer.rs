// Copyright 2013 Morris Jobke morris.jobke@gmail.com
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

use crate::share;

/// Parameters for configuration changes
pub struct ConfigChangeParams {
    pub app: String,
    pub key: String,
    pub value: String,
}

/// Maintains stuff around the sharing functionality
///
/// for example: on disable of "allow links" it removes all link shares
pub struct Maintainer;

impl Maintainer {
    /// Keeps track of the "allow links" config setting
    /// and removes all link shares if the config option is set to "no"
    ///
    /// # Arguments
    ///
    /// * `params` - Configuration parameters containing app, key, and value
    ///
    /// # Errors
    ///
    /// Returns an error if removing link shares fails
    pub async fn config_change_hook(params: &ConfigChangeParams) -> Result<(), share::Error> {
        if params.app == "core" && params.key == "shareapi_allow_links" && params.value == "no" {
            share::remove_all_link_shares().await?;
        }
        Ok(())
    }
}