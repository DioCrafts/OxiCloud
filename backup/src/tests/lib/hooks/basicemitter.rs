// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

/// Arguments for event handlers
pub type EventArgs = HashMap<String, Box<dyn Any>>;

/// Trait that event callback functions must implement
pub trait EventCallback: Fn(&[Box<dyn Any>]) + 'static {}
impl<T: Fn(&[Box<dyn Any>]) + 'static> EventCallback for T {}

/// A custom exception to check if an event is emitted
#[derive(Debug)]
pub struct EmittedException;

impl fmt::Display for EmittedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Emitted Exception")
    }
}

impl std::error::Error for EmittedException {}

/// Basic implementation of the Emitter trait
pub trait Emitter {
    fn emit(&self, scope: &str, method: &str, arguments: Vec<Box<dyn Any>>);
    fn listen(&mut self, scope: &str, method: &str, callback: Box<dyn EventCallback>);
    fn remove_listener(&mut self, scope: Option<&str>, method: Option<&str>, callback: Option<&Box<dyn EventCallback>>);
}

/// Basic implementation of the emitter
pub struct BasicEmitter {
    listeners: HashMap<String, HashMap<String, HashSet<Rc<Box<dyn EventCallback>>>>>,
    callback_ids: RefCell<HashMap<TypeId, HashSet<String>>>,
}

impl BasicEmitter {
    pub fn new() -> Self {
        BasicEmitter {
            listeners: HashMap::new(),
            callback_ids: RefCell::new(HashMap::new()),
        }
    }
}

impl Emitter for BasicEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: Vec<Box<dyn Any>>) {
        if let Some(methods) = self.listeners.get(scope) {
            if let Some(callbacks) = methods.get(method) {
                for callback in callbacks {
                    callback(&arguments);
                }
            }
        }
    }

    fn listen(&mut self, scope: &str, method: &str, callback: Box<dyn EventCallback>) {
        let callback = Rc::new(callback);
        let scope_methods = self.listeners.entry(scope.to_string()).or_insert_with(HashMap::new);
        let callbacks = scope_methods.entry(method.to_string()).or_insert_with(HashSet::new);
        
        let callback_id = format!("{:?}:{}", Rc::as_ptr(&callback), method);
        let mut callback_ids = self.callback_ids.borrow_mut();
        let type_id = (*callback).type_id();
        let ids = callback_ids.entry(type_id).or_insert_with(HashSet::new);
        
        if !ids.contains(&callback_id) {
            ids.insert(callback_id);
            callbacks.insert(callback);
        }
    }

    fn remove_listener(&mut self, scope: Option<&str>, method: Option<&str>, callback: Option<&Box<dyn EventCallback>>) {
        // If either scope is wildcarded, we need to iterate all scopes
        match scope {
            Some(scope_str) => {
                if let Some(methods) = self.listeners.get_mut(scope_str) {
                    self.remove_from_scope(methods, method, callback);
                }
            }
            None => {
                // Wildcard scope
                for (_, methods) in self.listeners.iter_mut() {
                    self.remove_from_scope(methods, method, callback);
                }
            }
        }
    }
}

impl BasicEmitter {
    fn remove_from_scope(
        &self,
        methods: &mut HashMap<String, HashSet<Rc<Box<dyn EventCallback>>>>,
        method: Option<&str>,
        callback: Option<&Box<dyn EventCallback>>,
    ) {
        match method {
            Some(method_str) => {
                if let Some(callbacks) = methods.get_mut(method_str) {
                    self.remove_callback(callbacks, callback);
                }
            }
            None => {
                // Wildcard method
                for (_, callbacks) in methods.iter_mut() {
                    self.remove_callback(callbacks, callback);
                }
            }
        }
    }

    fn remove_callback(
        &self,
        callbacks: &mut HashSet<Rc<Box<dyn EventCallback>>>,
        callback: Option<&Box<dyn EventCallback>>,
    ) {
        match callback {
            Some(_callback) => {
                // Remove specific callback (not implemented due to type comparison limitations)
                // In Rust we would need more complex identity tracking
            }
            None => {
                // Remove all callbacks
                callbacks.clear();
            }
        }
    }
}

/// Class to make BasicEmitter::emit publicly available
pub struct DummyEmitter {
    emitter: BasicEmitter,
}

impl DummyEmitter {
    pub fn new() -> Self {
        DummyEmitter {
            emitter: BasicEmitter::new(),
        }
    }

    pub fn emit_event(&self, scope: &str, method: &str, arguments: Vec<Box<dyn Any>>) {
        self.emitter.emit(scope, method, arguments);
    }
}

impl Emitter for DummyEmitter {
    fn emit(&self, scope: &str, method: &str, arguments: Vec<Box<dyn Any>>) {
        self.emitter.emit(scope, method, arguments);
    }

    fn listen(&mut self, scope: &str, method: &str, callback: Box<dyn EventCallback>) {
        self.emitter.listen(scope, method, callback);
    }

    fn remove_listener(&mut self, scope: Option<&str>, method: Option<&str>, callback: Option<&Box<dyn EventCallback>>) {
        self.emitter.remove_listener(scope, method, callback);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct BasicEmitterTest {
        emitter: DummyEmitter,
    }

    impl BasicEmitterTest {
        fn new() -> Self {
            BasicEmitterTest {
                emitter: DummyEmitter::new(),
            }
        }

        fn set_up(&mut self) {
            self.emitter = DummyEmitter::new();
        }

        fn non_static_call_back(&self) {
            panic!("{:?}", EmittedException);
        }

        fn static_call_back() {
            panic!("{:?}", EmittedException);
        }

        fn test_anonymous_function(&mut self) {
            self.emitter.listen("Test", "test", Box::new(|_| {
                panic!("{:?}", EmittedException);
            }));
            self.emitter.emit_event("Test", "test", vec![]);
        }

        fn test_static_callback(&mut self) {
            self.emitter.listen("Test", "test", Box::new(|_| {
                Self::static_call_back();
            }));
            self.emitter.emit_event("Test", "test", vec![]);
        }

        fn test_non_static_callback(&mut self) {
            let this = self.clone();
            self.emitter.listen("Test", "test", Box::new(move |_| {
                this.non_static_call_back();
            }));
            self.emitter.emit_event("Test", "test", vec![]);
        }

        fn test_only_call_once(&mut self) {
            let count = Rc::new(Cell::new(0));
            let count_clone = count.clone();
            let listener = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone.set(count_clone.get() + 1);
            });
            
            self.emitter.listen("Test", "test", listener.clone());
            self.emitter.listen("Test", "test", listener);
            self.emitter.emit_event("Test", "test", vec![]);
            
            assert_eq!(1, count.get(), "Listener called an invalid number of times ({}) expected 1", count.get());
        }

        fn test_different_methods(&mut self) {
            let count = Rc::new(Cell::new(0));
            let count_clone1 = count.clone();
            let count_clone2 = count.clone();
            
            let listener1 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone1.set(count_clone1.get() + 1);
            });
            
            let listener2 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone2.set(count_clone2.get() + 1);
            });
            
            self.emitter.listen("Test", "test", listener1);
            self.emitter.listen("Test", "foo", listener2);
            self.emitter.emit_event("Test", "test", vec![]);
            self.emitter.emit_event("Test", "foo", vec![]);
            
            assert_eq!(2, count.get(), "Listener called an invalid number of times ({}) expected 2", count.get());
        }

        fn test_different_scopes(&mut self) {
            let count = Rc::new(Cell::new(0));
            let count_clone1 = count.clone();
            let count_clone2 = count.clone();
            
            let listener1 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone1.set(count_clone1.get() + 1);
            });
            
            let listener2 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone2.set(count_clone2.get() + 1);
            });
            
            self.emitter.listen("Test", "test", listener1);
            self.emitter.listen("Bar", "test", listener2);
            self.emitter.emit_event("Test", "test", vec![]);
            self.emitter.emit_event("Bar", "test", vec![]);
            
            assert_eq!(2, count.get(), "Listener called an invalid number of times ({}) expected 2", count.get());
        }

        fn test_different_callbacks(&mut self) {
            let count = Rc::new(Cell::new(0));
            let count_clone1 = count.clone();
            let count_clone2 = count.clone();
            
            let listener1 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone1.set(count_clone1.get() + 1);
            });
            
            let listener2 = Box::new(move |_: &[Box<dyn Any>]| {
                count_clone2.set(count_clone2.get() + 1);
            });
            
            self.emitter.listen("Test", "test", listener1);
            self.emitter.listen("Test", "test", listener2);
            self.emitter.emit_event("Test", "test", vec![]);
            
            assert_eq!(2, count.get(), "Listener called an invalid number of times ({}) expected 2", count.get());
        }

        fn test_arguments(&mut self) {
            self.emitter.listen("Test", "test", Box::new(|args: &[Box<dyn Any>]| {
                if args.len() == 2 {
                    let foo = args[0].downcast_ref::<String>().unwrap();
                    let bar = args[1].downcast_ref::<String>().unwrap();
                    if foo == "foo" && bar == "bar" {
                        panic!("{:?}", EmittedException);
                    }
                }
            }));
            
            let args: Vec<Box<dyn Any>> = vec![
                Box::new("foo".to_string()),
                Box::new("bar".to_string()),
            ];
            
            self.emitter.emit_event("Test", "test", args);
        }

        fn test_named_arguments(&mut self) {
            // In Rust, we would use a HashMap for named arguments
            // However, the test implementation doesn't use named lookup
            self.test_arguments();
        }

        fn test_remove_all_specified(&mut self) {
            let listener = Box::new(|_: &[Box<dyn Any>]| {
                panic!("{:?}", EmittedException);
            });
            
            self.emitter.listen("Test", "test", listener.clone());
            self.emitter.remove_listener(Some("Test"), Some("test"), Some(&listener));
            self.emitter.emit_event("Test", "test", vec![]);
            
            // If we reach here, no exception was thrown, so test passes
        }

        fn test_remove_wildcard_listener(&mut self) {
            let listener1 = Box::new(|_: &[Box<dyn Any>]| {
                panic!("{:?}", EmittedException);
            });
            
            let listener2 = Box::new(|_: &[Box<dyn Any>]| {
                panic!("{:?}", EmittedException);
            });
            
            self.emitter.listen("Test", "test", listener1);
            self.emitter.listen("Test", "test", listener2);
            self.emitter.remove_listener(Some("Test"), Some("test"), None);
            self.emitter.emit_event("Test", "test", vec![]);
            
            // If we reach here, no exception was thrown, so test passes
        }
    }

    impl Clone for BasicEmitterTest {
        fn clone(&self) -> Self {
            BasicEmitterTest {
                emitter: DummyEmitter::new(),
            }
        }
    }

    // The actual test runner would be implemented differently in Rust
    // This is just to show how the structure of the tests would be
    #[test]
    #[should_panic(expected = "EmittedException")]
    fn test_anonymous_function() {
        let mut test = BasicEmitterTest::new();
        test.set_up();
        test.test_anonymous_function();
    }

    #[test]
    #[should_panic(expected = "EmittedException")]
    fn test_static_callback() {
        let mut test = BasicEmitterTest::new();
        test.set_up();
        test.test_static_callback();
    }

    #[test]
    fn test_only_call_once() {
        let mut test = BasicEmitterTest::new();
        test.set_up();
        test.test_only_call_once();
    }

    // Additional tests would be implemented similarly
}