use phf::phf_map;
use rust_i18n::translation::PluralForms;

#[derive(Debug, Clone)]
pub struct NdsLocale;

impl NdsLocale {
    pub fn plural_forms() -> PluralForms {
        PluralForms::new("nplurals=2; plural=(n != 1);")
    }
}

pub static TRANSLATIONS: phf::Map<&'static str, [&'static str; 2]> = phf_map! {
    "_%n minute ago_::_%n minutes ago_" => ["", ""],
    "_%n hour ago_::_%n hours ago_" => ["", ""],
    "_%n day ago_::_%n days ago_" => ["", ""],
    "_%n month ago_::_%n months ago_" => ["", ""],
    "_{count} file conflict_::_{count} file conflicts_" => ["", ""],
};