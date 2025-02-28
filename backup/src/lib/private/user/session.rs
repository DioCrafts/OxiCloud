use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

/// Session management for users
///
/// Hooks available in scope \OC\User:
/// - pre_set_password(User, String, String)
/// - post_set_password(User, String, String)
/// - pre_delete(User)
/// - post_delete(User)
/// - pre_create_user(String, String)
/// - post_create_user(User)
/// - pre_login(String, String)
/// - post_login(User)
/// - logout()
pub struct Session {
    manager: Arc<UserManager>,
    session: Arc<SessionStore>,
    active_user: Mutex<Option<Arc<User>>>,
}

pub trait Emitter {
    fn listen<F>(&self, scope: &str, method: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static;

    fn remove_listener<F>(&self, scope: Option<&str>, method: Option<&str>, callback: Option<F>)
    where
        F: Fn() + Send + Sync + 'static;
}

pub trait IUserSession {
    fn get_manager(&self) -> Arc<UserManager>;
    fn set_user(&self, user: Option<Arc<User>>);
    fn get_user(&self) -> Option<Arc<User>>;
    fn login(&self, uid: &str, password: &str) -> bool;
    fn logout(&self);
    fn set_magic_in_cookie(&self, username: &str, token: &str);
    fn unset_magic_in_cookie(&self);
}

impl Session {
    pub fn new(manager: Arc<UserManager>, session: Arc<SessionStore>) -> Self {
        Self {
            manager,
            session,
            active_user: Mutex::new(None),
        }
    }
}

impl Emitter for Session {
    fn listen<F>(&self, scope: &str, method: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.manager.listen(scope, method, callback);
    }

    fn remove_listener<F>(&self, scope: Option<&str>, method: Option<&str>, callback: Option<F>)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.manager.remove_listener(scope, method, callback);
    }
}

impl IUserSession for Session {
    fn get_manager(&self) -> Arc<UserManager> {
        Arc::clone(&self.manager)
    }

    fn set_user(&self, user: Option<Arc<User>>) {
        if let Some(user) = &user {
            self.session.set("user_id", user.get_uid());
        } else {
            self.session.remove("user_id");
        }
        let mut active_user = self.active_user.lock().unwrap();
        *active_user = user;
    }

    fn get_user(&self) -> Option<Arc<User>> {
        {
            let active_user = self.active_user.lock().unwrap();
            if let Some(user) = &*active_user {
                return Some(Arc::clone(user));
            }
        }

        if let Some(uid) = self.session.get("user_id") {
            if let Some(user) = self.manager.get(&uid) {
                let mut active_user = self.active_user.lock().unwrap();
                *active_user = Some(Arc::clone(&user));
                return Some(user);
            }
        }
        
        None
    }

    fn login(&self, uid: &str, password: &str) -> bool {
        self.manager.emit("\\OC\\User", "pre_login", &[uid, password]);
        
        if let Some(user) = self.manager.check_password(uid, password) {
            if user.is_enabled() {
                self.set_user(Some(Arc::clone(&user)));
                self.manager.emit("\\OC\\User", "post_login", &[&user, password]);
                return true;
            }
        }
        
        false
    }

    fn logout(&self) {
        self.manager.emit("\\OC\\User", "logout", &[]);
        self.set_user(None);
        self.unset_magic_in_cookie();
    }

    fn set_magic_in_cookie(&self, username: &str, token: &str) {
        let secure_cookie = CONFIG.get_value("forcessl", false);
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs() + CONFIG.get_value("remember_login_cookie_lifetime", 60 * 60 * 24 * 15);
        
        set_cookie("oc_username", username, expires, &WEBROOT, "", secure_cookie, false);
        set_cookie("oc_token", token, expires, &WEBROOT, "", secure_cookie, true);
        set_cookie("oc_remember_login", "true", expires, &WEBROOT, "", secure_cookie, false);
    }

    fn unset_magic_in_cookie(&self) {
        COOKIES.remove("oc_username");
        COOKIES.remove("oc_token");
        COOKIES.remove("oc_remember_login");
        
        let past_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs() - 3600;
            
        set_cookie("oc_username", "", past_time, &WEBROOT, "", false, false);
        set_cookie("oc_token", "", past_time, &WEBROOT, "", false, false);
        set_cookie("oc_remember_login", "", past_time, &WEBROOT, "", false, false);
        
        // old cookies might be stored under /webroot/ instead of /webroot
        // and Firefox doesn't like it!
        set_cookie("oc_username", "", past_time, &format!("{}/", WEBROOT), "", false, false);
        set_cookie("oc_token", "", past_time, &format!("{}/", WEBROOT), "", false, false);
        set_cookie("oc_remember_login", "", past_time, &format!("{}/", WEBROOT), "", false, false);
    }
}

// These structures would be defined elsewhere but are sketched here for completeness

pub struct User {
    uid: String,
    enabled: bool,
}

impl User {
    pub fn get_uid(&self) -> String {
        self.uid.clone()
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub struct UserManager {
    // Fields would be defined here
}

impl UserManager {
    pub fn get(&self, uid: &str) -> Option<Arc<User>> {
        // Implementation would be here
        None
    }
    
    pub fn check_password(&self, uid: &str, password: &str) -> Option<Arc<User>> {
        // Implementation would be here
        None
    }
    
    pub fn emit(&self, scope: &str, method: &str, args: &[&dyn std::any::Any]) {
        // Implementation would be here
    }
    
    pub fn listen<F>(&self, scope: &str, method: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // Implementation would be here
    }
    
    pub fn remove_listener<F>(&self, scope: Option<&str>, method: Option<&str>, callback: Option<F>)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // Implementation would be here
    }
}

pub struct SessionStore {
    // Fields would be defined here
}

impl SessionStore {
    pub fn get(&self, key: &str) -> Option<String> {
        // Implementation would be here
        None
    }
    
    pub fn set(&self, key: &str, value: String) {
        // Implementation would be here
    }
    
    pub fn remove(&self, key: &str) {
        // Implementation would be here
    }
}

// Global structures/statics needed
struct Config {
    // Fields would be defined here
}

impl Config {
    pub fn get_value<T>(&self, key: &str, default: T) -> T {
        // Implementation would be here
        default
    }
}

static CONFIG: Config = Config { /* initialization */ };
static WEBROOT: &str = "";
static mut COOKIES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

fn set_cookie(name: &str, value: &str, expires: u64, path: &str, domain: &str, secure: bool, http_only: bool) {
    // Implementation would be here
}