// ownCloud - LDAP LDAPUtility
//
// @author Arthur Schiwon
// @copyright 2013 Arthur Schiwon blizzz@owncloud.com
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

use std::sync::Arc;

/// Interface for LDAP wrapper
pub trait ILdapWrapper: Send + Sync {
    // Interface methods would be defined here
}

/// Abstract utility class for LDAP operations
pub struct LdapUtility {
    ldap: Arc<dyn ILdapWrapper>,
}

impl LdapUtility {
    /// Constructor for LdapUtility
    /// 
    /// # Arguments
    /// * `ldap_wrapper` - An implementation of the ILdapWrapper trait
    pub fn new(ldap_wrapper: Arc<dyn ILdapWrapper>) -> Self {
        Self {
            ldap: ldap_wrapper,
        }
    }
}

// Trait to be implemented by subclasses of LdapUtility
pub trait LdapUtilityTrait {
    fn ldap(&self) -> &Arc<dyn ILdapWrapper>;
}

impl LdapUtilityTrait for LdapUtility {
    fn ldap(&self) -> &Arc<dyn ILdapWrapper> {
        &self.ldap
    }
}