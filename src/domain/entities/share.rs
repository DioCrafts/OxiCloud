use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// Re-exportar errores de entidad desde el módulo centralizado
pub use super::entity_errors::ShareError;

#[derive(Debug, Clone, PartialEq)]
pub struct Share {
    pub id: String,
    pub item_id: String,
    pub item_type: ShareItemType,
    pub token: String,
    pub password_hash: Option<String>,
    pub expires_at: Option<u64>,
    pub permissions: SharePermissions,
    pub created_at: u64,
    pub created_by: String,
    pub access_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SharePermissions {
    pub read: bool,
    pub write: bool,
    pub reshare: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShareItemType {
    File,
    Folder,
}

impl Share {
    pub fn new(
        item_id: String, 
        item_type: ShareItemType,
        created_by: String,
        permissions: Option<SharePermissions>,
        password_hash: Option<String>,
        expires_at: Option<u64>,
    ) -> Result<Self, ShareError> {
        // Validate item_id
        if item_id.is_empty() {
            return Err(ShareError::ValidationError("Item ID cannot be empty".to_string()));
        }

        // Validate expiration date if provided
        if let Some(expires) = expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            
            if expires <= now {
                return Err(ShareError::InvalidExpiration("Expiration date must be in the future".to_string()));
            }
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            item_id,
            item_type,
            token: Uuid::new_v4().to_string(),
            password_hash,
            expires_at,
            permissions: permissions.unwrap_or(SharePermissions {
                read: true,
                write: false,
                reshare: false,
            }),
            created_at: now,
            created_by,
            access_count: 0,
        })
    }

    pub fn with_permissions(mut self, permissions: SharePermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_password(mut self, password_hash: Option<String>) -> Self {
        self.password_hash = password_hash;
        self
    }

    pub fn with_expiration(mut self, expires_at: Option<u64>) -> Self {
        self.expires_at = expires_at;
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = token;
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            
            return expires_at <= now;
        }
        
        false
    }

    pub fn increment_access_count(mut self) -> Self {
        self.access_count += 1;
        self
    }

    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password_hash {
            Some(hash) => {
                // In a real implementation, use a proper password hashing function like bcrypt
                // For simplicity, we're just comparing strings here
                hash == password
            }
            None => true,
        }
    }
}

impl SharePermissions {
    pub fn new(read: bool, write: bool, reshare: bool) -> Self {
        Self {
            read,
            write,
            reshare,
        }
    }
}

impl ToString for ShareItemType {
    fn to_string(&self) -> String {
        match self {
            ShareItemType::File => "file".to_string(),
            ShareItemType::Folder => "folder".to_string(),
        }
    }
}

impl TryFrom<&str> for ShareItemType {
    type Error = ShareError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "file" => Ok(ShareItemType::File),
            "folder" => Ok(ShareItemType::Folder),
            _ => Err(ShareError::ValidationError(format!("Invalid item type: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_share() {
        let share = Share::new(
            "test_file_id".to_string(),
            ShareItemType::File,
            "user123".to_string(),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(share.item_id, "test_file_id");
        assert_eq!(share.item_type, ShareItemType::File);
        assert_eq!(share.created_by, "user123");
        assert_eq!(share.permissions.read, true);
        assert_eq!(share.permissions.write, false);
        assert_eq!(share.permissions.reshare, false);
        assert!(share.password_hash.is_none());
        assert!(share.expires_at.is_none());
        assert_eq!(share.access_count, 0);
    }

    #[test]
    fn test_share_is_expired() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        // Create a share that expires in the future
        let future = now + 3600; // 1 hour in the future
        let share = Share::new(
            "test_file_id".to_string(),
            ShareItemType::File,
            "user123".to_string(),
            None,
            None,
            Some(future),
        )
        .unwrap();
        
        assert!(!share.is_expired());
        
        // Test with past expiration (should fail during creation)
        let past = now - 3600; // 1 hour in the past
        let share_result = Share::new(
            "test_file_id".to_string(),
            ShareItemType::File,
            "user123".to_string(),
            None,
            None,
            Some(past),
        );
        
        assert!(share_result.is_err());
    }
    
    #[test]
    fn test_share_item_type_conversion() {
        assert_eq!(ShareItemType::File.to_string(), "file");
        assert_eq!(ShareItemType::Folder.to_string(), "folder");
        
        assert_eq!(ShareItemType::try_from("file").unwrap(), ShareItemType::File);
        assert_eq!(ShareItemType::try_from("folder").unwrap(), ShareItemType::Folder);
        assert_eq!(ShareItemType::try_from("FILE").unwrap(), ShareItemType::File);
        assert!(ShareItemType::try_from("invalid").is_err());
    }
}
