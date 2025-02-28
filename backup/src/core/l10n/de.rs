use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static DE_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("%s shared »%s« with you", "%s teilte »%s« mit Ihnen");
    translations.insert("Couldn't send mail to following users: %s ", "Die E-Mail konnte nicht an folgende Benutzer gesendet werden: %s");
    translations.insert("Turned on maintenance mode", "Wartungsmodus eingeschaltet");
    translations.insert("Turned off maintenance mode", "Wartungsmodus ausgeschaltet");
    translations.insert("Updated database", "Datenbank aktualisiert");
    translations.insert("Updating filecache, this may take really long...", "Aktualisiere Dateicache, dies könnte eine Weile dauern...");
    translations.insert("Updated filecache", "Dateicache aktualisiert");
    translations.insert("... %d%% done ...", "... %d%% erledigt ...");
    translations.insert("No image or file provided", "Kein Bild oder Datei zur Verfügung gestellt");
    translations.insert("Unknown filetype", "Unbekannter Dateityp");
    translations.insert("Invalid image", "Ungültiges Bild");
    translations.insert("No temporary profile picture available, try again", "Kein temporäres Profilbild verfügbar, bitte versuche es nochmal");
    translations.insert("No crop data provided", "Keine Zuschnittdaten zur Verfügung gestellt");
    translations.insert("Sunday", "Sonntag");
    translations.insert("Monday", "Montag");
    translations.insert("Tuesday", "Dienstag");
    translations.insert("Wednesday", "Mittwoch");
    translations.insert("Thursday", "Donnerstag");
    translations.insert("Friday", "Freitag");
    translations.insert("Saturday", "Samstag");
    translations.insert("January", "Januar");
    translations.insert("February", "Februar");
    translations.insert("March", "März");
    translations.insert("April", "April");
    translations.insert("May", "Mai");
    translations.insert("June", "Juni");
    translations.insert("July", "Juli");
    translations.insert("August", "August");
    translations.insert("September", "September");
    translations.insert("October", "Oktober");
    translations.insert("November", "November");
    translations.insert("December", "Dezember");
    translations.insert("Settings", "Einstellungen");
    translations.insert("seconds ago", "Gerade eben");
    translations.insert("today", "Heute");
    translations.insert("yesterday", "Gestern");
    translations.insert("last month", "Letzten Monat");
    translations.insert("months ago", "Vor Monaten");
    translations.insert("last year", "Letztes Jahr");
    translations.insert("years ago", "Vor Jahren");
    translations.insert("Choose", "Auswählen");
    translations.insert("Error loading file picker template: {error}", "Fehler beim Laden der Dateiauswahlvorlage: {error}");
    translations.insert("Yes", "Ja");
    translations.insert("No", "Nein");
    translations.insert("Ok", "OK");
    translations.insert("Error loading message template: {error}", "Fehler beim Laden der Nachrichtenvorlage: {error}");
    translations.insert("One file conflict", "Ein Dateikonflikt");
    translations.insert("Which files do you want to keep?", "Welche Dateien möchtest du behalten?");
    translations.insert("If you select both versions, the copied file will have a number added to its name.", "Wenn du beide Versionen auswählst, erhält die kopierte Datei eine Zahl am Ende des Dateinamens.");
    translations.insert("Cancel", "Abbrechen");
    translations.insert("Continue", "Fortsetzen");
    translations.insert("(all selected)", "(Alle ausgewählt)");
    translations.insert("Error loading file exists template", "Fehler beim Laden der vorhanden Dateivorlage");
    translations.insert("Shared", "Geteilt");
    translations.insert("Share", "Teilen");
    translations.insert("Error", "Fehler");
    translations.insert("Error while sharing", "Fehler beim Teilen");
    translations.insert("Error while unsharing", "Fehler beim Aufheben der Freigabe");
    translations.insert("Error while changing permissions", "Fehler beim Ändern der Rechte");
    translations.insert("Shared with you and the group {group} by {owner}", "{owner} hat dies mit Dir und der Gruppe {group} geteilt");
    translations.insert("Shared with you by {owner}", "{owner} hat dies mit Dir geteilt");
    translations.insert("Share with user or group …", "Mit Benutzer oder Gruppe teilen ....");
    translations.insert("Share link", "Link Teilen");
    translations.insert("Password protect", "Passwortschutz");
    translations.insert("Password", "Passwort");
    translations.insert("Allow Public Upload", "Öffentliches Hochladen erlauben");
    translations.insert("Email link to person", "Link per E-Mail verschicken");
    translations.insert("Send", "Senden");
    translations.insert("Set expiration date", "Setze ein Ablaufdatum");
    translations.insert("Expiration date", "Ablaufdatum");
    translations.insert("Share via email:", "Über eine E-Mail teilen:");
    translations.insert("No people found", "Niemand gefunden");
    translations.insert("group", "Gruppe");
    translations.insert("Resharing is not allowed", "Weiterverteilen ist nicht erlaubt");
    translations.insert("Shared in {item} with {user}", "Für {user} in {item} freigegeben");
    translations.insert("Unshare", "Freigabe aufheben");
    translations.insert("notify by email", "Per E-Mail informieren");
    translations.insert("can edit", "kann bearbeiten");
    translations.insert("access control", "Zugriffskontrolle");
    translations.insert("create", "erstellen");
    translations.insert("update", "aktualisieren");
    translations.insert("delete", "löschen");
    translations.insert("share", "teilen");
    translations.insert("Password protected", "Durch ein Passwort geschützt");
    translations.insert("Error unsetting expiration date", "Fehler beim Entfernen des Ablaufdatums");
    translations.insert("Error setting expiration date", "Fehler beim Setzen des Ablaufdatums");
    translations.insert("Sending ...", "Sende ...");
    translations.insert("Email sent", "E-Mail wurde verschickt");
    translations.insert("Warning", "Warnung");
    translations.insert("The object type is not specified.", "Der Objekttyp ist nicht angegeben.");
    translations.insert("Enter new", "Unbekannter Fehler, bitte prüfe Deine Systemeinstellungen oder kontaktiere Deinen Administrator");
    translations.insert("Delete", "Löschen");
    translations.insert("Add", "Hinzufügen");
    translations.insert("Edit tags", "Schlagwörter bearbeiten");
    translations.insert("Error loading dialog template: {error}", "Fehler beim Laden der Gesprächsvorlage: {error}");
    translations.insert("No tags selected for deletion.", "Es wurden keine Schlagwörter zum Löschen ausgewählt.");
    translations.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Das Update ist fehlgeschlagen. Bitte melde dieses Problem an die <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud Community</a>.");
    translations.insert("The update was successful. Redirecting you to ownCloud now.", "Das Update war erfolgreich. Du wirst nun zu ownCloud weitergeleitet.");
    translations.insert("%s password reset", "%s-Passwort zurücksetzen");
    translations.insert("Use the following link to reset your password: {link}", "Nutze den nachfolgenden Link, um Dein Passwort zurückzusetzen: {link}");
    translations.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Der Link zum Rücksetzen Deines Passwort ist an Deine E-Mail-Adresse geschickt worden.<br>Wenn Du ihn nicht innerhalb einer vernünftigen Zeit empfängst, prüfe Deine Spam-Verzeichnisse.<br>Wenn er nicht dort ist, frage Deinen lokalen Administrator.");
    translations.insert("Request failed!<br>Did you make sure your email/username was right?", "Anfrage fehlgeschlagen!<br>Hast Du darauf geachtet, dass Deine E-Mail/Dein Benutzername korrekt war?");
    translations.insert("You will receive a link to reset your password via Email.", "Du erhältst einen Link per E-Mail, um Dein Passwort zurückzusetzen.");
    translations.insert("Username", "Benutzername");
    translations.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Ihre Dateien sind verschlüsselt. Sollten Sie keinen Wiederherstellungschlüssel aktiviert haben, gibt es keine Möglichkeit an Ihre Daten zu kommen, wenn das Passwort zurückgesetzt wird. Falls Sie sich nicht sicher sind, was Sie tun sollen, kontaktieren Sie bitte Ihren Administrator, bevor Sie fortfahren. Wollen Sie wirklich fortfahren?");
    translations.insert("Yes, I really want to reset my password now", "Ja, ich will mein Passwort jetzt wirklich zurücksetzen");
    translations.insert("Reset", "Zurücksetzen");
    translations.insert("Your password was reset", "Dein Passwort wurde zurückgesetzt.");
    translations.insert("To login page", "Zur Login-Seite");
    translations.insert("New password", "Neues Passwort");
    translations.insert("Reset password", "Passwort zurücksetzen");
    translations.insert("Personal", "Persönlich");
    translations.insert("Users", "Benutzer");
    translations.insert("Apps", "Apps");
    translations.insert("Admin", "Administration");
    translations.insert("Help", "Hilfe");
    translations.insert("Error loading tags", "Fehler beim Laden der Schlagwörter");
    translations.insert("Tag already exists", "Schlagwort ist bereits vorhanden");
    translations.insert("Error deleting tag(s)", "Fehler beim Löschen des Schlagwortes bzw. der Schlagwörter");
    translations.insert("Error tagging", "Fehler beim Hinzufügen der Schlagwörter");
    translations.insert("Error untagging", "Fehler beim Entfernen der Schlagwörter");
    translations.insert("Error favoriting", "Fehler beim Hinzufügen zu den Favoriten");
    translations.insert("Error unfavoriting", "Fehler beim Entfernen aus den Favoriten");
    translations.insert("Access forbidden", "Zugriff verboten");
    translations.insert("Cloud not found", "Cloud nicht gefunden");
    translations.insert("Hey there,\n\njust letting you know that %s shared %s with you.\nView it: %s\n\n", "Hallo,\n\nich wollte Dich nur wissen lassen, dass %s %s mit Dir teilt.\nSchaue es Dir an: %s\n\n");
    translations.insert("The share will expire on %s.\n\n", "Die Freigabe wird ablaufen am %s.\n\n");
    translations.insert("Cheers!", "Hallo!");
    translations.insert("Security Warning", "Sicherheitswarnung");
    translations.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Deine PHP Version ist durch die NULL Byte Attacke (CVE-2006-7243) angreifbar");
    translations.insert("Please update your PHP installation to use %s securely.", "Bitte aktualisiere deine PHP-Installation um %s sicher nutzen zu können.");
    translations.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Es ist kein sicherer Zufallszahlengenerator verfügbar, bitte aktiviere die PHP-Erweiterung für OpenSSL.");
    translations.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Ohne einen sicheren Zufallszahlengenerator sind Angreifer in der Lage die Tokens für das Zurücksetzen der Passwörter vorherzusehen und Konten  zu übernehmen.");
    translations.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Dein Datenverzeichnis und Deine Dateien sind wahrscheinlich vom Internet aus erreichbar, weil die .htaccess-Datei nicht funktioniert.");
    translations.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Für Informationen, wie du deinen Server richtig konfigurierst lese bitte die <a href=\"%s\" target=\"_blank\">Dokumentation</a>.");
    translations.insert("Create an <strong>admin account</strong>", "<strong>Administrator-Konto</strong> anlegen");
    translations.insert("Advanced", "Fortgeschritten");
    translations.insert("Data folder", "Datenverzeichnis");
    translations.insert("Configure the database", "Datenbank einrichten");
    translations.insert("will be used", "wird verwendet");
    translations.insert("Database user", "Datenbank-Benutzer");
    translations.insert("Database password", "Datenbank-Passwort");
    translations.insert("Database name", "Datenbank-Name");
    translations.insert("Database tablespace", "Datenbank-Tablespace");
    translations.insert("Database host", "Datenbank-Host");
    translations.insert("Finish setup", "Installation abschließen");
    translations.insert("Finishing …", "Abschließen ...");
    translations.insert("%s is available. Get more information on how to update.", "%s ist verfügbar. Holen Sie weitere Informationen zu Aktualisierungen ein.");
    translations.insert("Log out", "Abmelden");
    translations.insert("Automatic logon rejected!", "Automatischer Login zurückgewiesen!");
    translations.insert("If you did not change your password recently, your account may be compromised!", "Wenn Du Dein Passwort nicht vor kurzem geändert hast, könnte Dein\nAccount kompromittiert sein!");
    translations.insert("Please change your password to secure your account again.", "Bitte ändere Dein Passwort, um Deinen Account wieder zu schützen.");
    translations.insert("Server side authentication failed!", "Serverseitige Authentifizierung fehlgeschlagen!");
    translations.insert("Please contact your administrator.", "Bitte kontaktiere Deinen Administrator.");
    translations.insert("Lost your password?", "Passwort vergessen?");
    translations.insert("remember", "merken");
    translations.insert("Log in", "Einloggen");
    translations.insert("Alternative Logins", "Alternative Logins");
    translations.insert("Hey there,<br><br>just letting you know that %s shared »%s« with you.<br><a href=\"%s\">View it!</a><br><br>", "Hallo,<br/><br/>wollte dich nur kurz informieren, dass %s gerade %s mit dir geteilt hat.<br/><a href=\"%s\">Schau es dir an.</a><br/><br/>");
    translations.insert("The share will expire on %s.<br><br>", "Die Freigabe wird ablaufen am %s.<br><br>");
    translations.insert("Updating ownCloud to version %s, this may take a while.", "Aktualisiere ownCloud auf Version %s. Dies könnte eine Weile dauern.");
    translations.insert("This ownCloud instance is currently being updated, which may take a while.", "Diese OwnCloud-Instanz wird gerade aktualisiert, was eine Weile dauert.");
    translations.insert("Please reload this page after a short time to continue using ownCloud.", "Bitte lade diese Seite nach kurzer Zeit neu, um mit der Nutzung von OwnCloud fortzufahren.");
    translations.insert("Contact your system administrator if this message persists or appeared unexpectedly.", "Kontaktiere Deinen Systemadministrator, wenn diese Meldung dauerhaft oder unerwartet erscheint.");
    translations.insert("Thank you for your patience.", "Vielen Dank für Deine Geduld.");
    translations
});

pub static DE_PLURAL_FORMS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut plurals = HashMap::new();
    plurals.insert("_%n minute ago_::_%n minutes ago_", vec!["Vor %n Minute", "Vor %n Minuten"]);
    plurals.insert("_%n hour ago_::_%n hours ago_", vec!["Vor %n Stunde", "Vor %n Stunden"]);
    plurals.insert("_%n day ago_::_%n days ago_", vec!["Vor %n Tag", "Vor %n Tagen"]);
    plurals.insert("_%n month ago_::_%n months ago_", vec!["Vor %n Monat", "Vor %n Monaten"]);
    plurals.insert("_{count} file conflict_::_{count} file conflicts_", vec!["{count} Dateikonflikt", "{count} Dateikonflikte"]);
    plurals.insert("({count} selected)", vec!["({count} ausgewählt)"]);
    plurals
});

/// Gets plural form index according to the German language rules.
/// 
/// In German:
/// nplurals=2; plural=(n != 1);
///
/// Returns 0 for singular (n == 1)
/// Returns 1 for plural (n != 1)
pub fn get_german_plural_form(n: i64) -> usize {
    if n != 1 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_german_plural_forms() {
        assert_eq!(get_german_plural_form(0), 1);
        assert_eq!(get_german_plural_form(1), 0);
        assert_eq!(get_german_plural_form(2), 1);
        assert_eq!(get_german_plural_form(5), 1);
        assert_eq!(get_german_plural_form(99), 1);
    }
}