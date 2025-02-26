use std::collections::HashMap;
use once_cell::sync::Lazy;

// Hungarian translations
pub static HU_HU_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("This share is password-protected", "Ez egy jelszóval védett megosztás");
    m.insert("The password is wrong. Try again.", "A megadott jelszó nem megfelelő. Próbálja újra!");
    m.insert("Password", "Jelszó");
    m.insert("Sorry, this link doesn't seem to work anymore.", "Sajnos úgy tűnik, ez a link már nem működik.");
    m.insert("Reasons might be:", "Ennek az oka a következő lehet:");
    m.insert("the item was removed", "az állományt időközben eltávolították");
    m.insert("the link expired", "lejárt a link érvényességi ideje");
    m.insert("sharing is disabled", "letiltásra került a megosztás");
    m.insert("For more info, please ask the person who sent this link.", "További információért forduljon ahhoz, aki ezt a linket küldte Önnek!");
    m.insert("%s shared the folder %s with you", "%s megosztotta Önnel ezt a mappát: %s");
    m.insert("%s shared the file %s with you", "%s megosztotta Önnel ezt az állományt: %s");
    m.insert("Download", "Letöltés");
    m.insert("Upload", "Feltöltés");
    m.insert("Cancel upload", "A feltöltés megszakítása");
    m.insert("No preview available for", "Nem áll rendelkezésre előnézet ehhez: ");
    m.insert("Direct link", "Közvetlen link");
    m
});

// Plural forms rule for Hungarian
pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}