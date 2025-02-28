/*
 * Copyright (c) 2012 Thomas Müller <thomas.mueller@tmit.eu>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::lib::base;
use crate::lib::app::OcApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    base::init()?;

    OcApp::enable("files_sharing")?;
    OcApp::enable("files_encryption")?;
    OcApp::enable("calendar")?;
    OcApp::enable("contacts")?;
    OcApp::enable("apptemplateadvanced")?;
    OcApp::enable("appframework")?;
    // OcApp::enable("files_archive")?;
    // OcApp::enable("mozilla_sync")?;
    // OcApp::enable("news")?;
    // OcApp::enable("provisioning_api")?;
    // OcApp::enable("user_external")?;

    Ok(())
}