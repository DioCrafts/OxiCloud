// ownCloud
//
// @author Jakob Sack
// @copyright 2011 Jakob Sack kde@jakobsack.de
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
use sabre_dav::{
    auth::{AbstractBasicAuth, AuthBackend},
    Server,
};

/// Sabre authentication backend for ownCloud
#[derive(Default)]
pub struct ConnectorSabreAuth {
    current_user: Option<String>,
}

impl ConnectorSabreAuth {
    /// Create a new authentication backend instance
    pub fn new() -> Self {
        Self {
            current_user: None,
        }
    }

    /// Validates a username and password
    ///
    /// This method should return true or false depending on if login
    /// succeeded.
    async fn validate_user_pass(&mut self, username: &str, password: &str) -> bool {
        if oc_user::is_logged_in() {
            let user = oc_user::get_user().expect("User should be available when logged in");
            oc_util::setup_fs(&user).await;
            true
        } else {
            // Login hooks may need early access to the filesystem
            oc_util::set_up_fs().await;
            
            if oc_user::login(username, password).await {
                let user = oc_user::get_user().expect("User should be available after login");
                oc_util::set_up_fs(&user).await;
                true
            } else {
                false
            }
        }
    }
}

#[async_trait]
impl AbstractBasicAuth for ConnectorSabreAuth {
    async fn validate_user_pass(&mut self, username: &str, password: &str) -> bool {
        self.validate_user_pass(username, password).await
    }
}

#[async_trait]
impl AuthBackend for ConnectorSabreAuth {
    /// Returns information about the currently logged in username.
    ///
    /// If nobody is currently logged in, this method should return None.
    async fn get_current_user(&self) -> Option<String> {
        oc_user::get_user()
    }

    /// Override function here. We want to cache authentication cookies
    /// in the syncing client to avoid HTTP-401 roundtrips.
    /// If the sync client supplies the cookies, then OC_User::is_logged_in()
    /// will return true and we can see this WebDAV request as already authenticated,
    /// even if there are no HTTP Basic Auth headers.
    /// In other case, just fallback to the parent implementation.
    async fn authenticate(&mut self, server: &Server, realm: &str) -> bool {
        if oc_user::handle_apache_auth().await || oc_user::is_logged_in() {
            if let Some(user) = oc_user::get_user() {
                oc_util::setup_fs(&user).await;
                self.current_user = Some(user);
                return true;
            }
        }

        // Call to parent implementation (AbstractBasicAuth)
        <Self as AbstractBasicAuth>::authenticate(self, server, realm).await
    }
}