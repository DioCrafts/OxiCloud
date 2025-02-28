use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A trait for objects that can emit events.
pub trait Emitter {
    /// Register a listener for an event.
    ///
    /// * `scope` - The scope of the event.
    /// * `method` - The method/event name.
    /// * `callback` - The callback to be called when the event is emitted.
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static;

    /// Remove a listener for an event.
    ///
    /// * `scope` - The scope of the event (optional).
    /// * `method` - The method/event name (optional).
    /// * `callback` - The callback to remove (optional).
    fn remove_listener<F>(&mut self, scope: Option<&str>, method: Option<&str>, callback: Option<Arc<F>>)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static;
}

/// Type alias for a callback function.
type Callback = Arc<dyn Fn(&[&dyn std::any::Any]) + Send + Sync + 'static>;

/// Basic implementation of the Emitter trait.
pub struct BasicEmitter {
    /// Mapping of event names to their registered listeners.
    listeners: HashMap<String, Vec<Callback>>,
}

impl BasicEmitter {
    /// Creates a new BasicEmitter.
    pub fn new() -> Self {
        BasicEmitter {
            listeners: HashMap::new(),
        }
    }

    /// Emits an event with the given scope, method and arguments.
    ///
    /// * `scope` - The scope of the event.
    /// * `method` - The method/event name.
    /// * `arguments` - The arguments to pass to the callbacks.
    pub(crate) fn emit(&self, scope: &str, method: &str, arguments: &[&dyn std::any::Any]) {
        let event_name = format!("{}::{}", scope, method);
        if let Some(callbacks) = self.listeners.get(&event_name) {
            for callback in callbacks {
                callback(arguments);
            }
        }
    }
}

impl Emitter for BasicEmitter {
    fn listen<F>(&mut self, scope: &str, method: &str, callback: F)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        let event_name = format!("{}::{}", scope, method);
        let entry = self.listeners.entry(event_name).or_insert_with(Vec::new);
        let callback = Arc::new(callback) as Callback;
        
        // Only add if not already present
        if !entry.iter().any(|c| Arc::ptr_eq(c, &callback)) {
            entry.push(callback);
        }
    }

    fn remove_listener<F>(&mut self, scope: Option<&str>, method: Option<&str>, callback: Option<Arc<F>>)
    where
        F: Fn(&[&dyn std::any::Any]) + Send + Sync + 'static,
    {
        let mut names = Vec::new();
        let all_names: Vec<String> = self.listeners.keys().cloned().collect();

        match (scope, method) {
            (Some(s), Some(m)) => {
                let name = format!("{}::{}", s, m);
                if self.listeners.contains_key(&name) {
                    names.push(name);
                }
            },
            (Some(s), None) => {
                for name in &all_names {
                    let parts: Vec<&str> = name.split("::").collect();
                    if parts.len() == 2 && parts[0] == s {
                        names.push(name.clone());
                    }
                }
            },
            (None, Some(m)) => {
                for name in &all_names {
                    let parts: Vec<&str> = name.split("::").collect();
                    if parts.len() == 2 && parts[1] == m {
                        names.push(name.clone());
                    }
                }
            },
            (None, None) => {
                names = all_names;
            }
        }

        for name in names {
            if let Some(cb) = &callback {
                if let Some(listeners) = self.listeners.get_mut(&name) {
                    // Filter out the specific callback if it exists
                    let cb_any = Arc::new(cb.clone()) as Arc<dyn Fn(&[&dyn std::any::Any]) + Send + Sync + 'static>;
                    listeners.retain(|c| !Arc::ptr_eq(c, &cb_any));
                }
            } else {
                // Clear all listeners for this event
                if let Some(listeners) = self.listeners.get_mut(&name) {
                    listeners.clear();
                }
            }
        }
    }
}

impl Default for BasicEmitter {
    fn default() -> Self {
        Self::new()
    }
}