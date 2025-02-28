use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", "", ""]);
        m.insert("_%n day go_::_%n days ago_", vec!["", "", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", "", ""]);
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";