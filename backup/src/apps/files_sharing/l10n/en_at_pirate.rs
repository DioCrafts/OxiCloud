use phf::phf_map;
use once_cell::sync::Lazy;

/// English (Pirate) translations for the files_sharing app
pub static TRANSLATIONS: Lazy<phf::Map<&'static str, &'static str>> = Lazy::new(|| {
    phf_map! {
        "Password" => "Secret Code",
        "%s shared the folder %s with you" => "%s shared the folder %s with you",
        "%s shared the file %s with you" => "%s shared the file %s with you",
        "Download" => "Download",
        "No preview available for" => "No preview available for",
    }
});

/// Plural forms rule for English (Pirate)
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Returns a translation for the given key or the key itself if not found
pub fn translate(key: &str) -> &str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

/// Gets the plural form index for the given count
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}