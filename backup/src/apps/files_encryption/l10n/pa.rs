// Translation file for Rust
// Corresponds to apps/files_encryption/l10n/pa.php

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Saving...", "...ਸੰਭਾਲਿਆ ਜਾ ਰਿਹਾ ਹੈ");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}