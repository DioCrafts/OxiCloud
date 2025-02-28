use std::collections::HashMap;
use std::sync::{Arc, Weak};

/// Hook system for user management events
pub struct PublicEmitter {
    listeners: HashMap<String, Vec<Box<dyn Fn(&User) + Send + Sync>>>,
}

impl PublicEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn listen<F>(&mut self, scope: &str, event: &str, callback: F)
    where
        F: Fn(&User) + Send + Sync + 'static,
    {
        let key = format!("{}\t{}", scope, event);
        self.listeners.entry(key).or_insert_with(Vec::new).push(Box::new(callback));
    }

    pub fn emit(&self, scope: &str, event: &str, user: &User) {
        let key = format!("{}\t{}", scope, event);
        if let Some(callbacks) = self.listeners.get(&key) {
            for callback in callbacks {
                callback(user);
            }
        }
    }
}

/// User backend interface
pub trait UserBackend: Send + Sync {
    fn user_exists(&self, uid: &str) -> bool;
    fn check_password(&self, loginname: &str, password: &str) -> Option<String>;
    fn get_users(&self, pattern: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
    fn get_display_names(&self, pattern: &str, limit: Option<usize>, offset: Option<usize>) -> HashMap<String, String>;
    fn create_user(&self, uid: &str, password: &str) -> Result<(), String>;
    fn implements_actions(&self, actions: u32) -> bool;
}

// Constants for backend actions
pub const USER_BACKEND_CHECK_PASSWORD: u32 = 1;
pub const USER_BACKEND_CREATE_USER: u32 = 2;

/// User representation
pub struct User {
    uid: String,
    backend: Arc<dyn UserBackend>,
    manager: Weak<Manager>,
}

impl User {
    pub fn new(uid: String, backend: Arc<dyn UserBackend>, manager: Weak<Manager>) -> Self {
        Self { uid, backend, manager }
    }

    pub fn get_uid(&self) -> &str {
        &self.uid
    }

    pub fn get_display_name(&self) -> String {
        // Implementation would depend on actual requirements
        self.uid.clone()
    }
}

/**
 * Class Manager
 *
 * Hooks available in scope \OC\User:
 * - preSetPassword(\OC\User\User $user, string $password, string $recoverPassword)
 * - postSetPassword(\OC\User\User $user, string $password, string $recoverPassword)
 * - preDelete(\OC\User\User $user)
 * - postDelete(\OC\User\User $user)
 * - preCreateUser(string $uid, string $password)
 * - postCreateUser(\OC\User\User $user, string $password)
 */
pub struct Manager {
    emitter: PublicEmitter,
    backends: Vec<Arc<dyn UserBackend>>,
    cached_users: HashMap<String, Arc<User>>,
}

impl Manager {
    pub fn new() -> Arc<Self> {
        let manager = Arc::new(Self {
            emitter: PublicEmitter::new(),
            backends: Vec::new(),
            cached_users: HashMap::new(),
        });
        
        // In Rust we'd typically use interior mutability pattern (e.g., RefCell, Mutex)
        // to handle the mutable closure capturing, but for simplicity in this example
        // we'll skip the self-referential listening setup
        
        manager
    }

    /// Register a user backend
    pub fn register_backend(&mut self, backend: Arc<dyn UserBackend>) {
        self.backends.push(backend);
    }

    /// Remove a user backend
    pub fn remove_backend(&mut self, backend: Arc<dyn UserBackend>) {
        self.cached_users.clear();
        self.backends.retain(|b| !Arc::ptr_eq(b, &backend));
    }

    /// Remove all user backends
    pub fn clear_backends(&mut self) {
        self.cached_users.clear();
        self.backends.clear();
    }

    /// Get a user by user id
    pub fn get(&mut self, uid: &str) -> Option<Arc<User>> {
        // Check the cache first to prevent having to loop over the backends
        if let Some(user) = self.cached_users.get(uid) {
            return Some(Arc::clone(user));
        }
        
        for backend in &self.backends {
            if backend.user_exists(uid) {
                let user = self.get_user_object(uid, Arc::clone(backend));
                return Some(user);
            }
        }
        
        None
    }

    /// Get or construct the user object
    fn get_user_object(&mut self, uid: &str, backend: Arc<dyn UserBackend>) -> Arc<User> {
        if let Some(user) = self.cached_users.get(uid) {
            return Arc::clone(user);
        }
        
        let arc_self = Arc::downgrade(&Arc::new(self));
        let user = Arc::new(User::new(uid.to_string(), backend, arc_self));
        self.cached_users.insert(uid.to_string(), Arc::clone(&user));
        user
    }

    /// Check if a user exists
    pub fn user_exists(&mut self, uid: &str) -> bool {
        self.get(uid).is_some()
    }

    /// Remove deleted user from cache
    pub fn delete(&mut self, uid: &str) -> bool {
        self.cached_users.remove(uid).is_some()
    }

    /// Check if the password is valid for the user
    pub fn check_password(&mut self, loginname: &str, password: &str) -> Option<Arc<User>> {
        for backend in &self.backends {
            if backend.implements_actions(USER_BACKEND_CHECK_PASSWORD) {
                if let Some(uid) = backend.check_password(loginname, password) {
                    return Some(self.get_user_object(&uid, Arc::clone(backend)));
                }
            }
        }
        None
    }

    /// Search by user id
    pub fn search(&mut self, pattern: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<Arc<User>> {
        let mut users = Vec::new();
        let mut remaining_limit = limit;
        let mut remaining_offset = offset;
        
        for backend in &self.backends {
            let backend_users = backend.get_users(pattern, remaining_limit, remaining_offset);
            
            for uid in backend_users {
                users.push(self.get_user_object(&uid, Arc::clone(backend)));
                
                if let Some(ref mut limit) = remaining_limit {
                    *limit -= 1;
                    if *limit == 0 {
                        break;
                    }
                }
                
                if let Some(ref mut offset) = remaining_offset {
                    if *offset > 0 {
                        *offset -= 1;
                    }
                }
            }
        }

        users.sort_by(|a, b| a.get_uid().cmp(b.get_uid()));
        users
    }

    /// Search by displayName
    pub fn search_display_name(&mut self, pattern: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<Arc<User>> {
        let mut users = Vec::new();
        let mut remaining_limit = limit;
        let mut remaining_offset = offset;
        
        for backend in &self.backends {
            let backend_users = backend.get_display_names(pattern, remaining_limit, remaining_offset);
            
            for (uid, _) in backend_users {
                users.push(self.get_user_object(&uid, Arc::clone(backend)));
                
                if let Some(ref mut limit) = remaining_limit {
                    *limit -= 1;
                    if *limit == 0 {
                        break;
                    }
                }
                
                if let Some(ref mut offset) = remaining_offset {
                    if *offset > 0 {
                        *offset -= 1;
                    }
                }
            }
        }

        users.sort_by(|a, b| a.get_display_name().cmp(&b.get_display_name()));
        users
    }

    /// Create a new user
    pub fn create_user(&mut self, uid: &str, password: &str) -> Result<Arc<User>, String> {
        // Check the name for bad characters
        // Allowed are: "a-z", "A-Z", "0-9" and "_.@-"
        let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9_\.@\-]+$").unwrap();
        if !valid_chars.is_match(uid) {
            return Err("Only the following characters are allowed in a username: \"a-z\", \"A-Z\", \"0-9\", and \"_.@-\"".to_string());
        }
        
        // No empty username
        if uid.trim().is_empty() {
            return Err("A valid username must be provided".to_string());
        }
        
        // No empty password
        if password.trim().is_empty() {
            return Err("A valid password must be provided".to_string());
        }

        // Check if user already exists
        if self.user_exists(uid) {
            return Err("The username is already being used".to_string());
        }

        // Emit preCreateUser event
        // In a real implementation, you'd use the emitter here
        
        for backend in &self.backends {
            if backend.implements_actions(USER_BACKEND_CREATE_USER) {
                match backend.create_user(uid, password) {
                    Ok(()) => {
                        let user = self.get_user_object(uid, Arc::clone(backend));
                        // Emit postCreateUser event
                        // In a real implementation, you'd use the emitter here
                        return Ok(user);
                    },
                    Err(e) => return Err(e),
                }
            }
        }
        
        Err("No backend implemented create user".to_string())
    }
}