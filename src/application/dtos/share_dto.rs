use serde::{Deserialize, Serialize};

use crate::domain::entities::share::{Share, SharePermissions};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareDto {
    pub id: String,
    pub item_id: String,
    pub item_name: Option<String>,
    pub item_type: String,
    pub token: String,
    pub url: String,
    pub has_password: bool,
    pub expires_at: Option<u64>,
    pub permissions: SharePermissionsDto,
    pub created_at: u64,
    pub created_by: String,
    pub access_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharePermissionsDto {
    pub read: bool,
    pub write: bool,
    pub reshare: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareDto {
    pub item_id: String,
    pub item_name: Option<String>,
    pub item_type: String,
    pub password: Option<String>,
    pub expires_at: Option<u64>,
    pub permissions: Option<SharePermissionsDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShareDto {
    pub password: Option<String>,
    pub expires_at: Option<u64>,
    pub permissions: Option<SharePermissionsDto>,
}

/// Extension methods to convert between DTOs and domain entities
impl ShareDto {
    pub fn from_entity(share: &Share, base_url: &str) -> Self {
        let url = format!("{}/s/{}", base_url, share.token());

        Self {
            id: share.id().to_string(),
            item_id: share.item_id().to_string(),
            item_name: share.item_name().map(|s| s.to_string()),
            item_type: share.item_type().to_string(),
            token: share.token().to_string(),
            url,
            has_password: share.has_password(),
            expires_at: share.expires_at(),
            permissions: SharePermissionsDto::from_entity(share.permissions()),
            created_at: share.created_at(),
            created_by: share.created_by().to_string(),
            access_count: share.access_count(),
        }
    }
}

impl SharePermissionsDto {
    pub fn from_entity(permissions: &SharePermissions) -> Self {
        Self {
            read: permissions.read(),
            write: permissions.write(),
            reshare: permissions.reshare(),
        }
    }

    pub fn to_entity(&self) -> SharePermissions {
        SharePermissions::new(self.read, self.write, self.reshare)
    }
}
