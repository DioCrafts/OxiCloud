use rust_i18n::i18n;

i18n!("ku_IQ");

#[rust_i18n::translations("ku_IQ")]
pub(crate) const TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Saving..." => "پاشکه‌وتده‌کات...",
    "Encryption" => "نهێنیکردن",
};

#[allow(dead_code)]
pub(crate) const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";