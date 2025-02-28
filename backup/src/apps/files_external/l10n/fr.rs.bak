use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Accès autorisé");
        m.insert("Error configuring Dropbox storage", "Erreur lors de la configuration du support de stockage Dropbox");
        m.insert("Grant access", "Autoriser l'accès");
        m.insert("Please provide a valid Dropbox app key and secret.", "Veuillez fournir une clé d'application (app key) ainsi qu'un mot de passe valides.");
        m.insert("Error configuring Google Drive storage", "Erreur lors de la configuration du support de stockage Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Attention : </b> \"smbclient\" n'est pas installé. Le montage des partages CIFS/SMB n'est pas disponible. Contactez votre administrateur système pour l'installer.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Attention : </b> Le support FTP de PHP n'est pas activé ou installé. Le montage des partages FTP n'est pas disponible. Contactez votre administrateur système pour l'installer.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Attention :</b> Le support de Curl n'est pas activé ou installé dans PHP. Le montage de ownCloud / WebDAV ou GoogleDrive n'est pas possible. Contactez votre administrateur système pour l'installer.");
        m.insert("External Storage", "Stockage externe");
        m.insert("Folder name", "Nom du dossier");
        m.insert("External storage", "Stockage externe");
        m.insert("Configuration", "Configuration");
        m.insert("Options", "Options");
        m.insert("Applicable", "Disponible");
        m.insert("Add storage", "Ajouter un support de stockage");
        m.insert("None set", "Aucun spécifié");
        m.insert("All Users", "Tous les utilisateurs");
        m.insert("Groups", "Groupes");
        m.insert("Users", "Utilisateurs");
        m.insert("Delete", "Supprimer");
        m.insert("Enable User External Storage", "Activer le stockage externe pour les utilisateurs");
        m.insert("Allow users to mount their own external storage", "Autoriser les utilisateurs à monter leur propre stockage externe");
        m.insert("SSL root certificates", "Certificats racine SSL");
        m.insert("Import Root Certificate", "Importer un certificat racine");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n > 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}