use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Accesso consentito");
        m.insert("Error configuring Dropbox storage", "Errore durante la configurazione dell'archivio Dropbox");
        m.insert("Grant access", "Concedi l'accesso");
        m.insert("Please provide a valid Dropbox app key and secret.", "Fornisci chiave di applicazione e segreto di Dropbox validi.");
        m.insert("Error configuring Google Drive storage", "Errore durante la configurazione dell'archivio Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", 
                 "<b>Avviso:</b> \"smbclient\" non è installato. Impossibile montare condivisioni CIFS/SMB. Chiedi all'amministratore di sistema di installarlo.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", 
                 "<b>Avviso:</b> il supporto FTP di PHP non è abilitato o non è installato. Impossibile montare condivisioni FTP. Chiedi all'amministratore di sistema di installarlo.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", 
                 "<b>Avviso:</b> il supporto Curl di PHP non è abilitato o non è installato. Impossibile montare condivisioni ownCloud / WebDAV o GoogleDrive. Chiedi all'amministratore di sistema di installarlo.");
        m.insert("External Storage", "Archiviazione esterna");
        m.insert("Folder name", "Nome della cartella");
        m.insert("External storage", "Archiviazione esterna");
        m.insert("Configuration", "Configurazione");
        m.insert("Options", "Opzioni");
        m.insert("Applicable", "Applicabile");
        m.insert("Add storage", "Aggiungi archiviazione");
        m.insert("None set", "Nessuna impostazione");
        m.insert("All Users", "Tutti gli utenti");
        m.insert("Groups", "Gruppi");
        m.insert("Users", "Utenti");
        m.insert("Delete", "Elimina");
        m.insert("Enable User External Storage", "Abilita la memoria esterna dell'utente");
        m.insert("Allow users to mount their own external storage", "Consenti agli utenti di montare la propria memoria esterna");
        m.insert("SSL root certificates", "Certificati SSL radice");
        m.insert("Import Root Certificate", "Importa certificato radice");
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