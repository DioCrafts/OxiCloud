// Copyright (c) Robin Appelman
// Copyright (c) ownCloud, Inc
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use crate::user::backend::{TestUserBackend, UserBackend};
use std::sync::{Arc, Mutex};

pub struct TestUserDatabase {
    backend: Arc<dyn UserBackend>,
    users: Mutex<Vec<String>>,
}

impl TestUserDatabase {
    pub fn new() -> Self {
        TestUserDatabase {
            backend: Arc::new(OcUserDatabase::new()),
            users: Mutex::new(Vec::new()),
        }
    }

    pub fn get_user(&self) -> String {
        let user = TestUserBackend::get_user(self);
        self.users.lock().unwrap().push(user.clone());
        user
    }

    pub fn set_up(&mut self) {
        self.backend = Arc::new(OcUserDatabase::new());
    }

    pub fn tear_down(&self) {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            self.backend.delete_user(user);
        }
    }
}

impl TestUserBackend for TestUserDatabase {
    fn get_backend(&self) -> Arc<dyn UserBackend> {
        self.backend.clone()
    }
}

pub struct OcUserDatabase {
    // Internal implementation details would go here
}

impl OcUserDatabase {
    pub fn new() -> Self {
        OcUserDatabase {}
    }
}

impl UserBackend for OcUserDatabase {
    fn delete_user(&self, user: &str) -> bool {
        // Implementation would go here
        true
    }
    
    // Other required methods from UserBackend trait would be implemented here
}