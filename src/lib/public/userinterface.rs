// Copyright (c) 2012 Arthur Schiwon blizzz@owncloud.org
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

//! Public interface of ownCloud for apps to use.
//! User Interface

// Import the parent trait from internal namespace
use crate::oc::user_interface::OcUserInterface;

/// Public interface for User operations.
/// Applications should use this interface instead of internal ownCloud classes.
pub trait UserInterface: OcUserInterface {}