// Translation file for Kurdish Iraq (ku_IQ)

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Folder name", "ناوی بوخچه");
        m.insert("Users", "به‌كارهێنه‌ر");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}