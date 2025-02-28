use i18n_embed_fl::fl;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    // Plural forms for Bosnian language
    // nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);
    
    // These translations would typically be loaded from a file in a real implementation
    translations.insert(String::from("_%s group found_::_%s groups found_"), String::from(""));
    translations.insert(String::from("_%s user found_::_%s users found_"), String::from(""));
    translations.insert(String::from("Save"), String::from("Spasi"));
    
    translations
}

pub fn get_plural_form() -> String {
    String::from("nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);")
}