use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("This share is password-protected", "Este elemento compartido esta protegido por contraseña");
        map.insert("The password is wrong. Try again.", "La contraseña introducida es errónea. Inténtelo de nuevo.");
        map.insert("Password", "Contraseña");
        map.insert("Sorry, this link doesn't seem to work anymore.", "Vaya, este enlace parece que no volverá a funcionar.");
        map.insert("Reasons might be:", "Las causas podrían ser:");
        map.insert("the item was removed", "el elemento fue eliminado");
        map.insert("the link expired", "el enlace expiró");
        map.insert("sharing is disabled", "compartir está desactivado");
        map.insert("For more info, please ask the person who sent this link.", "Para mayor información, contacte a la persona que le envió el enlace.");
        map.insert("%s shared the folder %s with you", "%s compartió la carpeta %s contigo");
        map.insert("%s shared the file %s with you", "%s compartió el fichero %s contigo");
        map.insert("Download", "Descargar");
        map.insert("Upload", "Subir");
        map.insert("Cancel upload", "Cancelar subida");
        map.insert("No preview available for", "No hay vista previa disponible para");
        map.insert("Direct link", "Enlace directo");
        map
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}