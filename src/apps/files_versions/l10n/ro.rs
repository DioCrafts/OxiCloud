use rust_i18n::t;

/// Romanian translation definitions
pub fn register_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    // Register simple translations
    translations.insert("Could not revert: %s", "Nu a putut reveni: %s");
    translations.insert("Versions", "Versiuni");
    
    // Register plural forms rule
    translations.set_plural_rule(|n| {
        if n == 1 {
            0
        } else if (n % 100 > 19) || ((n % 100 == 0) && (n != 0)) {
            2
        } else {
            1
        }
    });
    
    translations
}