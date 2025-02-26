use rust_i18n::t;

rust_i18n::i18n!("de_CH");

// Translations for de_CH (Swiss German)
#[macro_export]
macro_rules! init_de_ch_translations {
    () => {
        rust_i18n::set_locale("de_CH");
        rust_i18n::translation!({
            de_CH: {
                "Could not revert: %s": "Konnte %s nicht zurücksetzen",
                "Versions": "Versionen",
                "Failed to revert {file} to revision {timestamp}.": "Konnte {file} der Revision {timestamp} nicht rückgänging machen.",
                "More versions...": "Mehrere Versionen...",
                "No other versions available": "Keine anderen Versionen verfügbar",
                "Restore": "Wiederherstellen"
            }
        });
    };
}

// Define plural forms
pub fn plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}