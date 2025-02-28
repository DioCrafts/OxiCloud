use phf::phf_map;
use rust_i18n::i18n;

// Croatian translations
static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Groups" => "Grupe",
    "Users" => "Korisnici",
    "Delete" => "Obriši",
};

i18n!("hr", plural_forms = "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;");

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}