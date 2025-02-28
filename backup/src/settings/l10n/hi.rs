use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Error", "त्रुटि");
    map.insert("Update", "अद्यतन");
    map.insert("Security Warning", "सुरक्षा चेतावनी ");
    map.insert("Password", "पासवर्ड");
    map.insert("New password", "नया पासवर्ड");
    map.insert("Abort", "रद्द करना ");
    map.insert("Username", "प्रयोक्ता का नाम");
    map
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";