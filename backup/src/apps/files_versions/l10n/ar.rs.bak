use std::collections::HashMap;
use rust_i18n::dialect::PluralForms;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Versions", "الإصدارات");
    translations.insert("Restore", "استعيد");
    translations
}

pub fn get_plural_forms() -> PluralForms {
    PluralForms::new(6, "n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5")
}