use std::collections::HashMap;
use std::sync::Arc;

// Constantes para las capacidades de los backends
pub const OC_GROUP_BACKEND_ADD_TO_GROUP: u32 = 0x00000001;
pub const OC_GROUP_BACKEND_REMOVE_FROM_GOUP: u32 = 0x00000002;
pub const OC_GROUP_BACKEND_GET_DISPLAYNAME: u32 = 0x00000004;
pub const OC_GROUP_BACKEND_DELETE_GROUP: u32 = 0x00000008;

/// Backend for group operations
pub trait GroupBackend {
    fn users_in_group(&self, gid: &str, search: Option<&str>, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
    fn in_group(&self, uid: &str, gid: &str) -> bool;
    fn implements_actions(&self, actions: u32) -> bool;
    fn add_to_group(&self, uid: &str, gid: &str) -> bool;
    fn remove_from_group(&self, uid: &str, gid: &str) -> bool;
    fn delete_group(&self, gid: &str) -> bool;
    fn display_names_in_group(&self, gid: &str, search: Option<&str>, limit: Option<usize>, offset: Option<usize>) -> HashMap<String, String> {
        HashMap::new() // Default implementation returns empty map
    }
}

/// User representation
pub struct User {
    uid: String,
    // Other fields would be here in a real implementation
}

impl User {
    pub fn get_uid(&self) -> &str {
        &self.uid
    }
}

/// User manager
pub trait UserManager {
    fn get(&self, uid: &str) -> Option<Arc<User>>;
}

/// Event emitter for hooks
pub trait PublicEmitter {
    fn emit(&self, app: &str, event: &str, args: Vec<Box<dyn std::any::Any>>);
}

/// Group class representation
pub struct Group {
    gid: String,
    users: Option<HashMap<String, Arc<User>>>,
    backends: Vec<Arc<dyn GroupBackend>>,
    user_manager: Arc<dyn UserManager>,
    emitter: Option<Arc<dyn PublicEmitter>>,
}

impl Group {
    /// Create a new group
    ///
    /// # Arguments
    /// * `gid` - The group ID
    /// * `backends` - List of backends to use for this group
    /// * `user_manager` - User manager for user lookups
    /// * `emitter` - Optional event emitter for hooks
    pub fn new(
        gid: String,
        backends: Vec<Arc<dyn GroupBackend>>,
        user_manager: Arc<dyn UserManager>,
        emitter: Option<Arc<dyn PublicEmitter>>,
    ) -> Self {
        Self {
            gid,
            users: None,
            backends,
            user_manager,
            emitter,
        }
    }

    /// Get the group ID
    pub fn get_gid(&self) -> &str {
        &self.gid
    }

    /// Get all users in the group
    pub fn get_users(&mut self) -> HashMap<String, Arc<User>> {
        if let Some(ref users) = self.users {
            return users.clone();
        }

        let mut user_ids = Vec::new();
        for backend in &self.backends {
            let backend_users = backend.users_in_group(&self.gid, None, None, None);
            let diff: Vec<_> = backend_users.into_iter()
                .filter(|id| !user_ids.contains(id))
                .collect();
            
            if !diff.is_empty() {
                user_ids.extend(diff);
            }
        }

        let users = self.get_verified_users(&user_ids);
        self.users = Some(users.clone());
        users
    }

    /// Check if a user is in the group
    pub fn in_group(&self, user: &User) -> bool {
        for backend in &self.backends {
            if backend.in_group(user.get_uid(), &self.gid) {
                return true;
            }
        }
        false
    }

    /// Add a user to the group
    pub fn add_user(&mut self, user: Arc<User>) {
        if self.in_group(&user) {
            return;
        }

        if let Some(ref emitter) = self.emitter {
            emitter.emit(
                r"\OC\Group",
                "preAddUser",
                vec![Box::new(self), Box::new(user.clone())],
            );
        }

        for backend in &self.backends {
            if backend.implements_actions(OC_GROUP_BACKEND_ADD_TO_GROUP) {
                backend.add_to_group(user.get_uid(), &self.gid);
                
                if let Some(ref mut users) = self.users {
                    users.insert(user.get_uid().to_string(), user.clone());
                }
                
                if let Some(ref emitter) = self.emitter {
                    emitter.emit(
                        r"\OC\Group",
                        "postAddUser",
                        vec![Box::new(self), Box::new(user.clone())],
                    );
                }
                
                return;
            }
        }
    }

    /// Remove a user from the group
    pub fn remove_user(&mut self, user: Arc<User>) {
        let mut result = false;
        
        if let Some(ref emitter) = self.emitter {
            emitter.emit(
                r"\OC\Group",
                "preRemoveUser",
                vec![Box::new(self), Box::new(user.clone())],
            );
        }
        
        for backend in &self.backends {
            if backend.implements_actions(OC_GROUP_BACKEND_REMOVE_FROM_GOUP) 
                && backend.in_group(user.get_uid(), &self.gid) {
                backend.remove_from_group(user.get_uid(), &self.gid);
                result = true;
            }
        }
        
        if result {
            if let Some(ref emitter) = self.emitter {
                emitter.emit(
                    r"\OC\Group",
                    "postRemoveUser",
                    vec![Box::new(self), Box::new(user.clone())],
                );
            }
            
            if let Some(ref mut users) = self.users {
                users.remove(user.get_uid());
            }
        }
    }

    /// Search for users in the group by userid
    pub fn search_users(
        &self,
        search: Option<&str>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Vec<Arc<User>> {
        let mut users = HashMap::new();
        let mut remaining_limit = limit;
        let mut remaining_offset = offset;
        
        for backend in &self.backends {
            let user_ids = backend.users_in_group(&self.gid, search, remaining_limit, remaining_offset);
            
            if let Some(limit) = remaining_limit.as_mut() {
                *limit = limit.saturating_sub(user_ids.len());
            }
            
            if let Some(offset) = remaining_offset.as_mut() {
                *offset = offset.saturating_sub(user_ids.len());
            }
            
            let verified_users = self.get_verified_users(&user_ids);
            users.extend(verified_users);
            
            if let Some(limit) = remaining_limit {
                if limit <= 0 {
                    break;
                }
            }
        }
        
        users.into_values().collect()
    }

    /// Search for users in the group by displayname
    pub fn search_display_name(
        &self,
        search: Option<&str>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Vec<Arc<User>> {
        let mut users = HashMap::new();
        let mut remaining_limit = limit;
        let mut remaining_offset = offset;
        
        for backend in &self.backends {
            let user_ids = if backend.implements_actions(OC_GROUP_BACKEND_GET_DISPLAYNAME) {
                backend.display_names_in_group(&self.gid, search, remaining_limit, remaining_offset)
                    .into_keys()
                    .collect()
            } else {
                backend.users_in_group(&self.gid, search, remaining_limit, remaining_offset)
            };
            
            if let Some(limit) = remaining_limit.as_mut() {
                *limit = limit.saturating_sub(user_ids.len());
            }
            
            if let Some(offset) = remaining_offset.as_mut() {
                *offset = offset.saturating_sub(user_ids.len());
            }
            
            let verified_users = self.get_verified_users(&user_ids);
            users.extend(verified_users);
            
            if let Some(limit) = remaining_limit {
                if limit <= 0 {
                    break;
                }
            }
        }
        
        users.into_values().collect()
    }

    /// Delete the group
    pub fn delete(&self) -> bool {
        let mut result = false;
        
        if let Some(ref emitter) = self.emitter {
            emitter.emit(r"\OC\Group", "preDelete", vec![Box::new(self)]);
        }
        
        for backend in &self.backends {
            if backend.implements_actions(OC_GROUP_BACKEND_DELETE_GROUP) {
                result = true;
                backend.delete_group(&self.gid);
            }
        }
        
        if result {
            if let Some(ref emitter) = self.emitter {
                emitter.emit(r"\OC\Group", "postDelete", vec![Box::new(self)]);
            }
        }
        
        result
    }

    /// Returns all the Users from an array that really exists
    fn get_verified_users(&self, user_ids: &[String]) -> HashMap<String, Arc<User>> {
        let mut users = HashMap::new();
        
        for user_id in user_ids {
            if let Some(user) = self.user_manager.get(user_id) {
                users.insert(user_id.clone(), user);
            }
        }
        
        users
    }
}