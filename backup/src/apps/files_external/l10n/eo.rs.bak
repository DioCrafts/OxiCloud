use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Alirpermeso donita");
        m.insert("Error configuring Dropbox storage", "Eraro dum agordado de la memorservo Dropbox");
        m.insert("Grant access", "Doni alirpermeson");
        m.insert("Please provide a valid Dropbox app key and secret.", "Bonvolu provizi ŝlosilon de la aplikaĵo Dropbox validan kaj sekretan.");
        m.insert("Error configuring Google Drive storage", "Eraro dum agordado de la memorservo Google Drive");
        m.insert("External Storage", "Malena memorilo");
        m.insert("Folder name", "Dosierujnomo");
        m.insert("Configuration", "Agordo");
        m.insert("Options", "Malneproj");
        m.insert("Applicable", "Aplikebla");
        m.insert("None set", "Nenio agordita");
        m.insert("All Users", "Ĉiuj uzantoj");
        m.insert("Groups", "Grupoj");
        m.insert("Users", "Uzantoj");
        m.insert("Delete", "Forigi");
        m.insert("Enable User External Storage", "Kapabligi malenan memorilon de uzanto");
        m.insert("Allow users to mount their own external storage", "Permesi al uzantoj surmeti siajn proprajn malenajn memorilojn");
        m.insert("SSL root certificates", "Radikaj SSL-atestoj");
        m.insert("Import Root Certificate", "Enporti radikan ateston");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}