// apps/files_trashbin/l10n/bn_BD.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Error", "সমস্যা");
    m.insert("Name", "রাম");
    m.insert("Delete", "মুছে");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";