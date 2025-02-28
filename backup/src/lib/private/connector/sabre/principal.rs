use std::error::Error;
use std::fmt;

/// Custom error type for Principal operations
#[derive(Debug)]
pub enum PrincipalError {
    PrincipalNotFound,
    NotImplemented(String),
    Other(String),
}

impl fmt::Display for PrincipalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrincipalError::PrincipalNotFound => write!(f, "Principal not found"),
            PrincipalError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            PrincipalError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for PrincipalError {}

/// Result type for Principal operations
pub type PrincipalResult<T> = Result<T, PrincipalError>;

/// Represents a DAV principal
#[derive(Debug, Clone)]
pub struct Principal {
    pub uri: String,
    pub properties: std::collections::HashMap<String, String>,
}

/// Principal backend trait
pub trait PrincipalBackend {
    /// Returns a list of principals based on a prefix.
    ///
    /// This prefix will often contain something like 'principals'. You are only
    /// expected to return principals that are in this base path.
    ///
    /// You are expected to return at least a 'uri' for every user, you can
    /// return any additional properties if you wish so. Common properties are:
    ///   {DAV:}displayname
    fn get_principals_by_prefix(&self, prefix_path: &str) -> Vec<Principal>;

    /// Returns a specific principal, specified by it's path.
    /// The returned structure should be the exact same as from
    /// get_principals_by_prefix.
    fn get_principal_by_path(&self, path: &str) -> Option<Principal>;

    /// Returns the list of members for a group-principal
    fn get_group_member_set(&self, principal: &str) -> PrincipalResult<Vec<String>>;

    /// Returns the list of groups a principal is a member of
    fn get_group_membership(&self, principal: &str) -> PrincipalResult<Vec<String>>;

    /// Updates the list of group members for a group principal.
    ///
    /// The principals should be passed as a list of uri's.
    fn set_group_member_set(&self, principal: &str, members: Vec<String>) -> PrincipalResult<()>;

    /// Update a principal
    fn update_principal(&self, path: &str, mutations: std::collections::HashMap<String, String>) -> PrincipalResult<usize>;

    /// Search for principals
    fn search_principals(&self, prefix_path: &str, search_properties: Vec<String>) -> Vec<String>;
}

/// Implementation of the Principal backend
pub struct OCPrincipal;

impl OCPrincipal {
    pub fn new() -> Self {
        Self {}
    }
}

impl PrincipalBackend for OCPrincipal {
    fn get_principals_by_prefix(&self, prefix_path: &str) -> Vec<Principal> {
        let mut principals = Vec::new();

        if prefix_path == "principals" {
            // Replace with actual user retrieval functionality
            for user in get_users() {
                let user_uri = format!("principals/{}", user);
                let mut properties = std::collections::HashMap::new();
                properties.insert("{DAV:}displayname".to_string(), user.clone());
                
                principals.push(Principal {
                    uri: user_uri,
                    properties,
                });
            }
        }

        principals
    }

    fn get_principal_by_path(&self, path: &str) -> Option<Principal> {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() != 2 {
            return None;
        }

        let prefix = parts[0];
        let name = parts[1];

        if prefix == "principals" && user_exists(name) {
            let mut properties = std::collections::HashMap::new();
            properties.insert("{DAV:}displayname".to_string(), name.to_string());
            
            return Some(Principal {
                uri: format!("principals/{}", name),
                properties,
            });
        }

        None
    }

    fn get_group_member_set(&self, principal: &str) -> PrincipalResult<Vec<String>> {
        // TODO: for now the group principal has only one member, the user itself
        if let Some(principal_data) = self.get_principal_by_path(principal) {
            Ok(vec![principal_data.uri])
        } else {
            Err(PrincipalError::PrincipalNotFound)
        }
    }

    fn get_group_membership(&self, principal: &str) -> PrincipalResult<Vec<String>> {
        let parts: Vec<&str> = principal.split('/').collect();
        if parts.len() < 2 {
            return Err(PrincipalError::Other("Invalid principal format".to_string()));
        }

        let prefix = parts[0];
        let name = parts[1];

        let mut group_membership = Vec::new();
        if prefix == "principals" {
            if self.get_principal_by_path(principal).is_none() {
                return Err(PrincipalError::PrincipalNotFound);
            }

            // TODO: for now the user principal has only its own groups
            group_membership.push(format!("principals/{}/calendar-proxy-read", name));
            group_membership.push(format!("principals/{}/calendar-proxy-write", name));
            // The addressbook groups are not supported in Sabre,
            // see http://groups.google.com/group/sabredav-discuss/browse_thread/thread/ef2fa9759d55f8c#msg_5720afc11602e753
            //group_membership.push(format!("principals/{}/addressbook-proxy-read", name));
            //group_membership.push(format!("principals/{}/addressbook-proxy-write", name));
        }
        
        Ok(group_membership)
    }

    fn set_group_member_set(&self, _principal: &str, _members: Vec<String>) -> PrincipalResult<()> {
        Err(PrincipalError::NotImplemented("Setting members of the group is not supported yet".to_string()))
    }

    fn update_principal(&self, _path: &str, _mutations: std::collections::HashMap<String, String>) -> PrincipalResult<usize> {
        Ok(0)
    }

    fn search_principals(&self, _prefix_path: &str, _search_properties: Vec<String>) -> Vec<String> {
        Vec::new()
    }
}

// Mock functions to simulate the original PHP dependencies
fn get_users() -> Vec<String> {
    // Replace with actual implementation
    vec!["user1".to_string(), "user2".to_string()]
}

fn user_exists(username: &str) -> bool {
    // Replace with actual implementation
    username == "user1" || username == "user2"
}