// Copyright (c) 2013 Bjoern Schiessle <schiessle@owncloud.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library. If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;

use crate::files::share;
use crate::user::{User, UserBackend};
use crate::hook::Hook;
use crate::app::App;
use crate::filesystem::{FilesystemView, Filesystem};
use crate::util::Util;
use crate::db::DB;

/// Base trait for sharing tests.
#[async_trait]
pub trait FileSharingTestBase: Send + Sync {
    const TEST_FILES_SHARING_API_USER1: &'static str = "test-share-user1";
    const TEST_FILES_SHARING_API_USER2: &'static str = "test-share-user2";
    const TEST_FILES_SHARING_API_USER3: &'static str = "test-share-user3";

    fn state_files_encryption(&self) -> bool;
    fn filename(&self) -> Option<String>;
    fn data(&self) -> String;
    fn view(&self) -> Option<Arc<FilesystemView>>;
    fn folder(&self) -> Option<String>;

    async fn set_up_before_class() {
        // reset backend
        User::clear_backends().await;
        User::use_backend("database").await;

        // clear share hooks
        Hook::clear("OCP\\Share").await;
        Hook::register_share_hooks().await;
        Util::connect_hook("OC_Filesystem", "setup", "\\OC\\Files\\Storage\\Shared", "setup").await;

        // create users
        Self::login_helper(Self::TEST_FILES_SHARING_API_USER1, true, None).await;
        Self::login_helper(Self::TEST_FILES_SHARING_API_USER2, true, None).await;
        Self::login_helper(Self::TEST_FILES_SHARING_API_USER3, true, None).await;
    }

    async fn set_up(&mut self);
    
    async fn tear_down(&self) {
        // reset app files_encryption
        if self.state_files_encryption() {
            App::enable("files_encryption").await.unwrap();
        } else {
            App::disable("files_encryption").await.unwrap();
        }
    }

    async fn tear_down_after_class() {
        // cleanup users
        User::delete_user(Self::TEST_FILES_SHARING_API_USER1).await.unwrap();
        User::delete_user(Self::TEST_FILES_SHARING_API_USER2).await.unwrap();
        User::delete_user(Self::TEST_FILES_SHARING_API_USER3).await.unwrap();
    }

    /// Helper function to login a user
    ///
    /// # Arguments
    ///
    /// * `user` - Username
    /// * `create` - Whether to create the user
    /// * `password` - Optional password (defaults to username)
    async fn login_helper(user: &str, create: bool, password: Option<&str>) {
        if create {
            User::create_user(user, user).await.unwrap();
        }

        let password = password.unwrap_or(user);

        Util::tear_down_fs().await;
        User::set_user_id("").await;
        Filesystem::tear_down().await;
        Util::setup_fs(user).await;
        User::set_user_id(user).await;

        let params = [
            ("uid", user.to_string()),
            ("password", password.to_string()),
        ].iter().cloned().collect();
        
        // Additional login logic would go here
    }

    /// Get some information from a given share
    ///
    /// # Arguments
    ///
    /// * `share_id` - The ID of the share
    ///
    /// # Returns
    ///
    /// A `Result` containing share information with fields:
    /// item_source, share_type, share_with, item_type, permissions
    async fn get_share_from_id(&self, share_id: i64) -> Option<share::ShareInfo> {
        let sql = "SELECT `item_source`, `share_type`, `share_with`, `item_type`, `permissions` FROM `*PREFIX*share` WHERE `id` = ?";
        let args = vec![share_id.to_string()];
        let query = DB::prepare(sql).await.unwrap();
        let result = query.execute(&args).await.unwrap();

        if result.num_rows() > 0 {
            let row = result.fetch_row().await.unwrap();
            Some(share::ShareInfo {
                item_source: row.get("item_source").unwrap(),
                share_type: row.get("share_type").unwrap(),
                share_with: row.get("share_with").unwrap(),
                item_type: row.get("item_type").unwrap(),
                permissions: row.get("permissions").unwrap(),
            })
        } else {
            None
        }
    }
}

#[automock]
pub struct TestFilesSharingBase {
    state_files_encryption: bool,
    filename: Option<String>,
    data: String,
    view: Option<Arc<FilesystemView>>,
    folder: Option<String>,
}

#[async_trait]
impl FileSharingTestBase for TestFilesSharingBase {
    fn state_files_encryption(&self) -> bool {
        self.state_files_encryption
    }

    fn filename(&self) -> Option<String> {
        self.filename.clone()
    }

    fn data(&self) -> String {
        self.data.clone()
    }

    fn view(&self) -> Option<Arc<FilesystemView>> {
        self.view.clone()
    }

    fn folder(&self) -> Option<String> {
        self.folder.clone()
    }

    async fn set_up(&mut self) {
        // login as user1
        Self::login_helper(Self::TEST_FILES_SHARING_API_USER1, false, None).await;

        self.data = "foobar".to_string();
        let path = format!("/{}/files", Self::TEST_FILES_SHARING_API_USER1);
        self.view = Some(Arc::new(FilesystemView::new(&path)));
        
        // remember files_encryption state
        self.state_files_encryption = App::is_enabled("files_encryption").await;

        // we don't want to test with app files_encryption enabled
        App::disable("files_encryption").await.unwrap();

        assert!(!App::is_enabled("files_encryption").await);
    }
}