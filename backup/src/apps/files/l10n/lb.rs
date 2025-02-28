use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("lb");

lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("There is no error, the file uploaded with success", "Keen Feeler, Datei ass komplett ropgelueden ginn");
        m.insert("The uploaded file exceeds the MAX_FILE_SIZE directive that was specified in the HTML form", "Déi ropgelueden Datei ass méi grouss wei d'MAX_FILE_SIZE Eegenschaft déi an der HTML form uginn ass");
        m.insert("The uploaded file was only partially uploaded", "Déi ropgelueden Datei ass nëmmen hallef ropgelueden ginn");
        m.insert("No file was uploaded", "Et ass kee Fichier ropgeluede ginn");
        m.insert("Missing a temporary folder", "Et feelt en temporären Dossier");
        m.insert("Failed to write to disk", "Konnt net op den Disk schreiwen");
        m.insert("Files", "Dateien");
        m.insert("Upload cancelled.", "Upload ofgebrach.");
        m.insert("File upload is in progress. Leaving the page now will cancel the upload.", "File Upload am gaang. Wann's de des Säit verléiss gëtt den Upload ofgebrach.");
        m.insert("Share", "Deelen");
        m.insert("Rename", "Ëm-benennen");
        m.insert("undo", "réckgängeg man");
        m.insert("Error", "Fehler");
        m.insert("Name", "Numm");
        m.insert("Size", "Gréisst");
        m.insert("Modified", "Geännert");
        m.insert("Upload", "Eroplueden");
        m.insert("File handling", "Fichier handling");
        m.insert("Maximum upload size", "Maximum Upload Gréisst ");
        m.insert("max. possible: ", "max. méiglech:");
        m.insert("Needed for multi-file and folder downloads.", "Gett gebraucht fir multi-Fichier an Dossier Downloads.");
        m.insert("Enable ZIP-download", "ZIP-download erlaben");
        m.insert("0 is unlimited", "0 ass onlimitéiert");
        m.insert("Maximum input size for ZIP files", "Maximal Gréisst fir ZIP Fichieren");
        m.insert("Save", "Späicheren");
        m.insert("New", "Nei");
        m.insert("Text file", "Text Fichier");
        m.insert("Folder", "Dossier");
        m.insert("Cancel upload", "Upload ofbriechen");
        m.insert("Nothing in here. Upload something!", "Hei ass näischt. Lued eppes rop!");
        m.insert("Download", "Download");
        m.insert("Unshare", "Net méi deelen");
        m.insert("Delete", "Läschen");
        m.insert("Upload too large", "Upload ze grouss");
        m.insert("The files you are trying to upload exceed the maximum size for file uploads on this server.", "Déi Dateien déi Dir probéiert erop ze lueden sinn méi grouss wei déi Maximal Gréisst déi op dësem Server erlaabt ass.");
        m.insert("Files are being scanned, please wait.", "Fichieren gi gescannt, war weg.");
        m.insert("Current scanning", "Momentane Scan");
        m
    };

    pub static ref PLURAL_FORMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["", ""]);
        m.insert("_%n file_::_%n files_", vec!["", ""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
        m
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn translate_plural(key: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_FORMS.get(key) {
        let index = get_plural_form(count);
        if index < forms.len() {
            return forms[index];
        }
    }
    key
}