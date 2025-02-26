use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Dutch (nl) translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "App \"%s\" kan niet worden geïnstalleerd omdat die niet compatible is met deze versie van ownCloud.");
    translations.insert("No app name specified", "De app naam is niet gespecificeerd.");
    translations.insert("Help", "Help");
    translations.insert("Personal", "Persoonlijk");
    translations.insert("Settings", "Instellingen");
    translations.insert("Users", "Gebruikers");
    translations.insert("Admin", "Beheerder");
    translations.insert("Failed to upgrade \"%s\".", "Upgrade \"%s\" mislukt.");
    translations.insert("Unknown filetype", "Onbekend bestandsformaat");
    translations.insert("Invalid image", "Ongeldige afbeelding");
    translations.insert("web services under your control", "Webdiensten in eigen beheer");
    translations.insert("cannot open \"%s\"", "Kon \"%s\" niet openen");
    translations.insert("ZIP download is turned off.", "ZIP download is uitgeschakeld.");
    translations.insert("Files need to be downloaded one by one.", "Bestanden moeten één voor één worden gedownload.");
    translations.insert("Back to Files", "Terug naar bestanden");
    translations.insert("Selected files too large to generate zip file.", "De geselecteerde bestanden zijn te groot om een zip bestand te maken.");
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Download de bestanden in kleinere brokken, appart of vraag uw administrator.");
    translations.insert("No source specified when installing app", "Geen bron opgegeven bij installatie van de app");
    translations.insert("No href specified when installing app from http", "Geen href opgegeven bij installeren van de app vanaf http");
    translations.insert("No path specified when installing app from local file", "Geen pad opgegeven bij installeren van de app vanaf een lokaal bestand");
    translations.insert("Archives of type %s are not supported", "Archiefbestanden van type %s niet ondersteund");
    translations.insert("Failed to open archive when installing app", "Kon archiefbestand bij installatie van de app niet openen");
    translations.insert("App does not provide an info.xml file", "De app heeft geen info.xml bestand");
    translations.insert("App can't be installed because of not allowed code in the App", "De app kan niet worden geïnstalleerd wegens onjuiste code in de app");
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "De app kan niet worden geïnstalleerd omdat die niet compatible is met deze versie van ownCloud");
    translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "De app kan niet worden geïnstallerd omdat het de <shipped>true</shipped> tag bevat die niet is toegestaan voor niet gepubliceerde apps");
    translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "De app kan niet worden geïnstalleerd omdat de versie in info.xml/version niet dezelfde is als de versie zoals die in de app store staat vermeld");
    translations.insert("App directory already exists", "App directory bestaat al");
    translations.insert("Can't create app folder. Please fix permissions. %s", "Kan de app map niet aanmaken, Herstel de permissies. %s");
    translations.insert("Application is not enabled", "De applicatie is niet actief");
    translations.insert("Authentication error", "Authenticatie fout");
    translations.insert("Token expired. Please reload page.", "Token verlopen.  Herlaad de pagina.");
    translations.insert("Files", "Bestanden");
    translations.insert("Text", "Tekst");
    translations.insert("Images", "Afbeeldingen");
    translations.insert("%s enter the database username.", "%s opgeven database gebruikersnaam.");
    translations.insert("%s enter the database name.", "%s opgeven databasenaam.");
    translations.insert("%s you may not use dots in the database name", "%s er mogen geen puntjes in de databasenaam voorkomen");
    translations.insert("MS SQL username and/or password not valid: %s", "MS SQL gebruikersnaam en/of wachtwoord niet geldig: %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Geef of een bestaand account op of het beheerdersaccount.");
    translations.insert("MySQL username and/or password not valid", "MySQL gebruikersnaam en/of wachtwoord ongeldig");
    translations.insert("DB Error: \"%s\"", "DB Fout: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Onjuiste commande was: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "MySQL gebruiker '%s'@'localhost' bestaat al.");
    translations.insert("Drop this user from MySQL", "Verwijder deze gebruiker uit MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "MySQL gebruiker '%s'@'%%' bestaat al");
    translations.insert("Drop this user from MySQL.", "Verwijder deze gebruiker uit MySQL.");
    translations.insert("Oracle connection could not be established", "Er kon geen verbinding met Oracle worden bereikt");
    translations.insert("Oracle username and/or password not valid", "Oracle gebruikersnaam en/of wachtwoord ongeldig");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Onjuiste commando was: \"%s\", naam: %s, wachtwoord: %s");
    translations.insert("PostgreSQL username and/or password not valid", "PostgreSQL gebruikersnaam en/of wachtwoord ongeldig");
    translations.insert("Set an admin username.", "Stel de gebruikersnaam van de beheerder in.");
    translations.insert("Set an admin password.", "Stel een beheerderswachtwoord in.");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Uw webserver is nog niet goed ingesteld voor bestandssynchronisatie omdat de WebDAV interface verbroken lijkt.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Controleer de <a href='%s'>installatiehandleiding</a> goed.");
    translations.insert("Could not find category \"%s\"", "Kon categorie \"%s\" niet vinden");
    translations.insert("seconds ago", "seconden geleden");
    translations.insert("today", "vandaag");
    translations.insert("yesterday", "gisteren");
    translations.insert("last month", "vorige maand");
    translations.insert("last year", "vorig jaar");
    translations.insert("years ago", "jaar geleden");
    translations.insert("Caused by:", "Gekomen door:");
    
    translations
});

/// Plural forms for Dutch language
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Pluralization function for Dutch translations
pub fn get_plural_form(n: usize, key: &str) -> &'static str {
    match key {
        "_%n minute ago_::_%n minutes ago_" => {
            if n == 1 {
                "%n minuut geleden"
            } else {
                "%n minuten geleden"
            }
        },
        "_%n hour ago_::_%n hours ago_" => {
            if n == 1 {
                "%n uur geleden"
            } else {
                "%n uur geleden"
            }
        },
        "_%n day go_::_%n days ago_" => {
            if n == 1 {
                "%n dag terug"
            } else {
                "%n dagen geleden"
            }
        },
        "_%n month ago_::_%n months ago_" => {
            if n == 1 {
                "%n maand geleden"
            } else {
                "%n maanden geleden"
            }
        },
        _ => "",
    }
}