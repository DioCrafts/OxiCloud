use sabre_dav::exception::DavException;

/// Unsupported Media Type
///
/// This exception is thrown whenever a user tries to upload a file which holds content which is not allowed
///
#[derive(Debug)]
pub struct UnsupportedMediaType {
    message: String,
    code: Option<i32>,
}

impl UnsupportedMediaType {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    /// Returns the HTTP status code for this exception
    ///
    /// @return int
    pub fn get_http_code(&self) -> u16 {
        415
    }
}

impl DavException for UnsupportedMediaType {
    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_code(&self) -> Option<i32> {
        self.code
    }
}

impl std::fmt::Display for UnsupportedMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UnsupportedMediaType {}