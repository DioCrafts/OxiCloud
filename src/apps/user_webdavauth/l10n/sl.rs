use rust_i18n::t;

// Define translations for Slovenian language
pub fn register_translations() {
    rust_i18n::set_locale("sl");
    
    rust_i18n::translations! {
        sl {
            "WebDAV Authentication" => "Overitev WebDAV",
        }
    }
}

// Plural forms configuration for Slovenian
// nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);
pub fn get_plural_form(n: usize) -> usize {
    let n_mod_100 = n % 100;
    
    if n_mod_100 == 1 {
        0
    } else if n_mod_100 == 2 {
        1
    } else if n_mod_100 == 3 || n_mod_100 == 4 {
        2
    } else {
        3
    }
}