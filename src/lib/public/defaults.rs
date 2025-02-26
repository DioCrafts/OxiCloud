//! ownCloud
//!
//! @author Björn Schießle
//! @copyright 2013 Björn Schießle schiessle@owncloud.com
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

//! Public interface of ownCloud for apps to use.
//! Defaults struct

use crate::lib::private::OcDefaults;

/// Public api to access default strings and urls for your templates
pub struct Defaults {
    defaults: OcDefaults,
}

impl Defaults {
    /// Create a new Defaults instance
    pub fn new() -> Self {
        Self {
            defaults: OcDefaults::new(),
        }
    }

    /// Get base URL for the organisation behind your ownCloud instance
    pub fn get_base_url(&self) -> String {
        self.defaults.get_base_url()
    }

    /// Link to the desktop sync client
    pub fn get_sync_client_url(&self) -> String {
        self.defaults.get_sync_client_url()
    }

    /// Base URL to the documentation of your ownCloud instance
    pub fn get_doc_base_url(&self) -> String {
        self.defaults.get_doc_base_url()
    }

    /// Name of your ownCloud instance
    pub fn get_name(&self) -> String {
        self.defaults.get_name()
    }

    /// Entity behind your ownCloud instance
    pub fn get_entity(&self) -> String {
        self.defaults.get_entity()
    }

    /// ownCloud slogan
    pub fn get_slogan(&self) -> String {
        self.defaults.get_slogan()
    }

    /// Logo claim
    pub fn get_logo_claim(&self) -> String {
        self.defaults.get_logo_claim()
    }

    /// Footer, short version
    pub fn get_short_footer(&self) -> String {
        self.defaults.get_short_footer()
    }

    /// Footer, long version
    pub fn get_long_footer(&self) -> String {
        self.defaults.get_long_footer()
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self::new()
    }
}