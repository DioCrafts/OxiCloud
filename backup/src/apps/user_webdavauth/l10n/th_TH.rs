use rust_i18n::t;

// Define translations for Thai (Thailand)
pub fn register_th_th_translations() {
    rust_i18n::set_locale("th_TH");
    
    // Main translations
    rust_i18n::translation!("th_TH", {
        "WebDAV Authentication" => "WebDAV Authentication",
    });
    
    // Set plural forms rule
    rust_i18n::set_pluralization_rule("th_TH", |n| {
        // "nplurals=1; plural=0;"
        0
    });
}