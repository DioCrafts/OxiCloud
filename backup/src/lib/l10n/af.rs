// lib/l10n/af.rs

use crate::l10n::PluralFormula;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    map.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    map.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    map
});

pub static PLURAL_FORMS: PluralFormula = |n| if n != 1 { 1 } else { 0 };