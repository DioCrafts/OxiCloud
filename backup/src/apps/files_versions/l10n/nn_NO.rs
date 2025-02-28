use rust_i18n::i18n;

i18n!("nn_NO");

pub fn get_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert("Could not revert: %s".to_string(), "Klarte ikkje å tilbakestilla: %s".to_string());
    translations.insert("Versions".to_string(), "Utgåver".to_string());
    translations.insert("Failed to revert {file} to revision {timestamp}.".to_string(), "Klarte ikkje å tilbakestilla {file} til utgåva {timestamp}.".to_string());
    translations.insert("More versions...".to_string(), "Fleire utgåver …".to_string());
    translations.insert("No other versions available".to_string(), "Ingen andre utgåver tilgjengeleg".to_string());
    translations.insert("Restore".to_string(), "Gjenopprett".to_string());
    
    translations.set_plural_form(|n| if n != 1 { 1 } else { 0 });
    
    translations
}