// Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use async_trait::async_trait;
use mockall::{mock, predicate::*};
use std::error::Error;
use std::fmt;

// Custom error types equivalent to PHP exceptions
#[derive(Debug)]
struct SabreDavException(String);

impl fmt::Display for SabreDavException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SabreDavException: {}", self.0)
    }
}

impl Error for SabreDavException {}

#[derive(Debug)]
struct SabreDavExceptionForbidden(String);

impl fmt::Display for SabreDavExceptionForbidden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SabreDavExceptionForbidden: {}", self.0)
    }
}

impl Error for SabreDavExceptionForbidden {}

// Mock FileView trait
#[async_trait]
trait FilesView {
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<usize, Box<dyn Error>>;
    async fn rename(&self, old_path: &str, new_path: &str) -> Result<bool, Box<dyn Error>>;
}

mock! {
    FilesView {}

    #[async_trait]
    impl FilesView for FilesView {
        async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<usize, Box<dyn Error>>;
        async fn rename(&self, old_path: &str, new_path: &str) -> Result<bool, Box<dyn Error>>;
    }
}

// The actual File implementation
struct OCConnectorSabreFile {
    path: String,
    file_view: Option<Box<dyn FilesView>>,
}

impl OCConnectorSabreFile {
    fn new(path: &str) -> Self {
        OCConnectorSabreFile {
            path: path.to_string(),
            file_view: None,
        }
    }

    async fn put(&self, data: &str) -> Result<String, Box<dyn Error>> {
        let file_view = self.file_view.as_ref().ok_or_else(|| {
            SabreDavException("FileView not initialized".to_string())
        })?;

        let result = file_view.file_put_contents(&self.path, data.as_bytes()).await?;
        if result == 0 {
            return Err(Box::new(SabreDavException("Could not write file".to_string())));
        }

        let rename_result = file_view.rename(&format!("{}.part", self.path), &self.path).await?;
        if !rename_result {
            return Err(Box::new(SabreDavException("Could not rename part file".to_string())));
        }

        // In a real implementation, we'd generate the etag here
        Ok("generated-etag".to_string())
    }

    async fn delete(&self) -> Result<(), Box<dyn Error>> {
        if self.path == "Shared" {
            return Err(Box::new(SabreDavExceptionForbidden(
                "Cannot delete Shared folder".to_string(),
            )));
        }
        
        // Actual deletion logic would go here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_simple_put_fails() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // setup
            let mut mock_view = MockFilesView::new();
            mock_view
                .expect_file_put_contents()
                .returning(|_, _| Ok(0));

            let mut file = OCConnectorSabreFile::new("/test.txt");
            file.file_view = Some(Box::new(mock_view));

            // action & assert
            let result = file.put("test data").await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.is::<SabreDavException>());
        });
    }

    #[test]
    fn test_simple_put_fails_on_rename() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // setup
            let mut mock_view = MockFilesView::new();
            mock_view
                .expect_file_put_contents()
                .returning(|_, _| Ok(10)); // Return success (10 bytes written)
            mock_view
                .expect_rename()
                .returning(|_, _| Ok(false)); // Return failure on rename

            let mut file = OCConnectorSabreFile::new("/test.txt");
            file.file_view = Some(Box::new(mock_view));

            // action & assert
            let result = file.put("test data").await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.is::<SabreDavException>());
        });
    }

    #[test]
    fn test_delete_shared_fails() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // setup
            let file = OCConnectorSabreFile::new("Shared");

            // action & assert
            let result = file.delete().await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.is::<SabreDavExceptionForbidden>());
        });
    }
}