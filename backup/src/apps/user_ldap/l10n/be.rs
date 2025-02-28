use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::{
    i18n_embed_i18n, 
    language_ids, 
    t, 
    I18nAssets, 
    I18nConfig, 
    I18nEmbed, 
    I18nEmbedError, 
    language_id, 
    LanguageIdentifier,
    PluralsSelector
};

lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%s group found_::_%s groups found_", vec!["", "", "", ""]);
        m.insert("_%s user found_::_%s users found_", vec!["", "", "", ""]);
        m
    };
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=4; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub struct BelarusianPlurals;

impl PluralsSelector for BelarusianPlurals {
    fn select(&self, n: usize) -> usize {
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    }
}

pub fn get_translation(key: &str) -> Option<&'static Vec<&'static str>> {
    TRANSLATIONS.get(key)
}