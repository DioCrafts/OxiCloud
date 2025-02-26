use std::collections::HashMap;
use rust_i18n::t;

/// Burmese (Myanmar) translations for user_ldap module
pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("_%s group found_::_%s groups found_".to_string(), "".to_string());
    translations.insert("_%s user found_::_%s users found_".to_string(), "".to_string());
    translations.insert("Help".to_string(), "အကူအညီ".to_string());
    translations.insert("Password".to_string(), "စကားဝှက်".to_string());
    translations
}

/// Plural forms configuration for Burmese language
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

/// Register translations to the i18n system
pub fn register_translations() {
    // This would integrate with whatever i18n system is being used
    // Example implementation depending on the i18n library
    let translations = get_translations();
    for (key, value) in translations {
        rust_i18n::set_translation("my_MM", &key, &value);
    }
    rust_i18n::set_plural_rule("my_MM", PLURAL_FORMS);
}