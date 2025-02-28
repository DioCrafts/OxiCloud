use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        m.insert("_%s user found_::_%s users found_", vec!["", ""]);
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}