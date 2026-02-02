use std::sync::Arc;
use crate::domain::entities::user::{User, UserRole};
use crate::domain::entities::session::Session;
use crate::application::ports::auth_ports::{UserStoragePort, SessionStoragePort, PasswordHasherPort, TokenServicePort};
use crate::application::dtos::user_dto::{UserDto, RegisterDto, LoginDto, AuthResponseDto, ChangePasswordDto, RefreshTokenDto};
use crate::application::dtos::folder_dto::CreateFolderDto;
use crate::application::ports::inbound::FolderUseCase;
use crate::common::errors::{DomainError, ErrorKind};

pub struct AuthApplicationService {
    user_storage: Arc<dyn UserStoragePort>,
    session_storage: Arc<dyn SessionStoragePort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
    folder_service: Option<Arc<dyn FolderUseCase>>,
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
        }
    }
    
    /// Configura el servicio de carpetas, necesario para crear carpetas personales
    pub fn with_folder_service(mut self, folder_service: Arc<dyn FolderUseCase>) -> Self {
        self.folder_service = Some(folder_service);
        self
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
}