use rust_i18n::Translations;

pub fn get_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    translations.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    translations.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    translations.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    
    translations
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";