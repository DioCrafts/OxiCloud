use std::collections::HashMap;
use rust_i18n::define_translation;

// Arabic translation file for files_external module
define_translation!(ar, [
    ("Folder name", "اسم المجلد"),
    ("All Users", "كل المستخدمين"),
    ("Groups", "مجموعات"),
    ("Users", "المستخدمين"),
    ("Delete", "إلغاء"),
]);

// Define plural forms for Arabic
pub fn get_plural_forms() -> &'static str {
    "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;"
}

// Alternative implementation using a HashMap
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Folder name", "اسم المجلد");
    translations.insert("All Users", "كل المستخدمين");
    translations.insert("Groups", "مجموعات");
    translations.insert("Users", "المستخدمين");
    translations.insert("Delete", "إلغاء");
    
    translations
}