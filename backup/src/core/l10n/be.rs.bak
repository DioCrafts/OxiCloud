use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", "", ""]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["", "", "", ""]);
    map.insert("_%n day ago_::_%n days ago_", vec!["", "", "", ""]);
    map.insert("_%n month ago_::_%n months ago_", vec!["", "", "", ""]);
    map.insert("_{count} file conflict_::_{count} file conflicts_", vec!["", "", "", ""]);
    map.insert("Advanced", vec!["Дасведчаны"]);
    map.insert("Finish setup", vec!["Завяршыць ўстаноўку."]);
    map
});

pub static PLURAL_FORMS: &str = "nplurals=4; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

pub fn get_translation(key: &str) -> Option<&'static Vec<&'static str>> {
    TRANSLATIONS.get(key)
}