use rust_i18n::t;

pub fn initialize() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    // Plural forms for groups found
    translations.add("_s group found_::_s groups found_", vec!["", ""]);
    
    // Plural forms for users found
    translations.add("_s user found_::_s users found_", vec!["", ""]);
    
    // Simple translations
    translations.add("Help", "Hulp");
    translations.add("Password", "Wagwoord");
    
    // Set plural forms rule
    translations.set_plural_rule("nplurals=2; plural=(n != 1);");
    
    translations
}