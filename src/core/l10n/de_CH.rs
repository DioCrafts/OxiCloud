use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%s shared »%s« with you", "%s teilt »%s« mit Ihnen");
        m.insert("Sunday", "Sonntag");
        m.insert("Monday", "Montag");
        m.insert("Tuesday", "Dienstag");
        m.insert("Wednesday", "Mittwoch");
        m.insert("Thursday", "Donnerstag");
        m.insert("Friday", "Freitag");
        m.insert("Saturday", "Samstag");
        m.insert("January", "Januar");
        m.insert("February", "Februar");
        m.insert("March", "März");
        m.insert("April", "April");
        m.insert("May", "Mai");
        m.insert("June", "Juni");
        m.insert("July", "Juli");
        m.insert("August", "August");
        m.insert("September", "September");
        m.insert("October", "Oktober");
        m.insert("November", "November");
        m.insert("December", "Dezember");
        m.insert("Settings", "Einstellungen");
        m.insert("seconds ago", "Gerade eben");
        m.insert("today", "Heute");
        m.insert("yesterday", "Gestern");
        m.insert("last month", "Letzten Monat");
        m.insert("months ago", "Vor Monaten");
        m.insert("last year", "Letztes Jahr");
        m.insert("years ago", "Vor Jahren");
        m.insert("Choose", "Auswählen");
        m.insert("Yes", "Ja");
        m.insert("No", "Nein");
        m.insert("Ok", "OK");
        m.insert("Cancel", "Abbrechen");
        m.insert("Shared", "Geteilt");
        m.insert("Share", "Teilen");
        m.insert("Error", "Fehler");
        m.insert("Error while sharing", "Fehler beim Teilen");
        m.insert("Error while unsharing", "Fehler beim Aufheben der Freigabe");
        m.insert("Error while changing permissions", "Fehler bei der Änderung der Rechte");
        m.insert("Shared with you and the group {group} by {owner}", "Von {owner} mit Ihnen und der Gruppe {group} geteilt.");
        m.insert("Shared with you by {owner}", "Von {owner} mit Ihnen geteilt.");
        m.insert("Password protect", "Passwortschutz");
        m.insert("Password", "Passwort");
        m.insert("Allow Public Upload", "Öffentliches Hochladen erlauben");
        m.insert("Email link to person", "Link per E-Mail verschicken");
        m.insert("Send", "Senden");
        m.insert("Set expiration date", "Ein Ablaufdatum setzen");
        m.insert("Expiration date", "Ablaufdatum");
        m.insert("Share via email:", "Mittels einer E-Mail teilen:");
        m.insert("No people found", "Niemand gefunden");
        m.insert("group", "Gruppe");
        m.insert("Resharing is not allowed", "Das Weiterverteilen ist nicht erlaubt");
        m.insert("Shared in {item} with {user}", "Freigegeben in {item} von {user}");
        m.insert("Unshare", "Freigabe aufheben");
        m.insert("can edit", "kann bearbeiten");
        m.insert("access control", "Zugriffskontrolle");
        m.insert("create", "erstellen");
        m.insert("update", "aktualisieren");
        m.insert("delete", "löschen");
        m.insert("share", "teilen");
        m.insert("Password protected", "Passwortgeschützt");
        m.insert("Error unsetting expiration date", "Fehler beim Entfernen des Ablaufdatums");
        m.insert("Error setting expiration date", "Fehler beim Setzen des Ablaufdatums");
        m.insert("Sending ...", "Sende ...");
        m.insert("Email sent", "Email gesendet");
        m.insert("Warning", "Warnung");
        m.insert("The object type is not specified.", "Der Objekttyp ist nicht angegeben.");
        m.insert("Delete", "Löschen");
        m.insert("Add", "Hinzufügen");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "Das Update ist fehlgeschlagen. Bitte melden Sie dieses Problem an die <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud Community</a>.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "Das Update war erfolgreich. Sie werden nun zu ownCloud weitergeleitet.");
        m.insert("%s password reset", "%s-Passwort zurücksetzen");
        m.insert("Use the following link to reset your password: {link}", "Nutzen Sie den nachfolgenden Link, um Ihr Passwort zurückzusetzen: {link}");
        m.insert("The link to reset your password has been sent to your email.<br>If you do not receive it within a reasonable amount of time, check your spam/junk folders.<br>If it is not there ask your local administrator .", "Der Link zum Rücksetzen Ihres Passworts ist an Ihre E-Mail-Adresse gesendet worde.<br>Wenn Sie ihn nicht innerhalb einer vernünftigen Zeitspanne erhalten, prüfen Sie bitte Ihre Spam-Verzeichnisse.<br>Wenn er nicht dort ist, fragen Sie Ihren lokalen Administrator.");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "Anfrage fehlgeschlagen!<br>Haben Sie darauf geachtet, dass E-Mail-Adresse/Nutzername korrekt waren?");
        m.insert("You will receive a link to reset your password via Email.", "Sie erhalten einen Link per E-Mail, um Ihr Passwort zurückzusetzen.");
        m.insert("Username", "Benutzername");
        m.insert("Your files are encrypted. If you haven't enabled the recovery key, there will be no way to get your data back after your password is reset. If you are not sure what to do, please contact your administrator before you continue. Do you really want to continue?", "Ihre Dateien sind verschlüsselt. Wenn Sie den Wiederherstellungsschlüssel nicht aktiviert haben, wird es keine Möglichkeit geben, um Ihre Daten wiederzubekommen, nachdem Ihr Passwort zurückgesetzt wurde. Wenn Sie sich nicht sicher sind, was Sie tun sollen, wenden Sie sich bitte an Ihren Administrator, bevor Sie fortfahren. Wollen Sie wirklich fortfahren?");
        m.insert("Yes, I really want to reset my password now", "Ja, ich möchte jetzt mein Passwort wirklich zurücksetzen.");
        m.insert("Reset", "Zurücksetzen");
        m.insert("Your password was reset", "Ihr Passwort wurde zurückgesetzt.");
        m.insert("To login page", "Zur Login-Seite");
        m.insert("New password", "Neues Passwort");
        m.insert("Reset password", "Passwort zurücksetzen");
        m.insert("Personal", "Persönlich");
        m.insert("Users", "Benutzer");
        m.insert("Apps", "Apps");
        m.insert("Admin", "Administrator");
        m.insert("Help", "Hilfe");
        m.insert("Access forbidden", "Zugriff verboten");
        m.insert("Cloud not found", "Cloud wurde nicht gefunden");
        m.insert("Security Warning", "Sicherheitshinweis");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "Ihre PHP Version ist durch die NULL Byte Attacke (CVE-2006-7243) angreifbar");
        m.insert("Please update your PHP installation to use %s securely.", "Bitte aktualisieren Sie Ihre PHP-Installation um %s sicher nutzen zu können.");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "Es ist kein sicherer Zufallszahlengenerator verfügbar, bitte aktivieren Sie die PHP-Erweiterung für OpenSSL.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "Ohne einen sicheren Zufallszahlengenerator sind Angreifer in der Lage, die Tokens für das Zurücksetzen der Passwörter vorherzusehen und Ihr Konto zu übernehmen.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", "Ihr Datenverzeichnis und Ihre Dateien sind wahrscheinlich vom Internet aus erreichbar, weil die .htaccess-Datei nicht funktioniert.");
        m.insert("For information how to properly configure your server, please see the <a href=\"%s\" target=\"_blank\">documentation</a>.", "Für Informationen, wie Sie Ihren Server richtig konfigurieren lesen Sie bitte die <a href=\"%s\" target=\"_blank\">Dokumentation</a>.");
        m.insert("Create an <strong>admin account</strong>", "<strong>Administrator-Konto</strong> anlegen");
        m.insert("Advanced", "Fortgeschritten");
        m.insert("Data folder", "Datenverzeichnis");
        m.insert("Configure the database", "Datenbank einrichten");
        m.insert("will be used", "wird verwendet");
        m.insert("Database user", "Datenbank-Benutzer");
        m.insert("Database password", "Datenbank-Passwort");
        m.insert("Database name", "Datenbank-Name");
        m.insert("Database tablespace", "Datenbank-Tablespace");
        m.insert("Database host", "Datenbank-Host");
        m.insert("Finish setup", "Installation abschliessen");
        m.insert("%s is available. Get more information on how to update.", "%s ist verfügbar. Holen Sie weitere Informationen zu Aktualisierungen ein.");
        m.insert("Log out", "Abmelden");
        m.insert("Automatic logon rejected!", "Automatische Anmeldung verweigert!");
        m.insert("If you did not change your password recently, your account may be compromised!", "Wenn Sie Ihr Passwort nicht vor kurzem geändert haben, könnte Ihr\nAccount kompromittiert sein!");
        m.insert("Please change your password to secure your account again.", "Bitte ändern Sie Ihr Passwort, um Ihr Konto wieder zu sichern.");
        m.insert("Lost your password?", "Passwort vergessen?");
        m.insert("remember", "merken");
        m.insert("Log in", "Einloggen");
        m.insert("Alternative Logins", "Alternative Logins");
        m.insert("Updating ownCloud to version %s, this may take a while.", "Aktualisiere ownCloud auf Version %s. Dies könnte eine Weile dauern.");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["Vor %n Minute", "Vor %n Minuten"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["Vor %n Stunde", "Vor %n Stunden"]);
        m.insert("_%n day ago_::_%n days ago_", vec!["Vor %n Tag", "Vor %n Tagen"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["Vor %n Monat", "Vor %n Monaten"]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}