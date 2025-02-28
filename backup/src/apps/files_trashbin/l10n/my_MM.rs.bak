use phf::phf_map;
use rust_i18n::locale::plurals::PluralCategory;

// Plural form: nplurals=1; plural=0;
pub fn get_plural_form(n: usize) -> PluralCategory {
    PluralCategory::One
}

pub static TRANSLATIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "_%n folder_::_%n folders_" => &[""],
    "_%n file_::_%n files_" => &[""],
};