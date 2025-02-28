use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Парола");
        m.insert("%s shared the folder %s with you", "%s сподели папката %s с Вас");
        m.insert("%s shared the file %s with you", "%s сподели файла %s с Вас");
        m.insert("Download", "Изтегляне");
        m.insert("Upload", "Качване");
        m.insert("Cancel upload", "Спри качването");
        m.insert("No preview available for", "Няма наличен преглед за");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}