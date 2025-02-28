// Copyright (C) 2012 Robin Appelman <icewind@owncloud.com>
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

use std::path::PathBuf;
use anyhow::Result;

use crate::test_cache::TestCache;
use crate::cache::file::FileCache as OCFileCache;
use crate::files::{
    filesystem::Filesystem,
    storage::temporary::Temporary,
    view::View
};
use crate::config::Config;
use crate::user::{
    User,
    backend::dummy::Dummy as DummyBackend
};
use crate::file_proxy::FileProxy;
use crate::hook::Hook;

pub struct FileCache {
    user: String,
    datadir: PathBuf,
    instance: OCFileCache,
}

impl TestCache for FileCache {
    fn skip(&self) {
        // This was commented out in original code
        // self.skip_unless(User::is_logged_in());
    }

    fn set_up(&mut self) -> Result<()> {
        // Clear all proxies and hooks for clean testing
        FileProxy::clear_proxies()?;
        Hook::clear("OC_Filesystem")?;

        // Disabled encryption hook code
        // if App::is_enabled("files_encryption") {
        //     FileProxy::register(Box::new(FileProxyEncryption::new()))?;
        // }

        // Set up temporary storage
        Filesystem::clear_mounts()?;
        let storage = Temporary::new()?;
        Filesystem::mount(storage.clone(), &[], "/")?;
        let datadir = storage.get_id().replace("local::", "");
        
        self.datadir = Config::get_value("datadirectory")
            .unwrap_or_else(|_| PathBuf::from(Config::server_root()).join("data"));
        Config::set_value("datadirectory", datadir.clone())?;

        // Set up user authentication
        User::clear_backends()?;
        User::use_backend(Box::new(DummyBackend::new()))?;
        
        // Login
        User::create_user("test", "test")?;
        
        self.user = User::get_user()?;
        User::set_user_id("test")?;

        // Set up user directory
        let root_view = View::new("")?;
        root_view.mkdir("/test")?;
        
        // Create cache instance
        self.instance = OCFileCache::new()?;
        
        Ok(())
    }

    fn tear_down(&mut self) -> Result<()> {
        User::set_user_id(&self.user)?;
        Config::set_value("datadirectory", self.datadir.to_string_lossy().to_string())?;
        Ok(())
    }
}

impl FileCache {
    pub fn new() -> Result<Self> {
        Ok(Self {
            user: String::new(),
            datadir: PathBuf::new(),
            instance: OCFileCache::new()?,
        })
    }
}