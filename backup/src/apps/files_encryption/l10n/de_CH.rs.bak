use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Recovery key successfully enabled", "Der Wiederherstellungsschlüssel wurde erfolgreich aktiviert.");
        m.insert("Could not enable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht aktiviert werden. Bitte überprüfen Sie das Passwort für den Wiederherstellungsschlüssel!");
        m.insert("Recovery key successfully disabled", "Der Wiederherstellungsschlüssel wurde erfolgreich deaktiviert.");
        m.insert("Could not disable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht deaktiviert werden. Bitte überprüfen Sie das Passwort für den Wiederherstellungsschlüssel!");
        m.insert("Password successfully changed.", "Das Passwort wurde erfolgreich geändert.");
        m.insert("Could not change the password. Maybe the old password was not correct.", "Das Passwort konnte nicht geändert werden. Vielleicht war das alte Passwort nicht richtig.");
        m.insert("Private key password successfully updated.", "Das Passwort des privaten Schlüssels wurde erfolgreich aktualisiert.");
        m.insert("Could not update the private key password. Maybe the old password was not correct.", "Das Passwort des privaten Schlüssels konnte nicht aktualisiert werden. Vielleicht war das alte Passwort nicht richtig.");
        m.insert("Missing requirements.", "Fehlende Voraussetzungen");
        m.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Bitte stellen Sie sicher, dass PHP 5.3.3 oder neuer installiert und das OpenSSL zusammen mit der PHP-Erweiterung aktiviert und richtig konfiguriert ist. Zur Zeit ist die Verschlüsselungs-App deaktiviert.");
        m.insert("Following users are not set up for encryption:", "Für folgende Nutzer ist keine Verschlüsselung eingerichtet:");
        m.insert("Saving...", "Speichern...");
        m.insert("personal settings", "Persönliche Einstellungen");
        m.insert("Encryption", "Verschlüsselung");
        m.insert("Enable recovery key (allow to recover users files in case of password loss):", "Aktivieren Sie den Wiederherstellungsschlüssel (erlaubt die Wiederherstellung des Zugangs zu den Benutzerdateien, wenn das Passwort verloren geht).");
        m.insert("Recovery key password", "Wiederherstellungschlüsselpasswort");
        m.insert("Enabled", "Aktiviert");
        m.insert("Disabled", "Deaktiviert");
        m.insert("Change recovery key password:", "Wiederherstellungsschlüsselpasswort ändern");
        m.insert("Old Recovery key password", "Altes Wiederherstellungsschlüsselpasswort");
        m.insert("New Recovery key password", "Neues Wiederherstellungsschlüsselpasswort ");
        m.insert("Change Password", "Passwort ändern");
        m.insert("Your private key password no longer match your log-in password:", "Das Privatschlüsselpasswort darf nicht länger mit den Login-Passwort übereinstimmen.");
        m.insert("Set your old private key password to your current log-in password.", "Setzen Sie Ihr altes Privatschlüsselpasswort auf Ihr aktuelles LogIn-Passwort.");
        m.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Falls Sie sich nicht an Ihr altes Passwort erinnern können, fragen Sie bitte Ihren Administrator, um Ihre Dateien wiederherzustellen.");
        m.insert("Old log-in password", "Altes Login-Passwort");
        m.insert("Current log-in password", "Momentanes Login-Passwort");
        m.insert("Update Private Key Password", "Das Passwort des privaten Schlüssels aktualisieren");
        m.insert("Enable password recovery:", "Die Passwort-Wiederherstellung aktivieren:");
        m.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Durch die Aktivierung dieser Option haben Sie die Möglichkeit, wieder auf Ihre verschlüsselten Dateien zugreifen zu können, wenn Sie Ihr Passwort verloren haben.");
        m.insert("File recovery settings updated", "Die Einstellungen für die Dateiwiederherstellung wurden aktualisiert.");
        m.insert("Could not update file recovery", "Die Dateiwiederherstellung konnte nicht aktualisiert werden.");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}