use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Diese Freigabe ist durch ein Passwort geschützt");
        m.insert("The password is wrong. Try again.", "Das Passwort ist falsch. Bitte versuchen Sie es erneut.");
        m.insert("Password", "Passwort");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Entschuldigung, dieser Link scheint nicht mehr zu funktionieren.");
        m.insert("Reasons might be:", "Gründe könnten sein:");
        m.insert("the item was removed", "Das Element wurde entfernt");
        m.insert("the link expired", "Der Link ist abgelaufen");
        m.insert("sharing is disabled", "Teilen ist deaktiviert");
        m.insert("For more info, please ask the person who sent this link.", "Für mehr Informationen, fragen Sie bitte die Person, die Ihnen diesen Link geschickt hat.");
        m.insert("%s shared the folder %s with you", "%s hat den Ordner %s mit Ihnen geteilt");
        m.insert("%s shared the file %s with you", "%s hat die Datei %s mit Ihnen geteilt");
        m.insert("Download", "Herunterladen");
        m.insert("Upload", "Hochladen");
        m.insert("Cancel upload", "Upload abbrechen");
        m.insert("No preview available for", "Es ist keine Vorschau verfügbar für");
        m.insert("Direct link", "Direkte Verlinkung");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn format_translation(key: &str, args: &[&str]) -> String {
    if let Some(translation) = get_translation(key) {
        let mut result = String::from(translation);
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("%s", i+1), arg);
        }
        result
    } else {
        key.to_string()
    }
}