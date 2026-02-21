use crate::application::dtos::user_dto::{
    AuthResponseDto, ChangePasswordDto, LoginDto, RefreshTokenDto, RegisterDto, UserDto,
};
use crate::application::ports::auth_ports::{
    OidcIdClaims, OidcServicePort, PasswordHasherPort, SessionStoragePort, TokenServicePort,
    UserStoragePort,
};
use crate::application::ports::inbound::FolderUseCase;
use crate::common::config::OidcConfig;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::session::Session;
use crate::domain::entities::user::{User, UserRole};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Instant;

/// Maximum age for pending OIDC flows (10 minutes)
const OIDC_FLOW_TTL_SECS: u64 = 600;
/// Maximum age for pending one-time token codes (60 seconds)
const OIDC_TOKEN_TTL_SECS: u64 = 60;

/// Tracks a pending OIDC authorization flow (CSRF + PKCE + nonce)
struct PendingOidcFlow {
    created_at: Instant,
    pkce_verifier: String,
    nonce: String,
}

/// Tracks a pending one-time token exchange after successful OIDC callback
struct PendingOidcToken {
    auth_response: AuthResponseDto,
    created_at: Instant,
}

/// Interior state for OIDC — protected by RwLock for hot-reload.
struct OidcState {
    service: Option<Arc<dyn OidcServicePort>>,
    config: Option<OidcConfig>,
}

/// Default quota: 100 GB
const DEFAULT_ADMIN_QUOTA: i64 = 107_374_182_400;
const DEFAULT_USER_QUOTA: i64 = 1_073_741_824; // 1 GB

pub struct AuthApplicationService {
    user_storage: Arc<dyn UserStoragePort>,
    session_storage: Arc<dyn SessionStoragePort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
    folder_service: Option<Arc<dyn FolderUseCase>>,
    /// Path to the storage directory, used for disk-space–aware quota calculation
    storage_path: PathBuf,
    oidc: RwLock<OidcState>,
    /// Pending OIDC authorization flows keyed by state token (CSRF + PKCE + nonce)
    pending_oidc_flows: Mutex<HashMap<String, PendingOidcFlow>>,
    /// Pending one-time token codes for secure token delivery after OIDC callback
    pending_oidc_tokens: Mutex<HashMap<String, PendingOidcToken>>,
}

impl AuthApplicationService {
    pub fn new(
        user_storage: Arc<dyn UserStoragePort>,
        session_storage: Arc<dyn SessionStoragePort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenServicePort>,
        storage_path: PathBuf,
    ) -> Self {
        Self {
            user_storage,
            session_storage,
            password_hasher,
            token_service,
            folder_service: None,
            storage_path,
            oidc: RwLock::new(OidcState {
                service: None,
                config: None,
            }),
            pending_oidc_flows: Mutex::new(HashMap::new()),
            pending_oidc_tokens: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the default quota for the given role, capped to the available
    /// disk space on the filesystem that hosts the storage directory.
    fn capped_quota(&self, role: &UserRole) -> i64 {
        let base_quota = match role {
            UserRole::Admin => DEFAULT_ADMIN_QUOTA,
            _ => DEFAULT_USER_QUOTA,
        };

        match Self::available_disk_space(&self.storage_path) {
            Some(avail) => {
                let avail_i64 = avail as i64;
                if avail_i64 < base_quota {
                    tracing::info!(
                        "Available disk space ({} bytes) is less than default {} quota ({} bytes) — capping quota",
                        avail_i64,
                        if *role == UserRole::Admin {
                            "admin"
                        } else {
                            "user"
                        },
                        base_quota,
                    );
                    avail_i64
                } else {
                    base_quota
                }
            }
            None => {
                tracing::warn!("Could not determine available disk space, using default quota");
                base_quota
            }
        }
    }

    /// Query the available space on the filesystem that contains `path`.
    fn available_disk_space(path: &std::path::Path) -> Option<u64> {
        use fs2::available_space;
        match available_space(path) {
            Ok(space) => Some(space),
            Err(e) => {
                tracing::warn!("Failed to query disk space for {:?}: {}", path, e);
                None
            }
        }
    }

    /// Configures the folder service, needed to create personal folders
    pub fn with_folder_service(mut self, folder_service: Arc<dyn FolderUseCase>) -> Self {
        self.folder_service = Some(folder_service);
        self
    }

    /// Configures the OIDC service
    pub fn with_oidc(
        self,
        oidc_service: Arc<dyn OidcServicePort>,
        oidc_config: OidcConfig,
    ) -> Self {
        {
            let mut state = self.oidc.write().unwrap();
            state.service = Some(oidc_service);
            state.config = Some(oidc_config);
        }
        self
    }

    /// Hot-reload OIDC configuration at runtime (called from admin settings service)
    pub fn reload_oidc(&self, oidc_service: Arc<dyn OidcServicePort>, oidc_config: OidcConfig) {
        let mut state = self.oidc.write().unwrap();
        state.service = Some(oidc_service);
        state.config = Some(oidc_config);
    }

    /// Disable OIDC at runtime (called from admin settings service)
    pub fn disable_oidc(&self) {
        let mut state = self.oidc.write().unwrap();
        state.service = None;
        state.config = None;
    }

    /// Returns whether OIDC is configured and enabled
    pub fn oidc_enabled(&self) -> bool {
        let state = self.oidc.read().unwrap();
        state.service.is_some() && state.config.as_ref().is_some_and(|c| c.enabled)
    }

    /// Returns whether password login is disabled (OIDC-only mode)
    pub fn password_login_disabled(&self) -> bool {
        let state = self.oidc.read().unwrap();
        state
            .config
            .as_ref()
            .is_some_and(|c| c.disable_password_login)
    }

    /// Returns a clone of the OIDC config if available
    pub fn oidc_config(&self) -> Option<OidcConfig> {
        let state = self.oidc.read().unwrap();
        state.config.clone()
    }

    /// Returns an Arc clone of the OIDC service if available
    pub fn oidc_service(&self) -> Option<Arc<dyn OidcServicePort>> {
        let state = self.oidc.read().unwrap();
        state.service.clone()
    }

    pub async fn register(&self, dto: RegisterDto) -> Result<UserDto, DomainError> {
        // Check for duplicate user
        if self
            .user_storage
            .get_user_by_username(&dto.username)
            .await
            .is_ok()
        {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("User '{}' already exists", dto.username),
            ));
        }

        if self
            .user_storage
            .get_user_by_email(&dto.email)
            .await
            .is_ok()
        {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("Email '{}' is already registered", dto.email),
            ));
        }

        // Check if the user wants to create an admin
        let is_admin_request = dto.username.to_lowercase() == "admin"
            || (dto.role.is_some() && dto.role.as_ref().unwrap().to_lowercase() == "admin");

        // If trying to create an admin, check if admins already exist in the system
        if is_admin_request {
            match self.count_admin_users().await {
                Ok(admin_count) => {
                    // If there are already admins in the system and this is not a clean install,
                    // we do not allow creating another admin from registration
                    if admin_count > 0 {
                        // Check if this is a clean install (only the default admin)
                        match self.count_all_users().await {
                            Ok(user_count) => {
                                // If there are more than 2 users (admin + test), it is not a clean install
                                if user_count > 2 {
                                    tracing::warn!(
                                        "Attempt to create additional admin rejected: at least one admin already exists"
                                    );
                                    return Err(DomainError::new(
                                        ErrorKind::AccessDenied,
                                        "User",
                                        "Creating additional admin users from the registration page is not allowed",
                                    ));
                                }
                                // Otherwise, it is a clean install and the first admin is allowed
                                tracing::info!("Allowing admin creation on clean install");
                            }
                            Err(e) => {
                                // Cannot verify user count — treat as bootstrap scenario
                                tracing::warn!(
                                    "Could not count users ({}). Allowing admin creation for bootstrap.",
                                    e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    // Any DB error (table missing, connection issue, etc.) means we
                    // cannot verify admin state. Allow admin creation so the user can
                    // bootstrap the system. If the DB is truly broken the INSERT will
                    // fail anyway with a clear error.
                    tracing::warn!(
                        "Could not count admin users ({}). Allowing admin creation for bootstrap.",
                        e
                    );
                }
            }
        }

        // Determine role and quota based on user type
        // If an explicit "admin" role is provided, use the administrator role
        let role = if let Some(role_str) = &dto.role {
            if role_str.to_lowercase() == "admin" {
                UserRole::Admin
            } else {
                UserRole::User
            }
        } else {
            // Special case: if the username is "admin", assign admin role even if not specified
            if dto.username.to_lowercase() == "admin" {
                UserRole::Admin
            } else {
                UserRole::User
            }
        };

        // Quota based on role, capped to available disk space
        let quota = self.capped_quota(&role);

        // Validate password length before hashing
        if dto.password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password must be at least 8 characters long",
            ));
        }

        // Hash the password using the infrastructure service
        let password_hash = self.password_hasher.hash_password(&dto.password)?;

        // Create user with the pre-generated hash
        let user = User::new(dto.username.clone(), dto.email, password_hash, role, quota).map_err(
            |e| {
                DomainError::new(
                    ErrorKind::InvalidInput,
                    "User",
                    format!("Error creating user: {}", e),
                )
            },
        )?;

        // Save user
        let created_user = self.user_storage.create_user(user).await?;

        // Create personal folder for the user
        self.create_personal_folder(&dto.username, created_user.id())
            .await;

        tracing::info!("User registered: {}", created_user.id());
        Ok(UserDto::from(created_user))
    }

    pub async fn login(&self, dto: LoginDto) -> Result<AuthResponseDto, DomainError> {
        // Find user
        let mut user = self
            .user_storage
            .get_user_by_username(&dto.username)
            .await
            .map_err(|_| {
                DomainError::new(ErrorKind::AccessDenied, "Auth", "Invalid credentials")
            })?;

        // Check if user is active
        if !user.is_active() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Account deactivated",
            ));
        }

        // Verify password using the injected hasher
        let is_valid = self
            .password_hasher
            .verify_password(&dto.password, user.password_hash())?;

        if !is_valid {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Invalid credentials",
            ));
        }

        // Update last login
        user.register_login();
        self.user_storage.update_user(user.clone()).await?;

        // Generate tokens using the injected token service
        let access_token = self.token_service.generate_access_token(&user)?;

        let refresh_token = self.token_service.generate_refresh_token();

        // Save session
        let session = Session::new(
            user.id().to_string(),
            refresh_token.clone(),
            None, // IP (can be added from the HTTP layer)
            None, // User-Agent (can be added from the HTTP layer)
            self.token_service.refresh_token_expiry_days(),
        );

        self.session_storage.create_session(session).await?;

        // Authentication response
        Ok(AuthResponseDto {
            user: UserDto::from(user),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_service.refresh_token_expiry_secs(),
        })
    }

    pub async fn refresh_token(
        &self,
        dto: RefreshTokenDto,
    ) -> Result<AuthResponseDto, DomainError> {
        // Get valid session
        let session = self
            .session_storage
            .get_session_by_refresh_token(&dto.refresh_token)
            .await?;

        // Check if the session is expired or revoked
        if session.is_expired() || session.is_revoked() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Session expired or invalid",
            ));
        }

        // Get user
        let user = self.user_storage.get_user_by_id(session.user_id()).await?;

        // Check if user is active
        if !user.is_active() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Account deactivated",
            ));
        }

        // Revoke current session
        self.session_storage.revoke_session(session.id()).await?;

        // Generate new tokens
        let access_token = self.token_service.generate_access_token(&user)?;

        let new_refresh_token = self.token_service.generate_refresh_token();

        // Create new session
        let new_session = Session::new(
            user.id().to_string(),
            new_refresh_token.clone(),
            None,
            None,
            self.token_service.refresh_token_expiry_days(),
        );

        self.session_storage.create_session(new_session).await?;

        Ok(AuthResponseDto {
            user: UserDto::from(user),
            access_token,
            refresh_token: new_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_service.refresh_token_expiry_secs(),
        })
    }

    pub async fn logout(&self, user_id: &str, refresh_token: &str) -> Result<(), DomainError> {
        // Get session
        let session = match self
            .session_storage
            .get_session_by_refresh_token(refresh_token)
            .await
        {
            Ok(s) => s,
            // If the session doesn't exist, we consider the logout successful
            Err(_) => return Ok(()),
        };

        // Verify that the session belongs to the user
        if session.user_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "The session does not belong to the user",
            ));
        }

        // Revoke session
        self.session_storage.revoke_session(session.id()).await?;

        Ok(())
    }

    pub async fn logout_all(&self, user_id: &str) -> Result<u64, DomainError> {
        // Revoke all user sessions
        let revoked_count = self
            .session_storage
            .revoke_all_user_sessions(user_id)
            .await?;

        Ok(revoked_count)
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        dto: ChangePasswordDto,
    ) -> Result<(), DomainError> {
        // Get user
        let mut user = self.user_storage.get_user_by_id(user_id).await?;

        // Block password changes for OIDC-provisioned users
        if user.is_oidc_user() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Password changes are not available for SSO/OIDC accounts. Your password is managed by your identity provider.",
            ));
        }

        // Verify current password using the injected hasher
        let is_valid = self
            .password_hasher
            .verify_password(&dto.current_password, user.password_hash())?;

        if !is_valid {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Current password is incorrect",
            ));
        }

        // Validate new password
        if dto.new_password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password must be at least 8 characters long",
            ));
        }

        // Hash new password and update user
        let new_hash = self.password_hasher.hash_password(&dto.new_password)?;
        user.update_password_hash(new_hash);

        // Save updated user
        self.user_storage.update_user(user).await?;

        // Optional: revoke all sessions to force re-login with new password
        self.session_storage
            .revoke_all_user_sessions(user_id)
            .await?;

        Ok(())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<UserDto, DomainError> {
        let user = self.user_storage.get_user_by_id(user_id).await?;
        Ok(UserDto::from(user))
    }

    // Alias for consistency with handler method
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<UserDto, DomainError> {
        self.get_user(user_id).await
    }

    // New method to get user by username - needed for admin user handling
    pub async fn get_user_by_username(&self, username: &str) -> Result<UserDto, DomainError> {
        let user = self.user_storage.get_user_by_username(username).await?;
        Ok(UserDto::from(user))
    }

    // Method to count how many admin users exist in the system
    // Used to determine if we have multiple admins or just the default one
    pub async fn count_admin_users(&self) -> Result<i64, DomainError> {
        // Use the list_users_by_role method or similar from user_storage port
        // For now, we'll use a basic implementation that counts all users with role = "admin"
        let admin_users = self
            .user_storage
            .list_users_by_role("admin")
            .await
            .map_err(|e| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "User",
                    format!("Error counting admin users: {}", e),
                )
            })?;

        Ok(admin_users.len() as i64)
    }

    // Method to count all users in the system
    // Used to determine if this is a fresh install
    pub async fn count_all_users(&self) -> Result<i64, DomainError> {
        // Get all users with large limit and 0 offset
        let all_users = self.user_storage.list_users(1000, 0).await.map_err(|e| {
            DomainError::new(
                ErrorKind::InternalError,
                "User",
                format!("Error counting users: {}", e),
            )
        })?;

        Ok(all_users.len() as i64)
    }

    // Method to delete the default admin user created by migrations
    // Used in fresh installations before creating a custom admin
    pub async fn delete_default_admin(&self) -> Result<(), DomainError> {
        // Find the default admin user (created by migrations)
        match self.get_user_by_username("admin").await {
            Ok(default_admin) => {
                // Delete the default admin user
                self.user_storage
                    .delete_user(&default_admin.id)
                    .await
                    .map_err(|e| {
                        DomainError::new(
                            ErrorKind::InternalError,
                            "User",
                            format!("Error deleting default admin user: {}", e),
                        )
                    })
            }
            Err(_) => {
                // Admin user doesn't exist, nothing to do
                tracing::info!("Default admin user not found, nothing to delete");
                Ok(())
            }
        }
    }

    // Method to replace the default admin user with a custom one
    // Used in fresh installations to allow users to set their own admin credentials
    pub async fn replace_default_admin(&self, dto: &RegisterDto) -> Result<UserDto, DomainError> {
        // 1. Get the default admin user
        let default_admin = self.get_user_by_username("admin").await?;

        // 2. Delete the default admin user
        self.user_storage
            .delete_user(&default_admin.id)
            .await
            .map_err(|e| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "User",
                    format!("Error deleting default admin user: {}", e),
                )
            })?;

        // 3. Create new admin user with the provided credentials but admin role
        let admin_role = UserRole::Admin;

        // Admin quota, capped to available disk space
        let admin_quota = self.capped_quota(&admin_role);

        // Hash the password (same as register / admin_create_user)
        let password_hash = self
            .password_hasher
            .hash_password(&dto.password)
            .map_err(|e| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "User",
                    format!("Error hashing password: {}", e),
                )
            })?;

        // Create the new admin user
        let user = User::new(
            dto.username.clone(),
            dto.email.clone(),
            password_hash,
            admin_role,
            admin_quota,
        )
        .map_err(|e| {
            DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                format!("Error creating admin user: {}", e),
            )
        })?;

        // 4. Save the new admin user
        let created_user = self.user_storage.create_user(user).await?;

        // 5. Create personal folder for the new admin
        self.create_personal_folder(&dto.username, created_user.id())
            .await;

        tracing::info!("Custom admin created: {}", created_user.id());
        Ok(UserDto::from(created_user))
    }

    pub async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<UserDto>, DomainError> {
        let users = self.user_storage.list_users(limit, offset).await?;
        Ok(users.into_iter().map(UserDto::from).collect())
    }

    // ========================================================================
    // Admin User Management Methods
    // ========================================================================

    /// Admin-only: create a user bypassing registration guards.
    pub async fn admin_create_user(
        &self,
        dto: crate::application::dtos::settings_dto::AdminCreateUserDto,
    ) -> Result<UserDto, DomainError> {
        // Validate username length
        if dto.username.len() < 3 || dto.username.len() > 32 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Username must be between 3 and 32 characters".to_string(),
            ));
        }

        // Check for duplicate username
        if self
            .user_storage
            .get_user_by_username(&dto.username)
            .await
            .is_ok()
        {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("User '{}' already exists", dto.username),
            ));
        }

        // Email: use provided or generate placeholder
        let email = dto
            .email
            .filter(|e| !e.trim().is_empty())
            .unwrap_or_else(|| format!("{}@oxicloud.local", dto.username));

        // Check email uniqueness
        if self.user_storage.get_user_by_email(&email).await.is_ok() {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("Email '{}' is already registered", email),
            ));
        }

        // Validate password
        if dto.password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password must be at least 8 characters long".to_string(),
            ));
        }

        // Determine role
        let role = match dto.role.as_deref() {
            Some("admin") => UserRole::Admin,
            _ => UserRole::User,
        };

        // Determine quota
        let quota = dto.quota_bytes.unwrap_or_else(|| {
            if role == UserRole::Admin {
                107_374_182_400
            } else {
                1_073_741_824
            }
        });

        // Hash password
        let password_hash = self.password_hasher.hash_password(&dto.password)?;

        // Create domain entity
        let user =
            User::new(dto.username.clone(), email, password_hash, role, quota).map_err(|e| {
                DomainError::new(
                    ErrorKind::InvalidInput,
                    "User",
                    format!("Error creating user: {}", e),
                )
            })?;

        // Persist
        let created = self.user_storage.create_user(user).await?;

        // Deactivate if requested (User::new always sets active=true)
        if let Some(false) = dto.active {
            self.user_storage
                .set_user_active_status(created.id(), false)
                .await?;
        }

        // Create personal folder
        self.create_personal_folder(&dto.username, created.id())
            .await;

        tracing::info!("Admin created user: {} ({})", dto.username, created.id());
        Ok(UserDto::from(created))
    }

    /// Admin-only: reset a user's password.
    pub async fn admin_reset_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), DomainError> {
        // Block password reset for OIDC-provisioned users
        let user = self.user_storage.get_user_by_id(user_id).await?;
        if user.is_oidc_user() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Auth",
                "Cannot reset password for SSO/OIDC accounts. The user's password is managed by their identity provider.",
            ));
        }

        if new_password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        let hash = self.password_hasher.hash_password(new_password)?;
        self.user_storage.change_password(user_id, &hash).await
    }

    /// Get a single user by ID (for admin panel)
    pub async fn get_user_admin(&self, user_id: &str) -> Result<UserDto, DomainError> {
        let user = self.user_storage.get_user_by_id(user_id).await?;
        Ok(UserDto::from(user))
    }

    /// Delete a user by ID (admin only)
    pub async fn delete_user_admin(&self, user_id: &str) -> Result<(), DomainError> {
        // Prevent deleting yourself
        let user = self.user_storage.get_user_by_id(user_id).await?;
        tracing::info!("Admin deleting user: {} ({})", user.username(), user_id);
        self.user_storage.delete_user(user_id).await
    }

    /// Activate or deactivate a user (admin only)
    pub async fn set_user_active(&self, user_id: &str, active: bool) -> Result<(), DomainError> {
        self.user_storage
            .set_user_active_status(user_id, active)
            .await
    }

    /// Change user role (admin only)
    pub async fn change_user_role(&self, user_id: &str, role: &str) -> Result<(), DomainError> {
        if role != "admin" && role != "user" {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                format!("Invalid role: {}. Must be 'admin' or 'user'", role),
            ));
        }
        self.user_storage.change_role(user_id, role).await
    }

    /// Update user's storage quota (admin only)
    pub async fn update_user_quota(
        &self,
        user_id: &str,
        quota_bytes: i64,
    ) -> Result<(), DomainError> {
        if quota_bytes < 0 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Quota must be non-negative".to_string(),
            ));
        }
        self.user_storage
            .update_storage_quota(user_id, quota_bytes)
            .await
    }

    /// Check if a user has enough quota for an upload of the given size
    pub async fn check_quota(
        &self,
        user_id: &str,
        additional_bytes: i64,
    ) -> Result<bool, DomainError> {
        let user = self.user_storage.get_user_by_id(user_id).await?;
        let quota = user.storage_quota_bytes();
        if quota <= 0 {
            // 0 or negative means unlimited
            return Ok(true);
        }
        Ok(user.storage_used_bytes() + additional_bytes <= quota)
    }

    /// Count users efficiently
    pub async fn count_users_efficient(&self) -> Result<i64, DomainError> {
        self.user_storage.count_users().await
    }

    // ========================================================================
    // OIDC Methods
    // ========================================================================

    /// Prepare the OIDC authorization flow: generates CSRF state, PKCE pair,
    /// nonce, stores them in pending_oidc_flows, and returns the authorize URL.
    pub async fn prepare_oidc_authorize(&self) -> Result<String, DomainError> {
        let oidc = self.oidc_service().ok_or_else(|| {
            DomainError::new(
                ErrorKind::InternalError,
                "OIDC",
                "OIDC service not configured",
            )
        })?;

        // Generate CSRF state token
        use rand_core::{OsRng, RngCore};
        let mut state_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut state_bytes);
        let state_token = hex::encode(state_bytes);

        // Generate nonce for ID token binding
        let mut nonce_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = hex::encode(nonce_bytes);

        // Generate PKCE pair (RFC 7636, S256)
        let mut verifier_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut verifier_bytes);
        let pkce_verifier = base64_url_encode(&verifier_bytes);
        let pkce_challenge = {
            use sha2::{Digest, Sha256};
            let hash = Sha256::digest(pkce_verifier.as_bytes());
            base64_url_encode(&hash)
        };

        // Store pending flow
        {
            let mut flows = self.pending_oidc_flows.lock().unwrap();
            // Cleanup expired entries
            let now = Instant::now();
            flows.retain(|_, f| now.duration_since(f.created_at).as_secs() < OIDC_FLOW_TTL_SECS);

            flows.insert(
                state_token.clone(),
                PendingOidcFlow {
                    created_at: now,
                    pkce_verifier,
                    nonce: nonce.clone(),
                },
            );
        }

        // Build authorization URL with state, nonce, and PKCE challenge
        let authorize_url = oidc
            .get_authorize_url(&state_token, &nonce, &pkce_challenge)
            .await?;

        tracing::info!(
            "OIDC authorize flow prepared (state={}...)",
            &state_token[..8]
        );

        Ok(authorize_url)
    }

    /// Handle the OIDC callback: validate CSRF state, exchange code with PKCE,
    /// validate ID token nonce, find or create user (JIT provisioning),
    /// issue internal tokens, and return a one-time exchange code.
    pub async fn oidc_callback(&self, code: &str, state: &str) -> Result<String, DomainError> {
        // 0. Validate CSRF state and retrieve PKCE verifier + nonce
        let (pkce_verifier, nonce) = {
            let mut flows = self.pending_oidc_flows.lock().unwrap();
            let flow = flows.remove(state).ok_or_else(|| {
                tracing::warn!("OIDC callback with invalid/expired state token");
                DomainError::new(
                    ErrorKind::AccessDenied, "OIDC",
                    "Invalid or expired OIDC state — possible CSRF attack. Please try logging in again.",
                )
            })?;

            // Check TTL
            if Instant::now().duration_since(flow.created_at).as_secs() >= OIDC_FLOW_TTL_SECS {
                tracing::warn!("OIDC callback with expired state token");
                return Err(DomainError::new(
                    ErrorKind::AccessDenied,
                    "OIDC",
                    "OIDC authorization flow expired. Please try logging in again.",
                ));
            }

            (flow.pkce_verifier, flow.nonce)
        };

        // Clone the Arc and config out of the RwLock so we don't hold the lock across await points
        let (oidc, oidc_config) = {
            let state = self.oidc.read().unwrap();
            let svc = state.service.clone().ok_or_else(|| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "OIDC",
                    "OIDC service not configured",
                )
            })?;
            let cfg = state.config.clone().ok_or_else(|| {
                DomainError::new(
                    ErrorKind::InternalError,
                    "OIDC",
                    "OIDC config not available",
                )
            })?;
            (svc, cfg)
        };

        // 1. Exchange authorization code for tokens (with PKCE verifier)
        let token_set = oidc.exchange_code(code, &pkce_verifier).await?;

        // 2. Validate ID token and extract claims (with nonce verification)
        let claims = oidc
            .validate_id_token(&token_set.id_token, Some(&nonce))
            .await?;

        // 3. Try to enrich claims from UserInfo endpoint if email is missing
        let claims = if claims.email.is_none() {
            match oidc.fetch_user_info(&token_set.access_token).await {
                Ok(user_info) => OidcIdClaims {
                    email: user_info.email.or(claims.email),
                    preferred_username: user_info.preferred_username.or(claims.preferred_username),
                    name: user_info.name.or(claims.name),
                    groups: if user_info.groups.is_empty() {
                        claims.groups
                    } else {
                        user_info.groups
                    },
                    ..claims
                },
                Err(e) => {
                    tracing::warn!(
                        "Failed to fetch UserInfo (continuing with ID token claims): {}",
                        e
                    );
                    claims
                }
            }
        } else {
            claims
        };

        let provider_name = oidc.provider_name().to_string();

        // 4. Determine username and email
        let oidc_username = claims
            .preferred_username
            .clone()
            .or(claims.name.clone())
            .unwrap_or_else(|| format!("oidc_{}", &claims.sub[..8.min(claims.sub.len())]));
        let oidc_email = claims
            .email
            .clone()
            .unwrap_or_else(|| format!("{}@oidc.local", oidc_username));

        // 5. Look up existing user by OIDC subject
        let user = match self
            .user_storage
            .get_user_by_oidc_subject(&provider_name, &claims.sub)
            .await
        {
            Ok(mut existing_user) => {
                // User exists — update last login
                existing_user.register_login();
                self.user_storage.update_user(existing_user.clone()).await?;
                existing_user
            }
            Err(_) => {
                // User doesn't exist — try to match by email
                let matched_user = self.user_storage.get_user_by_email(&oidc_email).await.ok();

                if let Some(_existing) = matched_user {
                    // Email match but no OIDC link — for security, don't auto-link
                    return Err(DomainError::new(
                        ErrorKind::AlreadyExists,
                        "OIDC",
                        format!(
                            "A user with email '{}' already exists. Contact admin to link your OIDC identity.",
                            oidc_email
                        ),
                    ));
                }

                // No match — JIT provision if enabled
                if !oidc_config.auto_provision {
                    return Err(DomainError::new(
                        ErrorKind::AccessDenied,
                        "OIDC",
                        "Auto-provisioning is disabled. Contact admin to create your account.",
                    ));
                }

                // Determine role from OIDC groups
                let role = self.map_oidc_role(&claims.groups, &oidc_config);

                let quota = self.capped_quota(&role);

                // Sanitize username (max 32 chars, ensure uniqueness)
                let mut username = oidc_username.chars().take(32).collect::<String>();
                if username.len() < 3 {
                    username = format!("user_{}", &claims.sub[..8.min(claims.sub.len())]);
                }

                // Check for username collision
                if self
                    .user_storage
                    .get_user_by_username(&username)
                    .await
                    .is_ok()
                {
                    let suffix = &claims.sub[..4.min(claims.sub.len())];
                    username = format!("{}_{}", &username[..username.len().min(27)], suffix);
                }

                let new_user = User::new_oidc(
                    username.clone(),
                    oidc_email,
                    role,
                    quota,
                    provider_name.clone(),
                    claims.sub.clone(),
                )
                .map_err(|e| {
                    DomainError::new(
                        ErrorKind::InvalidInput,
                        "OIDC",
                        format!("Failed to create OIDC user: {}", e),
                    )
                })?;

                let created_user = self.user_storage.create_user(new_user).await?;

                // Create personal folder
                self.create_personal_folder(&username, created_user.id())
                    .await;

                tracing::info!(
                    "OIDC user provisioned: {} (provider: {}, sub: {})",
                    created_user.id(),
                    provider_name,
                    claims.sub
                );

                created_user
            }
        };

        // 6. Issue internal tokens (same as regular login)
        let access_token = self.token_service.generate_access_token(&user)?;
        let refresh_token = self.token_service.generate_refresh_token();

        let session = Session::new(
            user.id().to_string(),
            refresh_token.clone(),
            None,
            None,
            self.token_service.refresh_token_expiry_days(),
        );
        self.session_storage.create_session(session).await?;

        let auth_response = AuthResponseDto {
            user: UserDto::from(user),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_service.refresh_token_expiry_secs(),
        };

        // 7. Store auth response behind a one-time exchange code (Fix #4: no tokens in URL)
        let mut code_bytes = [0u8; 32];
        use rand_core::{OsRng, RngCore};
        OsRng.fill_bytes(&mut code_bytes);
        let exchange_code = hex::encode(code_bytes);

        {
            let mut tokens = self.pending_oidc_tokens.lock().unwrap();
            // Cleanup expired entries
            let now = Instant::now();
            tokens.retain(|_, t| now.duration_since(t.created_at).as_secs() < OIDC_TOKEN_TTL_SECS);

            tokens.insert(
                exchange_code.clone(),
                PendingOidcToken {
                    auth_response,
                    created_at: now,
                },
            );
        }

        tracing::info!("OIDC login successful, one-time exchange code generated");

        Ok(exchange_code)
    }

    /// Exchange a one-time code for the authentication tokens.
    /// The code is single-use and expires after 60 seconds.
    pub fn exchange_oidc_token(&self, one_time_code: &str) -> Result<AuthResponseDto, DomainError> {
        let mut tokens = self.pending_oidc_tokens.lock().unwrap();
        let pending = tokens.remove(one_time_code).ok_or_else(|| {
            DomainError::new(
                ErrorKind::AccessDenied,
                "OIDC",
                "Invalid or expired exchange code. Please try logging in again.",
            )
        })?;

        // Check TTL
        if Instant::now().duration_since(pending.created_at).as_secs() >= OIDC_TOKEN_TTL_SECS {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "OIDC",
                "Exchange code expired. Please try logging in again.",
            ));
        }

        Ok(pending.auth_response)
    }

    /// Map OIDC groups to internal role
    fn map_oidc_role(&self, groups: &[String], config: &OidcConfig) -> UserRole {
        if config.admin_groups.is_empty() {
            return UserRole::User;
        }
        let admin_groups: Vec<&str> = config.admin_groups.split(',').map(|s| s.trim()).collect();
        for group in groups {
            if admin_groups.iter().any(|ag| ag.eq_ignore_ascii_case(group)) {
                return UserRole::Admin;
            }
        }
        UserRole::User
    }

    /// Helper to create a personal folder for a new user
    async fn create_personal_folder(&self, username: &str, user_id: &str) {
        if let Some(folder_service) = &self.folder_service {
            let folder_name = format!("My Folder - {}", username);
            match folder_service
                .create_home_folder(user_id, folder_name.clone())
                .await
            {
                Ok(folder) => {
                    tracing::info!(
                        "Personal folder created for user {}: {} (ID: {})",
                        user_id,
                        folder.name,
                        folder.id
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to create personal folder for user {}: {}",
                        user_id,
                        e
                    );
                }
            }
        }
    }
}

/// URL-safe base64 encoding without padding (RFC 4648 §5)
fn base64_url_encode(input: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}
