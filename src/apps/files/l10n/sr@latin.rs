use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("There is no error, the file uploaded with success", "Nema greške, fajl je uspešno poslat");
    m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Poslati fajl prevazilazi direktivu MAX_FILE_SIZE koja je navedena u HTML formi");
    m.insert("The uploaded file was only partially uploaded", "Poslati fajl je samo delimično otpremljen!");
    m.insert("No file was uploaded", "Nijedan fajl nije poslat");
    m.insert("Missing a temporary folder", "Nedostaje privremena fascikla");
    m.insert("Files", "Fajlovi");
    m.insert("Share", "Podeli");
    m.insert("_%n folder_::_%n folders_", "");
    m.insert("_%n file_::_%n files_", "");
    m.insert("_Uploading %n file_::_Uploading %n files_", "");
    m.insert("Error", "Greška");
    m.insert("Name", "Ime");
    m.insert("Size", "Veličina");
    m.insert("Modified", "Zadnja izmena");
    m.insert("Upload", "Pošalji");
    m.insert("Maximum upload size", "Maksimalna veličina pošiljke");
    m.insert("Save", "Snimi");
    m.insert("Nothing in here. Upload something!", "Ovde nema ničeg. Pošaljite nešto!");
    m.insert("Download", "Preuzmi");
    m.insert("Unshare", "Ukljoni deljenje");
    m.insert("Delete", "Obriši");
    m.insert("Upload too large", "Pošiljka je prevelika");
    m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Fajlovi koje želite da pošaljete prevazilaze ograničenje maksimalne veličine pošiljke na ovom serveru.");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

pub fn get_plural_form(n: usize) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: usize) -> &'static str {
    // This would need a more complex implementation for proper plural handling
    // but this is a basic implementation that returns the original key
    // since the PHP example didn't provide actual plural translations
    get_translation(key)
}