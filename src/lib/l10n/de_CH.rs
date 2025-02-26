// lib/l10n/de_ch.rs

use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_catalog() -> Catalog {
    let mut catalog = Catalog::new("de_CH");
    catalog.set_plural_forms("nplurals=2; plural=(n != 1);");
    
    let mut translations = HashMap::new();
    
    translations.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", 
        "Anwendung \"%s\" kann nicht installiert werden, da sie mit dieser Version von ownCloud nicht kompatibel ist."
    );
    translations.insert("No app name specified", "Kein App-Name spezifiziert");
    translations.insert("Help", "Hilfe");
    translations.insert("Personal", "Persönlich");
    translations.insert("Settings", "Einstellungen");
    translations.insert("Users", "Benutzer");
    translations.insert("Admin", "Administrator");
    translations.insert("Failed to upgrade \"%s\".", "Konnte \"%s\" nicht aktualisieren.");
    translations.insert("web services under your control", "Web-Services unter Ihrer Kontrolle");
    translations.insert("cannot open \"%s\"", "Öffnen von \"%s\" fehlgeschlagen");
    translations.insert("ZIP download is turned off.", "Der ZIP-Download ist deaktiviert.");
    translations.insert("Files need to be downloaded one by one.", "Die Dateien müssen einzeln heruntergeladen werden.");
    translations.insert("Back to Files", "Zurück zu \"Dateien\"");
    translations.insert("Selected files too large to generate zip file.", "Die gewählten Dateien sind zu gross, um eine ZIP-Datei zu erstellen.");
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Laden Sie die Dateien in kleineren, separaten, Stücken herunter oder bitten Sie Ihren Administrator.");
    translations.insert("App can't be installed because of not allowed code in the App", "Anwendung kann wegen nicht erlaubten Codes nicht installiert werden");
    translations.insert("App directory already exists", "Anwendungsverzeichnis existiert bereits");
    translations.insert("Application is not enabled", "Die Anwendung ist nicht aktiviert");
    translations.insert("Authentication error", "Authentifizierungs-Fehler");
    translations.insert("Token expired. Please reload page.", "Token abgelaufen. Bitte laden Sie die Seite neu.");
    translations.insert("Files", "Dateien");
    translations.insert("Text", "Text");
    translations.insert("Images", "Bilder");
    translations.insert("%s enter the database username.", "%s geben Sie den Datenbank-Benutzernamen an.");
    translations.insert("%s enter the database name.", "%s geben Sie den Datenbank-Namen an.");
    translations.insert("%s you may not use dots in the database name", "%s Der Datenbank-Name darf keine Punkte enthalten");
    translations.insert("MS SQL username and/or password not valid: %s", "MS SQL Benutzername und/oder Passwort ungültig: %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Sie müssen entweder ein existierendes Benutzerkonto oder das Administratoren-Konto angeben.");
    translations.insert("MySQL username and/or password not valid", "MySQL Benutzername und/oder Passwort ungültig");
    translations.insert("DB Error: \"%s\"", "DB Fehler: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Fehlerhafter Befehl war: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "MySQL Benutzer '%s'@'localhost' existiert bereits.");
    translations.insert("Drop this user from MySQL", "Lösche diesen Benutzer aus MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "MySQL Benutzer '%s'@'%%' existiert bereits");
    translations.insert("Drop this user from MySQL.", "Lösche diesen Benutzer aus MySQL.");
    translations.insert("Oracle connection could not be established", "Die Oracle-Verbindung konnte nicht aufgebaut werden.");
    translations.insert("Oracle username and/or password not valid", "Oracle Benutzername und/oder Passwort ungültig");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Fehlerhafter Befehl war: \"%s\", Name: %s, Passwort: %s");
    translations.insert("PostgreSQL username and/or password not valid", "PostgreSQL Benutzername und/oder Passwort ungültig");
    translations.insert("Set an admin username.", "Setze Administrator Benutzername.");
    translations.insert("Set an admin password.", "Setze Administrator Passwort");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Ihr Web-Server ist noch nicht für eine Datei-Synchronisation konfiguriert, weil die WebDAV-Schnittstelle vermutlich defekt ist.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Bitte prüfen Sie die <a href='%s'>Installationsanleitungen</a>.");
    translations.insert("Could not find category \"%s\"", "Die Kategorie «%s» konnte nicht gefunden werden.");
    translations.insert("seconds ago", "Gerade eben");
    translations.insert("today", "Heute");
    translations.insert("yesterday", "Gestern");
    translations.insert("last month", "Letzten Monat");
    translations.insert("last year", "Letztes Jahr");
    translations.insert("years ago", "Vor  Jahren");
    translations.insert("Caused by:", "Verursacht durch:");
    
    // Add pluralized translations
    catalog.add_plural_msg(
        "_%n minute ago_::_%n minutes ago_",
        vec!["", "Vor %n Minuten"],
    );
    catalog.add_plural_msg(
        "_%n hour ago_::_%n hours ago_",
        vec!["", "Vor %n Stunden"],
    );
    catalog.add_plural_msg(
        "_%n day go_::_%n days ago_",
        vec!["", "Vor %n Tagen"],
    );
    catalog.add_plural_msg(
        "_%n month ago_::_%n months ago_",
        vec!["", "Vor %n Monaten"],
    );
    
    // Add regular translations
    for (key, value) in translations {
        catalog.add_msg(key, value);
    }
    
    catalog
}