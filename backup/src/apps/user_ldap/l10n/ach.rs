use std::collections::HashMap;
use lazy_static::lazy_static;
use i18n::PluralForms;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        map.insert("_%s user found_::_%s users found_", vec!["", ""]);
        map
    };
    
    pub static ref PLURAL_FORMS: PluralForms = PluralForms {
        nplurals: 2,
        plural_function: |n| (n > 1) as usize,
    };
}