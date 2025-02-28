use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::Plural;

lazy_static! {
    /// Finnish (Finland) translations
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Palautus epäonnistui: %s");
        m.insert("Versions", "Versiot");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Tiedoston {file} palautus versioon {timestamp} epäonnistui.");
        m.insert("More versions...", "Lisää versioita...");
        m.insert("No other versions available", "Ei muita versioita saatavilla");
        m.insert("Restore", "Palauta");
        m
    };

    pub static ref PLURAL_FORMS: Plural = Plural {
        nplurals: 2,
        plural_fn: |n| -> usize { if n != 1 { 1 } else { 0 } },
    };
}