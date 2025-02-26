use rust_i18n::i18n;

i18n!("af");

#[derive(Default)]
pub struct AfLocale;

impl AfLocale {
    pub fn new() -> Self {
        Self
    }

    pub fn translations(&self) -> rust_i18n::Translations {
        let mut translations = rust_i18n::Translations::new();
        
        translations.insert(
            "_s group found_::_s groups found_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations.insert(
            "_s user found_::_s users found_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations
    }

    pub fn plural_forms(&self) -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }
}