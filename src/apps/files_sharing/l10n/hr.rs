use once_cell::sync::Lazy;
use std::collections::HashMap;
use i18n::PluralForm;

/// Croatian translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Password", "Lozinka");
    map.insert("Download", "Preuzimanje");
    map.insert("Upload", "Učitaj");
    map.insert("Cancel upload", "Prekini upload");
    map
});

/// Croatian plural forms
pub static PLURAL_FORMS: Lazy<PluralForm> = Lazy::new(|| {
    PluralForm::new(3, Box::new(|n| {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    }))
});