use std::collections::HashMap;
use rust_i18n::t;

/// German (Switzerland) translations for files_sharing module
pub fn register_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "The password is wrong. Try again.".to_string(),
        "Das Passwort ist falsch. Bitte versuchen Sie es erneut.".to_string(),
    );
    translations.insert(
        "Password".to_string(),
        "Passwort".to_string(),
    );
    translations.insert(
        "Sorry, this link doesn't seem to work anymore.".to_string(),
        "Entschuldigung, dieser Link scheint nicht mehr zu funktionieren.".to_string(),
    );
    translations.insert(
        "Reasons might be:".to_string(),
        "Gründe könnten sein:".to_string(),
    );
    translations.insert(
        "the item was removed".to_string(),
        "Das Element wurde entfernt".to_string(),
    );
    translations.insert(
        "the link expired".to_string(),
        "Der Link ist abgelaufen".to_string(),
    );
    translations.insert(
        "sharing is disabled".to_string(),
        "Teilen ist deaktiviert".to_string(),
    );
    translations.insert(
        "For more info, please ask the person who sent this link.".to_string(),
        "Für mehr Informationen, fragen Sie bitte die Person, die Ihnen diesen Link geschickt hat.".to_string(),
    );
    translations.insert(
        "%s shared the folder %s with you".to_string(),
        "%s hat den Ordner %s mit Ihnen geteilt".to_string(),
    );
    translations.insert(
        "%s shared the file %s with you".to_string(),
        "%s hat die Datei %s mit Ihnen geteilt".to_string(),
    );
    translations.insert(
        "Download".to_string(),
        "Herunterladen".to_string(),
    );
    translations.insert(
        "Upload".to_string(),
        "Hochladen".to_string(),
    );
    translations.insert(
        "Cancel upload".to_string(),
        "Upload abbrechen".to_string(),
    );
    translations.insert(
        "No preview available for".to_string(),
        "Es ist keine Vorschau verfügbar für".to_string(),
    );
    
    translations
}

/// Returns the plural forms rule for German (Switzerland)
pub fn plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

/// Initialize the German (Switzerland) locale for files_sharing module
pub fn init_de_ch_locale() {
    let translations = register_translations();
    // Initialization code would depend on the specific i18n system used
    // This is a placeholder for the actual implementation
}