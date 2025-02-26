//! ownCloud
//!
//! @author Florin Peter
//! @copyright 2013 Florin Peter <owncloud@florin-peter.de>
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::error::Error;
use std::path::Path;
use std::fs;

use crate::encryption;
use crate::encryption::crypt::Crypt;
use crate::encryption::keymanager::KeyManager;
use crate::encryption::proxy::Proxy;
use crate::encryption::stream::Stream;
use crate::encryption::util::Util;
use crate::encryption::helper::Helper;
use crate::tests::util as test_util;

// Mock imports for OC framework
use crate::oc_user::OcUser;
use crate::oc_appconfig::OcAppconfig;
use crate::oc_hook::OcHook;
use crate::oc_file_proxy::OcFileProxy;
use crate::oc_group::OcGroup;
use crate::oc_app::OcApp;
use crate::oc_filesystem_view::OcFilesystemView;
use crate::ocp_share::OcpShare;
use crate::ocp_permission;
use crate::ocp_util::OcpUtil;

/// Test case for encryption share functionality
pub struct TestEncryptionShare {
    data_short: String,
    view: OcFilesystemView,
    filename: String,
    folder1: String,
    subfolder: String,
    subsubfolder: String,
    state_files_trashbin: bool,
}

// Constants
const TEST_ENCRYPTION_SHARE_USER1: &str = "test-share-user1";
const TEST_ENCRYPTION_SHARE_USER2: &str = "test-share-user2";
const TEST_ENCRYPTION_SHARE_USER3: &str = "test-share-user3";
const TEST_ENCRYPTION_SHARE_USER4: &str = "test-share-user4";
const TEST_ENCRYPTION_SHARE_GROUP1: &str = "test-share-group1";

impl TestEncryptionShare {
    /// Set up test environment before all tests
    pub fn set_up_before_class() {
        // reset backend
        OcUser::clear_backends();
        OcUser::use_backend("database");

        // enable resharing
        OcAppconfig::set_value("core", "shareapi_allow_resharing", "yes");

        // clear share hooks
        OcHook::clear("OCP\\Share");
        // Register share hooks
        // Note: In Rust we'd use an explicit registration function rather than a global OC namespace
        crate::oc::register_share_hooks();
        OcpUtil::connect_hook("OC_Filesystem", "setup", "\\OC\\Files\\Storage\\Shared", "setup");

        // Sharing related hooks
        encryption::Helper::register_share_hooks();

        // Filesystem related hooks
        encryption::Helper::register_filesystem_hooks();

        // clear and register hooks
        OcFileProxy::clear_proxies();
        OcFileProxy::register(Proxy::new());

        // create users
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1, true);
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2, true);
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER3, true);
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER4, true);

        // create group and assign users
        OcGroup::create_group(TEST_ENCRYPTION_SHARE_GROUP1);
        OcGroup::add_to_group(TEST_ENCRYPTION_SHARE_USER3, TEST_ENCRYPTION_SHARE_GROUP1);
        OcGroup::add_to_group(TEST_ENCRYPTION_SHARE_USER4, TEST_ENCRYPTION_SHARE_GROUP1);
    }

    /// Initialize the test instance
    pub fn new() -> Self {
        let view = OcFilesystemView::new("/");
        
        // we don't want to tests with app files_trashbin enabled
        OcApp::disable("files_trashbin");
        
        // remember files_trashbin state
        let state_files_trashbin = OcApp::is_enabled("files_trashbin");
        
        Self {
            data_short: "hats".to_string(),
            view,
            filename: "share-tmp.test".to_string(),
            folder1: "/folder1".to_string(),
            subfolder: "/subfolder1".to_string(),
            subsubfolder: "/subsubfolder1".to_string(),
            state_files_trashbin,
        }
    }

    /// Clean up after each test
    pub fn tear_down(&self) {
        // reset app files_trashbin
        if self.state_files_trashbin {
            OcApp::enable("files_trashbin");
        } else {
            OcApp::disable("files_trashbin");
        }
    }

    /// Clean up after all tests
    pub fn tear_down_after_class() {
        // clean group
        OcGroup::delete_group(TEST_ENCRYPTION_SHARE_GROUP1);

        // cleanup users
        OcUser::delete_user(TEST_ENCRYPTION_SHARE_USER1);
        OcUser::delete_user(TEST_ENCRYPTION_SHARE_USER2);
        OcUser::delete_user(TEST_ENCRYPTION_SHARE_USER3);
        OcUser::delete_user(TEST_ENCRYPTION_SHARE_USER4);
    }

    /// Test sharing a file with another user
    pub fn test_share_file(&self, with_teardown: bool) -> Result<(), Box<dyn Error>> {
        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // save file with content
        let crypted_file = fs::write(
            format!("crypt:///{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename),
            &self.data_short
        )?;

        // test that data was successfully written
        assert!(crypted_file > 0);

        // disable encryption proxy to prevent recursive calls
        let proxy_status = OcFileProxy::is_enabled();
        OcFileProxy::set_enabled(false);

        // get the file info from previous created file
        let file_info = self.view.get_file_info(
            &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
        )?;

        // check if we have a valid file info
        assert!(file_info.is_some());
        let file_info = file_info.unwrap();

        // check if the unencrypted file size is stored
        assert!(file_info.unencrypted_size > 0);

        // re-enable the file proxy
        OcFileProxy::set_enabled(proxy_status);

        // share the file
        OcpShare::share_item(
            "file",
            file_info.fileid,
            OcpShare::SHARE_TYPE_USER,
            TEST_ENCRYPTION_SHARE_USER2,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // check if share key for user1 exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER2
            )
        ));

        // login as user1
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

        // get file contents
        let retrieved_crypted_file = self.view.file_get_contents(
            &format!("/{}/files/Shared/{}", TEST_ENCRYPTION_SHARE_USER2, self.filename)
        )?;

        // check if data is the same as we previously written
        assert_eq!(self.data_short, retrieved_crypted_file);

        // cleanup
        if with_teardown {
            // login as admin
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

            // unshare the file
            OcpShare::unshare(
                "file",
                file_info.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER2
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER2
                )
            ));

            // cleanup
            self.view.unlink(
                &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER1
                )
            ));
        }

        Ok(())
    }

    /// Test resharing a file
    pub fn test_re_share_file(&self, with_teardown: bool) -> Result<(), Box<dyn Error>> {
        self.test_share_file(false)?;

        // login as user1
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

        // get the file info
        let file_info = self.view.get_file_info(
            &format!("/{}/files/Shared/{}", TEST_ENCRYPTION_SHARE_USER2, self.filename)
        )?;

        // check if we have a valid file info
        assert!(file_info.is_some());
        let file_info = file_info.unwrap();

        // share the file with user2
        OcpShare::share_item(
            "file",
            file_info.fileid,
            OcpShare::SHARE_TYPE_USER,
            TEST_ENCRYPTION_SHARE_USER3,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // check if share key for user2 exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER3
            )
        ));

        // login as user2
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER3);

        // get file contents
        let retrieved_crypted_file = self.view.file_get_contents(
            &format!("/{}/files/Shared/{}", TEST_ENCRYPTION_SHARE_USER3, self.filename)
        )?;

        // check if data is the same as previously written
        assert_eq!(self.data_short, retrieved_crypted_file);

        // cleanup
        if with_teardown {
            // login as user1
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

            // unshare the file with user2
            OcpShare::unshare(
                "file",
                file_info.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER3
            )?;

            // login as admin
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER3
                )
            ));

            // unshare the file with user1
            OcpShare::unshare(
                "file",
                file_info.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER2
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER2
                )
            ));

            // cleanup
            self.view.unlink(
                &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER1
                )
            ));
        }

        Ok(())
    }

    /// Test sharing a folder with another user
    pub fn test_share_folder(&self, with_teardown: bool) -> Result<Option<FileInfo>, Box<dyn Error>> {
        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // create folder structure
        self.view.mkdir(&format!("/{}/files{}", TEST_ENCRYPTION_SHARE_USER1, self.folder1))?;
        self.view.mkdir(
            &format!("/{}/files{}{}", TEST_ENCRYPTION_SHARE_USER1, self.folder1, self.subfolder)
        )?;
        self.view.mkdir(
            &format!("/{}/files{}{}{}", 
                TEST_ENCRYPTION_SHARE_USER1, 
                self.folder1, 
                self.subfolder,
                self.subsubfolder
            )
        )?;

        // save file with content
        let crypted_file = fs::write(
            format!("crypt:///{}/files{}{}{}/{}",
                TEST_ENCRYPTION_SHARE_USER1,
                self.folder1,
                self.subfolder,
                self.subsubfolder,
                self.filename
            ),
            &self.data_short
        )?;

        // test that data was successfully written
        assert!(crypted_file > 0);

        // disable encryption proxy to prevent recursive calls
        let proxy_status = OcFileProxy::is_enabled();
        OcFileProxy::set_enabled(false);

        // get the file info from previous created folder
        let file_info = self.view.get_file_info(
            &format!("/{}/files{}", TEST_ENCRYPTION_SHARE_USER1, self.folder1)
        )?;

        // check if we have a valid file info
        assert!(file_info.is_some());
        let file_info = file_info.unwrap();

        // re-enable the file proxy
        OcFileProxy::set_enabled(proxy_status);

        // share the folder with user1
        OcpShare::share_item(
            "folder",
            file_info.fileid,
            OcpShare::SHARE_TYPE_USER,
            TEST_ENCRYPTION_SHARE_USER2,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // check if share key for user1 exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.folder1,
                self.subfolder,
                self.subsubfolder,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER2
            )
        ));

        // login as user1
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

        // get file contents
        let retrieved_crypted_file = self.view.file_get_contents(
            &format!("/{}/files/Shared{}{}{}/{}",
                TEST_ENCRYPTION_SHARE_USER2,
                self.folder1,
                self.subfolder,
                self.subsubfolder,
                self.filename
            )
        )?;

        // check if data is the same
        assert_eq!(self.data_short, retrieved_crypted_file);

        // cleanup
        if with_teardown {
            // login as admin
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

            // unshare the folder with user1
            OcpShare::unshare(
                "folder",
                file_info.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER2
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER2
                )
            ));

            // cleanup
            self.view.unlink(
                &format!("/{}/files{}", TEST_ENCRYPTION_SHARE_USER1, self.folder1)
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER1
                )
            ));
        }

        Ok(Some(file_info))
    }

    /// Test resharing a folder with another user
    pub fn test_re_share_folder(&self, with_teardown: bool) -> Result<(), Box<dyn Error>> {
        let file_info_folder1 = self.test_share_folder(false)?;
        assert!(file_info_folder1.is_some());
        let file_info_folder1 = file_info_folder1.unwrap();

        // login as user1
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

        // disable encryption proxy to prevent recursive calls
        let proxy_status = OcFileProxy::is_enabled();
        OcFileProxy::set_enabled(false);

        // get the file info from previous created folder
        let file_info_subfolder = self.view.get_file_info(
            &format!("/{}/files/Shared{}{}",
                TEST_ENCRYPTION_SHARE_USER2,
                self.folder1,
                self.subfolder
            )
        )?;

        // check if we have a valid file info
        assert!(file_info_subfolder.is_some());
        let file_info_subfolder = file_info_subfolder.unwrap();

        // re-enable the file proxy
        OcFileProxy::set_enabled(proxy_status);

        // share the file with user2
        OcpShare::share_item(
            "folder",
            file_info_subfolder.fileid,
            OcpShare::SHARE_TYPE_USER,
            TEST_ENCRYPTION_SHARE_USER3,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // check if share key for user2 exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.folder1,
                self.subfolder,
                self.subsubfolder,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER3
            )
        ));

        // login as user2
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER3);

        // get file contents
        let retrieved_crypted_file = self.view.file_get_contents(
            &format!("/{}/files/Shared{}{}/{}",
                TEST_ENCRYPTION_SHARE_USER3,
                self.subfolder,
                self.subsubfolder,
                self.filename
            )
        )?;

        // check if data is the same
        assert_eq!(self.data_short, retrieved_crypted_file);

        // get the file info
        let file_info = self.view.get_file_info(
            &format!("/{}/files/Shared{}{}/{}",
                TEST_ENCRYPTION_SHARE_USER3,
                self.subfolder,
                self.subsubfolder,
                self.filename
            )
        )?;

        // check if we have fileInfos
        assert!(file_info.is_some());
        let file_info = file_info.unwrap();

        // share the file with user3
        OcpShare::share_item(
            "file",
            file_info.fileid,
            OcpShare::SHARE_TYPE_USER,
            TEST_ENCRYPTION_SHARE_USER4,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // check if share key for user3 exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.folder1,
                self.subfolder,
                self.subsubfolder,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER4
            )
        ));

        // login as user3
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER4);

        // get file contents
        let retrieved_crypted_file = self.view.file_get_contents(
            &format!("/{}/files/Shared/{}",
                TEST_ENCRYPTION_SHARE_USER4,
                self.filename
            )
        )?;

        // check if data is the same
        assert_eq!(self.data_short, retrieved_crypted_file);

        // cleanup
        if with_teardown {
            // login as user2
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER3);

            // unshare the file with user3
            OcpShare::unshare(
                "file",
                file_info.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER4
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER4
                )
            ));

            // login as user1
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER2);

            // unshare the folder with user2
            OcpShare::unshare(
                "folder",
                file_info_subfolder.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER3
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER3
                )
            ));

            // login as admin
            test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

            // unshare the folder1 with user1
            OcpShare::unshare(
                "folder",
                file_info_folder1.fileid,
                OcpShare::SHARE_TYPE_USER,
                TEST_ENCRYPTION_SHARE_USER2
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER2
                )
            ));

            // cleanup
            self.view.unlink(
                &format!("/{}/files{}{}{}/{}",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename
                )
            )?;

            // check if share key not exists
            assert!(!self.view.file_exists(
                &format!("/{}/files_encryption/share-keys{}{}{}/{}.{}.shareKey",
                    TEST_ENCRYPTION_SHARE_USER1,
                    self.folder1,
                    self.subfolder,
                    self.subsubfolder,
                    self.filename,
                    TEST_ENCRYPTION_SHARE_USER1
                )
            ));
        }

        Ok(())
    }

    /// Test public sharing of a file
    pub fn test_public_share_file(&self) -> Result<(), Box<dyn Error>> {
        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // save file with content
        let crypted_file = fs::write(
            format!("crypt:///{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename),
            &self.data_short
        )?;

        // test that data was successfully written
        assert!(crypted_file > 0);

        // disable encryption proxy to prevent recursive calls
        let proxy_status = OcFileProxy::is_enabled();
        OcFileProxy::set_enabled(false);

        // get the file info from previous created file
        let file_info = self.view.get_file_info(
            &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
        )?;

        // check if we have a valid file info
        assert!(file_info.is_some());
        let file_info = file_info.unwrap();

        // check if the unencrypted file size is stored
        assert!(file_info.unencrypted_size > 0);

        // re-enable the file proxy
        OcFileProxy::set_enabled(proxy_status);

        // share the file
        OcpShare::share_item(
            "file",
            file_info.fileid,
            OcpShare::SHARE_TYPE_LINK,
            false,
            ocp_permission::PERMISSION_ALL
        )?;

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        let public_share_key_id = OcAppconfig::get_value("files_encryption", "publicShareKeyId")?;

        // check if share key for public exists
        assert!(self.view.file_exists(
            &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.filename,
                public_share_key_id
            )
        ));

        // some hacking to simulate public link
        crate::globals::set_app("files_sharing");
        crate::globals::set_file_owner(TEST_ENCRYPTION_SHARE_USER1);
        OcUser::set_user_id(None);

        // get file contents
        let retrieved_crypted_file = fs::read_to_string(
            format!("crypt:///{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
        )?;

        // check if data is the same as we previously written
        assert_eq!(self.data_short, retrieved_crypted_file);

        // tear down

        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // unshare the file
        OcpShare::unshare(
            "file",
            file_info.fileid,
            OcpShare::SHARE_TYPE_LINK,
            None
        )?;

        // check if share key not exists
        assert!(!self.view.file_exists(
            &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.filename,
                public_share_key_id
            )
        ));

        // cleanup
        self.view.unlink(
            &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename)
        )?;

        // check if share key not exists
        assert!(!self.view.file_exists(
            &format!("/{}/files_encryption/share-keys/{}.{}.shareKey",
                TEST_ENCRYPTION_SHARE_USER1,
                self.filename,
                TEST_ENCRYPTION_SHARE_USER1
            )
        ));

        Ok(())
    }

    /// Test sharing a file with a group
    pub fn test_share_file_with_group(&self) -> Result<(), Box<dyn Error>> {
        // login as admin
        test_util::login_helper(TEST_ENCRYPTION_SHARE_USER1);

        // save file with content
        let crypted_file = fs::write(
            format!("crypt:///{}/files/{}", TEST_ENCRYPTION_SHARE_USER1, self.filename),
            &self.data_short
        )?;

        // test that data was successfully written
        assert!(crypted_file > 0);

        // disable encryption proxy to prevent recursive calls
        let proxy_status = OcFileProxy::is_enabled();
        OcFileProxy::set_enabled(false);

        // get the file info from previous created file
        let file_info = self.view.get_file_info(
            &format!("/{}/files/{}", TEST_ENCRYPTION_SHARE_USER1,