use rust_i18n::translation_functions;

/// Translations for the Cymraeg-Great Britain language.
pub fn translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    translations.insert("Restore".to_string(), "Adfer".to_string());
    translations
}

/// Defines the plural forms for Cymraeg-Great Britain.
///
/// The formula follows:
/// nplurals=4; plural=(n==1) ? 0 : (n==2) ? 1 : (n != 8 && n != 11) ? 2 : 3;
pub fn plural_forms(n: usize) -> usize {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else if n != 8 && n != 11 {
        2
    } else {
        3
    }
}

// Register the translation functions
translation_functions!(cy_GB, "cy_GB");