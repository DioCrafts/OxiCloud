// Copyright notice
// ownCloud
//
// @author Bjoern Schiessle
// @copyright 2013 Bjoern Schiessle <schiessle@owncloud.com>
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

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mocking necessary imports and types
use crate::files::share::{self, Api, ShareResult};
use crate::files_sharing::test::base::TestFilesSharingBase;
use crate::ocp::share::{self, ShareType};

const TEST_FILES_SHARING_API_USER1: &str = "test_files_sharing_api_user1";
const TEST_FILES_SHARING_API_USER2: &str = "test_files_sharing_api_user2";
const TEST_FILES_SHARING_API_USER3: &str = "test_files_sharing_api_user3";

/// Test class for the Files Sharing API
pub struct TestFilesSharingApi {
    base: TestFilesSharingBase,
    folder: String,
    filename: String,
    data: Vec<u8>,
    view: Arc<Mutex<FileView>>,
    post_data: Arc<Mutex<HashMap<String, String>>>,
    get_data: Arc<Mutex<HashMap<String, String>>>,
}

// Mock file view
pub struct FileView;

impl FileView {
    pub fn file_put_contents(&self, path: &str, data: &[u8]) -> bool {
        // Mock implementation
        true
    }

    pub fn mkdir(&self, path: &str) -> bool {
        // Mock implementation
        true
    }

    pub fn unlink(&self, path: &str) -> bool {
        // Mock implementation
        true
    }

    pub fn delete_all(&self, path: &str) -> bool {
        // Mock implementation
        true
    }

    pub fn get_file_info(&self, path: &str) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("fileid".to_string(), "123".to_string());
        info
    }
}

// Mock for share result
pub struct ShareData {
    pub id: String,
    pub token: Option<String>,
    pub file_source: String,
    pub item_source: String,
    pub share_type: ShareType,
    pub share_with: Option<String>,
    pub permissions: String,
}

// Helper functions to simulate the PHP API calls
impl TestFilesSharingApi {
    pub fn new() -> Self {
        Self {
            base: TestFilesSharingBase::new(),
            folder: String::new(),
            filename: String::new(),
            data: Vec::new(),
            view: Arc::new(Mutex::new(FileView)),
            post_data: Arc::new(Mutex::new(HashMap::new())),
            get_data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set_up(&mut self) {
        self.base.set_up();

        self.folder = "/folder_share_api_test".to_string();
        self.filename = "share-api-test.txt".to_string();
        self.data = "some test data".as_bytes().to_vec();

        // save file with content
        self.view.lock().unwrap().file_put_contents(&self.filename, &self.data);
        self.view.lock().unwrap().mkdir(&self.folder);
        self.view.lock().unwrap().file_put_contents(&format!("{}/{}", self.folder, self.filename), &self.data);
    }

    pub fn tear_down(&mut self) {
        self.view.lock().unwrap().unlink(&self.filename);
        self.view.lock().unwrap().delete_all(&self.folder);

        self.base.tear_down();
    }

    fn get_share_from_id(&self, id: &str) -> Option<HashMap<String, String>> {
        // Mock implementation
        let mut share = HashMap::new();
        share.insert("id".to_string(), id.to_string());
        share.insert("item_source".to_string(), "123".to_string());
        Some(share)
    }

    fn login_helper(username: &str) {
        // Mock implementation for changing the current user
    }

    async fn test_create_share(&mut self) {
        // share to user
        {
            let mut post = self.post_data.lock().unwrap();
            post.insert("path".to_string(), self.filename.clone());
            post.insert("shareWith".to_string(), TEST_FILES_SHARING_API_USER2.to_string());
            post.insert("shareType".to_string(), ShareType::User.to_string());
        }

        let result = Api::create_share(HashMap::new()).await;

        assert!(result.succeeded());
        let data = result.get_data();

        let share = self.get_share_from_id(&data["id"]).unwrap();
        let items = share::get_item_shared("file", &share["item_source"]);

        assert!(!items.is_empty());

        // share link
        {
            let mut post = self.post_data.lock().unwrap();
            post.insert("path".to_string(), self.folder.clone());
            post.insert("shareType".to_string(), ShareType::Link.to_string());
        }

        let result = Api::create_share(HashMap::new()).await;

        // check if API call was successful
        assert!(result.succeeded());

        let data = result.get_data();

        // check if we have a token
        assert!(data.get("token").is_some());

        let share = self.get_share_from_id(&data["id"]).unwrap();
        let items = share::get_item_shared("file", &share["item_source"]);

        assert!(!items.is_empty());

        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);
        share::unshare("file", &fileinfo["fileid"], ShareType::User, TEST_FILES_SHARING_API_USER2);

        let fileinfo = self.view.lock().unwrap().get_file_info(&self.folder);
        share::unshare("folder", &fileinfo["fileid"], ShareType::Link, None);
    }

    async fn test_get_all_shares(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        share::share_item("file", &fileinfo["fileid"], ShareType::User, 
            TEST_FILES_SHARING_API_USER2, "31");

        let result = Api::get_all_shares(HashMap::new()).await;

        assert!(result.succeeded());

        // test should return one share created from test_create_share()
        assert_eq!(result.get_data().len(), 1);

        share::unshare("file", &fileinfo["fileid"], ShareType::User, TEST_FILES_SHARING_API_USER2);
    }

    async fn test_get_share_from_source(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        share::share_item("file", &fileinfo["fileid"], ShareType::Link,
            None, "1");

        {
            let mut get = self.get_data.lock().unwrap();
            get.insert("path".to_string(), self.filename.clone());
        }

        let result = Api::get_all_shares(HashMap::new()).await;

        assert!(result.succeeded());

        // test should return two shares
        assert_eq!(result.get_data().len(), 2);

        share::unshare("file", &fileinfo["fileid"], ShareType::User, TEST_FILES_SHARING_API_USER2);
        share::unshare("file", &fileinfo["fileid"], ShareType::Link, None);
    }

    async fn test_get_share_from_source_with_reshares(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        // share the file as user1 to user2
        share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        // login as user2 and reshare the file to user3
        Self::login_helper(TEST_FILES_SHARING_API_USER2);

        share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER3, "31");

        // login as user1 again
        Self::login_helper(TEST_FILES_SHARING_API_USER1);

        {
            let mut get = self.get_data.lock().unwrap();
            get.insert("path".to_string(), self.filename.clone());
        }

        let result = Api::get_all_shares(HashMap::new()).await;

        assert!(result.succeeded());

        // test should return one share
        assert_eq!(result.get_data().len(), 1);

        // now also ask for the reshares
        {
            let mut get = self.get_data.lock().unwrap();
            get.insert("reshares".to_string(), "true".to_string());
        }

        let result = Api::get_all_shares(HashMap::new()).await;

        assert!(result.succeeded());

        // now we should get two shares, the initial share and the reshare
        assert_eq!(result.get_data().len(), 2);

        // unshare files again
        Self::login_helper(TEST_FILES_SHARING_API_USER2);

        share::unshare("file", &fileinfo["fileid"], ShareType::User, TEST_FILES_SHARING_API_USER3);

        Self::login_helper(TEST_FILES_SHARING_API_USER1);

        share::unshare("file", &fileinfo["fileid"], ShareType::User, TEST_FILES_SHARING_API_USER2);
    }

    async fn test_get_share_from_id(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        let result = share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        // share was successful?
        assert!(result);

        // get item to determine share ID
        let result = share::get_item_shared("file", &fileinfo["fileid"]);

        assert_eq!(result.len(), 1);

        // get first element
        let share = &result[0];

        // call get_share() with share ID
        let mut params = HashMap::new();
        params.insert("id".to_string(), share["id"].clone());
        let result = Api::get_share(params).await;

        assert!(result.succeeded());

        // test should return one share created from test_create_share()
        assert_eq!(result.get_data().len(), 1);

        share::unshare("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2);
    }

    async fn test_get_share_from_folder(&mut self) {
        let fileinfo1 = self.view.lock().unwrap().get_file_info(&self.filename);
        let fileinfo2 = self.view.lock().unwrap().get_file_info(&format!("{}/{}", self.folder, self.filename));

        let result = share::share_item("file", &fileinfo1["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        // share was successful?
        assert!(result);

        let result = share::share_item("folder", &fileinfo2["fileid"], ShareType::Link,
            None, "1");

        // share was successful?
        assert!(result.is_some());

        {
            let mut get = self.get_data.lock().unwrap();
            get.insert("path".to_string(), self.folder.clone());
            get.insert("subfiles".to_string(), "true".to_string());
        }

        let result = Api::get_all_shares(HashMap::new()).await;

        assert!(result.succeeded());

        // test should return one share within self.folder
        assert_eq!(result.get_data().len(), 1);

        share::unshare("file", &fileinfo1["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2);

        share::unshare("folder", &fileinfo2["fileid"], ShareType::Link, None);
    }

    async fn test_get_share_from_unknown_id(&mut self) {
        let mut params = HashMap::new();
        params.insert("id".to_string(), "0".to_string());

        let result = Api::get_share(params).await;

        assert_eq!(result.get_status_code(), 404);
        let meta = result.get_meta();
        assert_eq!(meta["message"], "share doesn't exist");
    }

    async fn test_update_share(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        let result = share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        // share was successful?
        assert!(result);

        let result = share::share_item("file", &fileinfo["fileid"], ShareType::Link,
            None, "1");

        // share was successful?
        assert!(result.is_some());

        let items = share::get_item_shared("file", None);

        // make sure that we found a link share and a user share
        assert_eq!(items.len(), 2);

        let mut link_share = None;
        let mut user_share = None;

        for item in items {
            if item["share_type"] == ShareType::Link.to_string() {
                link_share = Some(item);
            }
            if item["share_type"] == ShareType::User.to_string() {
                user_share = Some(item);
            }
        }

        // make sure that we found a link share and a user share
        assert!(link_share.is_some());
        assert!(user_share.is_some());

        let link_share = link_share.unwrap();
        let user_share = user_share.unwrap();

        // update permissions
        assert_eq!(user_share["permissions"], "31");

        let mut params = HashMap::new();
        params.insert("id".to_string(), user_share["id"].clone());
        
        let mut put_params = HashMap::new();
        put_params.insert("permissions".to_string(), "1".to_string());
        params.insert("_put".to_string(), serde_json::to_string(&put_params).unwrap());

        let result = Api::update_share(params).await;

        let meta = result.get_meta();
        assert!(result.succeeded(), "{}", meta["message"]);

        let items = share::get_item_shared("file", &user_share["file_source"]);

        let mut new_user_share = None;
        for item in items {
            if item["share_with"] == TEST_FILES_SHARING_API_USER2 {
                new_user_share = Some(item);
                break;
            }
        }

        assert!(new_user_share.is_some());
        let new_user_share = new_user_share.unwrap();

        assert_eq!(new_user_share["permissions"], "1");

        // update password for link share
        assert!(link_share["share_with"].is_empty());

        let mut params = HashMap::new();
        params.insert("id".to_string(), link_share["id"].clone());
        
        let mut put_params = HashMap::new();
        put_params.insert("password".to_string(), "foo".to_string());
        params.insert("_put".to_string(), serde_json::to_string(&put_params).unwrap());

        let result = Api::update_share(params).await;

        assert!(result.succeeded());

        let items = share::get_item_shared("file", &link_share["file_source"]);

        let mut new_link_share = None;
        for item in items {
            if item["share_type"] == ShareType::Link.to_string() {
                new_link_share = Some(item);
                break;
            }
        }

        assert!(new_link_share.is_some());
        let new_link_share = new_link_share.unwrap();
        
        assert!(!new_link_share["share_with"].is_empty());

        share::unshare("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2);

        share::unshare("file", &fileinfo["fileid"], ShareType::Link, None);
    }

    async fn test_update_share_upload(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.folder);

        let result = share::share_item("folder", &fileinfo["fileid"], ShareType::Link,
            None, "1");

        // share was successful?
        assert!(result.is_some());

        let items = share::get_item_shared("file", None);

        // make sure that we found a link share
        assert_eq!(items.len(), 1);

        let mut link_share = None;

        for item in items {
            if item["share_type"] == ShareType::Link.to_string() {
                link_share = Some(item);
            }
        }

        // make sure that we found a link share
        assert!(link_share.is_some());
        let link_share = link_share.unwrap();

        // update public upload
        let mut params = HashMap::new();
        params.insert("id".to_string(), link_share["id"].clone());
        
        let mut put_params = HashMap::new();
        put_params.insert("publicUpload".to_string(), "true".to_string());
        params.insert("_put".to_string(), serde_json::to_string(&put_params).unwrap());

        let result = Api::update_share(params).await;

        assert!(result.succeeded());

        let items = share::get_item_shared("file", &link_share["file_source"]);

        let mut updated_link_share = None;
        for item in items {
            if item["share_type"] == ShareType::Link.to_string() {
                updated_link_share = Some(item);
                break;
            }
        }

        assert!(updated_link_share.is_some());
        let updated_link_share = updated_link_share.unwrap();
        
        assert_eq!(updated_link_share["permissions"], "7");

        // cleanup
        share::unshare("file", &fileinfo["fileid"], ShareType::Link, None);
    }

    async fn test_delete_share(&mut self) {
        let fileinfo = self.view.lock().unwrap().get_file_info(&self.filename);

        share::share_item("file", &fileinfo["fileid"], ShareType::User,
            TEST_FILES_SHARING_API_USER2, "31");

        share::share_item("file", &fileinfo["fileid"], ShareType::Link,
            None, "1");

        let items = share::get_item_shared("file", None);

        assert_eq!(items.len(), 2);

        for item in items {
            let mut params = HashMap::new();
            params.insert("id".to_string(), item["id"].clone());
            
            let result = Api::delete_share(params).await;

            assert!(result.succeeded());
        }

        let items_after_delete = share::get_item_shared("file", None);

        assert!(items_after_delete.is_empty());
    }
}

#[async_trait]
impl TestCase for TestFilesSharingApi {
    async fn run_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_up();
        
        self.test_create_share().await;
        self.test_get_all_shares().await;
        self.test_get_share_from_source().await;
        self.test_get_share_from_source_with_reshares().await;
        self.test_get_share_from_id().await;
        self.test_get_share_from_folder().await;
        self.test_get_share_from_unknown_id().await;
        self.test_update_share().await;
        self.test_update_share_upload().await;
        self.test_delete_share().await;
        
        self.tear_down();
        Ok(())
    }
}

// Trait for test case execution
#[async_trait]
pub trait TestCase {
    async fn run_tests(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

// Implementation for ShareType to enable string conversion
impl ToString for ShareType {
    fn to_string(&self) -> String {
        match self {
            ShareType::User => "0".to_string(),
            ShareType::Group => "1".to_string(),
            ShareType::Link => "3".to_string(),
            // Other types as needed
            _ => "0".to_string(),
        }
    }
}