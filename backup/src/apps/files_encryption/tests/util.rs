// Copyright (c) 2012 Sam Tuke <samtuke@owncloud.com>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::Path;
use std::fs;
use std::io::{self, Read};
use std::str;
use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::sync::Arc;

mod encryption {
    pub mod crypt;
    pub mod key_manager;
    pub mod proxy;
    pub mod stream;
    pub mod util;
    pub mod helper;
    pub mod hooks;
}

use crate::encryption::{crypt::Crypt, util::Util};

struct OcFilesystemView {
    base_path: String,
}

impl OcFilesystemView {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }

    fn file_put_contents(&self, path: &str, content: &str) -> Result<usize, io::Error> {
        let full_path = format!("{}{}", self.base_path, path);
        let parent = Path::new(&full_path).parent().unwrap();
        fs::create_dir_all(parent)?;
        fs::write(&full_path, content)?;
        Ok(content.len())
    }

    fn file_get_contents(&self, path: &str) -> Result<String, io::Error> {
        let full_path = format!("{}{}", self.base_path, path);
        let content = fs::read_to_string(full_path)?;
        Ok(content)
    }

    fn unlink(&self, path: &str) -> Result<(), io::Error> {
        let full_path = format!("{}{}", self.base_path, path);
        if Path::new(&full_path).is_dir() {
            fs::remove_dir_all(full_path)?;
        } else {
            fs::remove_file(full_path)?;
        }
        Ok(())
    }

    fn mkdir(&self, path: &str) -> Result<(), io::Error> {
        let full_path = format!("{}{}", self.base_path, path);
        fs::create_dir_all(full_path)?;
        Ok(())
    }

    fn get_file_info(&self, path: &str) -> Result<HashMap<String, serde_json::Value>, io::Error> {
        let full_path = format!("{}{}", self.base_path, path);
        let metadata = fs::metadata(full_path)?;
        
        let mut info = HashMap::new();
        info.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(metadata.len())));
        info.insert("is_dir".to_string(), serde_json::Value::Bool(metadata.is_dir()));
        
        Ok(info)
    }

    fn put_file_info(&self, path: &str, info: HashMap<String, serde_json::Value>) -> Result<(), io::Error> {
        // En un sistema real, esto probablemente actualizaría metadatos
        // Para simplificar, solo devolvemos Ok
        Ok(())
    }
}

struct Session {
    data: HashMap<String, String>,
}

impl Session {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

struct OcUserManager {
    users: HashMap<String, String>,
}

impl OcUserManager {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn create_user(&mut self, username: &str, password: &str) -> bool {
        self.users.insert(username.to_string(), password.to_string());
        true
    }

    fn delete_user(&mut self, username: &str) -> bool {
        self.users.remove(username);
        true
    }

    fn set_user_id(&mut self, user_id: &str) {
        // Simula establecer el ID de usuario actual
    }

    fn clear_backends(&mut self) {
        // Simula limpiar backends
    }

    fn use_backend(&mut self, backend: &str) {
        // Simula usar un backend específico
    }
}

struct OcApp {
    enabled_apps: HashMap<String, bool>,
}

impl OcApp {
    fn new() -> Self {
        Self {
            enabled_apps: HashMap::new(),
        }
    }

    fn is_enabled(&self, app_name: &str) -> bool {
        *self.enabled_apps.get(app_name).unwrap_or(&false)
    }

    fn enable(&mut self, app_name: &str) {
        self.enabled_apps.insert(app_name.to_string(), true);
    }

    fn disable(&mut self, app_name: &str) {
        self.enabled_apps.insert(app_name.to_string(), false);
    }
}

struct OcFileProxy {
    enabled: bool,
    proxies: Vec<Box<dyn Proxy>>,
}

#[async_trait]
trait Proxy: Send + Sync {
    // Método que sería implementado por clases proxy concretas
    async fn intercept(&self) -> bool;
}

impl OcFileProxy {
    fn new() -> Self {
        Self {
            enabled: true,
            proxies: Vec::new(),
        }
    }

    fn clear_proxies(&mut self) {
        self.proxies.clear();
    }

    fn register<T: Proxy + 'static>(&mut self, proxy: T) {
        self.proxies.push(Box::new(proxy));
    }
}

struct OcUtil {
    fs: Arc<OcFilesystemView>,
}

impl OcUtil {
    fn new(fs: Arc<OcFilesystemView>) -> Self {
        Self { fs }
    }

    fn tear_down_fs(&self) {
        // Simula desmontar el sistema de archivos
    }

    fn setup_fs(&self, user: &str) {
        // Simula configurar el sistema de archivos
    }
}

struct Database;

impl Database {
    fn prepare(&self, sql: &str) -> PreparedStatement {
        PreparedStatement::new(sql)
    }
}

struct PreparedStatement {
    sql: String,
}

impl PreparedStatement {
    fn new(sql: &str) -> Self {
        Self { sql: sql.to_string() }
    }

    fn execute(&self, args: Vec<&str>) -> bool {
        // Simula ejecución de consulta SQL
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    
    const TEST_ENCRYPTION_UTIL_USER1: &str = "test-util-user1";
    const TEST_ENCRYPTION_UTIL_LEGACY_USER: &str = "test-legacy-user";

    struct TestEncryptionUtil {
        user_id: String,
        encryption_dir: String,
        public_key_dir: String,
        pass: String,
        view: Arc<OcFilesystemView>,
        keyfiles_path: String,
        public_key_path: String,
        private_key_path: String,
        util: Util,
        data_short: String,
        legacy_encrypted_data: String,
        legacy_encrypted_data_key: String,
        legacy_key: String,
        state_files_trashbin: bool,
        
        // Sistemas simulados
        user_manager: Arc<Mutex<OcUserManager>>,
        app_manager: Arc<Mutex<OcApp>>,
        file_proxy: Arc<Mutex<OcFileProxy>>,
        session: Arc<Mutex<Session>>,
        db: Arc<Database>,
    }

    impl TestEncryptionUtil {
        async fn new() -> Self {
            let user_id = TEST_ENCRYPTION_UTIL_USER1.to_string();
            let pass = TEST_ENCRYPTION_UTIL_USER1.to_string();
            
            let view = Arc::new(OcFilesystemView::new("/"));
            
            let public_key_dir = "/public-keys".to_string();
            let encryption_dir = format!("/{}/files_encryption", user_id);
            let keyfiles_path = format!("{}/keyfiles", encryption_dir);
            let public_key_path = format!("{}/{}.public.key", public_key_dir, user_id);
            let private_key_path = format!("{}/{}.private.key", encryption_dir, user_id);
            
            let util = Util::new(Arc::clone(&view), &user_id);
            
            let user_manager = Arc::new(Mutex::new(OcUserManager::new()));
            let app_manager = Arc::new(Mutex::new(OcApp::new()));
            let file_proxy = Arc::new(Mutex::new(OcFileProxy::new()));
            let session = Arc::new(Mutex::new(Session::new()));
            let db = Arc::new(Database);
            
            Self {
                user_id,
                encryption_dir,
                public_key_dir,
                pass,
                view,
                keyfiles_path,
                public_key_path,
                private_key_path,
                util,
                data_short: "hats".to_string(),
                legacy_encrypted_data: "".to_string(), // Se llenaría desde un archivo
                legacy_encrypted_data_key: "".to_string(), // Se llenaría desde un archivo
                legacy_key: "30943623843030686906\0\0\0\0".to_string(),
                state_files_trashbin: false,
                user_manager,
                app_manager,
                file_proxy,
                session,
                db,
            }
        }
        
        async fn setup_before_class(&self) {
            let mut user_manager = self.user_manager.lock().unwrap();
            user_manager.clear_backends();
            user_manager.use_backend("database");
            drop(user_manager);
            
            // Registrar hooks del sistema de archivos
            encryption::helper::register_filesystem_hooks();
            
            // Limpiar y registrar proxies
            let mut file_proxy = self.file_proxy.lock().unwrap();
            file_proxy.clear_proxies();
            file_proxy.register(encryption::proxy::Proxy::new());
            drop(file_proxy);
            
            // Crear usuarios de prueba
            Self::login_helper(&self.user_manager, &self.session, TEST_ENCRYPTION_UTIL_USER1, true, None).await;
            Self::login_helper(&self.user_manager, &self.session, TEST_ENCRYPTION_UTIL_LEGACY_USER, true, None).await;
        }
        
        async fn setup(&mut self) {
            let mut user_manager = self.user_manager.lock().unwrap();
            user_manager.set_user_id(TEST_ENCRYPTION_UTIL_USER1);
            drop(user_manager);
            
            // Generar keypair
            let keypair = Crypt::create_keypair().await.unwrap();
            
            // Recordar estado de files_trashbin
            let app_manager = self.app_manager.lock().unwrap();
            self.state_files_trashbin = app_manager.is_enabled("files_trashbin");
            drop(app_manager);
            
            // Deshabilitar files_trashbin para las pruebas
            let mut app_manager = self.app_manager.lock().unwrap();
            app_manager.disable("files_trashbin");
        }
        
        async fn tear_down(&self) {
            // Restaurar estado de files_trashbin
            let mut app_manager = self.app_manager.lock().unwrap();
            if self.state_files_trashbin {
                app_manager.enable("files_trashbin");
            } else {
                app_manager.disable("files_trashbin");
            }
        }
        
        async fn tear_down_after_class(&self) {
            // Limpiar usuarios de prueba
            let mut user_manager = self.user_manager.lock().unwrap();
            user_manager.delete_user(TEST_ENCRYPTION_UTIL_USER1);
            user_manager.delete_user(TEST_ENCRYPTION_UTIL_LEGACY_USER);
        }
        
        async fn login_helper(
            user_manager: &Arc<Mutex<OcUserManager>>,
            session: &Arc<Mutex<Session>>,
            user: &str,
            create: bool,
            password: Option<&str>
        ) {
            if create {
                let mut manager = user_manager.lock().unwrap();
                manager.create_user(user, user);
            }
            
            let password = password.unwrap_or(user);
            
            // Simular teardown y setup del sistema de archivos
            let util = OcUtil::new(Arc::new(OcFilesystemView::new("/")));
            util.tear_down_fs();
            
            let mut manager = user_manager.lock().unwrap();
            manager.set_user_id("");
            drop(manager);
            
            util.setup_fs(user);
            
            let mut manager = user_manager.lock().unwrap();
            manager.set_user_id(user);
            drop(manager);
            
            let params = HashMap::from([
                ("uid".to_string(), user.to_string()),
                ("password".to_string(), password.to_string()),
            ]);
            
            encryption::hooks::login(params, session).await;
        }
        
        fn set_migration_status(&self, status: i32, user: &str) -> bool {
            let sql = "UPDATE `*PREFIX*encryption` SET `migration_status` = ? WHERE `uid` = ?";
            let args = vec![&status.to_string(), user];
            
            let statement = self.db.prepare(sql);
            statement.execute(args)
        }
        
        #[tokio::test]
        async fn test_key_paths() {
            let util = Util::new(Arc::clone(&self.view), &self.user_id);
            
            assert_eq!(self.public_key_dir, util.get_path("publicKeyDir"));
            assert_eq!(self.encryption_dir, util.get_path("encryptionDir"));
            assert_eq!(self.keyfiles_path, util.get_path("keyfilesPath"));
            assert_eq!(self.public_key_path, util.get_path("publicKeyPath"));
            assert_eq!(self.private_key_path, util.get_path("privateKeyPath"));
        }
        
        #[tokio::test]
        async fn test_setup_server_side() {
            assert!(self.util.setup_server_side(&self.pass).await.unwrap());
        }
        
        #[tokio::test]
        async fn test_user_is_ready() {
            assert!(self.util.ready().await.unwrap());
        }
        
        #[tokio::test]
        async fn test_is_legacy_user() {
            Self::login_helper(&self.user_manager, &self.session, TEST_ENCRYPTION_UTIL_LEGACY_USER, false, None).await;
            
            let user_view = OcFilesystemView::new(&format!("/{}", TEST_ENCRYPTION_UTIL_LEGACY_USER));
            
            // Deshabilitar proxy de cifrado para evitar llamadas recursivas
            let proxy_status = {
                let file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled
            };
            
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = false;
            }
            
            // Leer contenido de la clave de cifrado heredado
            let mut file = File::open(&self.legacy_encrypted_data_key).await.unwrap();
            let mut encryption_key_content = String::new();
            file.read_to_string(&mut encryption_key_content).await.unwrap();
            
            user_view.file_put_contents("/encryption.key", &encryption_key_content).unwrap();
            
            // Restablecer el estado del proxy
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = proxy_status;
            }
            
            let params = HashMap::from([
                ("uid".to_string(), TEST_ENCRYPTION_UTIL_LEGACY_USER.to_string()),
                ("password".to_string(), TEST_ENCRYPTION_UTIL_LEGACY_USER.to_string()),
            ]);
            
            self.set_migration_status(0, TEST_ENCRYPTION_UTIL_LEGACY_USER);
            
            assert!(encryption::hooks::login(params, &self.session).await.unwrap());
            
            let session = self.session.lock().unwrap();
            assert_eq!(self.legacy_key, session.get("legacyKey").unwrap());
        }
        
        #[tokio::test]
        async fn test_recovery_enabled_for_user() {
            let util = Util::new(Arc::clone(&self.view), &self.user_id);
            
            // Registrar el valor para poder devolverlo a su estado original después
            let enabled = util.recovery_enabled_for_user().await.unwrap();
            
            assert!(util.set_recovery_for_user(1).await.unwrap());
            
            assert_eq!(1, util.recovery_enabled_for_user().await.unwrap());
            
            assert!(util.set_recovery_for_user(0).await.unwrap());
            
            assert_eq!(0, util.recovery_enabled_for_user().await.unwrap());
            
            // Devolver la configuración a su estado anterior
            assert!(util.set_recovery_for_user(enabled).await.unwrap());
        }
        
        #[tokio::test]
        async fn test_get_uid_and_filename() {
            let mut user_manager = self.user_manager.lock().unwrap();
            user_manager.set_user_id(TEST_ENCRYPTION_UTIL_USER1);
            drop(user_manager);
            
            let filename = format!("/tmp-{}.test", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            
            // Deshabilitar proxy de cifrado para evitar llamadas recursivas
            let proxy_status = {
                let file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled
            };
            
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = false;
            }
            
            self.view.file_put_contents(&format!("{}/files{}", self.user_id, filename), &self.data_short).unwrap();
            
            // Reactivar proxy - nuestro trabajo está hecho
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = proxy_status;
            }
            
            let util = Util::new(Arc::clone(&self.view), &self.user_id);
            
            let (file_owner_uid, file) = util.get_uid_and_filename(&filename).await.unwrap();
            
            assert_eq!(TEST_ENCRYPTION_UTIL_USER1, file_owner_uid);
            
            assert_eq!(file, filename);
            
            self.view.unlink(&format!("{}/files{}", self.user_id, filename)).unwrap();
        }
        
        #[tokio::test]
        async fn test_get_file_size() {
            Self::login_helper(&self.user_manager, &self.session, TEST_ENCRYPTION_UTIL_USER1, false, None).await;
            
            let filename = format!("tmp-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            let external_filename = format!("/{}/files/{}", self.user_id, filename);
            
            // Probar con archivos de 0 bytes
            let problematic_file_size_data = "";
            let crypted_file = self.view.file_put_contents(&external_filename, problematic_file_size_data).unwrap();
            assert!(crypted_file >= 0);
            assert_eq!(self.util.get_file_size(&external_filename).await.unwrap(), 0);
            let decrypt = self.view.file_get_contents(&external_filename).unwrap();
            assert_eq!(problematic_file_size_data, decrypt);
            self.view.unlink(&format!("{}/files/{}", self.user_id, filename)).unwrap();
            
            // Probar un archivo con 18377 bytes como en https://github.com/owncloud/mirall/issues/1009
            let problematic_file_size_data = "abc".repeat(18377 / 3);
            let crypted_file = self.view.file_put_contents(&external_filename, &problematic_file_size_data).unwrap();
            assert!(crypted_file >= 0);
            assert_eq!(self.util.get_file_size(&external_filename).await.unwrap(), 18377);
            let decrypt = self.view.file_get_contents(&external_filename).unwrap();
            assert_eq!(problematic_file_size_data, decrypt);
            self.view.unlink(&format!("{}/files/{}", self.user_id, filename)).unwrap();
        }
        
        #[tokio::test]
        async fn test_is_shared_path() {
            let shared_path = "/user1/files/Shared/test";
            let path = "/user1/files/test";
            
            assert!(self.util.is_shared_path(shared_path));
            
            assert!(!self.util.is_shared_path(path));
        }
        
        #[tokio::test]
        async fn test_encrypt_legacy_files() {
            Self::login_helper(&self.user_manager, &self.session, TEST_ENCRYPTION_UTIL_LEGACY_USER, false, None).await;
            
            let user_view = OcFilesystemView::new(&format!("/{}", TEST_ENCRYPTION_UTIL_LEGACY_USER));
            let view = OcFilesystemView::new(&format!("/{}/files", TEST_ENCRYPTION_UTIL_LEGACY_USER));
            
            // Deshabilitar proxy de cifrado para evitar llamadas recursivas
            let proxy_status = {
                let file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled
            };
            
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = false;
            }
            
            // Leer contenido de la clave de cifrado heredado
            let mut file = File::open(&self.legacy_encrypted_data_key).await.unwrap();
            let mut encryption_key_content = String::new();
            file.read_to_string(&mut encryption_key_content).await.unwrap();
            
            user_view.file_put_contents("/encryption.key", &encryption_key_content).unwrap();
            
            // Leer datos encriptados heredados
            let mut file = File::open(&self.legacy_encrypted_data).await.unwrap();
            let mut legacy_encrypted_data = String::new();
            file.read_to_string(&mut legacy_encrypted_data).await.unwrap();
            
            view.mkdir("/test/").unwrap();
            view.mkdir("/test/subtest/").unwrap();
            view.file_put_contents("/test/subtest/legacy-encrypted-text.txt", &legacy_encrypted_data).unwrap();
            
            let mut file_info = view.get_file_info("/test/subtest/legacy-encrypted-text.txt").unwrap();
            file_info.insert("encrypted".to_string(), serde_json::Value::Bool(true));
            view.put_file_info("/test/subtest/legacy-encrypted-text.txt", file_info).unwrap();
            
            // Restablecer el estado del proxy
            {
                let mut file_proxy = self.file_proxy.lock().unwrap();
                file_proxy.enabled = proxy_status;
            }
            
            let params = HashMap::from([
                ("uid".to_string(), TEST_ENCRYPTION_UTIL_LEGACY_USER.to_string()),
                ("password".to_string(), TEST_ENCRYPTION_UTIL_LEGACY_USER.to_string()),
            ]);
            
            let util = Util::new(Arc::clone(&self.view), TEST_ENCRYPTION_UTIL_LEGACY_USER);
            self.set_migration_status(0, TEST_ENCRYPTION_UTIL_LEGACY_USER);
            
            assert!(encryption::hooks::login(params, &self.session).await.unwrap());
            
            let session = self.session.lock().unwrap();
            assert_eq!(self.legacy_key, session.get("legacyKey").unwrap());
            
            let files = util.find_enc_files(&format!("/{}/files/", TEST_ENCRYPTION_UTIL_LEGACY_USER)).await.unwrap();
            
            assert!(files.contains_key("encrypted"));
            
            let mut found = false;
            for encrypted_file in &files["encrypted"] {
                if let Some(name) = encrypted_file.get("name") {
                    if name == "legacy-encrypted-text.txt" {
                        found = true;
                        break;
                    }
                }
            }
            
            assert!(found);
        }
    }
}