use std::any::Any;

/// Loader manages storage wrappers and initializes storages
pub struct Loader {
    /// Storage wrappers that modify storage behavior
    storage_wrappers: Vec<Box<dyn Fn(&str, Box<dyn Storage>) -> Box<dyn Storage>>>,
}

/// Storage trait represents a file storage
pub trait Storage: Any {
    // Storage methods would go here
}

impl Loader {
    /// Create a new storage loader
    pub fn new() -> Self {
        Loader {
            storage_wrappers: Vec::new(),
        }
    }

    /// Allow modifier storage behavior by adding wrappers around storages
    ///
    /// `callback` should be a function of type (mountPoint: &str, storage: Box<dyn Storage>) -> Box<dyn Storage>
    pub fn add_storage_wrapper<F>(&mut self, callback: F)
    where
        F: Fn(&str, Box<dyn Storage>) -> Box<dyn Storage> + 'static,
    {
        self.storage_wrappers.push(Box::new(callback));
    }

    /// Load a storage with the given class and arguments
    pub fn load<T: Storage + 'static>(
        &self,
        mount_point: &str,
        create_storage: impl FnOnce() -> T,
    ) -> Box<dyn Storage> {
        let storage = Box::new(create_storage());
        self.wrap(mount_point, storage)
    }

    /// Wrap a storage with all registered wrappers
    pub fn wrap(&self, mount_point: &str, mut storage: Box<dyn Storage>) -> Box<dyn Storage> {
        for wrapper in &self.storage_wrappers {
            storage = wrapper(mount_point, storage);
        }
        storage
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}