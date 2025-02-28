use std::collections::HashMap;
use once_cell::sync::Lazy;

static DE_DE_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.",
        "Applikation \"%s\" kann nicht installiert werden, da sie mit dieser ownCloud Version nicht kompatibel ist.",
    );
    translations.insert(
        "No app name specified",
        "Es wurde kein Applikation-Name angegeben",
    );
    translations.insert("Help", "Hilfe");
    translations.insert("Personal", "Persönlich");
    translations.insert("Settings", "Einstellungen");
    translations.insert("Users", "Benutzer");
    translations.insert("Admin", "Administrator");
    translations.insert("Failed to upgrade \"%s\".", "Konnte \"%s\" nicht aktualisieren.");
    translations.insert("Unknown filetype", "Unbekannter Dateityp");
    translations.insert("Invalid image", "Ungültiges Bild");
    translations.insert(
        "web services under your control",
        "Web-Services unter Ihrer Kontrolle",
    );
    translations.insert("cannot open \"%s\"", "Öffnen von \"%s\" fehlgeschlagen");
    translations.insert("ZIP download is turned off.", "Der ZIP-Download ist deaktiviert.");
    translations.insert(
        "Files need to be downloaded one by one.",
        "Die Dateien müssen einzeln heruntergeladen werden.",
    );
    translations.insert("Back to Files", "Zurück zu \"Dateien\"");
    translations.insert(
        "Selected files too large to generate zip file.",
        "Die gewählten Dateien sind zu groß, um eine ZIP-Datei zu erstellen.",
    );
    translations.insert(
        "Download the files in smaller chunks, seperately or kindly ask your administrator.",
        "Laden Sie die Dateien in kleineren, separaten, Stücken herunter oder bitten Sie Ihren Administrator.",
    );
    translations.insert(
        "No source specified when installing app",
        "Für die Installation der Applikation wurde keine Quelle angegeben",
    );
    translations.insert(
        "No href specified when installing app from http",
        "Der Link (href) wurde nicht angegeben um die Applikation per http zu installieren",
    );
    translations.insert(
        "No path specified when installing app from local file",
        "Bei der Installation der Applikation aus einer lokalen Datei wurde kein Pfad angegeben",
    );
    translations.insert(
        "Archives of type %s are not supported",
        "Archive des Typs %s werden nicht unterstützt.",
    );
    translations.insert(
        "Failed to open archive when installing app",
        "Das Archiv konnte bei der Installation der Applikation nicht geöffnet werden",
    );
    translations.insert(
        "App does not provide an info.xml file",
        "Die Applikation enthält keine info,xml Datei",
    );
    translations.insert(
        "App can't be installed because of not allowed code in the App",
        "Die Applikation kann auf Grund von unerlaubten Code nicht installiert werden",
    );
    translations.insert(
        "App can't be installed because it is not compatible with this version of ownCloud",
        "Die Anwendung konnte nicht installiert werden, weil Sie nicht mit dieser Version von ownCloud kompatibel ist.",
    );
    translations.insert(
        "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps",
        "Die Applikation konnte nicht installiert werden, da diese das <shipped>true</shipped> Tag beinhaltet und dieses, bei nicht mitausgelieferten Applikationen, nicht erlaubt ist ist",
    );
    translations.insert(
        "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store",
        "Die Applikation konnte nicht installiert werden, da die Version in der info.xml nicht die gleiche Version wie im App-Store ist",
    );
    translations.insert(
        "App directory already exists",
        "Der Ordner für die Anwendung existiert bereits.",
    );
    translations.insert(
        "Can't create app folder. Please fix permissions. %s",
        "Der Ordner für die Anwendung konnte nicht angelegt werden. Bitte überprüfen Sie die Ordner- und Dateirechte und passen Sie diese entsprechend an. %s",
    );
    translations.insert(
        "Application is not enabled",
        "Die Anwendung ist nicht aktiviert",
    );
    translations.insert("Authentication error", "Authentifizierungs-Fehler");
    translations.insert(
        "Token expired. Please reload page.",
        "Token abgelaufen. Bitte laden Sie die Seite neu.",
    );
    translations.insert("Files", "Dateien");
    translations.insert("Text", "Text");
    translations.insert("Images", "Bilder");
    translations.insert(
        "%s enter the database username.",
        "%s geben Sie den Datenbank-Benutzernamen an.",
    );
    translations.insert(
        "%s enter the database name.",
        "%s geben Sie den Datenbank-Namen an.",
    );
    translations.insert(
        "%s you may not use dots in the database name",
        "%s Der Datenbank-Name darf keine Punkte enthalten",
    );
    translations.insert(
        "MS SQL username and/or password not valid: %s",
        "MS SQL Benutzername und/oder Passwort ungültig: %s",
    );
    translations.insert(
        "You need to enter either an existing account or the administrator.",
        "Sie müssen entweder ein existierendes Benutzerkonto oder das Administratoren-Konto angeben.",
    );
    translations.insert(
        "MySQL username and/or password not valid",
        "MySQL Benutzername und/oder Passwort ungültig",
    );
    translations.insert("DB Error: \"%s\"", "DB Fehler: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Fehlerhafter Befehl war: \"%s\"");
    translations.insert(
        "MySQL user '%s'@'localhost' exists already.",
        "MySQL Benutzer '%s'@'localhost' existiert bereits.",
    );
    translations.insert("Drop this user from MySQL", "Lösche diesen Benutzer aus MySQL");
    translations.insert(
        "MySQL user '%s'@'%%' already exists",
        "MySQL Benutzer '%s'@'%%' existiert bereits",
    );
    translations.insert(
        "Drop this user from MySQL.",
        "Lösche diesen Benutzer aus MySQL.",
    );
    translations.insert(
        "Oracle connection could not be established",
        "Die Oracle-Verbindung konnte nicht aufgebaut werden.",
    );
    translations.insert(
        "Oracle username and/or password not valid",
        "Oracle Benutzername und/oder Passwort ungültig",
    );
    translations.insert(
        "Offending command was: \"%s\", name: %s, password: %s",
        "Fehlerhafter Befehl war: \"%s\", Name: %s, Passwort: %s",
    );
    translations.insert(
        "PostgreSQL username and/or password not valid",
        "PostgreSQL Benutzername und/oder Passwort ungültig",
    );
    translations.insert(
        "Set an admin username.",
        "Setze Administrator Benutzername.",
    );
    translations.insert("Set an admin password.", "Setze Administrator Passwort");
    translations.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.",
        "Ihr Web-Server ist noch nicht für eine Datei-Synchronisation konfiguriert, weil die WebDAV-Schnittstelle vermutlich defekt ist.",
    );
    translations.insert(
        "Please double check the <a href='%s'>installation guides</a>.",
        "Bitte prüfen Sie die <a href='%s'>Installationsanleitungen</a>.",
    );
    translations.insert(
        "Could not find category \"%s\"",
        "Die Kategorie \"%s\" konnte nicht gefunden werden.",
    );
    translations.insert("seconds ago", "Gerade eben");
    translations.insert("today", "Heute");
    translations.insert("yesterday", "Gestern");
    translations.insert("last month", "Letzten Monat");
    translations.insert("last year", "Letztes Jahr");
    translations.insert("years ago", "Vor  Jahren");
    translations.insert("Caused by:", "Verursacht durch:");
    translations
});

static DE_DE_PLURALS: Lazy<HashMap<&'static str, &'static [&'static str]>> = Lazy::new(|| {
    let mut plurals = HashMap::new();
    plurals.insert(
        "_%n minute ago_::_%n minutes ago_",
        &["Vor %n Minute", "Vor %n Minuten"],
    );
    plurals.insert(
        "_%n hour ago_::_%n hours ago_",
        &["Vor %n Stunde", "Vor %n Stunden"],
    );
    plurals.insert(
        "_%n day go_::_%n days ago_",
        &["Vor %n Tag", "Vor %n Tagen"],
    );
    plurals.insert(
        "_%n month ago_::_%n months ago_",
        &["Vor %n Monat", "Vor %n Monaten"],
    );
    plurals
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(text: &str) -> Option<&'static str> {
    DE_DE_TRANSLATIONS.get(text).copied()
}

pub fn get_plural_translation(text: &str, count: usize) -> Option<&'static str> {
    let forms = DE_DE_PLURALS.get(text)?;
    let index = if count != 1 { 1 } else { 0 };
    forms.get(index).copied()
}

pub fn format_translation(text: &str, args: &[&str]) -> String {
    let mut result = get_translation(text).unwrap_or(text).to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("%s", i + 1), arg);
        result = result.replace("%s", arg); // Para compatibilidad con el formato original
    }
    result
}

pub fn format_plural_translation(text: &str, count: usize) -> String {
    match get_plural_translation(text, count) {
        Some(translation) => translation.replace("%n", &count.to_string()),
        None => text.to_string(),
    }
}