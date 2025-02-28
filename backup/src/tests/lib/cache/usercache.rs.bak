// Copyright (c) 2012 Robin Appelman <icewind@owncloud.com>
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

use crate::cache::test_cache::TestCache;
use crate::files::{
    file_proxy::FileProxy,
    filesystem::{self, FilesystemMounts},
    hook::Hook,
    storage::temporary::TemporaryStorage,
    view::View,
};
use crate::user::{dummy::DummyUserBackend, User};
use crate::{cache::user_cache::UserCache, config::Config, SERVERROOT};
use std::path::PathBuf;

pub struct UserCacheTest {
    user: Option<String>,
    datadir: Option<PathBuf>,
    instance: Option<UserCache>,
}

impl TestCache for UserCacheTest {}

impl UserCacheTest {
    pub fn new() -> Self {
        Self {
            user: None,
            datadir: None,
            instance: None,
        }
    }

    pub async fn set_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear all proxies and hooks for clean testing
        FileProxy::clear_proxies();
        Hook::clear("OC_Filesystem");

        // Disabled for now
        // Enable only the encryption hook if needed
        // if App::is_enabled("files_encryption") {
        //     FileProxy::register(Box::new(EncryptionFileProxy::new()));
        // }

        // Set up temporary storage
        FilesystemMounts::clear_mounts();
        let storage = TemporaryStorage::new()?;
        filesystem::mount(&storage, &[], "/")?;
        
        let datadir = storage.get_id().replace("local::", "");
        self.datadir = Some(Config::get_value("datadirectory", PathBuf::from(SERVERROOT).join("data")));
        Config::set_value("datadirectory", PathBuf::from(&datadir));

        // Configure user backend
        User::clear_backends();
        User::use_backend(Box::new(DummyUserBackend::new()));
        
        // Login
        User::create_user("test", "test")?;
        
        self.user = Some(User::get_user()?);
        User::set_user_id("test")?;

        // Set up user's directory
        let root_view = View::new("");
        root_view.mkdir("/test")?;
        
        self.instance = Some(UserCache::new());
        
        Ok(())
    }

    pub async fn tear_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(user) = &self.user {
            User::set_user_id(user)?;
        }
        
        if let Some(datadir) = &self.datadir {
            Config::set_value("datadirectory", datadir);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_cache() -> Result<(), Box<dyn std::error::Error>> {
        let mut test = UserCacheTest::new();
        test.set_up().await?;
        
        // Run actual tests here
        
        test.tear_down().await?;
        Ok(())
    }
}