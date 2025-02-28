use rust_i18n::i18n;
use std::collections::HashMap;
use phf::phf_map;

// Translation data for Serbian Latin locale
i18n!("sr@latin");

#[derive(Debug, Clone)]
pub struct SerbianLatinTranslations;

impl SerbianLatinTranslations {
    pub fn init() -> HashMap<&'static str, &'static str> {
        let translations = phf_map! {
            "Error" => "Greška",
            "Save" => "Snimi",
            "Help" => "Pomoć",
            "Password" => "Lozinka",
        };
        
        translations.into_iter().collect()
    }
    
    pub fn get_plural_forms() -> &'static str {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
    }
    
    pub fn translate_plural(key: &str, count: usize) -> &'static str {
        match key {
            "%s group found" | "%s groups found" => {
                match SerbianLatinTranslations::get_plural_index(count) {
                    0 => "",
                    1 => "",
                    _ => "",
                }
            },
            "%s user found" | "%s users found" => {
                match SerbianLatinTranslations::get_plural_index(count) {
                    0 => "",
                    1 => "",
                    _ => "",
                }
            },
            _ => "",
        }
    }
    
    fn get_plural_index(n: usize) -> usize {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    }
}