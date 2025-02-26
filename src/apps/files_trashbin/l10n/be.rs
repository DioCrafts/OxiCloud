use rust_i18n::translation_mapping;

/// Belarusian translation mapping for files_trashbin
/// 
/// This file defines plural forms and translations for Belarusian language

pub fn get_translation_mapping() -> rust_i18n::TranslationMapping {
    translation_mapping! {
        "_%n folder_::_%n folders_" => {
            0 => "",
            1 => "",
            2 => "",
            3 => "",
        },
        "_%n file_::_%n files_" => {
            0 => "",
            1 => "",
            2 => "",
            3 => "",
        },
    }
}

pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}