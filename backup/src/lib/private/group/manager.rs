// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Traits and structs for the Group Manager
pub mod group {
    use super::*;
    use crate::hooks::PublicEmitter;
    use crate::user::User;
    use crate::user::UserManager;

    // Constants for backend actions
    pub const GROUP_BACKEND_CREATE_GROUP: u32 = 0x00000001;

    /// Group backend trait
    pub trait GroupBackend {
        fn group_exists(&self, gid: &str) -> bool;
        fn get_groups(&self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<String>;
        fn get_user_groups(&self, uid: &str) -> Vec<String>;
        fn implements_actions(&self, actions: u32) -> bool;
        fn create_group(&self, gid: &str) -> bool;
    }

    pub type GroupId = String;

    /// Group struct representing a single group
    pub struct Group {
        gid: GroupId,
        backends: Vec<Rc<dyn GroupBackend>>,
        user_manager: Rc<UserManager>,
        group_manager: Weak<RefCell<Manager>>,
    }

    impl Group {
        pub fn new(
            gid: GroupId,
            backends: Vec<Rc<dyn GroupBackend>>,
            user_manager: Rc<UserManager>,
            group_manager: Weak<RefCell<Manager>>,
        ) -> Self {
            Self {
                gid,
                backends,
                user_manager,
                group_manager,
            }
        }

        pub fn get_gid(&self) -> &str {
            &self.gid
        }
    }

    /**
     * Class Manager
     *
     * Hooks available in scope \OC\Group:
     * - pre_add_user(Group, User)
     * - post_add_user(Group, User)
     * - pre_remove_user(Group, User)
     * - post_remove_user(Group, User)
     * - pre_delete(Group)
     * - post_delete(Group)
     * - pre_create(string groupId)
     * - post_create(Group)
     */
    pub struct Manager {
        backends: Vec<Rc<dyn GroupBackend>>,
        user_manager: Rc<UserManager>,
        cached_groups: HashMap<GroupId, Rc<Group>>,
        emitter: PublicEmitter,
    }

    impl Manager {
        /// Create a new group manager
        pub fn new(user_manager: Rc<UserManager>) -> Self {
            let mut manager = Self {
                backends: Vec::new(),
                user_manager,
                cached_groups: HashMap::new(),
                emitter: PublicEmitter::new(),
            };

            // Setup listeners
            manager.emitter.listen(r"\OC\Group", "post_delete", Box::new(move |args| {
                if let Some(group) = args.first().and_then(|g| g.downcast_ref::<Rc<Group>>()) {
                    // We can't modify self here due to borrow checker, so we'll handle this differently
                    // The actual implementation would use interior mutability or a different design
                }
            }));

            manager
        }

        /// Add a group backend
        pub fn add_backend(&mut self, backend: Rc<dyn GroupBackend>) {
            self.backends.push(backend);
        }

        /// Clear all backends
        pub fn clear_backends(&mut self) {
            self.backends.clear();
            self.cached_groups.clear();
        }

        /// Get a group by id
        pub fn get(&mut self, gid: &str) -> Option<Rc<Group>> {
            if let Some(group) = self.cached_groups.get(gid) {
                return Some(Rc::clone(group));
            }

            for backend in &self.backends {
                if backend.group_exists(gid) {
                    return Some(self.get_group_object(gid));
                }
            }
            
            None
        }

        /// Create a group object and cache it
        fn get_group_object(&mut self, gid: &str) -> Rc<Group> {
            let mut backends = Vec::new();
            
            for backend in &self.backends {
                if backend.group_exists(gid) {
                    backends.push(Rc::clone(backend));
                }
            }

            let group = Rc::new(Group::new(
                gid.to_string(),
                backends,
                Rc::clone(&self.user_manager),
                Rc::downgrade(&Rc::new(RefCell::new(self.clone()))), // This isn't ideal, but matches PHP semantics
            ));
            
            self.cached_groups.insert(gid.to_string(), Rc::clone(&group));
            group
        }

        /// Check if a group exists
        pub fn group_exists(&mut self, gid: &str) -> bool {
            self.get(gid).is_some()
        }

        /// Create a new group
        pub fn create_group(&mut self, gid: &str) -> Option<Rc<Group>> {
            if gid.is_empty() {
                return None;
            } else if self.group_exists(gid) {
                return self.get(gid);
            } else {
                self.emitter.emit(r"\OC\Group", "pre_create", vec![Box::new(gid.to_string())]);
                
                for backend in &self.backends {
                    if backend.implements_actions(GROUP_BACKEND_CREATE_GROUP) {
                        if backend.create_group(gid) {
                            let group = self.get_group_object(gid);
                            self.emitter.emit(r"\OC\Group", "post_create", vec![Box::new(Rc::clone(&group))]);
                            return Some(group);
                        }
                    }
                }
                
                None
            }
        }

        /// Search for groups
        pub fn search(&mut self, search: &str, limit: Option<usize>, offset: Option<usize>) -> Vec<Rc<Group>> {
            let mut groups: HashMap<String, Rc<Group>> = HashMap::new();
            let mut remaining_limit = limit;
            let mut remaining_offset = offset;
            
            for backend in &self.backends {
                let group_ids = backend.get_groups(search, remaining_limit, remaining_offset);
                
                if let Some(limit) = remaining_limit.as_mut() {
                    *limit = limit.saturating_sub(group_ids.len());
                }
                
                if let Some(offset) = remaining_offset.as_mut() {
                    *offset = offset.saturating_sub(group_ids.len());
                }
                
                for group_id in group_ids {
                    groups.insert(group_id.clone(), self.get_group_object(&group_id));
                }
                
                if let Some(limit) = remaining_limit {
                    if limit <= 0 {
                        break;
                    }
                }
            }
            
            groups.into_values().collect()
        }

        /// Get groups for a user
        pub fn get_user_groups(&mut self, user: &User) -> Vec<Rc<Group>> {
            let mut groups: HashMap<String, Rc<Group>> = HashMap::new();
            
            for backend in &self.backends {
                let group_ids = backend.get_user_groups(user.get_uid());
                
                for group_id in group_ids {
                    groups.insert(group_id.clone(), self.get_group_object(&group_id));
                }
            }
            
            groups.into_values().collect()
        }
        
        // Clone implementation for handling the closure in new()
        fn clone(&self) -> Self {
            Self {
                backends: self.backends.clone(),
                user_manager: Rc::clone(&self.user_manager),
                cached_groups: self.cached_groups.clone(),
                emitter: self.emitter.clone(),
            }
        }
    }
}