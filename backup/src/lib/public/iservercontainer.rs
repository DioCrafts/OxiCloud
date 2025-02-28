// Copyright 2013 Thomas Müller deepdiver@owncloud.com
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

// Public interface of ownCloud for apps to use.
// Server container interface

use std::sync::Arc;

/// Interface definitions for core traits
pub mod contacts {
    /// Manager for contacts information
    pub trait IManager: Send + Sync {}
}

/// Request interface
pub trait IRequest: Send + Sync {}

/// Preview manager interface
pub trait IPreview: Send + Sync {}

/// Tag manager interface
pub trait ITagManager: Send + Sync {
    /// Load tags for a specific object type
    fn load(&self);
}

/// Files namespace
pub mod files {
    /// Folder interface
    pub trait Folder: Send + Sync {}
}

/// User session interface
pub trait IUserSession: Send + Sync {}

/// Navigation manager interface
pub trait INavigationManager: Send + Sync {}

/// Configuration interface
pub trait IConfig: Send + Sync {}

/// Localization interface
pub trait IL10N: Send + Sync {}

/// URL generator interface
pub trait IURLGenerator: Send + Sync {}

/// Helper interface
pub trait IHelper: Send + Sync {}

/// Cache interface
pub trait ICache: Send + Sync {}

/// Session interface
pub trait ISession: Send + Sync {}

/// Activity namespace
pub mod activity {
    /// Activity manager interface
    pub trait IManager: Send + Sync {}
}

/// Database connection interface
pub trait IDBConnection: Send + Sync {}

/// This container holds all ownCloud services
pub trait IServerContainer: Send + Sync {
    /// The contacts manager will act as a broker between consumers for contacts information and
    /// providers which actual deliver the contact information.
    fn get_contacts_manager(&self) -> Arc<dyn contacts::IManager>;

    /// The current request object holding all information about the request currently being processed
    /// is returned from this method.
    /// In case the current execution was not initiated by a web request None is returned
    fn get_request(&self) -> Option<Arc<dyn IRequest>>;

    /// Returns the preview manager which can create preview images for a given file
    fn get_preview_manager(&self) -> Arc<dyn IPreview>;

    /// Returns the tag manager which can get and set tags for different object types
    ///
    /// See ITagManager::load()
    fn get_tag_manager(&self) -> Arc<dyn ITagManager>;

    /// Returns the root folder of ownCloud's data directory
    fn get_root_folder(&self) -> Arc<dyn files::Folder>;

    /// Returns a view to ownCloud's files folder
    fn get_user_folder(&self) -> Arc<dyn files::Folder>;

    /// Returns an app-specific view in ownClouds data directory
    fn get_app_folder(&self) -> Arc<dyn files::Folder>;

    /// Returns the user session
    fn get_user_session(&self) -> Arc<dyn IUserSession>;

    /// Returns the navigation manager
    fn get_navigation_manager(&self) -> Arc<dyn INavigationManager>;

    /// Returns the configuration
    fn get_config(&self) -> Arc<dyn IConfig>;

    /// get an L10N instance
    fn get_l10n(&self, app: &str) -> Arc<dyn IL10N>;

    /// Returns the URL generator
    fn get_url_generator(&self) -> Arc<dyn IURLGenerator>;

    /// Returns the helper
    fn get_helper(&self) -> Arc<dyn IHelper>;

    /// Returns an ICache instance
    fn get_cache(&self) -> Arc<dyn ICache>;

    /// Returns the current session
    fn get_session(&self) -> Arc<dyn ISession>;

    /// Returns the activity manager
    fn get_activity_manager(&self) -> Arc<dyn activity::IManager>;

    /// Returns the database connection
    fn get_database_connection(&self) -> Arc<dyn IDBConnection>;
}