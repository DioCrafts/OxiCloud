use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "ایرر");
        m.insert("Unshare", "شئیرنگ ختم کریں");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", ("", ""));
        m.insert("_%n file_::_%n files_", ("", ""));
        m.insert("_Uploading %n file_::_Uploading %n files_", ("", ""));
        m
    };
}

pub fn plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn get_plural_string(key: &str, count: usize) -> &'static str {
    if let Some((singular, plural)) = PLURAL_FORMS.get(key) {
        if plural_form(count) == 0 {
            return singular;
        } else {
            return plural;
        }
    }
    key
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}