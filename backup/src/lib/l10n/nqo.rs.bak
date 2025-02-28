use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct NqoLocalization;

impl NqoLocalization {
    pub fn translations() -> &'static HashMap<&'static str, Vec<&'static str>> {
        static TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
            let mut map = HashMap::new();
            map.insert("_%n minute ago_::_%n minutes ago_", vec![""]);
            map.insert("_%n hour ago_::_%n hours ago_", vec![""]);
            map.insert("_%n day go_::_%n days ago_", vec![""]);
            map.insert("_%n month ago_::_%n months ago_", vec![""]);
            map
        });
        &TRANSLATIONS
    }

    pub fn plural_forms() -> &'static str {
        "nplurals=1; plural=0;"
    }
}