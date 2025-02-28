// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
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

use std::collections::HashMap;

/// Represents a navigation entry
#[derive(Clone, Debug)]
pub struct NavigationEntry {
    pub id: String,
    pub name: String,
    pub order: i32,
    pub icon: String,
    pub href: String,
}

impl NavigationEntry {
    pub fn from_map(map: HashMap<String, String>) -> Option<Self> {
        Some(Self {
            id: map.get("id")?.clone(),
            name: map.get("name")?.clone(),
            order: map.get("order").and_then(|o| o.parse().ok()).unwrap_or(0),
            icon: map.get("icon")?.clone(),
            href: map.get("href")?.clone(),
        })
    }
}

/// Manages the ownCloud navigation
pub trait INavigationManager {
    /// Creates a new navigation entry
    ///
    /// # Parameters
    /// * `entry` - The navigation entry to add
    fn add(&mut self, entry: NavigationEntry);

    /// Sets the current navigation entry of the currently running app
    ///
    /// # Parameters
    /// * `app_id` - ID of the app entry to activate
    fn set_active_entry(&mut self, app_id: &str);
}