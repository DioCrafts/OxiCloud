// Copyright (c) 2013 Karl Beecher - karl@endocode.com
//
// SPDX-License-Identifier: AGPL-3.0-or-later
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

/// Trait representing an Apache authentication backend
pub trait ApacheBackend {
    /// In case the user has been authenticated by Apache true is returned.
    ///
    /// # Returns
    /// Whether Apache reports a user as currently logged in.
    fn is_session_active(&self) -> bool;

    /// Creates an attribute which is added to the logout hyperlink. It can
    /// supply any attribute(s) which are valid for <a>.
    ///
    /// # Returns
    /// String with one or more HTML attributes.
    fn get_logout_attribute(&self) -> String;

    /// Return the id of the current user
    ///
    /// # Returns
    /// The current user's ID
    fn get_current_user_id(&self) -> String;
}