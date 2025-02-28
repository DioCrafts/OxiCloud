// Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

#[cfg(test)]
mod mount_tests {
    use crate::files::mount::Mount;
    use crate::files::storage::{Storage, Temporary};
    use crate::files::storage::loader::Loader;
    use crate::files::storage::wrapper::Wrapper;
    use std::sync::Arc;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        pub Temporary {}
        impl Storage for Temporary {
            // Implementación simulada para Storage
        }
    }

    #[test]
    fn test_from_storage_object() {
        let storage = MockTemporary::new();
        let mount = Mount::new(Arc::new(storage), "/foo".to_string());
        assert!(mount.get_storage().is::<Temporary>());
    }

    #[test]
    fn test_from_storage_classname() {
        let mount = Mount::new_from_classname("Temporary".to_string(), "/foo".to_string());
        assert!(mount.get_storage().is::<Temporary>());
    }

    #[test]
    fn test_wrapper() {
        let test_path = "/foo/";
        
        let wrapper = |mount_point: &str, storage: Arc<dyn Storage>| {
            assert_eq!(mount_point, test_path);
            assert!(true, "storage implements Storage trait");
            
            let mut options = std::collections::HashMap::new();
            options.insert("storage".to_string(), storage);
            
            Arc::new(Wrapper::new(options)) as Arc<dyn Storage>
        };

        let mut loader = Loader::new();
        loader.add_storage_wrapper(Arc::new(wrapper));

        let storage = MockTemporary::new();
        let mount = Mount::new_with_loader(
            Arc::new(storage),
            "/foo".to_string(),
            std::collections::HashMap::new(),
            loader
        );
        
        assert!(mount.get_storage().is::<Wrapper>());
    }
}