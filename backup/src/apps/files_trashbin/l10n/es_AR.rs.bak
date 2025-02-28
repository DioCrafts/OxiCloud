use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "No fue posible borrar %s de manera permanente");
        m.insert("Couldn't restore %s", "No se pudo restaurar %s");
        m.insert("Error", "Error");
        m.insert("restored", "recuperado");
        m.insert("Nothing in here. Your trash bin is empty!", "No hay nada acá. ¡La papelera está vacía!");
        m.insert("Name", "Nombre");
        m.insert("Restore", "Recuperar");
        m.insert("Deleted", "Borrado");
        m.insert("Delete", "Borrar");
        m.insert("Deleted Files", "Archivos eliminados");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}