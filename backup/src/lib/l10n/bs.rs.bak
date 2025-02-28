use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", "", ""]);
        m.insert("_%n day go_::_%n days ago_", vec!["", "", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", "", ""]);
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}