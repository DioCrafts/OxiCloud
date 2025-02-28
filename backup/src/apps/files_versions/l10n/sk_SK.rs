use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Nemožno obnoviť: %s");
        m.insert("Versions", "Verzie");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Zlyhalo obnovenie súboru {file} na verziu {timestamp}.");
        m.insert("More versions...", "Viac verzií...");
        m.insert("No other versions available", "Žiadne ďalšie verzie nie sú dostupné");
        m.insert("Restore", "Obnoviť");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";