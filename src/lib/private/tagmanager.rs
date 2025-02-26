//! Factory module creating instances of ITags

use std::sync::Arc;

/// Interface for tag managers
pub trait ITagManager {
    /// Create a new ITags instance and load tags from db.
    ///
    /// # Arguments
    /// * `type_name` - The type identifier e.g. 'contact' or 'event'.
    /// * `default_tags` - An array of default tags to be used if none are stored.
    ///
    /// # Returns
    /// An Arc<dyn ITags> implementation
    fn load(&self, type_name: &str, default_tags: Vec<String>) -> Arc<dyn ITags>;
}

/// Interface for tags
pub trait ITags: Send + Sync {
    // ITags methods would go here
    // These methods aren't specified in the original code
}

/// Factory class creating instances of ITags
///
/// A tag can be e.g. 'Family', 'Work', 'Chore', 'Special Occation' or
/// anything else that is either parsed from a vobject or that the user chooses
/// to add.
/// Tag names are not case-sensitive, but will be saved with the case they
/// are entered in. If a user already has a tag 'family' for a type, and
/// tries to add a tag named 'Family' it will be silently ignored.
pub struct TagManager {
    /// User whose data the object will operate on
    user: String,
}

impl TagManager {
    /// Constructor.
    ///
    /// # Arguments
    /// * `user` - The user whose data the object will operate on.
    pub fn new(user: String) -> Self {
        Self { user }
    }
}

impl ITagManager for TagManager {
    fn load(&self, type_name: &str, default_tags: Vec<String>) -> Arc<dyn ITags> {
        Arc::new(Tags::new(self.user.clone(), type_name.to_string(), default_tags))
    }
}

/// Implementation of the ITags trait
pub struct Tags {
    user: String,
    type_name: String,
    default_tags: Vec<String>,
}

impl Tags {
    /// Create a new Tags instance
    ///
    /// # Arguments
    /// * `user` - The user whose data the object will operate on
    /// * `type_name` - The type identifier e.g. 'contact' or 'event'
    /// * `default_tags` - Default tags to be used if none are stored
    pub fn new(user: String, type_name: String, default_tags: Vec<String>) -> Self {
        Self {
            user,
            type_name,
            default_tags,
        }
    }
}

impl ITags for Tags {
    // Implementation of ITags methods would go here
}