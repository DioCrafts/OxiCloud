use phf::phf_map;
use rust_i18n::locale::PluralForms;

// Spanish (Mexico) translations
pub static ES_MX_TRANSLATIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "_%n folder_::_%n folders_" => &["", ""],
    "_%n file_::_%n files_" => &["", ""],
};

pub const ES_MX_PLURAL_FORMS: PluralForms = PluralForms {
    n_plurals: 2,
    plural_fn: |n| (n != 1) as usize,
};