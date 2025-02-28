use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Tidak dapat kembalikan: %s");
        m.insert("Versions", "Versi");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Gagal kembalikan {file} ke semakan {timestamp}.");
        m.insert("More versions...", "Lagi versi...");
        m.insert("No other versions available", "Tiada lagi versi lain");
        m.insert("Restore", "Pulihkan");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";