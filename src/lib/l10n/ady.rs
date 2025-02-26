use rust_i18n::locale::Translations;

pub fn get_ady_translations() -> Translations {
    let mut translations = Translations::new();
    
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n day go_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
    
    translations.set_plural_form("nplurals=2; plural=(n != 1);".to_string());
    
    translations
}