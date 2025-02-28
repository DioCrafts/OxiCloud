use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translation() -> (HashMap<&'static str, &'static str>, &'static str) {
    let mut translations = HashMap::new();
    
    translations.insert("Help", "Ajutor");
    translations.insert("Personal", "Personal");
    translations.insert("Settings", "Setări");
    translations.insert("Users", "Utilizatori");
    translations.insert("Admin", "Admin");
    translations.insert("Unknown filetype", "Tip fișier necunoscut");
    translations.insert("Invalid image", "Imagine invalidă");
    translations.insert("web services under your control", "servicii web controlate de tine");
    translations.insert("ZIP download is turned off.", "Descărcarea ZIP este dezactivată.");
    translations.insert("Files need to be downloaded one by one.", "Fișierele trebuie descărcate unul câte unul.");
    translations.insert("Back to Files", "Înapoi la fișiere");
    translations.insert("Selected files too large to generate zip file.", "Fișierele selectate sunt prea mari pentru a genera un fișier zip.");
    translations.insert("Application is not enabled", "Aplicația nu este activată");
    translations.insert("Authentication error", "Eroare la autentificare");
    translations.insert("Token expired. Please reload page.", "Token expirat. Te rugăm să reîncarci pagina.");
    translations.insert("Files", "Fișiere");
    translations.insert("Text", "Text");
    translations.insert("Images", "Imagini");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Serverul de web nu este încă setat corespunzător pentru a permite sincronizarea fișierelor deoarece interfața WebDAV pare a fi întreruptă.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Vă rugăm să verificați <a href='%s'>ghiduri de instalare</a>.");
    translations.insert("Could not find category \"%s\"", "Cloud nu a gasit categoria \"%s\"");
    translations.insert("seconds ago", "secunde în urmă");
    translations.insert("_%n minute ago_::_%n minutes ago_", "acum %n minute");
    translations.insert("_%n hour ago_::_%n hours ago_", "acum %n ore");
    translations.insert("today", "astăzi");
    translations.insert("yesterday", "ieri");
    translations.insert("_%n day go_::_%n days ago_", "acum %n zile");
    translations.insert("last month", "ultima lună");
    translations.insert("_%n month ago_::_%n months ago_", "");
    translations.insert("last year", "ultimul an");
    translations.insert("years ago", "ani în urmă");

    let plural_forms = "nplurals=3; plural=(n==1?0:(((n%100>19)||((n%100==0)&&(n!=0)))?2:1));";
    
    (translations, plural_forms)
}

// Register the translations with the i18n system
pub fn register_ro_translations() {
    let (translations, plural_forms) = get_translation();
    i18n::register_translation("ro", translations, Some(plural_forms));
}