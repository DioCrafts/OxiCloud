use std::sync::Arc;

/// Trait defining the Navigation Manager interface
pub trait INavigationManager: Send + Sync {
    /// Creates a new navigation entry
    fn add(&mut self, entry: NavigationEntry);
    
    /// Returns all the added Menu entries
    fn get_all(&self) -> Vec<NavigationEntry>;
    
    /// Removes all the entries
    fn clear(&mut self);
    
    /// Sets the current navigation entry of the currently running app
    fn set_active_entry(&mut self, id: String);
    
    /// Gets the active Menu entry
    fn get_active_entry(&self) -> Option<String>;
}

/// Represents a navigation entry in the menu
#[derive(Clone, Debug)]
pub struct NavigationEntry {
    pub id: String,
    pub name: String,
    pub order: i32,
    pub icon: String,
    pub href: String,
    pub active: bool,
}

impl NavigationEntry {
    pub fn new(id: String, name: String, order: i32, href: String, icon: Option<String>) -> Self {
        NavigationEntry {
            id,
            name,
            order,
            href,
            icon: icon.unwrap_or_default(),
            active: false,
        }
    }
}

/// Manages the ownCloud navigation
#[derive(Default)]
pub struct NavigationManager {
    entries: Vec<NavigationEntry>,
    active_entry: Option<String>,
}

impl NavigationManager {
    /// Creates a new NavigationManager instance
    pub fn new() -> Self {
        NavigationManager {
            entries: Vec::new(),
            active_entry: None,
        }
    }
}

impl INavigationManager for NavigationManager {
    /// Creates a new navigation entry
    /// 
    /// # Arguments
    /// * `entry` - The navigation entry to add
    fn add(&mut self, mut entry: NavigationEntry) {
        entry.active = false;
        self.entries.push(entry);
    }

    /// Returns all the added Menu entries
    fn get_all(&self) -> Vec<NavigationEntry> {
        self.entries.clone()
    }

    /// Removes all the entries
    fn clear(&mut self) {
        self.entries.clear();
    }

    /// Sets the current navigation entry of the currently running app
    /// 
    /// # Arguments
    /// * `id` - ID of the app entry to activate (from added entries)
    fn set_active_entry(&mut self, id: String) {
        self.active_entry = Some(id);
    }

    /// Gets the active Menu entry
    /// 
    /// This function returns the id of the active navigation entry (set by
    /// set_active_entry)
    fn get_active_entry(&self) -> Option<String> {
        self.active_entry.clone()
    }
}

// Factory function to create a thread-safe NavigationManager instance
pub fn create_navigation_manager() -> Arc<dyn INavigationManager> {
    Arc::new(NavigationManager::new())
}