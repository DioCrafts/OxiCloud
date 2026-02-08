use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// Re-exportar errores de entidad desde el módulo centralizado
pub use super::entity_errors::ShareError;

#[derive(Debug, Clone, PartialEq)]
pub struct Share {
    id: String,
    item_id: String,
    item_type: ShareItemType,
    token: String,
    password_hash: Option<String>,
    expires_at: Option<u64>,
    permissions: SharePermissions,
    created_at: u64,
    created_by: String,
    access_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SharePermissions {
    read: bool,
    write: bool,
    reshare: bool,
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

    /// Reconstruct a Share from persisted data (e.g. filesystem/database).
    /// Skips validation and ID generation — uses the provided values directly.
    pub fn from_raw(
        id: String,
        item_id: String,
        item_type: ShareItemType,
        token: String,
        password_hash: Option<String>,
        expires_at: Option<u64>,
        permissions: SharePermissions,
        created_at: u64,
        created_by: String,
        access_count: u64,
    ) -> Self {
        Self {
            id,
            item_id,
            item_type,
            token,
            password_hash,
            expires_at,
            permissions,
            created_at,
            created_by,
            access_count,
        }
    }

    // ── Getters ──

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    pub fn item_type(&self) -> &ShareItemType {
        &self.item_type
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }

    pub fn permissions(&self) -> &SharePermissions {
        &self.permissions
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    pub fn access_count(&self) -> u64 {
        self.access_count
    }

    // ── Builder-style modifiers (immutable) ──

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

    /// Returns whether this share requires a password to access.
    pub fn has_password(&self) -> bool {
        self.password_hash.is_some()
    }

    /// Returns a reference to the password hash, if one is set.
    /// 
    /// Password verification should be performed externally via PasswordHasherPort
    /// to keep cryptographic dependencies out of the domain layer.
    pub fn password_hash(&self) -> Option<&str> {
        self.password_hash.as_deref()
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

    pub fn read(&self) -> bool {
        self.read
    }

    pub fn write(&self) -> bool {
        self.write
    }

    pub fn reshare(&self) -> bool {
        self.reshare
    }
}

impl std::fmt::Display for ShareItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShareItemType::File => write!(f, "file"),
            ShareItemType::Folder => write!(f, "folder"),
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

        assert_eq!(share.item_id(), "test_file_id");
        assert_eq!(*share.item_type(), ShareItemType::File);
        assert_eq!(share.created_by(), "user123");
        assert_eq!(share.permissions().read(), true);
        assert_eq!(share.permissions().write(), false);
        assert_eq!(share.permissions().reshare(), false);
        assert!(!share.has_password());
        assert!(share.expires_at().is_none());
        assert_eq!(share.access_count(), 0);
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
    
    #[test]
    fn test_has_password_with_hash() {
        let share = Share::new(
            "test_file_id".to_string(),
            ShareItemType::File,
            "user123".to_string(),
            None,
            Some("some_hash_value".to_string()),
            None,
        )
        .unwrap();
        
        assert!(share.has_password());
        assert_eq!(share.password_hash(), Some("some_hash_value"));
    }
    
    #[test]
    fn test_has_password_without_hash() {
        let share = Share::new(
            "test_file_id".to_string(),
            ShareItemType::File,
            "user123".to_string(),
            None,
            None, // No password
            None,
        )
        .unwrap();
        
        assert!(!share.has_password());
        assert_eq!(share.password_hash(), None);
    }
}
