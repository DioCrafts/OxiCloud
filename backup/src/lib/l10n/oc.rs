use std::collections::HashMap;
use rust_gettext::prelude::*;

/// Occitan translations for the application
pub fn get_oc_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Help", "Ajuda");
    translations.insert("Personal", "Personal");
    translations.insert("Settings", "Configuracion");
    translations.insert("Users", "Usancièrs");
    translations.insert("Admin", "Admin");
    translations.insert("web services under your control", "Services web jos ton contraròtle");
    translations.insert("ZIP download is turned off.", "Avalcargar los ZIP es inactiu.");
    translations.insert("Files need to be downloaded one by one.", "Los fichièrs devan èsser avalcargats un per un.");
    translations.insert("Back to Files", "Torna cap als fichièrs");
    translations.insert("Authentication error", "Error d'autentificacion");
    translations.insert("Files", "Fichièrs");
    translations.insert("seconds ago", "segonda a");
    translations.insert("_%n minute ago_::_%n minutes ago_", "");
    translations.insert("_%n hour ago_::_%n hours ago_", "");
    translations.insert("today", "uèi");
    translations.insert("yesterday", "ièr");
    translations.insert("_%n day go_::_%n days ago_", "");
    translations.insert("last month", "mes passat");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("last year", "an passat");
    translations.insert("years ago", "ans a");
    
    translations
}

/// Returns the plural form rule for Occitan language
pub fn get_oc_plural_form() -> &'static str {
    "nplurals=2; plural=(n > 1);"
}

/// Initialize Occitan translations
pub fn init_oc_translations() -> Catalog {
    let mut catalog = Catalog::new("oc", get_oc_plural_form());
    
    for (key, value) in get_oc_translations() {
        catalog.add_translation(key, value);
    }
    
    catalog
}