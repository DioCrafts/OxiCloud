use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use crate::domain::entities::user::{User, UserRole};
use crate::domain::entities::session::Session;
use crate::application::ports::auth_ports::{UserStoragePort, SessionStoragePort, PasswordHasherPort, TokenServicePort, OidcServicePort, OidcIdClaims};
use crate::application::dtos::user_dto::{UserDto, RegisterDto, LoginDto, AuthResponseDto, ChangePasswordDto, RefreshTokenDto};
use crate::application::dtos::folder_dto::CreateFolderDto;
use crate::application::ports::inbound::FolderUseCase;
use crate::common::errors::{DomainError, ErrorKind};
use crate::common::config::OidcConfig;

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

pub struct AuthApplicationService {
    user_storage: Arc<dyn UserStoragePort>,
    session_storage: Arc<dyn SessionStoragePort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
    folder_service: Option<Arc<dyn FolderUseCase>>,
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
    ) -> Self {
        Self {
            user_storage,
            session_storage,
            password_hasher,
            token_service,
            folder_service: None,
            oidc: RwLock::new(OidcState { service: None, config: None }),
            pending_oidc_flows: Mutex::new(HashMap::new()),
            pending_oidc_tokens: Mutex::new(HashMap::new()),
        }
    }
    
    /// Configura el servicio de carpetas, necesario para crear carpetas personales
    pub fn with_folder_service(mut self, folder_service: Arc<dyn FolderUseCase>) -> Self {
        self.folder_service = Some(folder_service);
        self
    }

    /// Configura el servicio OIDC
    pub fn with_oidc(self, oidc_service: Arc<dyn OidcServicePort>, oidc_config: OidcConfig) -> Self {
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
        state.service.is_some() && state.config.as_ref().map_or(false, |c| c.enabled)
    }

    /// Returns whether password login is disabled (OIDC-only mode)
    pub fn password_login_disabled(&self) -> bool {
        let state = self.oidc.read().unwrap();
        state.config.as_ref().map_or(false, |c| c.disable_password_login)
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
        // Verificar usuario duplicado
        if self.user_storage.get_user_by_username(&dto.username).await.is_ok() {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("El usuario '{}' ya existe", dto.username)
            ));
        }
        
        if self.user_storage.get_user_by_email(&dto.email).await.is_ok() {
            return Err(DomainError::new(
                ErrorKind::AlreadyExists,
                "User",
                format!("El email '{}' ya está registrado", dto.email)
            ));
        }
        
        // Verificar si el usuario quiere crear un admin
        let is_admin_request = dto.username.to_lowercase() == "admin" || 
            (dto.role.is_some() && dto.role.as_ref().unwrap().to_lowercase() == "admin");
            
        // Si está intentando crear un admin, verificar si ya existen admins en el sistema
        if is_admin_request {
            match self.count_admin_users().await {
                Ok(admin_count) => {
                    // Si ya hay admins en el sistema y no estamos en instalación limpia,
                    // no permitimos crear otro admin desde el registro
                    if admin_count > 0 {
                        // Verificar si es una instalación limpia (solo el admin predeterminado)
                        match self.count_all_users().await {
                            Ok(user_count) => {
                                // Si hay más de 2 usuarios (admin + test), no es instalación limpia
                                if user_count > 2 {
                                    tracing::warn!("Intento de crear admin adicional rechazado: ya existe al menos un admin");
                                    return Err(DomainError::new(
                                        ErrorKind::AccessDenied,
                                        "User",
                                        "No se permite crear usuarios admin adicionales desde la página de registro"
                                    ));
                                }
                                // En caso contrario, es instalación limpia y se permite el primer admin
                                tracing::info!("Permitiendo creación de admin en instalación limpia");
                            },
                            Err(e) => {
                                tracing::error!("Error al contar usuarios: {}", e);
                                // Por seguridad, si no podemos verificar, rechazamos la creación de admin
                                return Err(DomainError::new(
                                    ErrorKind::AccessDenied,
                                    "User",
                                    "No se permite crear usuarios admin adicionales"
                                ));
                            }
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Error al contar usuarios admin: {}", e);
                    // Por seguridad, si no podemos verificar, rechazamos la creación de admin
                    return Err(DomainError::new(
                        ErrorKind::AccessDenied,
                        "User",
                        "No se permite crear usuarios admin adicionales"
                    ));
                }
            }
        }
        
        // Determinar rol y cuota según el tipo de usuario
        // Si se proporciona un rol explícito de "admin", usar rol de administrador
        let role = if let Some(role_str) = &dto.role {
            if role_str.to_lowercase() == "admin" {
                UserRole::Admin
            } else {
                UserRole::User
            }
        } else {
            // Caso especial: si el nombre es "admin", asignar rol de admin aunque no se especifique
            if dto.username.to_lowercase() == "admin" {
                UserRole::Admin
            } else {
                UserRole::User
            }
        };
        
        // Cuota según el rol: 100GB para admin, 1GB para usuarios normales
        let quota = if role == UserRole::Admin {
            107374182400 // 100GB para admin
        } else {
            1024 * 1024 * 1024 // 1GB para usuarios normales
        };
        
        // Validar longitud de password antes de hashear
        if dto.password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password debe tener al menos 8 caracteres"
            ));
        }
        
        // Hashear el password usando el servicio de infraestructura
        let password_hash = self.password_hasher.hash_password(&dto.password)?;
        
        // Crear usuario con el hash pre-generado
        let user = User::new(
            dto.username.clone(),
            dto.email,
            password_hash,
            role,
            quota,
        ).map_err(|e| DomainError::new(
            ErrorKind::InvalidInput,
            "User",
            format!("Error al crear usuario: {}", e)
        ))?;
        
        // Guardar usuario
        let created_user = self.user_storage.create_user(user).await?;
        
        // Crear carpeta personal para el usuario
        if let Some(folder_service) = &self.folder_service {
            let folder_name = format!("Mi Carpeta - {}", dto.username);
            
            match folder_service.create_folder(CreateFolderDto {
                name: folder_name,
                parent_id: None,
            }).await {
                Ok(folder) => {
                    tracing::info!(
                        "Carpeta personal creada para el usuario {}: {} (ID: {})", 
                        created_user.id(), 
                        folder.name, 
                        folder.id
                    );
                    
                    // Aquí se podría guardar la asociación de la carpeta al usuario
                    // por ejemplo, en una tabla de relación carpeta-usuario
                },
                Err(e) => {
                    // No fallamos el registro por un error en la creación de la carpeta
                    // pero lo registramos para investigación
                    tracing::error!(
                        "No se pudo crear la carpeta personal para el usuario {}: {}", 
                        created_user.id(), 
                        e
                    );
                }
            }
        } else {
            tracing::warn!(
                "No se configuró el servicio de carpetas, no se puede crear carpeta personal para el usuario: {}", 
                created_user.id()
            );
        }
        
        tracing::info!("Usuario registrado: {}", created_user.id());
        Ok(UserDto::from(created_user))
    }
    
    pub async fn login(&self, dto: LoginDto) -> Result<AuthResponseDto, DomainError> {
        // Buscar usuario
        let mut user = self.user_storage
            .get_user_by_username(&dto.username)
            .await
            .map_err(|_| DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Credenciales inválidas"
            ))?;
        
        // Verificar si usuario está activo
        if !user.is_active() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Cuenta desactivada"
            ));
        }
        
        // Verificar contraseña usando el hasher inyectado
        let is_valid = self.password_hasher.verify_password(&dto.password, user.password_hash())?;
            
        if !is_valid {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Credenciales inválidas"
            ));
        }
        
        // Actualizar último login
        user.register_login();
        self.user_storage.update_user(user.clone()).await?;
        
        // Generar tokens usando el servicio de tokens inyectado
        let access_token = self.token_service.generate_access_token(&user)?;
        
        let refresh_token = self.token_service.generate_refresh_token();
        
        // Guardar sesión
        let session = Session::new(
            user.id().to_string(),
            refresh_token.clone(),
            None, // IP (se puede añadir desde la capa HTTP)
            None, // User-Agent (se puede añadir desde la capa HTTP)
            self.token_service.refresh_token_expiry_days(),
        );
        
        self.session_storage.create_session(session).await?;
        
        // Respuesta de autenticación
        Ok(AuthResponseDto {
            user: UserDto::from(user),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_service.refresh_token_expiry_secs(),
        })
    }
    
    pub async fn refresh_token(&self, dto: RefreshTokenDto) -> Result<AuthResponseDto, DomainError> {
        // Obtener sesión válida
        let session = self.session_storage
            .get_session_by_refresh_token(&dto.refresh_token)
            .await?;
        
        // Verificar si la sesión está expirada o revocada
        if session.is_expired() || session.is_revoked() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Sesión expirada o inválida"
            ));
        }
        
        // Obtener usuario
        let user = self.user_storage
            .get_user_by_id(session.user_id())
            .await?;
        
        // Verificar si usuario está activo
        if !user.is_active() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Cuenta desactivada"
            ));
        }
        
        // Revocar sesión actual
        self.session_storage.revoke_session(session.id()).await?;
        
        // Generar nuevos tokens
        let access_token = self.token_service.generate_access_token(&user)?;
        
        let new_refresh_token = self.token_service.generate_refresh_token();
        
        // Crear nueva sesión
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
        // Obtener sesión
        let session = match self.session_storage.get_session_by_refresh_token(refresh_token).await {
            Ok(s) => s,
            // Si la sesión no existe, consideramos el logout como exitoso
            Err(_) => return Ok(()),
        };
        
        // Verificar que la sesión pertenece al usuario
        if session.user_id() != user_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "La sesión no pertenece al usuario"
            ));
        }
        
        // Revocar sesión
        self.session_storage.revoke_session(session.id()).await?;
        
        Ok(())
    }
    
    pub async fn logout_all(&self, user_id: &str) -> Result<u64, DomainError> {
        // Revocar todas las sesiones del usuario
        let revoked_count = self.session_storage.revoke_all_user_sessions(user_id).await?;
        
        Ok(revoked_count)
    }
    
    pub async fn change_password(&self, user_id: &str, dto: ChangePasswordDto) -> Result<(), DomainError> {
        // Obtener usuario
        let mut user = self.user_storage.get_user_by_id(user_id).await?;
        
        // Verificar contraseña actual usando el hasher inyectado
        let is_valid = self.password_hasher.verify_password(&dto.current_password, user.password_hash())?;
            
        if !is_valid {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Auth",
                "Contraseña actual incorrecta"
            ));
        }
        
        // Validar nueva contraseña
        if dto.new_password.len() < 8 {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "User",
                "Password debe tener al menos 8 caracteres"
            ));
        }
        
        // Hashear nueva contraseña y actualizar usuario
        let new_hash = self.password_hasher.hash_password(&dto.new_password)?;
        user.update_password_hash(new_hash);
        
        // Guardar usuario actualizado
        self.user_storage.update_user(user).await?;
        
        // Opcional: revocar todas las sesiones para forzar re-login con nueva contraseña
        self.session_storage.revoke_all_user_sessions(user_id).await?;
        
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
        let admin_users = self.user_storage.list_users_by_role("admin").await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "User",
                format!("Error al contar usuarios administradores: {}", e)
            ))?;
        
        Ok(admin_users.len() as i64)
    }
    
    // Method to count all users in the system
    // Used to determine if this is a fresh install
    pub async fn count_all_users(&self) -> Result<i64, DomainError> {
        // Get all users with large limit and 0 offset
        let all_users = self.user_storage.list_users(1000, 0).await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "User", 
                format!("Error al contar usuarios: {}", e)
            ))?;
            
        Ok(all_users.len() as i64)
    }
    
    // Method to delete the default admin user created by migrations
    // Used in fresh installations before creating a custom admin
    pub async fn delete_default_admin(&self) -> Result<(), DomainError> {
        // Find the default admin user (created by migrations)
        match self.get_user_by_username("admin").await {
            Ok(default_admin) => {
                // Delete the default admin user
                self.user_storage.delete_user(&default_admin.id).await
                    .map_err(|e| DomainError::new(
                        ErrorKind::InternalError,
                        "User",
                        format!("Error al eliminar usuario admin predeterminado: {}", e)
                    ))
            },
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
        self.user_storage.delete_user(&default_admin.id).await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError,
                "User",
                format!("Error al eliminar usuario admin predeterminado: {}", e)
            ))?;
            
        // 3. Create new admin user with the provided credentials but admin role
        let admin_role = UserRole::Admin;
        
        // Use 100GB for admin quota
        let admin_quota = 107374182400;
        
        // Create the new admin user
        let user = User::new(
            dto.username.clone(),
            dto.email.clone(),
            dto.password.clone(),
            admin_role,
            admin_quota,
        ).map_err(|e| DomainError::new(
            ErrorKind::InvalidInput,
            "User",
            format!("Error al crear usuario admin: {}", e)
        ))?;
        
        // 4. Save the new admin user
        let created_user = self.user_storage.create_user(user).await?;
        
        // 5. Create personal folder for the new admin if folder service is available
        if let Some(folder_service) = &self.folder_service {
            let folder_name = format!("Mi Carpeta - {}", dto.username);
            
            match folder_service.create_folder(CreateFolderDto {
                name: folder_name,
                parent_id: None,
            }).await {
                Ok(folder) => {
                    tracing::info!(
                        "Carpeta personal creada para el admin {}: {} (ID: {})", 
                        created_user.id(), 
                        folder.name, 
                        folder.id
                    );
                },
                Err(e) => {
                    tracing::error!(
                        "No se pudo crear la carpeta personal para el admin {}: {}", 
                        created_user.id(), 
                        e
                    );
                }
            }
        }
        
        tracing::info!("Admin personalizado creado: {}", created_user.id());
        Ok(UserDto::from(created_user))
    }
    
    pub async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<UserDto>, DomainError> {
        let users = self.user_storage.list_users(limit, offset).await?;
        Ok(users.into_iter().map(UserDto::from).collect())
    }

    // ========================================================================
    // OIDC Methods
    // ========================================================================

    /// Prepare the OIDC authorization flow: generates CSRF state, PKCE pair,
    /// nonce, stores them in pending_oidc_flows, and returns the authorize URL.
    pub fn prepare_oidc_authorize(&self) -> Result<String, DomainError> {
        let oidc = self.oidc_service().ok_or_else(|| DomainError::new(
            ErrorKind::InternalError, "OIDC", "OIDC service not configured",
        ))?;

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
            use sha2::{Sha256, Digest};
            let hash = Sha256::digest(pkce_verifier.as_bytes());
            base64_url_encode(&hash)
        };

        // Store pending flow
        {
            let mut flows = self.pending_oidc_flows.lock().unwrap();
            // Cleanup expired entries
            let now = Instant::now();
            flows.retain(|_, f| now.duration_since(f.created_at).as_secs() < OIDC_FLOW_TTL_SECS);

            flows.insert(state_token.clone(), PendingOidcFlow {
                created_at: now,
                pkce_verifier,
                nonce: nonce.clone(),
            });
        }

        // Build authorization URL with state, nonce, and PKCE challenge
        let authorize_url = oidc.get_authorize_url(&state_token, &nonce, &pkce_challenge)?;

        tracing::info!("OIDC authorize flow prepared (state={}...)", &state_token[..8]);

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
                    ErrorKind::AccessDenied, "OIDC",
                    "OIDC authorization flow expired. Please try logging in again.",
                ));
            }

            (flow.pkce_verifier, flow.nonce)
        };

        // Clone the Arc and config out of the RwLock so we don't hold the lock across await points
        let (oidc, oidc_config) = {
            let state = self.oidc.read().unwrap();
            let svc = state.service.clone().ok_or_else(|| DomainError::new(
                ErrorKind::InternalError, "OIDC", "OIDC service not configured",
            ))?;
            let cfg = state.config.clone().ok_or_else(|| DomainError::new(
                ErrorKind::InternalError, "OIDC", "OIDC config not available",
            ))?;
            (svc, cfg)
        };

        // 1. Exchange authorization code for tokens (with PKCE verifier)
        let token_set = oidc.exchange_code(code, &pkce_verifier).await?;

        // 2. Validate ID token and extract claims (with nonce verification)
        let claims = oidc.validate_id_token(&token_set.id_token, Some(&nonce)).await?;

        // 3. Try to enrich claims from UserInfo endpoint if email is missing
        let claims = if claims.email.is_none() {
            match oidc.fetch_user_info(&token_set.access_token).await {
                Ok(user_info) => OidcIdClaims {
                    email: user_info.email.or(claims.email),
                    preferred_username: user_info.preferred_username.or(claims.preferred_username),
                    name: user_info.name.or(claims.name),
                    groups: if user_info.groups.is_empty() { claims.groups } else { user_info.groups },
                    ..claims
                },
                Err(e) => {
                    tracing::warn!("Failed to fetch UserInfo (continuing with ID token claims): {}", e);
                    claims
                }
            }
        } else {
            claims
        };

        let provider_name = oidc.provider_name().to_string();

        // 4. Determine username and email
        let oidc_username = claims.preferred_username.clone()
            .or(claims.name.clone())
            .unwrap_or_else(|| format!("oidc_{}", &claims.sub[..8.min(claims.sub.len())]));
        let oidc_email = claims.email.clone()
            .unwrap_or_else(|| format!("{}@oidc.local", oidc_username));

        // 5. Look up existing user by OIDC subject
        let user = match self.user_storage.get_user_by_oidc_subject(&provider_name, &claims.sub).await {
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
                        ErrorKind::AlreadyExists, "OIDC",
                        format!("A user with email '{}' already exists. Contact admin to link your OIDC identity.", oidc_email),
                    ));
                }

                // No match — JIT provision if enabled
                if !oidc_config.auto_provision {
                    return Err(DomainError::new(
                        ErrorKind::AccessDenied, "OIDC",
                        "Auto-provisioning is disabled. Contact admin to create your account.",
                    ));
                }

                // Determine role from OIDC groups
                let role = self.map_oidc_role(&claims.groups, &oidc_config);

                let quota = if role == UserRole::Admin {
                    107374182400 // 100GB
                } else {
                    1024 * 1024 * 1024 // 1GB
                };

                // Sanitize username (max 32 chars, ensure uniqueness)
                let mut username = oidc_username.chars().take(32).collect::<String>();
                if username.len() < 3 {
                    username = format!("user_{}", &claims.sub[..8.min(claims.sub.len())]);
                }

                // Check for username collision
                if self.user_storage.get_user_by_username(&username).await.is_ok() {
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
                ).map_err(|e| DomainError::new(
                    ErrorKind::InvalidInput, "OIDC",
                    format!("Failed to create OIDC user: {}", e),
                ))?;

                let created_user = self.user_storage.create_user(new_user).await?;

                // Create personal folder
                self.create_personal_folder(&username, created_user.id()).await;

                tracing::info!("OIDC user provisioned: {} (provider: {}, sub: {})", 
                    created_user.id(), provider_name, claims.sub);

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

            tokens.insert(exchange_code.clone(), PendingOidcToken {
                auth_response,
                created_at: now,
            });
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
                ErrorKind::AccessDenied, "OIDC",
                "Invalid or expired exchange code. Please try logging in again.",
            )
        })?;

        // Check TTL
        if Instant::now().duration_since(pending.created_at).as_secs() >= OIDC_TOKEN_TTL_SECS {
            return Err(DomainError::new(
                ErrorKind::AccessDenied, "OIDC",
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
            let folder_name = format!("Mi Carpeta - {}", username);
            match folder_service.create_folder(CreateFolderDto {
                name: folder_name.clone(),
                parent_id: None,
            }).await {
                Ok(folder) => {
                    tracing::info!("Personal folder created for user {}: {} (ID: {})", 
                        user_id, folder.name, folder.id);
                }
                Err(e) => {
                    tracing::error!("Failed to create personal folder for user {}: {}", user_id, e);
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