use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "S'ha concedit l'accés");
    m.insert("Error configuring Dropbox storage", "Error en configurar l'emmagatzemament Dropbox");
    m.insert("Grant access", "Concedeix accés");
    m.insert("Please provide a valid Dropbox app key and secret.", "Proporcioneu una clau d'aplicació i secret vàlids per a Dropbox");
    m.insert("Error configuring Google Drive storage", "Error en configurar l'emmagatzemament Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Avís:</b> \"smbclient\" no està instal·lat. No es pot muntar la compartició CIFS/SMB. Demaneu a l'administrador del sistema que l'instal·li.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Avís:</b> El suport FTP per PHP no està activat o no està instal·lat. No es pot muntar la compartició FTP. Demaneu a l'administrador del sistema que l'instal·li.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Avís:</b>El suport Curl de PHP no està activat o instal·lat. No es pot muntar ownCloud / WebDAV o GoogleDrive. Demaneu a l'administrador que l'instal·li.");
    m.insert("External Storage", "Emmagatzemament extern");
    m.insert("Folder name", "Nom de la carpeta");
    m.insert("External storage", "Emmagatzemament extern");
    m.insert("Configuration", "Configuració");
    m.insert("Options", "Options");
    m.insert("Applicable", "Aplicable");
    m.insert("Add storage", "Afegeix emmagatzemament");
    m.insert("None set", "Cap d'establert");
    m.insert("All Users", "Tots els usuaris");
    m.insert("Groups", "Grups");
    m.insert("Users", "Usuaris");
    m.insert("Delete", "Esborra");
    m.insert("Enable User External Storage", "Habilita l'emmagatzemament extern d'usuari");
    m.insert("Allow users to mount their own external storage", "Permet als usuaris muntar el seu emmagatzemament extern propi");
    m.insert("SSL root certificates", "Certificats SSL root");
    m.insert("Import Root Certificate", "Importa certificat root");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";