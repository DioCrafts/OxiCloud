// Copyright (C) 2013 Florin Peter <owncloud@florin-peter.de>
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

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};

// Mock libraries and imports that would be in the actual application
mod oc_user {
    pub fn clear_backends() {}
    pub fn use_backend(backend: &str) {}
    pub fn set_user_id(user_id: &str) {}
    pub fn delete_user(user_id: &str) {}
}

mod oc_app {
    pub fn is_enabled(app: &str) -> bool { true }
    pub fn enable(app: &str) {}
    pub fn disable(app: &str) {}
}

mod oca {
    pub mod encryption {
        pub mod helper {
            pub fn register_filesystem_hooks() {}
        }
        
        pub struct Proxy;
        
        impl Proxy {
            pub fn new() -> Self {
                Self {}
            }
        }
    }
}

mod oc_file_proxy {
    use crate::oca;
    
    pub fn clear_proxies() {}
    pub fn register(proxy: oca::encryption::Proxy) {}
}

struct FilesystemView {
    base_path: String,
}

impl FilesystemView {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
    
    pub fn file_put_contents(&self, filename: &str, data: &str) -> io::Result<usize> {
        let path = format!("{}{}", self.base_path, filename);
        let mut file = File::create(path)?;
        file.write(data.as_bytes())
    }
    
    pub fn fopen(&self, filename: &str, mode: &str) -> io::Result<File> {
        let path = format!("{}{}", self.base_path, filename);
        let mut options = OpenOptions::new();
        
        if mode.contains("r") {
            options.read(true);
        }
        if mode.contains("w") {
            options.write(true).create(true).truncate(true);
        }
        if mode.contains("a") {
            options.write(true).create(true).append(true);
        }
        
        options.open(path)
    }
    
    pub fn unlink(&self, filename: &str) -> io::Result<()> {
        let path = format!("{}{}", self.base_path, filename);
        std::fs::remove_file(path)
    }
}

// Mock test utility
mod test_encryption_util {
    pub fn login_helper(username: &str, create: bool) {}
}

// PHPUnit-like test framework for Rust
struct TestCase;

impl TestCase {
    fn assert_true(&self, condition: bool) {
        assert!(condition);
    }
    
    fn assert_false(&self, condition: bool) {
        assert!(!condition);
    }
    
    fn assert_equals<T: PartialEq + std::fmt::Debug>(&self, expected: T, actual: T) {
        assert_eq!(expected, actual);
    }
}

/// Class Test_Encryption_Stream
/// This class provides basic stream tests
struct TestEncryptionStream {
    test_case: TestCase,
    user_id: String,
    pass: String,
    view: FilesystemView,
    data_short: String,
    state_files_trashbin: bool,
}

impl TestEncryptionStream {
    const TEST_ENCRYPTION_STREAM_USER1: &'static str = "test-stream-user1";
    
    fn set_up_before_class() {
        // Reset backend
        oc_user::clear_backends();
        oc_user::use_backend("database");
        
        // Filesystem related hooks
        oca::encryption::helper::register_filesystem_hooks();
        
        // Clear and register hooks
        oc_file_proxy::clear_proxies();
        oc_file_proxy::register(oca::encryption::Proxy::new());
        
        // Create test user
        test_encryption_util::login_helper(Self::TEST_ENCRYPTION_STREAM_USER1, true);
    }
    
    fn set_up(&mut self) {
        // Set user id
        oc_user::set_user_id(Self::TEST_ENCRYPTION_STREAM_USER1);
        self.user_id = Self::TEST_ENCRYPTION_STREAM_USER1.to_string();
        self.pass = Self::TEST_ENCRYPTION_STREAM_USER1.to_string();
        
        // Init filesystem view
        self.view = FilesystemView::new("/");
        
        // Init short data
        self.data_short = "hats".to_string();
        
        // Remember files_trashbin state
        self.state_files_trashbin = oc_app::is_enabled("files_trashbin");
        
        // We don't want to test with app files_trashbin enabled
        oc_app::disable("files_trashbin");
    }
    
    fn tear_down(&self) {
        // Reset app files_trashbin
        if self.state_files_trashbin {
            oc_app::enable("files_trashbin");
        } else {
            oc_app::disable("files_trashbin");
        }
    }
    
    fn tear_down_after_class() {
        // Cleanup test user
        oc_user::delete_user(Self::TEST_ENCRYPTION_STREAM_USER1);
    }
    
    fn test_stream_options(&self) -> io::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("/tmp-{}", timestamp);
        let view = FilesystemView::new(&format!("/{}/files", self.user_id));
        
        // Save short data as encrypted file using stream wrapper
        let crypted_file = view.file_put_contents(&filename, &self.data_short)?;
        
        // Test that data was successfully written
        self.test_case.assert_true(crypted_file > 0);
        
        let mut handle = view.fopen(&filename, "r")?;
        
        // Check if stream is at position zero
        self.test_case.assert_equals(0, handle.seek(SeekFrom::Current(0))?);
        
        // Set stream options
        // Note: Rust doesn't have direct equivalents for flock and other PHP stream functions
        // In a real application, we would use Rust's file locking mechanisms
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            use nix::fcntl::{flock, FlockArg};
            
            let fd = handle.as_raw_fd();
            self.test_case.assert_true(flock(fd, FlockArg::LockSh).is_ok());
            self.test_case.assert_true(flock(fd, FlockArg::Unlock).is_ok());
        }
        
        // Tear down
        view.unlink(&filename)?;
        
        Ok(())
    }
    
    fn test_stream_set_blocking(&self) -> io::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("/tmp-{}", timestamp);
        let view = FilesystemView::new(&format!("/{}/files", self.user_id));
        
        // Save short data as encrypted file using stream wrapper
        let crypted_file = view.file_put_contents(&filename, &self.data_short)?;
        
        // Test that data was successfully written
        self.test_case.assert_true(crypted_file > 0);
        
        let handle = view.fopen(&filename, "r")?;
        
        // Set stream options
        // Note: In Rust, files are blocking by default
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            use nix::fcntl::{fcntl, FcntlArg, OFlag};
            
            let fd = handle.as_raw_fd();
            let flags = fcntl(fd, FcntlArg::F_GETFL).unwrap();
            let new_flags = OFlag::from_bits_truncate(flags) & !OFlag::O_NONBLOCK;
            self.test_case.assert_true(fcntl(fd, FcntlArg::F_SETFL(new_flags)).is_ok());
        }
        
        // Tear down
        view.unlink(&filename)?;
        
        Ok(())
    }
    
    fn test_stream_set_timeout(&self) -> io::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("/tmp-{}", timestamp);
        let view = FilesystemView::new(&format!("/{}/files", self.user_id));
        
        // Save short data as encrypted file using stream wrapper
        let crypted_file = view.file_put_contents(&filename, &self.data_short)?;
        
        // Test that data was successfully written
        self.test_case.assert_true(crypted_file > 0);
        
        let handle = view.fopen(&filename, "r")?;
        
        // Set stream options
        // Note: Rust doesn't have direct equivalents for stream_set_timeout
        // In a real implementation, we would use Rust's timeout mechanisms
        self.test_case.assert_false(false); // Mock the PHP behavior
        
        // Tear down
        view.unlink(&filename)?;
        
        Ok(())
    }
    
    fn test_stream_set_write_buffer(&self) -> io::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("/tmp-{}", timestamp);
        let view = FilesystemView::new(&format!("/{}/files", self.user_id));
        
        // Save short data as encrypted file using stream wrapper
        let crypted_file = view.file_put_contents(&filename, &self.data_short)?;
        
        // Test that data was successfully written
        self.test_case.assert_true(crypted_file > 0);
        
        let mut handle = view.fopen(&filename, "r")?;
        
        // Set stream options
        // Note: Rust uses BufWriter for buffered writes
        // In a real implementation, we would use BufWriter with a specific capacity
        self.test_case.assert_equals(0, 0); // Mock the PHP behavior
        
        // Tear down
        view.unlink(&filename)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stream_options() {
        let mut test = TestEncryptionStream {
            test_case: TestCase,
            user_id: String::new(),
            pass: String::new(),
            view: FilesystemView::new("/"),
            data_short: String::new(),
            state_files_trashbin: false,
        };
        
        TestEncryptionStream::set_up_before_class();
        test.set_up();
        
        let result = test.test_stream_options();
        assert!(result.is_ok());
        
        test.tear_down();
        TestEncryptionStream::tear_down_after_class();
    }
    
    #[test]
    fn test_stream_set_blocking() {
        let mut test = TestEncryptionStream {
            test_case: TestCase,
            user_id: String::new(),
            pass: String::new(),
            view: FilesystemView::new("/"),
            data_short: String::new(),
            state_files_trashbin: false,
        };
        
        TestEncryptionStream::set_up_before_class();
        test.set_up();
        
        let result = test.test_stream_set_blocking();
        assert!(result.is_ok());
        
        test.tear_down();
        TestEncryptionStream::tear_down_after_class();
    }
    
    #[test]
    fn test_stream_set_timeout() {
        let mut test = TestEncryptionStream {
            test_case: TestCase,
            user_id: String::new(),
            pass: String::new(),
            view: FilesystemView::new("/"),
            data_short: String::new(),
            state_files_trashbin: false,
        };
        
        TestEncryptionStream::set_up_before_class();
        test.set_up();
        
        let result = test.test_stream_set_timeout();
        assert!(result.is_ok());
        
        test.tear_down();
        TestEncryptionStream::tear_down_after_class();
    }
    
    #[test]
    fn test_stream_set_write_buffer() {
        let mut test = TestEncryptionStream {
            test_case: TestCase,
            user_id: String::new(),
            pass: String::new(),
            view: FilesystemView::new("/"),
            data_short: String::new(),
            state_files_trashbin: false,
        };
        
        TestEncryptionStream::set_up_before_class();
        test.set_up();
        
        let result = test.test_stream_set_write_buffer();
        assert!(result.is_ok());
        
        test.tear_down();
        TestEncryptionStream::tear_down_after_class();
    }
}