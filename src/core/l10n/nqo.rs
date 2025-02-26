use rust_i18n::locale::{LocaleTranslations, PluralForm};

pub fn get_nqo_translations() -> LocaleTranslations {
    let mut translations = LocaleTranslations::new();
    
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(), 
        vec!["".to_string()]
    );
    
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(), 
        vec!["".to_string()]
    );
    
    translations.insert(
        "_%n day ago_::_%n days ago_".to_string(), 
        vec!["".to_string()]
    );
    
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(), 
        vec!["".to_string()]
    );
    
    translations.insert(
        "_{count} file conflict_::_{count} file conflicts_".to_string(), 
        vec!["".to_string()]
    );
    
    translations.set_plural_form(PluralForm::new(1, "n==0".to_string()));
    
    translations
}