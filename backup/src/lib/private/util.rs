use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

// Constantes
const SPACE_UNLIMITED: i64 = -1;

/// Clase para funciones de utilidad
pub struct Util {
    // Campos estáticos
    pub static_scripts: Arc<Mutex<Vec<String>>>,
    pub static_styles: Arc<Mutex<Vec<String>>>,
    pub static_headers: Arc<Mutex<Vec<Header>>>,
    pub static_core_styles: Arc<Mutex<Vec<String>>>,
    pub static_core_scripts: Arc<Mutex<Vec<String>>>,
    // Campos internos
    root_mounted: Arc<Mutex<bool>>,
    fs_setup: Arc<Mutex<bool>>,
}

#[derive(Clone, Debug)]
pub struct Header {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub text: String,
}

impl Util {
    /// Crear una nueva instancia de Util
    pub fn new() -> Self {
        Self {
            static_scripts: Arc::new(Mutex::new(Vec::new())),
            static_styles: Arc::new(Mutex::new(Vec::new())),
            static_headers: Arc::new(Mutex::new(Vec::new())),
            static_core_styles: Arc::new(Mutex::new(Vec::new())),
            static_core_scripts: Arc::new(Mutex::new(Vec::new())),
            root_mounted: Arc::new(Mutex::new(false)),
            fs_setup: Arc::new(Mutex::new(false)),
        }
    }

    /// Configurar el sistema de archivos
    /// 
    /// # Argumentos
    /// 
    /// * `user` - Usuario opcional para configurar el sistema de archivos
    /// 
    /// # Retorno
    /// 
    /// * `Result<bool>` - Resultado de la operación
    pub fn setup_fs(&self, user: Option<&str>) -> Result<bool, Box<dyn std::error::Error>> {
        // Si ya está configurado, retornar false
        if *self.fs_setup.lock().unwrap() {
            return Ok(false);
        }

        // Si no se proporciona un usuario, usar el que ha iniciado sesión
        let user = match user {
            Some(u) if !u.is_empty() => u.to_string(),
            _ => match User::is_logged_in() {
                true => User::get_user()?,
                false => String::new(),
            },
        };

        // Cargar todas las aplicaciones de sistema de archivos antes
        if !std::env::var("RUNTIME_NOAPPS").unwrap_or_default().eq_ignore_ascii_case("true") {
            App::load_apps(&["filesystem"])?;
        }

        // Marcar el sistema de archivos como configurado si el usuario no está vacío
        if !user.is_empty() {
            *self.fs_setup.lock().unwrap() = true;
        }

        let config_data_directory = Config::get_value("datadirectory", 
            format!("{}/data", Server::get_server_root()))?.to_string();
        
        // Primero configurar el almacenamiento "root" local
        Files::Filesystem::init_mounts()?;
        
        if !*self.root_mounted.lock().unwrap() {
            let options = HashMap::from([("datadir".to_string(), config_data_directory)]);
            Files::Filesystem::mount("\\OC\\Files\\Storage\\Local", options, "/")?;
            *self.root_mounted.lock().unwrap() = true;
        }

        // Si no hay sesión iniciada, no hay razón para configurar el sistema de archivos
        if !user.is_empty() {
            let quota = self.get_user_quota(&user)?;
            
            if quota != SPACE_UNLIMITED {
                Files::Filesystem::add_storage_wrapper(Box::new(move |mount_point, storage| {
                    if mount_point == format!("/{}/", user) {
                        Box::new(Files::Storage::Wrapper::Quota::new(storage, quota))
                    } else {
                        storage
                    }
                }))?;
            }
            
            let user_dir = format!("/{}/files", user);
            let user_root = User::get_home(&user)?;
            let user_directory = format!("{}/files", user_root);
            
            if !Path::new(&user_directory).is_dir() {
                fs::create_dir_all(&user_directory)?;
                self.copy_skeleton(&user_directory)?;
            }
            
            // Encerrar al usuario en su directorio "home"
            Files::Filesystem::init(&user, &user_dir)?;

            let file_operation_proxy = FileProxy::FileOperations::new();
            FileProxy::register(Box::new(file_operation_proxy))?;

            Hook::emit("OC_Filesystem", "setup", &[
                ("user", user.as_str()),
                ("user_dir", user_dir.as_str()),
            ])?;
        }
        
        Ok(true)
    }

    /// Obtener la cuota de usuario
    pub fn get_user_quota(&self, user: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let user_quota = Preferences::get_value(user, "files", "quota", "default")?;
        
        let user_quota = if user_quota == "default" {
            AppConfig::get_value("files", "default_quota", "none")?
        } else {
            user_quota
        };
        
        if user_quota == "none" {
            Ok(SPACE_UNLIMITED)
        } else {
            Ok(Helper::computer_file_size(&user_quota)?)
        }
    }

    /// Copiar el esqueleto de archivos al directorio del usuario
    pub fn copy_skeleton(&self, user_directory: &str) -> Result<(), Box<dyn std::error::Error>> {
        let skeleton_path = format!("{}/core/skeleton", Server::get_server_root());
        self.copy_r(&skeleton_path, user_directory)?;
        Ok(())
    }

    /// Copiar un directorio de forma recursiva
    pub fn copy_r(&self, source: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
        let source_path = Path::new(source);
        let target_path = Path::new(target);
        
        if !source_path.is_dir() {
            return Err(format!("Source path is not a directory: {}", source).into());
        }
        
        fs::create_dir_all(target_path)?;
        
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if Files::Filesystem::is_ignored_dir(&file_name_str) {
                continue;
            }
            
            let entry_path = entry.path();
            let target_entry = target_path.join(&file_name);
            
            if entry_path.is_dir() {
                self.copy_r(&entry_path.to_string_lossy(), &target_entry.to_string_lossy())?;
            } else {
                fs::copy(&entry_path, &target_entry)?;
            }
        }
        
        Ok(())
    }

    /// Desmontar el sistema de archivos
    pub fn tear_down_fs(&self) -> Result<(), Box<dyn std::error::Error>> {
        Files::Filesystem::tear_down()?;
        *self.fs_setup.lock().unwrap() = false;
        *self.root_mounted.lock().unwrap() = false;
        Ok(())
    }

    /// Obtener la versión actual instalada de ownCloud
    pub fn get_version() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        Self::load_version()?;
        Ok(Server::get_server()?.get_session()?.get("OC_Version")?)
    }

    /// Obtener la cadena de versión actual instalada de ownCloud
    pub fn get_version_string() -> Result<String, Box<dyn std::error::Error>> {
        Self::load_version()?;
        Ok(Server::get_server()?.get_session()?.get("OC_VersionString")?)
    }

    /// Obtener la edición actual instalada de ownCloud
    pub fn get_edition_string() -> Result<String, Box<dyn std::error::Error>> {
        Self::load_version()?;
        Ok(Server::get_server()?.get_session()?.get("OC_Edition")?)
    }

    /// Obtener el canal de actualización de la instalación actual de ownCloud
    pub fn get_channel() -> Result<String, Box<dyn std::error::Error>> {
        Self::load_version()?;
        Ok(Server::get_server()?.get_session()?.get("OC_Channel")?)
    }

    /// Obtener el número de compilación de la instalación actual de ownCloud
    pub fn get_build() -> Result<String, Box<dyn std::error::Error>> {
        Self::load_version()?;
        Ok(Server::get_server()?.get_session()?.get("OC_Build")?)
    }

    /// Cargar la versión en la sesión como caché
    fn load_version() -> Result<(), Box<dyn std::error::Error>> {
        let version_path = format!("{}/version.php", Server::get_server_root());
        let timestamp = fs::metadata(&version_path)?.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
        
        let session = Server::get_server()?.get_session()?;
        
        if !session.exists("OC_Version") || session.get::<u64>("OC_Version_Timestamp")? != timestamp {
            // En Rust, no podemos incluir dinámicamente archivos PHP
            // Debemos simular la carga del archivo version.php de otra forma
            // Por ejemplo, usando un archivo de configuración o una función que devuelva los valores
            let version_info = load_version_info()?;
            
            session.set("OC_Version_Timestamp", timestamp)?;
            session.set("OC_Version", version_info.version)?;
            session.set("OC_VersionString", version_info.version_string)?;
            session.set("OC_Edition", version_info.edition)?;
            session.set("OC_Channel", version_info.channel)?;
            session.set("OC_Build", version_info.build)?;
        }
        
        Ok(())
    }

    /// Añadir un archivo JavaScript
    pub fn add_script(&self, application: &str, file: Option<&str>) {
        let mut scripts = self.static_scripts.lock().unwrap();
        
        match file {
            Some(f) => {
                if !application.is_empty() {
                    scripts.push(format!("{}/js/{}", application, f));
                } else {
                    scripts.push(format!("js/{}", f));
                }
            },
            None => {
                scripts.push(format!("js/{}", application));
            }
        }
    }

    /// Añadir un archivo CSS
    pub fn add_style(&self, application: &str, file: Option<&str>) {
        let mut styles = self.static_styles.lock().unwrap();
        
        match file {
            Some(f) => {
                if !application.is_empty() {
                    styles.push(format!("{}/css/{}", application, f));
                } else {
                    styles.push(format!("css/{}", f));
                }
            },
            None => {
                styles.push(format!("css/{}", application));
            }
        }
    }

    /// Añadir un elemento personalizado al encabezado
    pub fn add_header(&self, tag: &str, attributes: HashMap<String, String>, text: &str) {
        let mut headers = self.static_headers.lock().unwrap();
        
        headers.push(Header {
            tag: tag.to_string(),
            attributes,
            text: text.to_string(),
        });
    }

    /// Formatear una marca de tiempo de la manera "correcta"
    pub fn format_date(timestamp: i64, date_only: bool) -> Result<String, Box<dyn std::error::Error>> {
        let mut timestamp = timestamp;
        
        if Server::get_session()?.exists("timezone") {
            let system_time_zone = chrono::Local::now().offset().local_minus_utc() / 60;
            let client_time_zone = Server::get_session()?.get::<i32>("timezone")? * 60;
            let offset = client_time_zone - system_time_zone;
            timestamp = timestamp + (offset * 60) as i64;
        }
        
        let l = L10N::get("lib")?;
        Ok(l.l(if date_only { "date" } else { "datetime" }, timestamp)?)
    }

    /// Comprobar si la configuración actual del servidor es adecuada para ownCloud
    pub fn check_server() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
        // Asumir que si checkServer() tuvo éxito antes en esta sesión, entonces todo está bien.
        if Server::get_session()?.exists("checkServer_suceeded") && 
           Server::get_session()?.get::<bool>("checkServer_suceeded")? {
            return Ok(Vec::new());
        }

        let mut errors = Vec::new();
        let defaults = Defaults::new()?;
        let mut web_server_restart = false;

        // Comprobar los controladores de base de datos
        if !has_database_drivers() {
            errors.push(HashMap::from([
                ("error".to_string(), "No database drivers (sqlite, mysql, or postgresql) installed.".to_string()),
                ("hint".to_string(), "".to_string()), // TODO: sane hint
            ]));
            web_server_restart = true;
        }

        // Consejo común para todos los mensajes de error de permisos de archivo
        let permissions_hint = "Permissions can usually be fixed by giving the webserver write access to the root directory.";

        // Comprobar si el directorio de configuración es escribible
        if !is_writable(&format!("{}/config/", Server::get_server_root())) || 
           !is_readable(&format!("{}/config/", Server::get_server_root())) {
            errors.push(HashMap::from([
                ("error".to_string(), "Can't write into config directory".to_string()),
                ("hint".to_string(), format!("This can usually be fixed by giving the webserver write access to the config directory.")),
            ]));
        }

        // Comprobar si hay una carpeta de instalación escribible
        if Config::get_value("appstoreenabled", true)? {
            match App::get_install_path() {
                None => {
                    errors.push(HashMap::from([
                        ("error".to_string(), "Can't write into apps directory".to_string()),
                        ("hint".to_string(), format!("This can usually be fixed by giving the webserver write access to the apps directory or disabling the appstore in the config file.")),
                    ]));
                },
                Some(path) => {
                    if !is_writable(&path) || !is_readable(&path) {
                        errors.push(HashMap::from([
                            ("error".to_string(), "Can't write into apps directory".to_string()),
                            ("hint".to_string(), format!("This can usually be fixed by giving the webserver write access to the apps directory or disabling the appstore in the config file.")),
                        ]));
                    }
                }
            }
        }

        let config_data_directory = Config::get_value(
            "datadirectory", 
            format!("{}/data", Server::get_server_root())
        )?.to_string();

        // Crear directorio raíz
        if !Path::new(&config_data_directory).is_dir() {
            match fs::create_dir_all(&config_data_directory) {
                Ok(_) => {
                    errors.extend(Self::check_data_directory_permissions(&config_data_directory)?);
                },
                Err(_) => {
                    errors.push(HashMap::from([
                        ("error".to_string(), format!("Can't create data directory ({})", config_data_directory)),
                        ("hint".to_string(), format!("This can usually be fixed by giving the webserver write access to the root directory.")),
                    ]));
                }
            }
        } else if !is_writable(&config_data_directory) || !is_readable(&config_data_directory) {
            errors.push(HashMap::from([
                ("error".to_string(), format!("Data directory ({}) not writable by ownCloud", config_data_directory)),
                ("hint".to_string(), permissions_hint.to_string()),
            ]));
        } else {
            errors.extend(Self::check_data_directory_permissions(&config_data_directory)?);
        }

        let module_hint = "Please ask your server administrator to install the module.";

        // Comprobar si todos los módulos PHP requeridos están presentes
        if !has_zip_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module zip not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_dom_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module dom not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_libxml_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module libxml not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_mb_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module mb multibyte not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_ctype_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module ctype is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_json_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module JSON is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_gd_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module GD is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_zlib_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module zlib is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_iconv_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module iconv is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_simplexml_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP module SimpleXML is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_php_version_5_3() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP 5.3 is required.".to_string()),
                ("hint".to_string(), "Please ask your server administrator to update PHP to version 5.3 or higher. PHP 5.2 is no longer supported by ownCloud and the PHP community.".to_string()),
            ]));
            web_server_restart = true;
        }

        if !has_pdo_module() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP PDO module is not installed.".to_string()),
                ("hint".to_string(), module_hint.to_string()),
            ]));
            web_server_restart = true;
        }

        if is_safe_mode_enabled() {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP Safe Mode is enabled. ownCloud requires that it is disabled to work properly.".to_string()),
                ("hint".to_string(), "PHP Safe Mode is a deprecated and mostly useless setting that should be disabled. Please ask your server administrator to disable it in php.ini or in your webserver config.".to_string()),
            ]));
            web_server_restart = true;
        }

        if is_magic_quotes_enabled() {
            errors.push(HashMap::from([
                ("error".to_string(), "Magic Quotes is enabled. ownCloud requires that it is disabled to work properly.".to_string()),
                ("hint".to_string(), "Magic Quotes is a deprecated and mostly useless setting that should be disabled. Please ask your server administrator to disable it in php.ini or in your webserver config.".to_string()),
            ]));
            web_server_restart = true;
        }

        if web_server_restart {
            errors.push(HashMap::from([
                ("error".to_string(), "PHP modules have been installed, but they are still listed as missing?".to_string()),
                ("hint".to_string(), "Please ask your server administrator to restart the web server.".to_string()),
            ]));
        }

        // Almacenar en caché el resultado de esta función
        Server::get_session()?.set("checkServer_suceeded", errors.is_empty())?;

        Ok(errors)
    }

    /// Comprobar si todavía hay algunos archivos cifrados almacenados
    pub fn encrypted_files() -> Result<bool, Box<dyn std::error::Error>> {
        // Comprobar si el cifrado estaba habilitado en el pasado
        let mut encrypted_files = false;
        
        if !App::is_enabled("files_encryption")? {
            let view = Files::View::new(&format!("/{}", User::get_user()?));
            let keyfile_path = "/files_encryption/keyfiles";
            
            if view.is_dir(keyfile_path)? {
                let dircontent = view.get_directory_content(keyfile_path)?;
                if !dircontent.is_empty() {
                    encrypted_files = true;
                }
            }
        }
        
        Ok(encrypted_files)
    }

    /// Comprobar los permisos correctos del directorio de datos
    pub fn check_data_directory_permissions(data_directory: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
        let mut errors = Vec::new();
        
        if Self::running_on_windows() {
            // TODO: comprobaciones de permisos para hosts Windows
        } else {
            let permissions_mod_hint = "Please change the permissions to 0770 so that the directory cannot be listed by other users.";
            
            let perms = fs::metadata(data_directory)?.permissions().mode() & 0o777;
            let perms_octal = format!("{:o}", perms);
            
            if perms_octal.chars().last().unwrap_or('0') != '0' {
                chmod_r(data_directory, 0o770)?;
                
                let perms = fs::metadata(data_directory)?.permissions().mode() & 0o777;
                let perms_octal = format!("{:o}", perms);
                
                if perms_octal.chars().nth(1).unwrap_or('0') != '0' {
                    errors.push(HashMap::from([
                        ("error".to_string(), format!("Data directory ({}) is readable for other users", data_directory)),
                        ("hint".to_string(), permissions_mod_hint.to_string()),
                    ]));
                }
            }
        }
        
        Ok(errors)
    }

    /// Mostrar la página de inicio de sesión
    pub fn display_login_page(errors: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let mut parameters = HashMap::new();
        
        for error in errors {
            parameters.insert((*error).to_string(), true);
        }
        
        // Obtener el nombre de usuario del formulario
        let username = match std::env::var("POST_user") {
            Ok(user) if !user.is_empty() => {
                parameters.insert("user_autofocus".to_string(), false);
                user
            },
            _ => {
                parameters.insert("user_autofocus".to_string(), true);
                String::new()
            }
        };
        
        parameters.insert("username".to_string(), username);
        
        // Obtener la URL de redirección
        if let Ok(redirect_url) = std::env::var("REQUEST_redirect_url") {
            parameters.insert("redirect_url".to_string(), urlencoding::encode(&redirect_url).to_string());
        }
        
        parameters.insert("alt_login".to_string(), App::get_alternative_logins()?);
        parameters.insert("rememberLoginAllowed".to_string(), Self::remember_login_allowed()?);
        
        Template::print_guest_page("", "login", parameters)?;
        
        Ok(())
    }

    /// Comprobar si la aplicación está habilitada, redirige a inicio si no lo está
    pub fn check_app_enabled(app: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !App::is_enabled(app)? {
            let location = Helper::link_to_absolute("", "index.php");
            Self::redirect(&location)?;
        }
        
        Ok(())
    }

    /// Comprobar si el usuario ha iniciado sesión, redirige a inicio si no.
    /// Con parámetro de URL de redirección a la URI de solicitud.
    pub fn check_logged_in() -> Result<(), Box<dyn std::error::Error>> {
        if !User::is_logged_in()? {
            let mut params = HashMap::new();
            params.insert("redirectUrl".to_string(), Request::request_uri()?);
            
            let location = Helper::link_to_absolute("", "index.php", params);
            Self::redirect(&location)?;
        }
        
        Ok(())
    }

    /// Comprobar si el usuario es administrador, redirige a inicio si no lo es
    pub fn check_admin_user() -> Result<(), Box<dyn std::error::Error>> {
        Self::check_logged_in()?;
        
        if !User::is_admin_user(&User::get_user()?)? {
            let location = Helper::link_to_absolute("", "index.php");
            Self::redirect(&location)?;
        }
        
        Ok(())
    }

    /// Comprobar si está permitido recordar el inicio de sesión.
    pub fn remember_login_allowed() -> Result<bool, Box<dyn std::error::Error>> {
        let apps = App::get_enabled_apps()?;
        
        for app in apps {
            let app_info = App::get_app_info(&app)?;
            
            if let Some(remember_login) = app_info.get("rememberlogin") {
                if remember_login == "false" {
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
    }

    /// Comprobar si el usuario es un subadministrador, redirige a inicio si no lo es
    pub fn check_subadmin_user() -> Result<bool, Box<dyn std::error::Error>> {
        Self::check_logged_in()?;
        
        if !SubAdmin::is_sub_admin(&User::get_user()?)? {
            let location = Helper::link_to_absolute("", "index.php");
            Self::redirect(&location)?;
        }
        
        Ok(true)
    }

    /// Redirigir a la página predeterminada del usuario
    pub fn redirect_to_default_page() -> Result<(), Box<dyn std::error::Error>> {
        let location = if let Ok(redirect_url) = std::env::var("REQUEST_redirect_url") {
            Helper::make_url_absolute(&urlencoding::decode(&redirect_url)?.to_string())?
        } else if let Some(requested_app) = env::var("REQUESTED_APP").ok().filter(|s| !s.is_empty()) {
            Helper::link_to_absolute(&requested_app, "index.php")
        } else {
            let default_page = AppConfig::get_value("core", "defaultpage", "")?;
            
            if !default_page.is_empty() {
                Helper::make_url_absolute(&format!("{}/{}", Server::get_webroot(), default_page))?
            } else {
                Helper::link_to_absolute("files", "index.php")
            }
        };
        
        Log::write("core", &format!("redirectToDefaultPage: {}", location), Log::DEBUG)?;
        Self::redirect(&location)?;
        
        Ok(())
    }

    /// Obtener un ID único para esta instancia
    pub fn get_instance_id() -> Result<String, Box<dyn std::error::Error>> {
        let id = Config::get_value("instanceid", String::new())?;
        
        if id.is_empty() {
            // Necesitamos garantizar al menos una letra en instanceid para que pueda usarse como session_name
            let new_id = format!("oc{}", Self::generate_random_bytes(10)?);
            Config::set_value("instanceid", &new_id)?;
            Ok(new_id)
        } else {
            Ok(

}}} // Añadido por reparador automático