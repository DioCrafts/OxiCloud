use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Forward declaration of the Emitter trait
pub trait Emitter {
    fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]);
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static;
}

/// Basic implementation of the Emitter trait
pub struct BasicEmitter {
    listeners: HashMap<String, Vec<Arc<dyn Fn(&[&dyn std::any::Any]) + Send + Sync>>>,
}

impl BasicEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
}

impl Emitter for BasicEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        let key = format!("{}::{}", scope, method);
        if let Some(callbacks) = self.listeners.get(&key) {
            for callback in callbacks {
                callback(arguments);
            }
        }
    }

    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        let key = format!("{}::{}", scope, method);
        let callbacks = self.listeners.entry(key).or_insert_with(Vec::new);
        callbacks.push(Arc::new(callback));
    }
}

/// Public emitter for testing purposes
pub struct PublicEmitter {
    basic_emitter: BasicEmitter,
}

impl PublicEmitter {
    pub fn new() -> Self {
        Self {
            basic_emitter: BasicEmitter::new(),
        }
    }
}

impl Emitter for PublicEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        self.basic_emitter.emit(scope, method, arguments);
    }

    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        self.basic_emitter.listen(scope, method, callback);
    }
}

/**
 * allows forwarding all listen calls to other emitters
 */
pub struct ForwardingEmitter {
    basic_emitter: BasicEmitter,
    emitters: Vec<Arc<Mutex<dyn Emitter + Send>>>,
}

impl ForwardingEmitter {
    pub fn new() -> Self {
        Self {
            basic_emitter: BasicEmitter::new(),
            emitters: Vec::new(),
        }
    }

    pub fn forward(&mut self, emitter: Arc<Mutex<dyn Emitter + Send>>) {
        self.emitters.push(emitter);
    }
}

impl Emitter for ForwardingEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        self.basic_emitter.emit(scope, method, arguments);
    }

    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        self.basic_emitter.listen(scope, method, callback);
        
        // Forward the listener to all registered emitters
        for emitter in &self.emitters {
            if let Ok(mut emitter) = emitter.lock() {
                emitter.listen(scope, method, callback.clone());
            }
        }
    }
}

/// Dummy implementation for testing
pub struct DummyForwardingEmitter {
    forwarding_emitter: ForwardingEmitter,
}

impl DummyForwardingEmitter {
    pub fn new() -> Self {
        Self {
            forwarding_emitter: ForwardingEmitter::new(),
        }
    }

    pub fn emit_event(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        self.emit(scope, method, arguments);
    }

    pub fn forward(&mut self, emitter: Arc<Mutex<dyn Emitter + Send>>) {
        self.forwarding_emitter.forward(emitter);
    }
}

impl Emitter for DummyForwardingEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        self.forwarding_emitter.emit(scope, method, arguments);
    }

    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        self.forwarding_emitter.listen(scope, method, callback);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct ForwardingEmitterTest;

    impl ForwardingEmitterTest {
        pub fn test_single_forward() {
            let base_emitter = Arc::new(Mutex::new(PublicEmitter::new()));
            let mut forwarding_emitter = DummyForwardingEmitter::new();
            
            forwarding_emitter.forward(base_emitter.clone());
            
            let hook_called = Rc::new(Cell::new(false));
            let hook_called_clone = hook_called.clone();
            
            forwarding_emitter.listen("Test", "test", move |_| {
                hook_called_clone.set(true);
            });
            
            if let Ok(emitter) = base_emitter.lock() {
                emitter.emit("Test", "test", &[]);
            }
            
            assert!(hook_called.get());
        }

        pub fn test_multiple_forwards() {
            let base_emitter1 = Arc::new(Mutex::new(PublicEmitter::new()));
            let base_emitter2 = Arc::new(Mutex::new(PublicEmitter::new()));
            let mut forwarding_emitter = DummyForwardingEmitter::new();
            
            forwarding_emitter.forward(base_emitter1.clone());
            forwarding_emitter.forward(base_emitter2.clone());
            
            let hook_called = Rc::new(Cell::new(0));
            let hook_called_clone1 = hook_called.clone();
            let hook_called_clone2 = hook_called.clone();
            
            forwarding_emitter.listen("Test", "test1", move |_| {
                hook_called_clone1.set(hook_called_clone1.get() + 1);
            });
            
            forwarding_emitter.listen("Test", "test2", move |_| {
                hook_called_clone2.set(hook_called_clone2.get() + 1);
            });
            
            if let Ok(emitter) = base_emitter1.lock() {
                emitter.emit("Test", "test1", &[]);
                emitter.emit("Test", "test2", &[]);
            }
            
            assert_eq!(hook_called.get(), 2);
        }

        pub fn test_forward_existing_hooks() {
            let base_emitter = Arc::new(Mutex::new(PublicEmitter::new()));
            let mut forwarding_emitter = DummyForwardingEmitter::new();
            
            let hook_called = Rc::new(Cell::new(false));
            let hook_called_clone = hook_called.clone();
            
            forwarding_emitter.listen("Test", "test", move |_| {
                hook_called_clone.set(true);
            });
            
            forwarding_emitter.forward(base_emitter.clone());
            
            if let Ok(emitter) = base_emitter.lock() {
                emitter.emit("Test", "test", &[]);
            }
            
            assert!(hook_called.get());
        }
    }
}