use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Share".to_string(), "Podijeli".to_string());
    translations.insert("Add".to_string(), "Dodaj".to_string());
    
    translations
}

pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string(), "".to_string()]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string(), "".to_string()]);
    plural_translations.insert("_%n day ago_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string(), "".to_string()]);
    plural_translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string(), "".to_string()]);
    plural_translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), vec!["".to_string(), "".to_string(), "".to_string()]);
    
    plural_translations
}

pub fn get_plural_form() -> String {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);".to_string()
}

pub fn plural_index(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}