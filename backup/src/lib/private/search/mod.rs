// Módulos generados automáticamente

pub mod provider;

pub mod result;
pub mod provider;

// Contenido fusionado desde src/lib/private/search.rs
// Copyright (c) 2012 Frank Karlitschek frank@owncloud.org
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

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// Result type for search operations
pub type SearchResult = Box<dyn Any + Send + Sync>;

/// Trait that all search providers must implement
pub trait SearchProvider: Send + Sync {
    /// Search for a query and return results
    fn search(&self, query: &str) -> Vec<SearchResult>;
}

/// Provider registration information
struct ProviderRegistration {
    class_name: String,
    options: HashMap<String, String>,
}

/// Provides an interface to all search providers
pub struct Search {
    providers: Mutex<Vec<Arc<dyn SearchProvider>>>,
    registered_providers: Mutex<Vec<ProviderRegistration>>,
}

impl Search {
    /// Get the singleton instance
    pub fn instance() -> &'static Search {
        static INSTANCE: Lazy<Search> = Lazy::new(|| Search {
            providers: Mutex::new(Vec::new()),
            registered_providers: Mutex::new(Vec::new()),
        });
        &INSTANCE
    }

    /// Remove all registered search providers
    pub fn clear_providers(&self) {
        let mut providers = self.providers.lock().unwrap();
        let mut registered_providers = self.registered_providers.lock().unwrap();
        
        providers.clear();
        registered_providers.clear();
    }

    /// Register a new search provider to be used
    pub fn register_provider<P>(&self, provider: P, options: HashMap<String, String>) 
    where
        P: SearchProvider + 'static
    {
        let mut registered_providers = self.registered_providers.lock().unwrap();
        
        registered_providers.push(ProviderRegistration {
            class_name: std::any::type_name::<P>().to_string(),
            options,
        });
        
        // Force regeneration of providers on next search
        let mut providers = self.providers.lock().unwrap();
        providers.clear();
    }

    /// Search all providers for a query
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        self.init_providers();
        
        let providers = self.providers.lock().unwrap();
        let mut results = Vec::new();
        
        for provider in providers.iter() {
            results.extend(provider.search(query));
        }
        
        results
    }

    /// Remove an existing search provider
    pub fn remove_provider(&self, provider_name: &str) {
        let mut registered_providers = self.registered_providers.lock().unwrap();
        
        registered_providers.retain(|reg| reg.class_name != provider_name);
        
        // Force regeneration of providers on next search
        let mut providers = self.providers.lock().unwrap();
        providers.clear();
    }

    /// Create instances of all the registered search providers
    fn init_providers(&self) {
        let mut providers = self.providers.lock().unwrap();
        
        if !providers.is_empty() {
            return;
        }
        
        let registered_providers = self.registered_providers.lock().unwrap();
        
        // In a real implementation, we would need a provider factory to instantiate
        // providers by name with their options. This is a simplified version.
        // The actual implementation would depend on the application's dependency injection system.
        
        // For demonstration purposes only:
        // providers = registered_providers.iter().map(|reg| {
        //     // Create provider instance based on class name and options
        //     // This would require a factory pattern or registry in real code
        // }).collect();
    }
}