use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Chiave di ripristino abilitata correttamente");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Impossibile abilitare la chiave di ripristino. Verifica la password della chiave di ripristino.");
        m.insert("Recovery key successfully disabled", "Chiave di ripristinata disabilitata correttamente");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Impossibile disabilitare la chiave di ripristino. Verifica la password della chiave di ripristino.");
        m.insert("Password successfully changed.", "Password modificata correttamente.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Impossibile cambiare la password. Forse la vecchia password non era corretta.");
        m.insert("Private key password successfully updated.", "Password della chiave privata aggiornata correttamente.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Impossibile aggiornare la password della chiave privata. Forse la vecchia password non era corretta.");
        m.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Applicazione di cifratura non inizializzata. Forse l'applicazione è stata riabilitata durante la tua sessione. Prova a disconnetterti e ad effettuare nuovamente l'accesso per inizializzarla.");
        m.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "La tua chiave privata non è valida! Forse la password è stata cambiata al di fuori di %s (ad es. la directory aziendale). Puoi aggiornare la password della chiave privata nelle impostazioni personali per ottenere nuovamente l'accesso ai file cifrati.");
        m.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Impossibile decifrare questo file, probabilmente è un file condiviso. Chiedi al proprietario del file di condividere nuovamente il file con te.");
        m.insert("Unknown error please check your system settings or contact your administrator", "Errore sconosciuto, controlla le impostazioni di sistema o contatta il tuo amministratore");
        m.insert("Missing requirements.", "Requisiti mancanti.");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Assicurati che sia installato PHP 5.3.3 o versioni successive e che l'estensione OpenSSL di PHP sia abilitata e configurata correttamente. Per ora, l'applicazione di cifratura è disabilitata.");
        m.insert("Following users are not set up for encryption:", "I seguenti utenti non sono configurati per la cifratura:");
        m.insert("Saving...", "Salvataggio in corso...");
        m.insert("Go directly to your ", "Passa direttamente a");
        m.insert("personal settings", "impostazioni personali");
        m.insert("Encryption", "Cifratura");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Abilita la chiave di recupero (permette di recuperare i file utenti in caso di perdita della password):");
        m.insert("Recovery key password", "Password della chiave di recupero");
        m.insert("Repeat Recovery key password", "Ripeti la password della chiave di recupero");
        m.insert("Enabled", "Abilitata");
        m.insert("Disabled", "Disabilitata");
        m.insert("Change recovery key password:", "Cambia la password della chiave di recupero:");
        m.insert("Old Recovery key password", "Vecchia password della chiave di recupero");
        m.insert("New Recovery key password", "Nuova password della chiave di recupero");
        m.insert("Repeat New Recovery key password", "Ripeti la nuova password della chiave di recupero");
        m.insert("Change Password", "Modifica password");
        m.insert("Your private key password no longer match your log-in password:", "La password della chiave privata non corrisponde più alla password di accesso:");
        m.insert("Set your old private key password to your current log-in password.", "Imposta la vecchia password della chiave privata sull'attuale password di accesso.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Se non ricordi la vecchia password puoi chiedere al tuo amministratore di recuperare i file.");
        m.insert("Old log-in password", "Vecchia password di accesso");
        m.insert("Current log-in password", "Password di accesso attuale");
        m.insert("Update Private Key Password", "Aggiorna la password della chiave privata");
        m.insert("Enable password recovery:", "Abilita il ripristino della password:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "L'abilitazione di questa opzione ti consentirà di accedere nuovamente ai file cifrati in caso di perdita della password");
        m.insert("File recovery settings updated", "Impostazioni di ripristino dei file aggiornate");
        m.insert("Could not update file recovery", "Impossibile aggiornare il ripristino dei file");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}