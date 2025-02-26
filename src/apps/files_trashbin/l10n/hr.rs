use rust_i18n::I18n;

pub struct Croatian;

impl I18n for Croatian {
    fn translations(&self) -> std::collections::HashMap<String, String> {
        let mut translations = std::collections::HashMap::new();
        translations.insert("Error".to_string(), "Greška".to_string());
        translations.insert("Name".to_string(), "Ime".to_string());
        translations.insert("Delete".to_string(), "Obriši".to_string());
        translations
    }

    fn plural_forms(&self) -> String {
        "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;".to_string()
    }
}