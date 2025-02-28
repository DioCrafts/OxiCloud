use rust_i18n::i18n;

// Define the translation struct for nds locale
pub fn register_nds_translations() -> rust_i18n::Translations {
    let mut translations = rust_i18n::Translations::new();
    
    // Add plural forms
    translations.set_plural_rule("nplurals=2; plural=(n != 1);");
    
    // Add translations with plural forms
    translations.add("%n folder", vec!["", ""]);
    translations.add("%n file", vec!["", ""]);
    translations.add("Uploading %n file", vec!["", ""]);
    
    translations
}

// Register translations when module loads
#[cfg(feature = "nds")]
i18n!("nds" => register_nds_translations());