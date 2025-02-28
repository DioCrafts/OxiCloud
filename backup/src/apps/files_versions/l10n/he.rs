use rust_i18n::t;

/// Translation strings for Hebrew
pub fn register_he_translations() {
    rust_i18n::set_locale("he");
    
    rust_i18n::translation! {
        he {
            "Could not revert: %s" => "לא ניתן להחזיר: %s",
            "Versions" => "גרסאות",
            "Restore" => "שחזור"
        }
    }
    
    // Set plural form rules for Hebrew
    rust_i18n::set_plural_rule("he", |n| if n != 1 { 1 } else { 0 });
}