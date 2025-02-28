use phf::phf_map;

// Static map of translations for Georgian (ka)
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Password" => "პაროლი",
    "Download" => "გადმოწერა",
};

// Plural forms definition for Georgian language
pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";