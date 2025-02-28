use phf::phf_map;
use rust_gettext::gettext_ngettext;

// Translation map for nds locale
static TRANSLATIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "%s group found_%s groups found" => &["", ""],
    "%s user found_%s users found" => &["", ""],
};

// Plural form expression for nds locale
static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Returns the translation map for nds locale
pub fn get_translations() -> &'static phf::Map<&'static str, &'static [&'static str]> {
    &TRANSLATIONS
}

/// Returns the plural form expression for nds locale
pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}