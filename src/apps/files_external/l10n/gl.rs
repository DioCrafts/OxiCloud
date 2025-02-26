use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Concedeuse acceso");
        m.insert("Error configuring Dropbox storage", "Produciuse un erro ao configurar o almacenamento en Dropbox");
        m.insert("Grant access", "Permitir o acceso");
        m.insert("Please provide a valid Dropbox app key and secret.", "Forneza unha chave correcta e segreda do Dropbox.");
        m.insert("Error configuring Google Drive storage", "Produciuse un erro ao configurar o almacenamento en Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Aviso:</b> «smbclient» non está instalado. Non é posibel a montaxe de comparticións CIFS/SMB. Consulte co administrador do sistema para instalalo.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Aviso:</b> A compatibilidade de FTP en PHP non está activada ou instalada. Non é posibel a montaxe de comparticións FTP. Consulte co administrador do sistema para instalalo.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Aviso:</ b> A compatibilidade de Curl en PHP non está activada ou instalada. Non é posíbel a montaxe de ownCloud / WebDAV ou GoogleDrive. Consulte co administrador do sistema para instalala.");
        m.insert("External Storage", "Almacenamento externo");
        m.insert("Folder name", "Nome do cartafol");
        m.insert("External storage", "Almacenamento externo");
        m.insert("Configuration", "Configuración");
        m.insert("Options", "Opcións");
        m.insert("Applicable", "Aplicábel");
        m.insert("Add storage", "Engadir almacenamento");
        m.insert("None set", "Ningún definido");
        m.insert("All Users", "Todos os usuarios");
        m.insert("Groups", "Grupos");
        m.insert("Users", "Usuarios");
        m.insert("Delete", "Eliminar");
        m.insert("Enable User External Storage", "Activar o almacenamento externo do usuario");
        m.insert("Allow users to mount their own external storage", "Permitir aos usuarios montar os seus propios almacenamentos externos");
        m.insert("SSL root certificates", "Certificados SSL root");
        m.insert("Import Root Certificate", "Importar o certificado root");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}