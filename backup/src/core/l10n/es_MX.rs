// core/l10n/es_mx.rs

use rust_i18n::i18n;

pub fn register_translations() -> i18n::Translations {
    let mut translations = i18n::Translations::new();
    
    // Add plural forms for Spanish (Mexico)
    translations.set_plural_forms("nplurals=2; plural=(n != 1);");
    
    // Register translations with plural variants
    translations.add_plural("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    translations.add_plural("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    translations.add_plural("_%n day ago_::_%n days ago_", vec!["", ""]);
    translations.add_plural("_%n month ago_::_%n months ago_", vec!["", ""]);
    translations.add_plural("_{count} file conflict_::_{count} file conflicts_", vec!["", ""]);
    
    translations
}

pub fn get_translations() -> i18n::Translations {
    register_translations()
}