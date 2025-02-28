// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod oc {
    pub mod hooks {
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        pub struct LegacyEmitter {
            pub listeners: HashMap<String, Vec<Box<dyn Fn(&HashMap<String, String>)>>>,
        }

        impl LegacyEmitter {
            pub fn new() -> Self {
                LegacyEmitter {
                    listeners: HashMap::new(),
                }
            }

            pub fn emit(&self, scope: &str, method: &str, arguments: HashMap<String, String>) {
                let key = format!("{}.{}", scope, method);
                if let Some(listeners) = self.listeners.get(&key) {
                    for listener in listeners {
                        listener(&arguments);
                    }
                }
            }
        }
    }

    pub mod hook {
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        type HookCallback = fn(&HashMap<String, String>);

        thread_local! {
            static HOOKS: RefCell<HashMap<String, Vec<HookCallback>>> = RefCell::new(HashMap::new());
        }

        pub fn connect(scope: &str, method: &str, _class: &str, callback: HookCallback) {
            let key = format!("{}.{}", scope, method);
            HOOKS.with(|hooks| {
                let mut hooks = hooks.borrow_mut();
                hooks.entry(key).or_insert_with(Vec::new).push(callback);
            });
        }

        pub fn clear(scope: &str, method: &str) {
            let key = format!("{}.{}", scope, method);
            HOOKS.with(|hooks| {
                let mut hooks = hooks.borrow_mut();
                hooks.remove(&key);
            });
        }

        pub fn emit(scope: &str, method: &str, arguments: HashMap<String, String>) {
            let key = format!("{}.{}", scope, method);
            HOOKS.with(|hooks| {
                let hooks = hooks.borrow();
                if let Some(callbacks) = hooks.get(&key) {
                    for callback in callbacks {
                        callback(&arguments);
                    }
                }
            });
        }
    }
}

/// Basic implementation of an event emitter
pub struct BasicEmitter {
    // This would be the actual implementation
}

/**
 * Class DummyLegacyEmitter
 *
 * class to make LegacyEmitter::emit publicly available
 */
pub struct DummyLegacyEmitter {
    listeners: HashMap<String, Vec<Box<dyn Fn(&HashMap<String, String>)>>>,
}

impl DummyLegacyEmitter {
    pub fn new() -> Self {
        DummyLegacyEmitter {
            listeners: HashMap::new(),
        }
    }

    pub fn emit_event(&self, scope: &str, method: &str, arguments: HashMap<String, String>) {
        self.emit(scope, method, arguments);
    }

    fn emit(&self, scope: &str, method: &str, arguments: HashMap<String, String>) {
        oc::hook::emit(scope, method, arguments);
    }
}

/// LegacyEmitter test implementation
pub struct LegacyEmitter {
    emitter: DummyLegacyEmitter,
}

thread_local! {
    static EMITTED: RefCell<bool> = RefCell::new(false);
}

impl LegacyEmitter {
    pub fn new() -> Self {
        LegacyEmitter {
            emitter: DummyLegacyEmitter::new(),
        }
    }

    pub fn set_up(&mut self) {
        self.emitter = DummyLegacyEmitter::new();
        EMITTED.with(|e| *e.borrow_mut() = false);
        oc::hook::clear("Test", "test");
    }

    pub fn static_legacy_call_back(_arguments: &HashMap<String, String>) {
        EMITTED.with(|e| *e.borrow_mut() = true);
    }

    pub fn static_legacy_arguments_call_back(arguments: &HashMap<String, String>) {
        if arguments.get("foo") == Some(&"foo".to_string()) && arguments.get("bar") == Some(&"bar".to_string()) {
            EMITTED.with(|e| *e.borrow_mut() = true);
        }
    }

    pub fn test_legacy_hook(&self) {
        oc::hook::connect("Test", "test", "Test\\Hooks\\LegacyEmitter", Self::static_legacy_call_back);
        self.emitter.emit_event("Test", "test", HashMap::new());
        EMITTED.with(|e| assert_eq!(*e.borrow(), true));
    }

    pub fn test_legacy_arguments(&self) {
        oc::hook::connect("Test", "test", "Test\\Hooks\\LegacyEmitter", Self::static_legacy_arguments_call_back);
        let mut args = HashMap::new();
        args.insert("foo".to_string(), "foo".to_string());
        args.insert("bar".to_string(), "bar".to_string());
        self.emitter.emit_event("Test", "test", args);
        EMITTED.with(|e| assert_eq!(*e.borrow(), true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_hook() {
        let mut emitter = LegacyEmitter::new();
        emitter.set_up();
        emitter.test_legacy_hook();
    }

    #[test]
    fn test_legacy_arguments() {
        let mut emitter = LegacyEmitter::new();
        emitter.set_up();
        emitter.test_legacy_arguments();
    }
}