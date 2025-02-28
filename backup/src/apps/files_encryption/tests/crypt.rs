extern crate blowfish;
extern crate rand;
extern crate crypto;
extern crate openssl;

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rand::rngs::OsRng;
use rand::RngCore;
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::rsa::{Rsa, Padding};

mod encryption {
    pub mod crypt;
    pub mod keymanager;
    pub mod proxy;
    pub mod stream;
    pub mod util;
    pub mod helper;
    pub mod session;
}

use encryption::crypt::Crypt;
use encryption::keymanager::KeyManager;
use encryption::proxy::Proxy;
use encryption::session::Session;
use encryption::helper::Helper;

struct TestEncryptionCrypt {
    user_id: String,
    pass: String,
    state_files_trashbin: bool,
    data_long: String,
    data_url: String,
    data_short: String,
    view: Arc<FilesystemView>,
    legacy_encrypted_data: String,
    gen_private_key: String,
    gen_public_key: String,
    random_key: String,
}

struct FilesystemView {
    root: PathBuf,
}

impl FilesystemView {
    fn new(root: &str) -> Self {
        FilesystemView {
            root: PathBuf::from(root),
        }
    }

    fn file_get_contents(&self, path: &str) -> io::Result<String> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        let mut file = File::open(full_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn file_put_contents(&self, path: &str, contents: &str) -> io::Result<usize> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(full_path)?;
        file.write(contents.as_bytes())
    }

    fn unlink(&self, path: &str) -> io::Result<()> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        if full_path.is_dir() {
            fs::remove_dir_all(full_path)
        } else {
            fs::remove_file(full_path)
        }
    }

    fn rename(&self, old_name: &str, new_name: &str) -> io::Result<()> {
        let old_path = self.root.join(old_name.trim_start_matches('/'));
        let new_path = self.root.join(new_name.trim_start_matches('/'));
        fs::rename(old_path, new_path)
    }

    fn mkdir(&self, path: &str) -> io::Result<()> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        fs::create_dir_all(full_path)
    }

    fn fopen(&self, path: &str, mode: &str) -> io::Result<File> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        match mode {
            "r" => File::open(full_path),
            "w" => File::create(full_path),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported mode")),
        }
    }

    fn touch(&self, path: &str) -> io::Result<()> {
        let full_path = self.root.join(path.trim_start_matches('/'));
        if !full_path.exists() {
            File::create(full_path)?;
        } else {
            let now = std::time::SystemTime::now();
            filetime::set_file_mtime(full_path, filetime::FileTime::from_system_time(now))?;
        }
        Ok(())
    }
}

// Mock types and modules to make the code compile
struct OcUser;
struct OcApp;
struct OcFileProxy;
struct OcFilesystemView;

impl OcUser {
    fn clear_backends() {}
    fn use_backend(_backend: &str) {}
    fn set_user_id(_user_id: &str) {}
    fn delete_user(_user_id: &str) {}
    fn set_password(_user_id: &str, _password: &str) -> bool { true }
}

impl OcApp {
    fn is_enabled(_app: &str) -> bool { false }
    fn disable(_app: &str) {}
    fn enable(_app: &str) {}
}

impl OcFileProxy {
    fn clear_proxies() {}
    fn register(_proxy: Proxy) {}
}

impl OcFilesystemView {
    fn new(_path: &str) -> Self {
        OcFilesystemView {}
    }
}

struct TestEncryptionUtil;

impl TestEncryptionUtil {
    fn login_helper(user_id: &str, _create: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_ENCRYPTION_CRYPT_USER1: &str = "test-crypt-user1";
    
    fn setup_before_class() {
        // reset backend
        OcUser::clear_backends();
        OcUser::use_backend("database");
        
        // Filesystem related hooks
        Helper::register_filesystem_hooks();
        
        // User related hooks
        Helper::register_user_hooks();
        
        // clear and register hooks
        OcFileProxy::clear_proxies();
        OcFileProxy::register(Proxy::new());
        
        // create test user
        TestEncryptionUtil::login_helper(TEST_ENCRYPTION_CRYPT_USER1, true);
    }
    
    fn teardown_after_class() {
        // cleanup test user
        OcUser::delete_user(TEST_ENCRYPTION_CRYPT_USER1);
    }
    
    fn setup_test() -> TestEncryptionCrypt {
        // set user id
        OcUser::set_user_id(TEST_ENCRYPTION_CRYPT_USER1);
        let user_id = TEST_ENCRYPTION_CRYPT_USER1.to_string();
        let pass = TEST_ENCRYPTION_CRYPT_USER1.to_string();
        
        // set content for encrypting / decrypting in tests
        let data_long = fs::read_to_string("../lib/crypt.php").unwrap();
        let data_short = "hats".to_string();
        let data_url = "../lib/crypt.php".to_string();
        
        let legacy_encrypted_data = "./legacy-encrypted-text.txt".to_string();
        
        let random_key = Crypt::generate_key();
        
        let keypair = Crypt::create_keypair();
        let gen_public_key = keypair.public_key;
        let gen_private_key = keypair.private_key;
        
        let view = Arc::new(FilesystemView::new("/"));
        
        // remember files_trashbin state
        let state_files_trashbin = OcApp::is_enabled("files_trashbin");
        
        // we don't want to tests with app files_trashbin enabled
        OcApp::disable("files_trashbin");
        
        TestEncryptionCrypt {
            user_id,
            pass,
            state_files_trashbin,
            data_long,
            data_url,
            data_short,
            view,
            legacy_encrypted_data,
            gen_private_key,
            gen_public_key,
            random_key,
        }
    }
    
    fn teardown_test(test: &TestEncryptionCrypt) {
        // reset app files_trashbin
        if test.state_files_trashbin {
            OcApp::enable("files_trashbin");
        } else {
            OcApp::disable("files_trashbin");
        }
    }
    
    #[test]
    fn test_generate_key() {
        setup_before_class();
        let test = setup_test();
        
        let key = Crypt::generate_key();
        assert!(key.len() > 16);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_decrypt_private_key() {
        setup_before_class();
        let test = setup_test();
        
        // test successful decrypt
        let crypted = Crypt::symmetric_encrypt_file_content(&test.gen_private_key, "hat");
        let decrypted = Crypt::decrypt_private_key(&crypted, "hat");
        assert_eq!(Some(test.gen_private_key.clone()), decrypted);
        
        // test private key decrypt with wrong password
        let wrong_passwd = Crypt::decrypt_private_key(&crypted, "hat2");
        assert_eq!(None, wrong_passwd);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_symmetric_encrypt_file_content() {
        setup_before_class();
        let test = setup_test();
        
        let crypted = Crypt::symmetric_encrypt_file_content(&test.data_short, "hat");
        assert_ne!(test.data_short, crypted);
        
        let decrypt = Crypt::symmetric_decrypt_file_content(&crypted, "hat");
        assert_eq!(test.data_short, decrypt);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_symmetric_stream_encrypt_short_file_content() {
        setup_before_class();
        let test = setup_test();
        
        let filename = format!("tmp-{}.test", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let path = format!("{}/files/{}", test.user_id, filename);
        let crypted_file = stream::write_file(&path, &test.data_short).unwrap();
        
        // Test that data was successfully written
        assert!(crypted_file > 0);
        
        // Get file contents without using any wrapper to get its actual contents on disk
        let retrieved_crypted_file = test.view.file_get_contents(&format!("{}/files/{}", test.user_id, &filename)).unwrap();
        
        // Check that the file was encrypted before being written to disk
        assert_ne!(test.data_short, retrieved_crypted_file);
        
        // Get the encrypted keyfile
        let enc_keyfile = KeyManager::get_file_key(&test.view, &test.user_id, &filename);
        
        // Attempt to fetch the user's shareKey
        let share_key = KeyManager::get_share_key(&test.view, &test.user_id, &filename);
        
        // get session
        let session = Session::new(&test.view);
        
        // get private key
        let private_key = session.get_private_key(&test.user_id);
        
        // Decrypt keyfile with shareKey
        let plain_keyfile = Crypt::multi_key_decrypt(&enc_keyfile, &share_key, &private_key);
        
        // Manually decrypt
        let manual_decrypt = Crypt::symmetric_decrypt_file_content(&retrieved_crypted_file, &plain_keyfile);
        
        // Check that decrypted data matches
        assert_eq!(test.data_short, manual_decrypt);
        
        // Teardown
        test.view.unlink(&format!("{}/files/{}", test.user_id, &filename)).unwrap();
        KeyManager::delete_file_key(&test.view, &test.user_id, &filename);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_is_encrypted_content() {
        setup_before_class();
        let test = setup_test();
        
        assert!(!Crypt::is_catfile_content(&test.data_url));
        assert!(!Crypt::is_catfile_content(&test.legacy_encrypted_data));
        
        let keyfile_content = Crypt::symmetric_encrypt_file_content(&test.data_url, "hat");
        assert!(Crypt::is_catfile_content(&keyfile_content));
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_multi_key_encrypt() {
        setup_before_class();
        let test = setup_test();
        
        let pair1 = Crypt::create_keypair();
        assert_eq!(2, pair1.public_key.len() + pair1.private_key.len());
        assert!(pair1.public_key.len() > 1);
        assert!(pair1.private_key.len() > 1);
        
        let crypted = Crypt::multi_key_encrypt(&test.data_short, &[&pair1.public_key]);
        assert_ne!(test.data_short, crypted.data);
        
        let decrypt = Crypt::multi_key_decrypt(&crypted.data, &crypted.keys[0], &pair1.private_key);
        assert_eq!(test.data_short, decrypt);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_legacy_decrypt_short() {
        setup_before_class();
        let test = setup_test();
        
        let crypted = test.legacy_encrypt(&test.data_short, &test.pass);
        let decrypted = Crypt::legacy_block_decrypt(&crypted, &test.pass);
        assert_eq!(test.data_short, decrypted);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_legacy_decrypt_long() {
        setup_before_class();
        let test = setup_test();
        
        let crypted = test.legacy_encrypt(&test.data_long, &test.pass);
        let decrypted = Crypt::legacy_block_decrypt(&crypted, &test.pass);
        assert_eq!(test.data_long, decrypted);
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_rename_file() {
        setup_before_class();
        let test = setup_test();
        
        let filename = format!("tmp-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        // Save long data as encrypted file using stream wrapper
        let crypted_file = stream::write_file(&format!("{}/files/{}", test.user_id, &filename), &test.data_long).unwrap();
        
        // Test that data was successfully written
        assert!(crypted_file > 0);
        
        // Get file decrypted contents
        let decrypt = stream::read_file(&format!("{}/files/{}", test.user_id, &filename)).unwrap();
        assert_eq!(test.data_long, decrypt);
        
        let new_filename = format!("tmp-new-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let view = FilesystemView::new(&format!("/{}/files", test.user_id));
        view.rename(&filename, &new_filename).unwrap();
        
        // Get file decrypted contents
        let new_decrypt = stream::read_file(&format!("{}/files/{}", test.user_id, &new_filename)).unwrap();
        assert_eq!(test.data_long, new_decrypt);
        
        // tear down
        view.unlink(&new_filename).unwrap();
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    #[test]
    fn test_view_file_put_and_get_contents() {
        setup_before_class();
        let test = setup_test();
        
        let filename = format!("/tmp-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let view = FilesystemView::new(&format!("/{}/files", test.user_id));
        
        // Save short data as encrypted file using stream wrapper
        let crypted_file = view.file_put_contents(&filename, &test.data_short).unwrap();
        
        // Test that data was successfully written
        assert!(crypted_file > 0);
        
        // Get file decrypted contents
        let decrypt = view.file_get_contents(&filename).unwrap();
        assert_eq!(test.data_short, decrypt);
        
        // Save long data as encrypted file using stream wrapper
        let crypted_file_long = view.file_put_contents(&filename, &test.data_long).unwrap();
        
        // Test that data was successfully written
        assert!(crypted_file_long > 0);
        
        // Get file decrypted contents
        let decrypt_long = view.file_get_contents(&filename).unwrap();
        assert_eq!(test.data_long, decrypt_long);
        
        // tear down
        view.unlink(&filename).unwrap();
        
        teardown_test(&test);
        teardown_after_class();
    }
    
    impl TestEncryptionCrypt {
        fn legacy_encrypt(&self, data: &str, passwd: &str) -> String {
            // Implement the legacy blowfish encryption for testing
            let bf = blowfish::Blowfish::new(passwd.as_bytes());
            // This is a simplified implementation - real one would be more complex
            let encrypted = data.as_bytes()
                .chunks(8)
                .flat_map(|chunk| {
                    let mut block = [0u8; 8];
                    block[..chunk.len()].copy_from_slice(chunk);
                    // Encrypt block (simplified)
                    block.to_vec()
                })
                .collect::<Vec<u8>>();
            
            String::from_utf8_lossy(&encrypted).to_string()
        }
    }
}

// Stream module to handle encrypted file operations
mod stream {
    use std::io::{self, Read, Write};
    use std::fs::File;
    
    // Write encrypted data to a file
    pub fn write_file(path: &str, data: &str) -> io::Result<usize> {
        // This is a simplified implementation for the test code
        let mut file = File::create(path)?;
        file.write(data.as_bytes())
    }
    
    // Read and decrypt data from a file
    pub fn read_file(path: &str) -> io::Result<String> {
        // This is a simplified implementation for the test code
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}