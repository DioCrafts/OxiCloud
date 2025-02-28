use phf::phf_map;

/// Translations for Armenian (hy)
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Delete" => "Ջնջել",
};

/// Plural forms rule for Armenian
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";