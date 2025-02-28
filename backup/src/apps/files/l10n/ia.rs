use lazy_static::lazy_static;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use rust_fluent::FluentValue;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("The uploaded file was only partially uploaded", "Le file incargate solmente esseva incargate partialmente");
        m.insert("No file was uploaded", "Nulle file esseva incargate.");
        m.insert("Missing a temporary folder", "Manca un dossier temporari");
        m.insert("Files", "Files");
        m.insert("Share", "Compartir");
        m.insert("Error", "Error");
        m.insert("Name", "Nomine");
        m.insert("Size", "Dimension");
        m.insert("Modified", "Modificate");
        m.insert("Upload", "Incargar");
        m.insert("Maximum upload size", "Dimension maxime de incargamento");
        m.insert("Save", "Salveguardar");
        m.insert("New", "Nove");
        m.insert("Text file", "File de texto");
        m.insert("Folder", "Dossier");
        m.insert("Nothing in here. Upload something!", "Nihil hic. Incarga alcun cosa!");
        m.insert("Download", "Discargar");
        m.insert("Delete", "Deler");
        m.insert("Upload too large", "Incargamento troppo longe");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, PluralMapping> = {
        let mut m = HashMap::new();
        
        m.insert("_%n folder_::_%n folders_", PluralMapping {
            zero: None,
            one: Some(""),
            other: Some(""),
        });
        
        m.insert("_%n file_::_%n files_", PluralMapping {
            zero: None,
            one: Some(""),
            other: Some(""),
        });
        
        m.insert("_Uploading %n file_::_Uploading %n files_", PluralMapping {
            zero: None,
            one: Some(""),
            other: Some(""),
        });
        
        m
    };
}

pub struct PluralMapping {
    pub zero: Option<&'static str>,
    pub one: Option<&'static str>,
    pub other: Option<&'static str>,
}

pub fn get_plural_form(n: i64) -> &'static str {
    if n != 1 {
        "other"
    } else {
        "one"
    }
}

pub fn format_plural(key: &str, n: i64) -> Option<String> {
    PLURAL_FORMS.get(key).and_then(|mapping| {
        match get_plural_form(n) {
            "zero" => mapping.zero.map(|s| s.replace("%n", &n.to_string())),
            "one" => mapping.one.map(|s| s.replace("%n", &n.to_string())),
            _ => mapping.other.map(|s| s.replace("%n", &n.to_string())),
        }
    })
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}