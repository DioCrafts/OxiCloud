use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Acceso concedido");
        m.insert("Error configuring Dropbox storage", "Error configurando el almacenamiento de Dropbox");
        m.insert("Grant access", "Conceder acceso");
        m.insert("Please provide a valid Dropbox app key and secret.", "Por favor, proporcione un una clave válida de la app Dropbox y una clave secreta.");
        m.insert("Error configuring Google Drive storage", "Error configurando el almacenamiento de Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El cliente smb (smbclient) no se encuentra instalado. El montado de archivos o ficheros CIFS/SMB no es posible. Por favor pida al administrador de su sistema que lo instale.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El soporte de FTP en PHP no se encuentra instalado. El montado de archivos o ficheros FTP no es posible. Por favor pida al administrador de su sistema que lo instale.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El soporte de Curl en PHP no está activado ni instalado. El montado de ownCloud, WebDAV o GoogleDrive no es posible. Pida al administrador de su sistema que lo instale.");
        m.insert("External Storage", "Almacenamiento externo");
        m.insert("Folder name", "Nombre de la carpeta");
        m.insert("External storage", "Almacenamiento externo");
        m.insert("Configuration", "Configuración");
        m.insert("Options", "Opciones");
        m.insert("Applicable", "Aplicable");
        m.insert("Add storage", "Añadir almacenamiento");
        m.insert("None set", "No se ha configurado");
        m.insert("All Users", "Todos los usuarios");
        m.insert("Groups", "Grupos");
        m.insert("Users", "Usuarios");
        m.insert("Delete", "Eliminar");
        m.insert("Enable User External Storage", "Habilitar almacenamiento externo de usuario");
        m.insert("Allow users to mount their own external storage", "Permitir a los usuarios montar su propio almacenamiento externo");
        m.insert("SSL root certificates", "Certificados raíz SSL");
        m.insert("Import Root Certificate", "Importar certificado raíz");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}