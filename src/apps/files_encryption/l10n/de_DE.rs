use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Recovery key successfully enabled", "Der Wiederherstellungsschlüssel wurde erfolgreich aktiviert.");
    map.insert("Could not enable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht aktiviert werden. Bitte überprüfen Sie das Passwort für den Wiederherstellungsschlüssel!");
    map.insert("Recovery key successfully disabled", "Der Wiederherstellungsschlüssel wurde erfolgreich deaktiviert.");
    map.insert("Could not disable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht deaktiviert werden. Bitte überprüfen Sie das Passwort für den Wiederherstellungsschlüssel!");
    map.insert("Password successfully changed.", "Das Passwort wurde erfolgreich geändert.");
    map.insert("Could not change the password. Maybe the old password was not correct.", "Das Passwort konnte nicht geändert werden. Vielleicht war das alte Passwort nicht richtig.");
    map.insert("Private key password successfully updated.", "Das Passwort des privaten Schlüssels wurde erfolgreich aktualisiert.");
    map.insert("Could not update the private key password. Maybe the old password was not correct.", "Das Passwort des privaten Schlüssels konnte nicht aktualisiert werden. Vielleicht war das alte Passwort nicht richtig.");
    map.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Verschlüsselung-App ist nicht initialisiert! Vielleicht wurde die Verschlüsselung-App in der aktuellen Sitzung reaktiviert. Bitte versuchen Sie sich ab- und wieder anzumelden, um die Verschlüsselung-App zu initialisieren.");
    map.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Ihr privater Schlüssel ist ungültig. Möglicher Weise wurde außerhalb von %s Ihr Passwort geändert (z.B. in Ihrem gemeinsamen Verzeichnis). Sie können das Passwort Ihres privaten Schlüssels in den persönlichen Einstellungen aktualisieren, um wieder an Ihre Dateien zu gelangen.");
    map.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Die Datei kann nicht entschlüsselt werden, da die Datei möglicherweise eine geteilte Datei ist. Bitte fragen Sie den Datei-Besitzer, dass er die Datei nochmals mit Ihnen teilt.");
    map.insert("Unknown error please check your system settings or contact your administrator", "Unbekannter Fehler, bitte prüfen Sie die Systemeinstellungen oder kontaktieren Sie Ihren Administrator");
    map.insert("Missing requirements.", "Fehlende Voraussetzungen");
    map.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Bitte stellen Sie sicher, dass PHP 5.3.3 oder neuer installiert und das OpenSSL zusammen mit der PHP-Erweiterung aktiviert und richtig konfiguriert ist. Zur Zeit ist die Verschlüsselungs-App deaktiviert.");
    map.insert("Following users are not set up for encryption:", "Für folgende Nutzer ist keine Verschlüsselung eingerichtet:");
    map.insert("Saving...", "Speichern...");
    map.insert("Go directly to your ", "Direkt wechseln zu Ihrem");
    map.insert("personal settings", "Persönliche Einstellungen");
    map.insert("Encryption", "Verschlüsselung");
    map.insert("Enable recovery key (allow to recover users files in case of password loss):", "Aktivieren Sie den Wiederherstellungsschlüssel (erlaubt die Wiederherstellung des Zugangs zu den Benutzerdateien, wenn das Passwort verloren geht).");
    map.insert("Recovery key password", "Wiederherstellungschlüsselpasswort");
    map.insert("Repeat Recovery key password", "Schlüssel-Passwort zur Wiederherstellung wiederholen");
    map.insert("Enabled", "Aktiviert");
    map.insert("Disabled", "Deaktiviert");
    map.insert("Change recovery key password:", "Wiederherstellungsschlüsselpasswort ändern");
    map.insert("Old Recovery key password", "Altes Wiederherstellungsschlüsselpasswort");
    map.insert("New Recovery key password", "Neues Wiederherstellungsschlüsselpasswort ");
    map.insert("Repeat New Recovery key password", "Neues Schlüssel-Passwort zur Wiederherstellung wiederholen");
    map.insert("Change Password", "Passwort ändern");
    map.insert("Your private key password no longer match your log-in password:", "Das Privatschlüsselpasswort darf nicht länger mit den Login-Passwort übereinstimmen.");
    map.insert("Set your old private key password to your current log-in password.", "Setzen Sie Ihr altes Privatschlüsselpasswort auf Ihr aktuelles LogIn-Passwort.");
    map.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Falls Sie sich nicht an Ihr altes Passwort erinnern können, fragen Sie bitte Ihren Administrator, um Ihre Dateien wiederherzustellen.");
    map.insert("Old log-in password", "Altes Login-Passwort");
    map.insert("Current log-in password", "Momentanes Login-Passwort");
    map.insert("Update Private Key Password", "Das Passwort des privaten Schlüssels aktualisieren");
    map.insert("Enable password recovery:", "Die Passwort-Wiederherstellung aktivieren:");
    map.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Durch die Aktivierung dieser Option haben Sie die Möglichkeit, wieder auf Ihre verschlüsselten Dateien zugreifen zu können, wenn Sie Ihr Passwort verloren haben.");
    map.insert("File recovery settings updated", "Die Einstellungen für die Dateiwiederherstellung wurden aktualisiert.");
    map.insert("Could not update file recovery", "Die Dateiwiederherstellung konnte nicht aktualisiert werden.");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}