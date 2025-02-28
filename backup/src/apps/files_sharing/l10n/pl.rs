use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Udział ten jest chroniony hasłem");
        m.insert("The password is wrong. Try again.", "To hasło jest niewłaściwe. Spróbuj ponownie.");
        m.insert("Password", "Hasło");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Przepraszamy ale wygląda na to, że ten link już nie działa.");
        m.insert("Reasons might be:", "Możliwe powody:");
        m.insert("the item was removed", "element został usunięty");
        m.insert("the link expired", "link wygasł");
        m.insert("sharing is disabled", "Udostępnianie jest wyłączone");
        m.insert("For more info, please ask the person who sent this link.", "Aby uzyskać więcej informacji proszę poprosić osobę, która wysłał ten link.");
        m.insert("%s shared the folder %s with you", "%s współdzieli folder z tobą %s");
        m.insert("%s shared the file %s with you", "%s współdzieli z tobą plik %s");
        m.insert("Download", "Pobierz");
        m.insert("Upload", "Wyślij");
        m.insert("Cancel upload", "Anuluj wysyłanie");
        m.insert("No preview available for", "Podgląd nie jest dostępny dla");
        m.insert("Direct link", "Bezpośredni link");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}