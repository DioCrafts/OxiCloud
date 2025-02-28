use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "กำลังบันทึกข้อมูล...");
    map.insert("Encryption", "การเข้ารหัส");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";