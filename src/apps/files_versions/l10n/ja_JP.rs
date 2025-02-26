use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "元に戻せませんでした: %s");
        m.insert("Versions", "バージョン");
        m.insert("Failed to revert {file} to revision {timestamp}.", "{file} を {timestamp} のリヴィジョンに戻すことができません。");
        m.insert("More versions...", "もっと他のバージョン...");
        m.insert("No other versions available", "利用可能な他のバージョンはありません");
        m.insert("Restore", "復元");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

i18n!("ja_JP");