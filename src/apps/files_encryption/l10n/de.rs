use std::collections::HashMap;
use once_cell::sync::Lazy;

/// German language translations for the files_encryption app
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Recovery key successfully enabled", "Wiederherstellungsschlüssel wurde erfolgreich aktiviert");
    map.insert("Could not enable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht aktiviert werden. Überprüfen Sie Ihr Wiederherstellungspasswort!");
    map.insert("Recovery key successfully disabled", "Wiederherstellungsschlüssel deaktiviert.");
    map.insert("Could not disable recovery key. Please check your recovery key password!", "Der Wiederherstellungsschlüssel konnte nicht deaktiviert werden. Überprüfen Sie Ihr Wiederherstellungspasswort!");
    map.insert("Password successfully changed.", "Dein Passwort wurde geändert.");
    map.insert("Could not change the password. Maybe the old password was not correct.", "Das Passwort konnte nicht geändert werden. Vielleicht war das alte Passwort falsch.");
    map.insert("Private key password successfully updated.", "Passwort des privaten Schlüssels erfolgreich aktualisiert");
    map.insert("Could not update the private key password. Maybe the old password was not correct.", "Das Passwort des privaten Schlüssels konnte nicht aktualisiert werden. Eventuell war das alte Passwort falsch.");
    map.insert("Encryption app not initialized! Maybe the encryption app was re-enabled during your session. Please try to log out and log back in to initialize the encryption app.", "Verschlüsselung-App ist nicht initialisiert! Vielleicht wurde die Verschlüsselung-App in der aktuellen Sitzung reaktiviert. Bitte versuche Dich ab- und wieder anzumelden, um die Verschlüsselung-App zu initialisieren.");
    map.insert("Your private key is not valid! Likely your password was changed outside of %s (e.g. your corporate directory). You can update your private key password in your personal settings to recover access to your encrypted files.", "Dein privater Schlüssel ist ungültig. Möglicher Weise wurde außerhalb von%s Dein Passwort geändert (z.B. in deinem gemeinsamen Verzeichnis). Du kannst das Passwort deines privaten Schlüssels in den persönlichen Einstellungen aktualisieren, um wieder an deine Dateien zu gelangen.");
    map.insert("Can not decrypt this file, probably this is a shared file. Please ask the file owner to reshare the file with you.", "Die Datei kann nicht entschlüsselt werden, da die Datei möglicherweise eine geteilte Datei ist. Bitte frage den Datei-Besitzer, dass er die Datei nochmals mit Dir teilt.");
    map.insert("Unknown error please check your system settings or contact your administrator", "Unbekannter Fehler, bitte prüfe Deine Systemeinstellungen oder kontaktiere Deinen Administrator");
    map.insert("Missing requirements.", "Fehlende Vorraussetzungen");
    map.insert("Please make sure that PHP 5.3.3 or newer is installed and that OpenSSL together with the PHP extension is enabled and configured properly. For now, the encryption app has been disabled.", "Bitte stelle sicher, dass PHP 5.3.3 oder neuer installiert und das OpenSSL zusammen mit der PHP-Erweiterung aktiviert und richtig konfiguriert ist. Zur Zeit ist die Verschlüsselungs-App deaktiviert.");
    map.insert("Following users are not set up for encryption:", "Für folgende Nutzer ist keine Verschlüsselung eingerichtet:");
    map.insert("Saving...", "Speichern...");
    map.insert("Go directly to your ", "Direkt wechseln zu Deinem");
    map.insert("personal settings", "Private Einstellungen");
    map.insert("Encryption", "Verschlüsselung");
    map.insert("Enable recovery key (allow to recover users files in case of password loss):", "Wiederherstellungsschlüssel aktivieren (ermöglicht das Wiederherstellen von Dateien, falls das Passwort vergessen wurde):");
    map.insert("Recovery key password", "Wiederherstellungsschlüssel-Passwort");
    map.insert("Repeat Recovery key password", "Schlüssel-Passwort zur Wiederherstellung wiederholen");
    map.insert("Enabled", "Aktiviert");
    map.insert("Disabled", "Deaktiviert");
    map.insert("Change recovery key password:", "Wiederherstellungsschlüssel-Passwort ändern:");
    map.insert("Old Recovery key password", "Altes Wiederherstellungsschlüssel-Passwort");
    map.insert("New Recovery key password", "Neues Wiederherstellungsschlüssel-Passwort");
    map.insert("Repeat New Recovery key password", "Neues Schlüssel-Passwort zur Wiederherstellung wiederholen");
    map.insert("Change Password", "Passwort ändern");
    map.insert("Your private key password no longer match your log-in password:", "Ihr Passwort für ihren privaten Schlüssel stimmt nicht mehr mit ihrem Loginpasswort überein.");
    map.insert("Set your old private key password to your current log-in password.", "Setzen Sie ihr altes Passwort für ihren privaten Schlüssel auf ihr aktuelles Login-Passwort");
    map.insert(" If you don't remember your old password you can ask your administrator to recover your files.", "Wenn Sie Ihr altes Passwort vergessen haben, können Sie den Administrator bitten, Ihre Daten wiederherzustellen.");
    map.insert("Old log-in password", "Altes login Passwort");
    map.insert("Current log-in password", "Aktuelles Passwort");
    map.insert("Update Private Key Password", "Passwort für den privaten Schlüssel aktualisieren");
    map.insert("Enable password recovery:", "Passwortwiederherstellung aktivvieren:");
    map.insert("Enabling this option will allow you to reobtain access to your encrypted files in case of password loss", "Wenn Sie diese Option aktivieren, können Sie Ihre verschlüsselten Dateien wiederherstellen, falls Sie Ihr Passwort vergessen");
    map.insert("File recovery settings updated", "Einstellungen zur Wiederherstellung von Dateien wurden aktualisiert");
    map.insert("Could not update file recovery", "Dateiwiederherstellung konnte nicht aktualisiert werden");
    map
});

/// Define the plural forms for the German language
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Translates a string to German
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

/// Gets the plural form for a number
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}