// Copyright (C) 2012 Thomas Tanghus (thomas@tanghus.net)
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

// This file defines common constants used in ownCloud

/// CRUDS permissions.
pub mod constants {
    /// Create permission
    pub const PERMISSION_CREATE: u32 = 4;
    /// Read permission
    pub const PERMISSION_READ: u32 = 1;
    /// Update permission
    pub const PERMISSION_UPDATE: u32 = 2;
    /// Delete permission
    pub const PERMISSION_DELETE: u32 = 8;
    /// Share permission
    pub const PERMISSION_SHARE: u32 = 16;
    /// All permissions combined
    pub const PERMISSION_ALL: u32 = 31;
}