use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The password is wrong. Try again.", "La contraseña no es correcta. Probá de nuevo.");
        m.insert("Password", "Contraseña");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Perdón, este enlace parece no funcionar más.");
        m.insert("Reasons might be:", "Las causas podrían ser:");
        m.insert("the item was removed", "el elemento fue borrado");
        m.insert("the link expired", "el enlace expiró");
        m.insert("sharing is disabled", "compartir está desactivado");
        m.insert("For more info, please ask the person who sent this link.", "Para mayor información, contactá a la persona que te mandó el enlace.");
        m.insert("%s shared the folder %s with you", "%s compartió la carpeta %s con vos");
        m.insert("%s shared the file %s with you", "%s compartió el archivo %s con vos");
        m.insert("Download", "Descargar");
        m.insert("Upload", "Subir");
        m.insert("Cancel upload", "Cancelar subida");
        m.insert("No preview available for", "La vista preliminar no está disponible para");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}