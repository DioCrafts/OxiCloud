use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Spanish (Argentina) translations for files_external
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Acceso permitido");
    m.insert("Error configuring Dropbox storage", "Error al configurar el almacenamiento de Dropbox");
    m.insert("Grant access", "Permitir acceso");
    m.insert("Please provide a valid Dropbox app key and secret.", "Por favor, proporcioná un secreto y una contraseña válida para la aplicación Dropbox.");
    m.insert("Error configuring Google Drive storage", "Error al configurar el almacenamiento de Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El cliente smb \"smbclient\" no está instalado. Montar archivos CIFS/SMB no es posible. Por favor, pedile al administrador de tu sistema que lo instale.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El soporte de FTP en PHP no está instalado. Montar archivos FTP no es posible. Por favor, pedile al administrador de tu sistema que lo instale.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Advertencia:</b> El soporte de Curl de PHP no está activo ni instalado. Montar servicios ownCloud, WebDAV y/o GoogleDrive no será posible. Pedile al administrador del sistema que lo instale.");
    m.insert("External Storage", "Almacenamiento externo");
    m.insert("Folder name", "Nombre de la carpeta");
    m.insert("External storage", "Almacenamiento externo");
    m.insert("Configuration", "Configuración");
    m.insert("Options", "Opciones");
    m.insert("Applicable", "Aplicable");
    m.insert("Add storage", "Añadir almacenamiento");
    m.insert("None set", "No fue configurado");
    m.insert("All Users", "Todos los usuarios");
    m.insert("Groups", "Grupos");
    m.insert("Users", "Usuarios");
    m.insert("Delete", "Borrar");
    m.insert("Enable User External Storage", "Habilitar almacenamiento de usuario externo");
    m.insert("Allow users to mount their own external storage", "Permitir a los usuarios montar su propio almacenamiento externo");
    m.insert("SSL root certificates", "certificados SSL raíz");
    m.insert("Import Root Certificate", "Importar certificado raíz");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";