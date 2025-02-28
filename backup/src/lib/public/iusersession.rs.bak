// Copyright (C) 2013 Bart Visscher bartv@thisnet.nl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Public interface of ownCloud for apps to use.
// User session interface

/// User session
///
/// This trait defines the interface for a user session
pub trait UserSession {
    /// Do a user login
    ///
    /// # Arguments
    ///
    /// * `user` - the username
    /// * `password` - the password
    ///
    /// # Returns
    ///
    /// `true` if login was successful, `false` otherwise
    fn login(&self, user: &str, password: &str) -> bool;

    /// Logs the user out including all the session data
    /// Logout, destroys session
    fn logout(&self);
}