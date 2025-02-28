use std::collections::HashMap;

// File: apps/files_external/l10n/eu.rs

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Access granted", "Sarrera baimendua");
    translations.insert("Error configuring Dropbox storage", "Errore bat egon da Dropbox biltegiratzea konfiguratzean");
    translations.insert("Grant access", "Baimendu sarrera");
    translations.insert("Please provide a valid Dropbox app key and secret.", "Mesedez eman baliozkoa den Dropbox app giltza eta sekretua");
    translations.insert("Error configuring Google Drive storage", "Errore bat egon da Google Drive biltegiratzea konfiguratzean");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Abisua:</b> \"smbclient\" ez dago instalatuta. CIFS/SMB partekatutako karpetak montatzea ez da posible. Mesedez eskatu zure sistema kudeatzaileari instalatzea.");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Abisua:</b> PHPren FTP modulua ez dago instalatuta edo gaitua. FTP partekatutako karpetak montatzea ez da posible. Mesedez eskatu zure sistema kudeatzaileari instalatzea.");
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Abisua:</b> Curl euskarri PHP modulua ez dago instalatuta edo gaitua. Ezinezko da ownCloud /WebDAV GoogleDrive-n muntatzea. Mesedez eskatu sistema kudeatzaileari instala dezan. ");
    translations.insert("External Storage", "Kanpoko Biltegiratzea");
    translations.insert("Folder name", "Karpetaren izena");
    translations.insert("External storage", "Kanpoko biltegiratzea");
    translations.insert("Configuration", "Konfigurazioa");
    translations.insert("Options", "Aukerak");
    translations.insert("Applicable", "Aplikagarria");
    translations.insert("Add storage", "Gehitu biltegiratzea");
    translations.insert("None set", "Ezarri gabe");
    translations.insert("All Users", "Erabiltzaile guztiak");
    translations.insert("Groups", "Taldeak");
    translations.insert("Users", "Erabiltzaileak");
    translations.insert("Delete", "Ezabatu");
    translations.insert("Enable User External Storage", "Gaitu erabiltzaileentzako Kanpo Biltegiratzea");
    translations.insert("Allow users to mount their own external storage", "Baimendu erabiltzaileak bere kanpo biltegiratzeak muntatzen");
    translations.insert("SSL root certificates", "SSL erro ziurtagiriak");
    translations.insert("Import Root Certificate", "Inportatu Erro Ziurtagiria");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}