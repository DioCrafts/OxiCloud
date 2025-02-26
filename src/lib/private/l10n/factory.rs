// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// L10N Factory for managing localization instances
pub struct Factory {
    /// Cached instances of localization objects
    instances: Mutex<HashMap<String, Arc<L10n>>>,
}

/// L10n is the trait that defines localization functionality
pub trait L10n {
    // Define localization interface methods here
}

/// OcL10n implements the L10n trait
pub struct OcL10n {
    app: String,
    // Add other fields as needed
}

impl L10n for OcL10n {
    // Implementation of L10n trait methods
}

impl OcL10n {
    /// Create a new OcL10n instance
    pub fn new(app: &str) -> Self {
        Self {
            app: app.to_string(),
            // Initialize other fields
        }
    }
}

impl Factory {
    /// Create a new Factory
    pub fn new() -> Self {
        Self {
            instances: Mutex::new(HashMap::new()),
        }
    }

    /// Get an L10N instance for the specified app
    ///
    /// # Arguments
    ///
    /// * `app` - The application name
    ///
    /// # Returns
    ///
    /// An Arc reference to the L10n implementation
    pub fn get(&self, app: &str) -> Arc<dyn L10n> {
        let mut instances = self.instances.lock().unwrap();
        
        if !instances.contains_key(app) {
            let l10n = Arc::new(OcL10n::new(app)) as Arc<dyn L10n>;
            instances.insert(app.to_string(), l10n.clone());
            return l10n;
        }
        
        instances.get(app).unwrap().clone()
    }
}

impl Default for Factory {
    fn default() -> Self {
        Self::new()
    }
}