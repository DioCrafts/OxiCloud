// This module contains the Adyghe localizations.

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
        m.insert("_%n day ago_::_%n days ago_", vec!["", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
        m.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";