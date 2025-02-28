// tests/lib/files/node/file.rs

use std::cell::RefCell;
use std::io::{Read, Write, Seek, SeekFrom};
use std::rc::Rc;

use mockall::predicate::*;
use mockall::{mock, Sequence};
use once_cell::sync::Lazy;
use rstest::*;
use tempfile::NamedTempFile;

use nextcloud_app_api::files::{FileInfo, Node, Permission, PermissionSet};
use nextcloud_app_api::files::errors::{NotFoundError, NotPermittedError};
use nextcloud_app_api::user::User;

// Mocks
mock! {
    pub Manager {}
}

mock! {
    pub View {
        fn get_file_info(&self, path: &str) -> Option<FileInfo>;
        fn unlink(&self, path: &str) -> bool;
        fn file_get_contents(&self, path: &str) -> Result<String, std::io::Error>;
        fn file_put_contents(&self, path: &str, content: &str) -> bool;
        fn get_mime_type(&self, path: &str) -> String;
        fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn std::io::Read + std::io::Write + std::io::Seek>, std::io::Error>;
        fn copy(&self, source: &str, target: &str) -> bool;
        fn rename(&self, source: &str, target: &str) -> bool;
        fn resolve_path(&self, path: &str) -> Option<(String, String)>;
    }
}

mock! {
    pub Root {
        fn emit(&self, event: &str, node: Rc<dyn Node>) -> bool;
        fn get_user(&self) -> Rc<User>;
        fn get(&self, path: &str) -> Result<Rc<dyn Node>, NotFoundError>;
        fn listen(&self, namespace: &str, event: &str, callback: Box<dyn Fn(Rc<dyn Node>)>);
    }
}

mock! {
    pub DummyUser {}
    
    impl Clone for DummyUser {
        fn clone(&self) -> Self;
    }
}

struct File {
    root: Rc<MockRoot>,
    view: Rc<MockView>,
    path: String,
}

impl File {
    fn new(root: Rc<MockRoot>, view: Rc<MockView>, path: &str) -> Self {
        Self {
            root,
            view,
            path: path.to_string(),
        }
    }

    fn delete(&self) -> Result<(), NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        if !file_info.permissions.contains(Permission::DELETE) {
            return Err(NotPermittedError::new("Not permitted to delete file"));
        }
        
        self.root.emit("preDelete", Rc::new(self.clone()));
        
        if !self.view.unlink(&self.path) {
            return Err(NotPermittedError::new("Failed to delete file"));
        }
        
        self.root.emit("postDelete", Rc::new(self.clone()));
        
        Ok(())
    }

    fn get_content(&self) -> Result<String, NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        if !file_info.permissions.contains(Permission::READ) {
            return Err(NotPermittedError::new("Not permitted to read file"));
        }
        
        self.view.file_get_contents(&self.path).map_err(|e| {
            NotPermittedError::new(&format!("Failed to read file: {}", e))
        })
    }

    fn put_content(&self, content: &str) -> Result<(), NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        if !file_info.permissions.contains(Permission::UPDATE) {
            return Err(NotPermittedError::new("Not permitted to write to file"));
        }
        
        if !self.view.file_put_contents(&self.path, content) {
            return Err(NotPermittedError::new("Failed to write to file"));
        }
        
        Ok(())
    }

    fn get_mime_type(&self) -> String {
        self.view.get_mime_type(&self.path)
    }

    fn fopen(&self, mode: &str) -> Result<Box<dyn Read + Write + Seek>, NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        let needs_read = mode.contains('r') || mode.contains('+');
        let needs_write = mode.contains('w') || mode.contains('a') || mode.contains('+');
        
        if needs_read && !file_info.permissions.contains(Permission::READ) {
            return Err(NotPermittedError::new("Not permitted to read from file"));
        }
        
        if needs_write && !file_info.permissions.contains(Permission::UPDATE) {
            return Err(NotPermittedError::new("Not permitted to write to file"));
        }
        
        if needs_write {
            self.root.emit("preWrite", Rc::new(self.clone()));
        }
        
        let handle = self.view.fopen(&self.path, mode).map_err(|e| {
            NotPermittedError::new(&format!("Failed to open file: {}", e))
        })?;
        
        if needs_write {
            self.root.emit("postWrite", Rc::new(self.clone()));
        }
        
        Ok(handle)
    }

    fn copy(&self, target_path: &str) -> Result<Rc<dyn Node>, NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        if !file_info.permissions.contains(Permission::READ) {
            return Err(NotPermittedError::new("Not permitted to read file for copy"));
        }
        
        let target_parent_path = get_parent_path(target_path);
        let target_parent = self.root.get(&target_parent_path).map_err(|_| {
            NotPermittedError::new("Target parent not found")
        })?;
        
        // Check if target_parent is a folder
        if !target_parent.is_dir() {
            return Err(NotPermittedError::new("Target parent is not a directory"));
        }
        
        if !self.view.copy(&self.path, target_path) {
            return Err(NotPermittedError::new("Failed to copy file"));
        }
        
        self.root.get(target_path).map_err(|e| {
            NotPermittedError::new(&format!("Failed to get target node after copy: {}", e))
        })
    }

    fn move_to(&self, target_path: &str) -> Result<Rc<dyn Node>, NotPermittedError> {
        let file_info = self.view.get_file_info(&self.path).ok_or_else(|| {
            NotPermittedError::new("File not found")
        })?;
        
        if !file_info.permissions.contains(Permission::READ) || 
           !file_info.permissions.contains(Permission::DELETE) {
            return Err(NotPermittedError::new("Not permitted to move file"));
        }
        
        let target_parent_path = get_parent_path(target_path);
        let target_parent = self.root.get(&target_parent_path).map_err(|_| {
            NotPermittedError::new("Target parent not found")
        })?;
        
        // Check if target_parent is a folder
        if !target_parent.is_dir() {
            return Err(NotPermittedError::new("Target parent is not a directory"));
        }
        
        if !self.view.rename(&self.path, target_path) {
            return Err(NotPermittedError::new("Failed to move file"));
        }
        
        self.root.get(target_path).map_err(|e| {
            NotPermittedError::new(&format!("Failed to get target node after move: {}", e))
        })
    }

    fn get_id(&self) -> u64 {
        self.view.get_file_info(&self.path)
            .map(|info| info.file_id)
            .unwrap_or(0)
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_internal_path(&self) -> String {
        self.view.resolve_path(&self.path)
            .map(|(_, internal_path)| internal_path)
            .unwrap_or_default()
    }

    fn is_dir(&self) -> bool {
        false
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            view: self.view.clone(),
            path: self.path.clone(),
        }
    }
}

fn get_parent_path(path: &str) -> String {
    path.rsplitn(2, '/').last().unwrap_or("").to_string()
}

struct NonExistingFile {
    path: String,
    internal_path: String,
}

impl NonExistingFile {
    fn new(path: &str, internal_path: &str) -> Self {
        Self {
            path: path.to_string(),
            internal_path: internal_path.to_string(),
        }
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_internal_path(&self) -> &str {
        &self.internal_path
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::fs::File as StdFile;
    use std::io::{Read, Write, Seek, SeekFrom};
    use std::rc::Rc;

    struct TestFile {
        file: RefCell<NamedTempFile>,
    }

    impl TestFile {
        fn new(content: &str) -> Self {
            let mut file = NamedTempFile::new().unwrap();
            file.write_all(content.as_bytes()).unwrap();
            Self { file: RefCell::new(file) }
        }
    }

    impl Read for TestFile {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.file.borrow_mut().read(buf)
        }
    }

    impl Write for TestFile {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.file.borrow_mut().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.file.borrow_mut().flush()
        }
    }

    impl Seek for TestFile {
        fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
            self.file.borrow_mut().seek(pos)
        }
    }

    fn create_user() -> Rc<User> {
        Rc::new(User::new("", Rc::new(MockDummyUser::new())))
    }

    #[test]
    fn test_delete() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_emit()
            .times(2)
            .returning(|_, _| true);
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 0,
                })
            });
        
        view.expect_unlink()
            .with(eq("/bar/foo"))
            .returning(|_| true);
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.delete().is_ok());
    }

    #[test]
    fn test_delete_hooks() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        let hooks_run = Rc::new(RefCell::new(0));
        let hooks_run_pre = hooks_run.clone();
        let hooks_run_post = hooks_run.clone();
        
        root.expect_listen()
            .withf(|ns, event, _| {
                ns == "\\OC\\Files" && event == "preDelete"
            })
            .times(1)
            .returning(move |_, _, callback| {
                // Pre callback
                let node = Rc::new(File::new(
                    Rc::new(MockRoot::new()),
                    Rc::new(MockView::new()),
                    "/bar/foo"
                ));
                
                let hooks_run = hooks_run_pre.clone();
                *hooks_run.borrow_mut() += 1;
                callback(node);
            });
        
        root.expect_listen()
            .withf(|ns, event, _| {
                ns == "\\OC\\Files" && event == "postDelete"
            })
            .times(1)
            .returning(move |_, _, callback| {
                // Post callback
                let node = Rc::new(NonExistingFile::new("/bar/foo", "foo"));
                
                let hooks_run = hooks_run_post.clone();
                *hooks_run.borrow_mut() += 1;
                callback(Rc::new(node));
            });
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 1,
                })
            });
        
        view.expect_unlink()
            .with(eq("/bar/foo"))
            .returning(|_| true);
        
        view.expect_resolve_path()
            .with(eq("/bar/foo"))
            .returning(|_| Some((String::new(), "foo".to_string())));
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.delete().is_ok());
        assert_eq!(*hooks_run.borrow(), 2);
    }

    #[test]
    fn test_delete_not_permitted() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::READ,
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.delete().is_err());
    }

    #[test]
    fn test_get_content() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_listen()
            .returning(|_, _, _| {});
        
        view.expect_file_get_contents()
            .with(eq("/bar/foo"))
            .returning(|_| Ok("bar".to_string()));
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::READ,
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert_eq!(file.get_content().unwrap(), "bar");
    }

    #[test]
    fn test_get_content_not_permitted() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::empty(),
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.get_content().is_err());
    }

    #[test]
    fn test_put_content() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_get_user()
            .returning(move || user.clone());
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 0,
                })
            });
        
        view.expect_file_put_contents()
            .with(eq("/bar/foo"), eq("bar"))
            .returning(|_, _| true);
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.put_content("bar").is_ok());
    }

    #[test]
    fn test_put_content_not_permitted() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::READ,
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.put_content("bar").is_err());
    }

    #[test]
    fn test_get_mime_type() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_get_mime_type()
            .with(eq("/bar/foo"))
            .returning(|_| "text/plain".to_string());
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert_eq!(file.get_mime_type(), "text/plain");
    }

    #[test]
    fn test_fopen_read() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        view.expect_fopen()
            .with(eq("/bar/foo"), eq("r"))
            .returning(|_, _| {
                let test_file = Box::new(TestFile::new("bar"));
                Ok(test_file as Box<dyn Read + Write + Seek>)
            });
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        let mut fh = file.fopen("r").unwrap();
        let mut content = String::new();
        fh.read_to_string(&mut content).unwrap();
        assert_eq!(content, "bar");
    }

    #[test]
    fn test_fopen_write() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        root.expect_emit()
            .times(2)
            .returning(|_, _| true);
        
        view.expect_fopen()
            .with(eq("/bar/foo"), eq("w"))
            .returning(|_, _| {
                let test_file = Box::new(TestFile::new(""));
                Ok(test_file as Box<dyn Read + Write + Seek>)
            });
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        let mut fh = file.fopen("w").unwrap();
        fh.write_all(b"bar").unwrap();
        fh.flush().unwrap();
        fh.seek(SeekFrom::Start(0)).unwrap();
        
        let mut content = String::new();
        fh.read_to_string(&mut content).unwrap();
        assert_eq!(content, "bar");
    }

    #[test]
    fn test_fopen_read_not_permitted() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::empty(),
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.fopen("r").is_err());
    }

    #[test]
    fn test_fopen_read_write_no_read_permissions() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::UPDATE,
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.fopen("w").is_err());
    }

    #[test]
    fn test_fopen_read_write_no_write_permissions() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        let user = create_user();
        
        view.expect_get_file_info()
            .with(eq("/bar/foo"))
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::READ,
                    file_id: 0,
                })
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.fopen("w").is_err());
    }

    #[test]
    fn test_copy_same_storage() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_copy()
            .with(eq("/bar/foo"), eq("/bar/asd"))
            .returning(|_, _| true);
        
        view.expect_get_file_info()
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 3,
                })
            });
        
        let parent_node = Rc::new(File::new(
            Rc::new(MockRoot::new()), 
            Rc::new(MockView::new()), 
            "/bar"
        ));
        
        let new_node = Rc::new(File::new(
            Rc::new(MockRoot::new()), 
            Rc::new(MockView::new()), 
            "/bar/asd"
        ));
        
        root.expect_get()
            .times(2)
            .returning(move |path| {
                match path {
                    "/bar/asd" => Ok(new_node.clone()),
                    "/bar" => Ok(parent_node.clone()),
                    _ => Err(NotFoundError::new("Not found")),
                }
            });
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        let target = file.copy("/bar/asd").unwrap();
        assert_eq!(target.get_id(), 3);
    }

    #[test]
    fn test_copy_not_permitted() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_get_file_info()
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::READ,
                    file_id: 3,
                })
            });
        
        let parent_node = Rc::new(File::new(
            Rc::new(MockRoot::new()), 
            Rc::new(MockView::new()), 
            "/bar"
        ));
        
        root.expect_get()
            .with(eq("/bar"))
            .returning(move |_| Ok(parent_node.clone()));
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.copy("/bar/asd").is_err());
    }

    #[test]
    fn test_copy_no_parent() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_get_file_info()
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 3,
                })
            });
        
        root.expect_get()
            .with(eq("/bar"))
            .returning(|_| Err(NotFoundError::new("Not found")));
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.copy("/bar/asd/foo").is_err());
    }

    #[test]
    fn test_copy_parent_is_file() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_get_file_info()
            .returning(|_| {
                Some(FileInfo {
                    permissions: PermissionSet::all(),
                    file_id: 3,
                })
            });
        
        let parent_node = Rc::new(File::new(
            Rc::new(MockRoot::new()), 
            Rc::new(MockView::new()), 
            "/bar"
        ));
        
        root.expect_get()
            .with(eq("/bar"))
            .returning(move |_| Ok(parent_node.clone()));
        
        let root_rc = Rc::new(root);
        let view_rc = Rc::new(view);
        let file = File::new(root_rc, view_rc, "/bar/foo");
        
        assert!(file.copy("/bar/asd").is_err());
    }

    #[test]
    fn test_move_same_storage() {
        let mut manager = MockManager::new();
        let mut view = MockView::new();
        let mut root = MockRoot::new();
        
        view.expect_rename()
            .with(eq("/bar/foo"), eq("/bar/asd"))
            .returning(|_, _| true);
        
        view.expect_get_file_info()

}} // Añadido por reparador automático