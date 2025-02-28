/*
 * Copyright (c) 2013 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::core::command::{Status, Db, Upgrade};

pub fn register_commands(application: &mut impl Application) {
    application.add(Box::new(Status::new()));
    application.add(Box::new(Db::GenerateChangeScript::new()));
    application.add(Box::new(Upgrade::new()));
}

pub trait Application {
    fn add(&mut self, command: Box<dyn Command>);
}

pub trait Command {
    // Command trait methods would be defined here
}