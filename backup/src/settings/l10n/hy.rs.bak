use std::collections::HashMap;
use rust_gettext::prelude::*;

/// Armenian translations for the application
pub fn get_hy_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Delete", "Ջնջել");
    translations.insert("Other", "Այլ");
    
    translations
}

/// Returns the plural form expression for Armenian
pub fn get_hy_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

/// Initializes Armenian translations for gettext
pub fn init_hy_localization() -> PluralForms {
    let translations = get_hy_translations();
    
    // Create a plural forms parser for Armenian
    let plural_forms = PluralForms::from_str(get_hy_plural_form())
        .expect("Failed to parse Armenian plural forms");
    
    // Here you would typically register translations with your i18n system
    
    plural_forms
}