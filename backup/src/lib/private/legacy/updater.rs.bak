/*
 * Copyright (c) 2013 Robin Appelman <icewind@owncloud.com>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::updater::Updater;
use anyhow::Result;

pub struct OcUpdater;

impl OcUpdater {
    pub async fn check() -> Result<bool> {
        let updater = Updater::new();
        updater.check("http://apps.owncloud.com/updater.php").await
    }
}