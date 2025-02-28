use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Nie można było przywrócić: %s");
        m.insert("Versions", "Wersje");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Nie udało się przywrócić zmiany {sygnatura czasowa} {plik}.");
        m.insert("More versions...", "Więcej wersji...");
        m.insert("No other versions available", "Nie są dostępne żadne inne wersje");
        m.insert("Restore", "Przywróć");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}