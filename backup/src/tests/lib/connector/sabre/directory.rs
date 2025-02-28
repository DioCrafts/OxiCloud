/*
 * Copyright (c) 2013 Thomas Müller <thomas.mueller@tmit.eu>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use crate::connector::sabre::directory::OcConnectorSabreDirectory;
use crate::sabre::dav::exception::forbidden::SabreDavExceptionForbidden;

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::error::Error;
    use tokio::test;

    struct MockOcConnectorSabreDirectory {
        path: String,
    }

    #[async_trait]
    impl OcConnectorSabreDirectory for MockOcConnectorSabreDirectory {
        fn new(path: &str) -> Self {
            Self {
                path: path.to_string(),
            }
        }

        async fn create_file(&self, name: &str) -> Result<(), Box<dyn Error>> {
            if name == "Shared" {
                return Err(Box::new(SabreDavExceptionForbidden::new(
                    "Cannot create Shared file",
                )));
            }
            Ok(())
        }

        async fn create_directory(&self, name: &str) -> Result<(), Box<dyn Error>> {
            if name == "Shared" {
                return Err(Box::new(SabreDavExceptionForbidden::new(
                    "Cannot create Shared directory",
                )));
            }
            Ok(())
        }

        async fn delete(&self) -> Result<(), Box<dyn Error>> {
            if self.path == "Shared" {
                return Err(Box::new(SabreDavExceptionForbidden::new(
                    "Cannot delete Shared directory",
                )));
            }
            Ok(())
        }
    }

    #[test]
    async fn test_create_shared_file_fails() {
        let dir = MockOcConnectorSabreDirectory::new("");
        let result = dir.create_file("Shared").await;
        assert!(result.is_err());
        
        // Verify it's the right error type
        let error = result.unwrap_err();
        assert!(error.downcast_ref::<SabreDavExceptionForbidden>().is_some());
    }

    #[test]
    async fn test_create_shared_folder_fails() {
        let dir = MockOcConnectorSabreDirectory::new("");
        let result = dir.create_directory("Shared").await;
        assert!(result.is_err());
        
        // Verify it's the right error type
        let error = result.unwrap_err();
        assert!(error.downcast_ref::<SabreDavExceptionForbidden>().is_some());
    }

    #[test]
    async fn test_delete_shared_folder_fails() {
        let dir = MockOcConnectorSabreDirectory::new("Shared");
        let result = dir.delete().await;
        assert!(result.is_err());
        
        // Verify it's the right error type
        let error = result.unwrap_err();
        assert!(error.downcast_ref::<SabreDavExceptionForbidden>().is_some());
    }
}