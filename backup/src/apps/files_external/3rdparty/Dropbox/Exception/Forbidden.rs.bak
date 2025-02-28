/// Dropbox Forbidden exception
///
/// This exception is thrown when we receive the 403 forbidden response
#[derive(Debug)]
pub struct ForbiddenError {
    message: String,
    // Additional fields can be added as needed
}

impl ForbiddenError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        ForbiddenError {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ForbiddenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forbidden: {}", self.message)
    }
}

impl std::error::Error for ForbiddenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Base Dropbox error that ForbiddenError extends
#[derive(Debug)]
pub struct DropboxError {
    message: String,
}

impl DropboxError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        DropboxError {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DropboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dropbox Error: {}", self.message)
    }
}

impl std::error::Error for DropboxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<DropboxError> for ForbiddenError {
    fn from(error: DropboxError) -> Self {
        ForbiddenError::new(error.message)
    }
}