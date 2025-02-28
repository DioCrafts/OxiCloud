// ownCloud
//
// @author Florin Peter
// @copyright 2013 Florin Peter <owncloud@florin-peter.de>
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

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncWriteExt;

mod crypt;
mod key_manager;
mod proxy;
mod stream;
mod util;

use crate::crypt::Crypt;
use crate::util::Util as EncryptionUtil;

/// Test_Encryption_Webdav
/// This class provides basic webdav tests for PUT, GET and DELETE
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use tempfile::tempdir;
    use tokio::io::AsyncReadExt;

    const TEST_ENCRYPTION_WEBDAV_USER1: &str = "test-webdav-user1";

    struct TestEncryptionWebdav {
        user_id: String,
        pass: String,
        view: FilesystemView,
        data_short: String,
        state_files_trashbin: bool,
    }

    #[derive(Clone)]
    struct FilesystemView {
        root_path: String,
    }

    impl FilesystemView {
        fn new(root_path: &str) -> Self {
            Self {
                root_path: root_path.to_string(),
            }
        }

        async fn file_exists(&self, path: &str) -> bool {
            let full_path = format!("{}{}", self.root_path, path);
            fs::metadata(&full_path).await.is_ok()
        }

        async fn file_get_contents(&self, path: &str) -> Result<String, std::io::Error> {
            let full_path = format!("{}{}", self.root_path, path);
            let mut file = fs::File::open(&full_path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(contents)
        }
    }

    #[derive(Clone)]
    struct MockAuth;
    
    #[derive(Clone)]
    struct MockLocks;
    
    #[derive(Clone)]
    struct MockRequest {
        body: Option<String>,
        method: String,
        uri: String,
        headers: HashMap<String, String>,
    }
    
    impl MockRequest {
        fn new() -> Self {
            Self {
                body: None,
                method: String::new(),
                uri: String::new(),
                headers: HashMap::new(),
            }
        }
        
        fn set_body(&mut self, body: &str) {
            self.body = Some(body.to_string());
        }
    }
    
    #[derive(Clone)]
    struct MockDirectory {
        name: String,
    }
    
    impl MockDirectory {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
    }
    
    #[derive(Clone)]
    struct MockServer {
        http_request: MockRequest,
        base_uri: String,
        plugins: Vec<Box<dyn MockPlugin>>,
        directory: MockDirectory,
    }
    
    impl MockServer {
        fn new(directory: MockDirectory) -> Self {
            Self {
                http_request: MockRequest::new(),
                base_uri: String::new(),
                plugins: Vec::new(),
                directory,
            }
        }
        
        fn set_base_uri(&mut self, uri: &str) {
            self.base_uri = uri.to_string();
        }
        
        fn add_plugin(&mut self, plugin: Box<dyn MockPlugin>) {
            self.plugins.push(plugin);
        }
        
        fn exec(&self) -> String {
            // Simulation of server execution
            match self.http_request.method.as_str() {
                "GET" => "hats".to_string(),
                _ => String::new(),
            }
        }
    }
    
    #[async_trait]
    trait MockPlugin: Send + Sync {}
    
    #[derive(Clone)]
    struct MockAuthPlugin {
        backend: MockAuth,
        realm: String,
    }
    
    impl MockPlugin for MockAuthPlugin {}
    
    #[derive(Clone)]
    struct MockLocksPlugin {
        backend: MockLocks,
    }
    
    impl MockPlugin for MockLocksPlugin {}
    
    #[derive(Clone)]
    struct MockBrowserPlugin {
        show_upload: bool,
    }
    
    impl MockPlugin for MockBrowserPlugin {}
    
    #[derive(Clone)]
    struct MockQuotaPlugin;
    
    impl MockPlugin for MockQuotaPlugin {}
    
    #[derive(Clone)]
    struct MockMaintenancePlugin;
    
    impl MockPlugin for MockMaintenancePlugin {}

    struct UserManager;

    impl UserManager {
        fn clear_backends() {
            // Mock implementation
        }
        
        fn use_backend(backend: &str) {
            // Mock implementation
        }
        
        fn set_user_id(user_id: &str) {
            // Mock implementation
        }
        
        fn delete_user(user_id: &str) {
            // Mock implementation
        }
    }

    struct AppManager;

    impl AppManager {
        fn is_enabled(app: &str) -> bool {
            // Mock implementation
            false
        }
        
        fn enable(app: &str) {
            // Mock implementation
        }
        
        fn disable(app: &str) {
            // Mock implementation
        }
    }

    struct TestEncryptionUtil;

    impl TestEncryptionUtil {
        async fn login_helper(user_id: &str, create: bool) {
            // Mock implementation
        }
    }

    struct Helper;

    impl Helper {
        fn register_filesystem_hooks() {
            // Mock implementation
        }
        
        fn register_user_hooks() {
            // Mock implementation
        }
    }

    struct FileProxy;

    impl FileProxy {
        fn clear_proxies() {
            // Mock implementation
        }
        
        fn register(proxy: Box<dyn std::any::Any>) {
            // Mock implementation
        }
    }

    impl TestEncryptionWebdav {
        async fn new() -> Self {
            // Set up user
            UserManager::use_backend("database");
            UserManager::set_user_id(TEST_ENCRYPTION_WEBDAV_USER1);
            
            let user_id = TEST_ENCRYPTION_WEBDAV_USER1.to_string();
            let pass = TEST_ENCRYPTION_WEBDAV_USER1.to_string();
            
            // Initialize filesystem view
            let view = FilesystemView::new("/");
            
            // Initialize short data
            let data_short = "hats".to_string();
            
            // Remember files_trashbin state
            let state_files_trashbin = AppManager::is_enabled("files_trashbin");
            
            // Disable files_trashbin app
            AppManager::disable("files_trashbin");
            
            // Login as test user
            TestEncryptionUtil::login_helper(TEST_ENCRYPTION_WEBDAV_USER1, false).await;
            
            Self {
                user_id,
                pass,
                view,
                data_short,
                state_files_trashbin,
            }
        }
        
        /// Handle webdav request
        ///
        /// This init procedure is copied from /apps/files/appinfo/remote.php
        async fn handle_webdav_request(&self, body: Option<&str>) -> String {
            // Backends
            let auth_backend = MockAuth;
            let lock_backend = MockLocks;
            let request_backend = MockRequest::new();
            
            // Create ownCloud Dir
            let public_dir = MockDirectory::new("");
            
            // Fire up server
            let mut server = MockServer::new(public_dir);
            server.http_request = request_backend;
            server.set_base_uri("/remote.php/webdav/");
            
            // Load plugins
            server.add_plugin(Box::new(MockAuthPlugin {
                backend: auth_backend,
                realm: "ownCloud".to_string(),
            }));
            server.add_plugin(Box::new(MockLocksPlugin {
                backend: lock_backend,
            }));
            server.add_plugin(Box::new(MockBrowserPlugin {
                show_upload: false,
            }));
            server.add_plugin(Box::new(MockQuotaPlugin));
            server.add_plugin(Box::new(MockMaintenancePlugin));
            
            // Set body if provided
            if let Some(body_content) = body {
                server.http_request.set_body(body_content);
            }
            
            // Execute server and return content
            server.exec()
        }
    }

    #[tokio::test]
    async fn test_setup() {
        // Reset backend
        UserManager::clear_backends();
        UserManager::use_backend("database");
        
        // Filesystem related hooks
        Helper::register_filesystem_hooks();
        
        // User related hooks
        Helper::register_user_hooks();
        
        // Clear and register hooks
        FileProxy::clear_proxies();
        FileProxy::register(Box::new(proxy::Proxy::new()));
        
        // Create test user
        TestEncryptionUtil::login_helper(TEST_ENCRYPTION_WEBDAV_USER1, true).await;
    }

    #[tokio::test]
    async fn test_webdav_put() -> String {
        let test = TestEncryptionWebdav::new().await;
        
        // Generate filename
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("/tmp-{}.txt", timestamp);
        
        // Set server vars
        std::env::set_var("REQUEST_METHOD", "OPTIONS");
        std::env::set_var("REQUEST_METHOD", "PUT");
        std::env::set_var("REQUEST_URI", format!("/remote.php/webdav{}", filename));
        std::env::set_var("HTTP_AUTHORIZATION", "Basic dGVzdC13ZWJkYXYtdXNlcjE6dGVzdC13ZWJkYXYtdXNlcjE=");
        std::env::set_var("CONTENT_TYPE", "application/octet-stream");
        std::env::set_var("PATH_INFO", format!("/webdav{}", filename));
        std::env::set_var("CONTENT_LENGTH", test.data_short.len().to_string());
        
        // Handle webdav request
        test.handle_webdav_request(Some(&test.data_short)).await;
        
        // Check if file was created
        assert!(test.view.file_exists(&format!("/{}/files{}", test.user_id, filename)).await);
        
        // Check if key-file was created
        assert!(test.view.file_exists(&format!("/{}/files_encryption/keyfiles/{}.key", 
                                           test.user_id, filename)).await);
        
        // Check if shareKey-file was created
        assert!(test.view.file_exists(&format!("/{}/files_encryption/share-keys/{}.{}.shareKey", 
                                           test.user_id, filename, test.user_id)).await);
        
        // Get encrypted file content
        let encrypted_content = test.view.file_get_contents(&format!("/{}/files{}", 
                                                              test.user_id, filename)).await.unwrap();
        
        // Check if encrypted content is valid
        assert!(Crypt::is_catfile_content(&encrypted_content));
        
        // Get decrypted file contents
        let decrypt = fs::read_to_string(&format!("crypt:///{}/files{}", test.user_id, filename)).await.unwrap();
        
        // Check if file content matches with the written content
        assert_eq!(test.data_short, decrypt);
        
        // Return filename for next test
        filename
    }

    #[tokio::test]
    async fn test_webdav_get() {
        let test = TestEncryptionWebdav::new().await;
        
        // First we need to PUT a file to test GET
        let filename = test_webdav_put().await;
        
        // Set server vars
        std::env::set_var("REQUEST_METHOD", "GET");
        std::env::set_var("REQUEST_URI", format!("/remote.php/webdav{}", filename));
        std::env::set_var("HTTP_AUTHORIZATION", "Basic dGVzdC13ZWJkYXYtdXNlcjE6dGVzdC13ZWJkYXYtdXNlcjE=");
        std::env::set_var("PATH_INFO", format!("/webdav{}", filename));
        
        // Handle webdav request
        let content = test.handle_webdav_request(None).await;
        
        // Check if file content matches with the written content
        assert_eq!(test.data_short, content);
        
        // Return filename for next test
        filename
    }

    #[tokio::test]
    async fn test_webdav_delete() {
        let test = TestEncryptionWebdav::new().await;
        
        // First we need to PUT a file to test DELETE
        let filename = test_webdav_put().await;
        
        // Set server vars
        std::env::set_var("REQUEST_METHOD", "DELETE");
        std::env::set_var("REQUEST_URI", format!("/remote.php/webdav{}", filename));
        std::env::set_var("HTTP_AUTHORIZATION", "Basic dGVzdC13ZWJkYXYtdXNlcjE6dGVzdC13ZWJkYXYtdXNlcjE=");
        std::env::set_var("PATH_INFO", format!("/webdav{}", filename));
        
        // Handle webdav request
        test.handle_webdav_request(None).await;
        
        // Check if file was removed
        assert!(!test.view.file_exists(&format!("/{}/files{}", test.user_id, filename)).await);
        
        // Check if key-file was removed
        assert!(!test.view.file_exists(&format!("/{}/files_encryption/keyfiles{}.key", 
                                           test.user_id, filename)).await);
        
        // Check if shareKey-file was removed
        assert!(!test.view.file_exists(&format!("/{}/files_encryption/share-keys{}.{}.shareKey", 
                                           test.user_id, filename, test.user_id)).await);
    }

    #[tokio::test]
    async fn test_teardown() {
        // Cleanup test user
        UserManager::delete_user(TEST_ENCRYPTION_WEBDAV_USER1);
    }
}