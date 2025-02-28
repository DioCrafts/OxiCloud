use std::collections::HashMap;
use rust_i18n::i18n;

/// Translations for Spanish language
pub fn load_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Couldn't delete %s permanently", "No se puede eliminar %s permanentemente");
    translations.insert("Couldn't restore %s", "No se puede restaurar %s");
    translations.insert("Error", "Error");
    translations.insert("restored", "recuperado");
    translations.insert("Nothing in here. Your trash bin is empty!", "No hay nada aquí. ¡Tu papelera esta vacía!");
    translations.insert("Name", "Nombre");
    translations.insert("Restore", "Recuperar");
    translations.insert("Deleted", "Eliminado");
    translations.insert("Delete", "Eliminar");
    translations.insert("Deleted Files", "Archivos Eliminados");
    
    translations
}

/// Returns the plural form rule for Spanish
pub fn get_plural_form() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations_loaded() {
        let translations = load_translations();
        assert!(!translations.is_empty());
        assert_eq!(translations.get("Error"), Some(&"Error"));
    }

    #[test]
    fn test_plural_form() {
        assert_eq!(get_plural_form(), "nplurals=2; plural=(n != 1);");
    }
}