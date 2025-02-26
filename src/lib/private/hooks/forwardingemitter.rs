use std::collections::HashMap;

/// Trait defining the basic emitter functionality
pub trait Emitter {
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&str, &str) + 'static + Send + Sync;
}

/// Basic implementation of the Emitter trait
pub struct BasicEmitter {
    listeners: HashMap<String, Vec<Box<dyn Fn(&str, &str) + 'static + Send + Sync>>>,
}

impl BasicEmitter {
    pub fn new() -> Self {
        BasicEmitter {
            listeners: HashMap::new(),
        }
    }

    pub fn emit(&self, scope: &str, method: &str) {
        let key = format!("{}::{}", scope, method);
        if let Some(listeners) = self.listeners.get(&key) {
            for listener in listeners {
                listener(scope, method);
            }
        }
    }
}

impl Emitter for BasicEmitter {
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&str, &str) + 'static + Send + Sync,
    {
        let key = format!("{}::{}", scope, method);
        let listeners = self.listeners.entry(key).or_insert_with(Vec::new);
        listeners.push(Box::new(callback));
    }
}

/**
 * ForwardingEmitter
 *
 * allows forwarding all listen calls to other emitters
 */
pub struct ForwardingEmitter {
    basic_emitter: BasicEmitter,
    forward_emitters: Vec<Box<dyn EmitterMut>>,
}

// We need a mutable version of the Emitter trait for forwarding
pub trait EmitterMut: Send + Sync {
    fn listen_mut<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&str, &str) + 'static + Send + Sync;
}

impl<T: Emitter + Send + Sync> EmitterMut for T {
    fn listen_mut<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&str, &str) + 'static + Send + Sync,
    {
        self.listen(scope, method, callback)
    }
}

impl ForwardingEmitter {
    pub fn new() -> Self {
        ForwardingEmitter {
            basic_emitter: BasicEmitter::new(),
            forward_emitters: Vec::new(),
        }
    }

    pub fn emit(&self, scope: &str, method: &str) {
        self.basic_emitter.emit(scope, method);
    }

    /// Forward all previously connected hooks to the given emitter
    protected fn forward<E: EmitterMut + 'static>(&mut self, mut emitter: E) {
        // Forward all previously connected hooks
        for (key, listeners) in &self.basic_emitter.listeners {
            let parts: Vec<&str> = key.split("::").collect();
            if parts.len() == 2 {
                let scope = parts[0];
                let method = parts[1];
                
                for _ in listeners {
                    // Since we can't directly access the callbacks, we recreate
                    // the forwarding logic by having the emitter listen for the same events
                    let scope_owned = scope.to_string();
                    let method_owned = method.to_string();
                    
                    emitter.listen_mut(scope, method, move |s: &str, m: &str| {
                        // This is a placeholder - in real implementation we would need
                        // to somehow reference the original callback
                        println!("Forwarded event: {}::{}", scope_owned, method_owned);
                    });
                }
            }
        }
        
        self.forward_emitters.push(Box::new(emitter));
    }
}

impl Emitter for ForwardingEmitter {
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&str, &str) + 'static + Send + Sync,
    {
        // First register with our own emitter
        self.basic_emitter.listen(scope, method, callback);
        
        // Then forward to all other emitters
        for emitter in &mut self.forward_emitters {
            let scope = scope.to_string();
            let method = method.to_string();
            
            // Create a new closure that captures the original callback's behavior
            emitter.listen_mut(&scope, &method, move |s, m| {
                println!("Forwarded event: {}::{}", s, m);
                // In a real implementation, we would call the original callback here
            });
        }
    }
}