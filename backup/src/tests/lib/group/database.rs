#[cfg(test)]
mod test_group_database {
    use crate::group::{Backend, Database};
    use crate::test_group_backend::TestGroupBackend;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    pub struct TestGroupDatabase {
        backend: Arc<Database>,
        groups: Mutex<Vec<String>>,
    }

    impl TestGroupDatabase {
        /// get a new unique group name
        /// test cases can override this in order to clean up created groups
        pub fn get_group_name(&self) -> String {
            let name = format!("test_{}", Uuid::new_v4());
            self.groups.lock().unwrap().push(name.clone());
            name
        }

        /// get a new unique user name
        /// test cases can override this in order to clean up created user
        pub fn get_user_name(&self) -> String {
            format!("test_{}", Uuid::new_v4())
        }
    }

    impl TestGroupBackend for TestGroupDatabase {
        fn setup(&mut self) {
            self.backend = Arc::new(Database::new());
        }

        fn teardown(&mut self) {
            let groups = self.groups.lock().unwrap().clone();
            for group in groups {
                let _ = self.backend.delete_group(&group);
            }
        }

        fn backend(&self) -> Arc<dyn Backend> {
            self.backend.clone()
        }
    }

    impl Default for TestGroupDatabase {
        fn default() -> Self {
            Self {
                backend: Arc::new(Database::new()),
                groups: Mutex::new(Vec::new()),
            }
        }
    }
}