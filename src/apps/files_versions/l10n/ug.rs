use rust_i18n::t;

lazy_static::lazy_static! {
    static ref TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
        "Could not revert: %s" => "ئەسلىگە قايتۇرالمايدۇ: %s",
        "Versions" => "نەشرى",
    };
}

#[derive(Debug, Clone, Copy)]
pub struct UgLocale;

impl rust_i18n::Locale for UgLocale {
    fn locale_code(&self) -> &'static str {
        "ug"
    }

    fn plural_forms(&self) -> &'static str {
        "nplurals=1; plural=0;"
    }

    fn get_translation(&self, key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }
}

pub fn register_locale() {
    rust_i18n::register_locale(Box::new(UgLocale));
}