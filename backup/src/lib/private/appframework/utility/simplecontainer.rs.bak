use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Trait defining a container for dependency injection
pub trait IContainer {
    fn query(&self, name: &str) -> Arc<dyn Any + Send + Sync>;
    fn register_parameter(&mut self, name: &str, value: Arc<dyn Any + Send + Sync>);
    fn register_service(&mut self, name: &str, factory: Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>, shared: bool);
}

/// SimpleContainer is a simple implementation of IContainer
///
/// This is a Rust implementation of the original PHP SimpleContainer,
/// which was based on Pimple.
pub struct SimpleContainer {
    services: Mutex<HashMap<String, ServiceDefinition>>,
    instances: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

enum ServiceDefinition {
    Shared(Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>),
    Factory(Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>),
    Parameter(Arc<dyn Any + Send + Sync>),
}

impl SimpleContainer {
    pub fn new() -> Self {
        SimpleContainer {
            services: Mutex::new(HashMap::new()),
            instances: Mutex::new(HashMap::new()),
        }
    }
}

impl IContainer for SimpleContainer {
    /// Query for a registered service
    ///
    /// # Arguments
    ///
    /// * `name` - name of the service to query for
    ///
    /// # Returns
    ///
    /// The registered service for the given name
    ///
    /// # Panics
    ///
    /// Panics if the service doesn't exist
    fn query(&self, name: &str) -> Arc<dyn Any + Send + Sync> {
        // Check if we have a cached instance for shared services
        {
            let instances = self.instances.lock().unwrap();
            if let Some(instance) = instances.get(name) {
                return instance.clone();
            }
        }

        // Create new instance
        let services = self.services.lock().unwrap();
        let service_def = services.get(name).unwrap_or_else(|| {
            panic!("Service '{}' not found in container", name)
        });

        match service_def {
            ServiceDefinition::Shared(factory) => {
                let instance = factory();
                let mut instances = self.instances.lock().unwrap();
                instances.insert(name.to_string(), instance.clone());
                instance
            },
            ServiceDefinition::Factory(factory) => {
                factory()
            },
            ServiceDefinition::Parameter(value) => {
                value.clone()
            },
        }
    }

    fn register_parameter(&mut self, name: &str, value: Arc<dyn Any + Send + Sync>) {
        let mut services = self.services.lock().unwrap();
        services.insert(name.to_string(), ServiceDefinition::Parameter(value));
    }

    /// Register a service factory
    ///
    /// The given closure is called the first time the given service is queried.
    /// The closure has to return the instance for the given service.
    /// Created instance will be cached in case shared is true.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the service to register
    /// * `factory` - the closure to be called on service creation
    /// * `shared` - whether to cache the instance
    fn register_service(&mut self, name: &str, factory: Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>, shared: bool) {
        let mut services = self.services.lock().unwrap();
        
        if shared {
            services.insert(name.to_string(), ServiceDefinition::Shared(factory));
        } else {
            services.insert(name.to_string(), ServiceDefinition::Factory(factory));
        }
    }
}

impl Default for SimpleContainer {
    fn default() -> Self {
        Self::new()
    }
}