use rust_i18n::i18n;

i18n!("af_ZA");

pub fn get_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    translations.insert("Help".to_string(), "Hulp".to_string());
    translations.insert("Personal".to_string(), "Persoonlik".to_string());
    translations.insert("Settings".to_string(), "Instellings".to_string());
    translations.insert("Users".to_string(), "Gebruikers".to_string());
    translations.insert("Admin".to_string(), "Admin".to_string());
    translations.insert("web services under your control".to_string(), "webdienste onder jou beheer".to_string());
    
    // Plurals
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
    translations.insert("_%n day go_::_%n days ago_".to_string(), "".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
    
    translations
}

pub fn get_plural_form() -> String {
    "nplurals=2; plural=(n != 1);".to_string()
}