use sabre_dav::Exception as SabreDAVException;

/// Entity Too Large
///
/// This exception is thrown whenever a user tries to upload a file which exceeds hard limitations
#[derive(Debug)]
pub struct EntityTooLarge {
    // Inherited fields from SabreDAVException
    inner: Box<dyn SabreDAVException>,
}

impl EntityTooLarge {
    /// Creates a new EntityTooLarge exception
    pub fn new(message: &str) -> Self {
        Self {
            inner: Box::new(sabre_dav::Exception::new(message)),
        }
    }

    /// Returns the HTTP status code for this exception
    pub fn get_http_code(&self) -> u16 {
        413
    }
}

impl std::fmt::Display for EntityTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity Too Large: {}", self.inner)
    }
}

impl std::error::Error for EntityTooLarge {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl SabreDAVException for EntityTooLarge {
    fn get_http_code(&self) -> u16 {
        self.get_http_code()
    }
    
    // Implement other required methods from the SabreDAVException trait
}