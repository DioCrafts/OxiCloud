use std::collections::HashMap;
use rust_gettext::prelude::*;

lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec![""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec![""]);
        m.insert("_%n day ago_::_%n days ago_", vec![""]);
        m.insert("_%n month ago_::_%n months ago_", vec![""]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec![""]);
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub fn initialize_translation() -> Option<GettextTranslation> {
    let mut gt = GettextTranslation::new("uz");
    for (key, values) in TRANSLATIONS.iter() {
        let parts: Vec<&str> = key.split("::").collect();
        if parts.len() == 2 {
            gt.add_plural_translation(parts[0], parts[1], values.to_vec());
        }
    }
    gt.set_plural_forms_rule(get_plural_forms());
    Some(gt)
}