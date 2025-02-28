/// ownCloud - LDAP Wizard Result
///
/// # Copyright
/// Copyright 2013 Arthur Schiwon blizzz@owncloud.com
///
/// This library is free software; you can redistribute it and/or
/// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
/// License as published by the Free Software Foundation; either
/// version 3 of the License, or any later version.
///
/// This library is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
/// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;

/// Result container for the LDAP configuration wizard
#[derive(Default, Debug, Clone)]
pub struct WizardResult {
    changes: HashMap<String, String>,
    options: HashMap<String, Vec<String>>,
    marked_change: bool,
}

impl WizardResult {
    /// Creates a new empty WizardResult
    pub fn new() -> Self {
        WizardResult {
            changes: HashMap::new(),
            options: HashMap::new(),
            marked_change: false,
        }
    }

    /// Adds a change to the result
    pub fn add_change<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.changes.insert(key.into(), value.into());
    }

    /// Marks that a change occurred without specifying details
    pub fn mark_change(&mut self) {
        self.marked_change = true;
    }

    /// Adds options to the result
    pub fn add_options<K, V>(&mut self, key: K, values: V)
    where
        K: Into<String>,
        V: Into<Vec<String>>,
    {
        self.options.insert(key.into(), values.into());
    }

    /// Returns true if there are changes
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty() || self.marked_change
    }

    /// Gets the result as a structured object
    pub fn get_result(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        
        // Convert changes to serde_json::Value
        let changes_value = serde_json::to_value(&self.changes).unwrap_or_default();
        result.insert("changes".to_string(), changes_value);
        
        // Add options if there are any
        if !self.options.is_empty() {
            let options_value = serde_json::to_value(&self.options).unwrap_or_default();
            result.insert("options".to_string(), options_value);
        }
        
        result
    }
}