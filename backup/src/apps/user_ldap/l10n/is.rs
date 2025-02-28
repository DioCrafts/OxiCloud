use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::Plurals;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Keep settings?", "Geyma stillingar ?");
        map.insert("Error", "Villa");
        map.insert("_%s group found_::_%s groups found_", "");
        map.insert("_%s user found_::_%s users found_", "");
        map.insert("Save", "Vista");
        map.insert("Test Configuration", "Prúfa uppsetningu");
        map.insert("Help", "Hjálp");
        map.insert("Host", "Netþjónn");
        map.insert("Password", "Lykilorð");
        map
    };

    pub static ref PLURAL_FORMS: Plurals = Plurals {
        nplurals: 2,
        plural_fn: |n| (n != 1) as usize
    };
}