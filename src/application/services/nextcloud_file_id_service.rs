use std::sync::Arc;

use crate::common::errors::{DomainError, ErrorKind, Result};
use crate::infrastructure::repositories::pg::NextcloudObjectIdRepository;

#[derive(Clone)]
pub struct NextcloudFileIdService {
    repo: Option<Arc<NextcloudObjectIdRepository>>,
    instance_id: String,
}

impl NextcloudFileIdService {
    pub fn new(repo: Arc<NextcloudObjectIdRepository>, instance_id: String) -> Self {
        Self {
            repo: Some(repo),
            instance_id,
        }
    }

    pub fn new_stub() -> Self {
        Self {
            repo: None,
            instance_id: "ocnca".to_string(),
        }
    }

    pub async fn get_or_create_file_id(&self, file_id: &str) -> Result<i64> {
        let repo = self.repo.as_ref().ok_or_else(|| {
            DomainError::internal_error("NextcloudFileId", "Repository not initialized")
        })?;
        repo.get_or_create("file", file_id).await
    }

    pub async fn get_or_create_folder_id(&self, folder_id: &str) -> Result<i64> {
        let repo = self.repo.as_ref().ok_or_else(|| {
            DomainError::internal_error("NextcloudFileId", "Repository not initialized")
        })?;
        repo.get_or_create("folder", folder_id).await
    }

    /// Get the OxiCloud file UUID from a Nextcloud numeric ID.
    pub async fn get_oxicloud_id(&self, nc_file_id: i64) -> Result<String> {
        let repo = self.repo.as_ref().ok_or_else(|| {
            DomainError::internal_error("NextcloudFileId", "Repository not initialized")
        })?;
        repo.get_object_id(nc_file_id, "file").await
    }

    pub fn format_oc_id(&self, id: i64) -> String {
        format!("{:08}{}", id, self.instance_id)
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    #[cfg(test)]
    pub fn new_test(instance_id: &str) -> Self {
        Self {
            repo: None,
            instance_id: instance_id.to_string(),
        }
    }

    pub fn ensure_ready(&self) -> Result<()> {
        if self.repo.is_none() {
            return Err(DomainError::new(
                ErrorKind::InternalError,
                "NextcloudFileId",
                "Repository not initialized",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_oc_id_default_instance() {
        let svc = NextcloudFileIdService::new_stub();
        assert_eq!(svc.format_oc_id(42), "00000042ocnca");
    }

    #[test]
    fn test_format_oc_id_custom_instance() {
        let svc = NextcloudFileIdService::new_test("myinst");
        assert_eq!(svc.format_oc_id(1), "00000001myinst");
    }

    #[test]
    fn test_format_oc_id_large_number() {
        let svc = NextcloudFileIdService::new_stub();
        assert_eq!(svc.format_oc_id(123456789), "123456789ocnca");
    }

    #[test]
    fn test_instance_id() {
        let svc = NextcloudFileIdService::new_stub();
        assert_eq!(svc.instance_id(), "ocnca");
    }

    #[test]
    fn test_ensure_ready_fails_on_stub() {
        let svc = NextcloudFileIdService::new_stub();
        assert!(svc.ensure_ready().is_err());
    }
}
