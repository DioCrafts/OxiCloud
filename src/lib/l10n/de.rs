use lazy_static::lazy_static;
use std::collections::HashMap;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Applikation \"%s\" kann nicht installiert werden, da sie mit dieser ownCloud Version nicht kompatibel ist.");
        m.insert("No app name specified", "Es wurde kein Applikation-Name angegeben");
        m.insert("Help", "Hilfe");
        m.insert("Personal", "Persönlich");
        m.insert("Settings", "Einstellungen");
        m.insert("Users", "Benutzer");
        m.insert("Admin", "Administration");
        m.insert("Failed to upgrade \"%s\".", "Konnte \"%s\" nicht aktualisieren.");
        m.insert("Unknown filetype", "Unbekannter Dateityp");
        m.insert("Invalid image", "Ungültiges Bild");
        m.insert("web services under your control", "Web-Services unter Deiner Kontrolle");
        m.insert("cannot open \"%s\"", "Öffnen von \"%s\" fehlgeschlagen");
        m.insert("ZIP download is turned off.", "Der ZIP-Download ist deaktiviert.");
        m.insert("Files need to be downloaded one by one.", "Die Dateien müssen einzeln heruntergeladen werden.");
        m.insert("Back to Files", "Zurück zu \"Dateien\"");
        m.insert("Selected files too large to generate zip file.", "Die gewählten Dateien sind zu groß, um eine ZIP-Datei zu erstellen.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Lade die Dateien in kleineren, separaten, Stücken herunter oder bitte deinen Administrator.");
        m.insert("No source specified when installing app", "Für die Installation der Applikation wurde keine Quelle angegeben");
        m.insert("No href specified when installing app from http", "Der Link (href) wurde nicht angegeben um die Applikation per http zu installieren");
        m.insert("No path specified when installing app from local file", "Bei der Installation der Applikation aus einer lokalen Datei wurde kein Pfad angegeben");
        m.insert("Archives of type %s are not supported", "Archive vom Typ %s werden nicht unterstützt");
        m.insert("Failed to open archive when installing app", "Das Archiv konnte bei der Installation der Applikation nicht geöffnet werden");
        m.insert("App does not provide an info.xml file", "Die Applikation enthält keine info,xml Datei");
        m.insert("App can't be installed because of not allowed code in the App", "Die Applikation kann auf Grund von unerlaubten Code nicht installiert werden");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "Die Anwendung konnte nicht installiert werden, weil Sie nicht mit dieser Version von ownCloud kompatibel ist.");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "Die Applikation konnte nicht installiert werden, da diese das <shipped>true</shipped> Tag beinhaltet und dieses, bei nicht mitausgelieferten Applikationen, nicht erlaubt ist ist");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "Die Applikation konnte nicht installiert werden, da die Version in der info.xml nicht die gleiche Version wie im App-Store ist");
        m.insert("App directory already exists", "Das Applikationsverzeichnis existiert bereits");
        m.insert("Can't create app folder. Please fix permissions. %s", "Es kann kein Applikationsordner erstellt werden. Bitte passen sie die  Berechtigungen an. %s");
        m.insert("Application is not enabled", "Die Anwendung ist nicht aktiviert");
        m.insert("Authentication error", "Fehler bei der Anmeldung");
        m.insert("Token expired. Please reload page.", "Token abgelaufen. Bitte lade die Seite neu.");
        m.insert("Files", "Dateien");
        m.insert("Text", "Text");
        m.insert("Images", "Bilder");
        m.insert("%s enter the database username.", "%s gib den Datenbank-Benutzernamen an.");
        m.insert("%s enter the database name.", "%s gib den Datenbank-Namen an.");
        m.insert("%s you may not use dots in the database name", "%s Der Datenbank-Name darf keine Punkte enthalten");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL Benutzername und/oder Password ungültig: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Du musst entweder ein existierendes Benutzerkonto oder das Administratoren-Konto angeben.");
        m.insert("MySQL username and/or password not valid", "MySQL Benutzername und/oder Passwort ungültig");
        m.insert("DB Error: \"%s\"", "DB Fehler: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Fehlerhafter Befehl war: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL Benutzer '%s'@'localhost' existiert bereits.");
        m.insert("Drop this user from MySQL", "Lösche diesen Benutzer von MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL Benutzer '%s'@'%%' existiert bereits");
        m.insert("Drop this user from MySQL.", "Lösche diesen Benutzer aus MySQL.");
        m.insert("Oracle connection could not be established", "Es konnte keine Verbindung zur Oracle-Datenbank hergestellt werden");
        m.insert("Oracle username and/or password not valid", "Oracle Benutzername und/oder Passwort ungültig");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Fehlerhafter Befehl war: \"%s\", Name: %s, Passwort: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL Benutzername und/oder Passwort ungültig");
        m.insert("Set an admin username.", "Setze Administrator Benutzername.");
        m.insert("Set an admin password.", "Setze Administrator Passwort");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Dein Web-Server ist noch nicht für Datei-Synchronisation bereit, weil die WebDAV-Schnittstelle vermutlich defekt ist.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Bitte prüfe die <a href='%s'>Installationsanleitungen</a>.");
        m.insert("Could not find category \"%s\"", "Die Kategorie \"%s\" konnte nicht gefunden werden.");
        m.insert("seconds ago", "Gerade eben");
        m.insert("today", "Heute");
        m.insert("yesterday", "Gestern");
        m.insert("last month", "Letzten Monat");
        m.insert("last year", "Letztes Jahr");
        m.insert("years ago", "Vor Jahren");
        m.insert("Caused by:", "Verursacht durch:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["Vor %n Minute", "Vor %n Minuten"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["Vor %n Stunde", "Vor %n Stunden"]);
        m.insert("_%n day go_::_%n days ago_", vec!["Vor %n Tag", "Vor %n Tagen"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["Vor %n Monat", "Vor %n Monaten"]);
        m
    };
}

/// Returns the translation for the given string
pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

/// Returns the plural translation for the given string and count
pub fn translate_plural(key: &str, count: i64) -> &'static str {
    let plural_idx = if count != 1 { 1 } else { 0 };
    
    if let Some(translations) = PLURAL_TRANSLATIONS.get(key) {
        if let Some(translation) = translations.get(plural_idx) {
            return translation;
        }
    }
    key
}

// For format string replacements
pub fn translate_format(key: &str, args: &[&str]) -> String {
    let mut result = translate(key).to_string();
    
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%s", i+1), arg);
        result = result.replace("%s", arg); // Replace first occurrence if no index
    }
    
    result
}

pub fn translate_plural_format(key: &str, count: i64, args: &[&str]) -> String {
    let mut result = translate_plural(key, count).to_string();
    
    // Replace %n with the count
    result = result.replace("%n", &count.to_string());
    
    // Replace other arguments
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%s", i+1), arg);
        result = result.replace("%s", arg); // Replace first occurrence if no index
    }
    
    result
}