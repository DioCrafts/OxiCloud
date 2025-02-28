use rust_i18n::t;

// Definición de cadenas de traducción
pub fn register_translations() {
    rust_i18n::set_locale("ku_IQ");
    
    rust_i18n::translation!({
        ku_IQ: {
            "Success": "سه‌رکه‌وتن",
            "Error": "هه‌ڵه",
            "_{count}_group_found": {
                one: "",
                other: ""
            },
            "_{count}_user_found": {
                one: "",
                other: ""
            },
            "Save": "پاشکه‌وتکردن",
            "Help": "یارمەتی",
            "Password": "وشەی تێپەربو"
        }
    });
    
    // Configuración de pluralización
    rust_i18n::set_plural_rule("ku_IQ", |n| {
        if n != 1 { "other" } else { "one" }
    });
}