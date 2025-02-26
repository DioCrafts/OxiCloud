use phf::phf_map;
use rust_i18n::plural::PluralCategory;

// Serbian translations
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Password" => "Лозинка",
    "Download" => "Преузми",
    "Upload" => "Отпреми",
    "Cancel upload" => "Прекини отпремање",
};

pub fn get_plural_category(n: i64) -> PluralCategory {
    if n % 10 == 1 && n % 100 != 11 {
        PluralCategory::One
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        PluralCategory::Few
    } else {
        PluralCategory::Other
    }
}