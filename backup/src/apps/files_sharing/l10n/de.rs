use std::collections::HashMap;
use rust_i18n::t;

pub fn init_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("This share is password-protected", "Diese Freigabe ist durch ein Passwort geschützt");
    translations.insert("The password is wrong. Try again.", "Bitte überprüfen sie Ihr Passwort und versuchen Sie es erneut.");
    translations.insert("Password", "Passwort");
    translations.insert("Sorry, this link doesn't seem to work anymore.", "Entschuldigung, dieser Link scheint nicht mehr zu funktionieren.");
    translations.insert("Reasons might be:", "Gründe könnten sein:");
    translations.insert("the item was removed", "Die Elemente wurden entfernt");
    translations.insert("the link expired", "Der Link ist abgelaufen");
    translations.insert("sharing is disabled", "Teilen ist deaktiviert");
    translations.insert("For more info, please ask the person who sent this link.", "Für mehr Informationen, frage bitte die Person, die dir diesen Link geschickt hat.");
    translations.insert("%s shared the folder %s with you", "%s hat den Ordner %s mit Dir geteilt");
    translations.insert("%s shared the file %s with you", "%s hat die Datei %s mit Dir geteilt");
    translations.insert("Download", "Download");
    translations.insert("Upload", "Hochladen");
    translations.insert("Cancel upload", "Upload abbrechen");
    translations.insert("No preview available for", "Es ist keine Vorschau verfügbar für");
    translations.insert("Direct link", "Direkte Verlinkung");

    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}