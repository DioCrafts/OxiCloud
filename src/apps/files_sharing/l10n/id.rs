use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Sandi");
        m.insert("%s shared the folder %s with you", "%s membagikan folder %s dengan Anda");
        m.insert("%s shared the file %s with you", "%s membagikan file %s dengan Anda");
        m.insert("Download", "Unduh");
        m.insert("Upload", "Unggah");
        m.insert("Cancel upload", "Batal pengunggahan");
        m.insert("No preview available for", "Tidak ada pratinjau tersedia untuk");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}