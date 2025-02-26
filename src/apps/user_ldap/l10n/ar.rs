use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Deletion failed", "فشل الحذف");
    translations.insert("Error", "خطأ");
    translations.insert("Select groups", "إختر مجموعة");
    translations.insert("Save", "حفظ");
    translations.insert("Help", "المساعدة");
    translations.insert("Host", "المضيف");
    translations.insert("Password", "كلمة المرور");
    translations.insert("Back", "رجوع");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5;"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%s group found_::_%s groups found_", vec!["", "", "", "", "", ""]);
    plural_translations.insert("_%s user found_::_%s users found_", vec!["", "", "", "", "", ""]);
    
    plural_translations
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translations_exist() {
        let translations = get_translations();
        assert!(translations.contains_key("Error"));
        assert_eq!(translations.get("Error"), Some(&"خطأ"));
    }
    
    #[test]
    fn test_plural_forms() {
        assert!(!get_plural_forms().is_empty());
    }
}