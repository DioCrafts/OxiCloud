// Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use clap::{App, AppSettings};
use nextcloud_app_files::command::Scan;
use nextcloud_app_lib::user::UserManager;

pub fn register_command(app: &mut App) -> &mut App {
    let user_manager = UserManager::get_instance();
    
    app.add_subcommand(Scan::new(user_manager).into())
        .setting(AppSettings::SubcommandRequiredElseHelp)
}