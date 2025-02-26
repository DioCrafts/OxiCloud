use phf::phf_map;
use rust_i18n::locale::PluralRules;

static TRANSLATIONS: phf::Map<&'static str, [&'static str; 2]> = phf_map! {
    "_%n folder_::_%n folders_" => ["", ""],
    "_%n file_::_%n files_" => ["", ""],
    "_Uploading %n file_::_Uploading %n files_" => ["", ""],
};

pub struct Ady;

impl Ady {
    pub fn get_plural_forms() -> PluralRules {
        // "nplurals=2; plural=(n != 1);"
        Box::new(|n| if n != 1 { 1 } else { 0 })
    }

    pub fn get_translations() -> &'static phf::Map<&'static str, [&'static str; 2]> {
        &TRANSLATIONS
    }
}