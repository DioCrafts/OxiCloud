use phf::phf_map;

// Translations mapping
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Download" => "Բեռնել",
};

// Plural forms rule
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";