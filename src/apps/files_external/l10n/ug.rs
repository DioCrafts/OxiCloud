use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Folder name", "قىسقۇچ ئاتى");
        m.insert("External storage", "سىرتقى ساقلىغۇچ");
        m.insert("Configuration", "سەپلىمە");
        m.insert("Options", "تاللانما");
        m.insert("Groups", "گۇرۇپپا");
        m.insert("Users", "ئىشلەتكۈچىلەر");
        m.insert("Delete", "ئۆچۈر");
        m
    };
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}