use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%s group found_::_%s groups found_", vec![""]);
        map.insert("_%s user found_::_%s users found_", vec![""]);
        map
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}