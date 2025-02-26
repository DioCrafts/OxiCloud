/// A result of a search
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub text: String,
    pub link: String,
    pub type_: String,
    pub container: String,
}

impl SearchResult {
    /// Create a new search result
    ///
    /// # Arguments
    ///
    /// * `name` - Short name for the result
    /// * `text` - Some more information about the result
    /// * `link` - Link for the result
    /// * `type_` - The type of result as human readable string ('File', 'Music', etc)
    /// * `container` - The container of the result
    pub fn new(name: String, text: String, link: String, type_: String, container: String) -> Self {
        Self {
            name,
            text,
            link,
            type_,
            container,
        }
    }
}