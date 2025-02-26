use std::collections::HashMap;
use once_cell::sync::Lazy;

static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("This share is password-protected", "Den här delningen är lösenordsskyddad");
    translations.insert("The password is wrong. Try again.", "Lösenordet är fel. Försök igen.");
    translations.insert("Password", "Lösenord");
    translations.insert("Sorry, this link doesn't seem to work anymore.", "Tyvärr, denna länk verkar inte fungera längre.");
    translations.insert("Reasons might be:", "Orsaker kan vara:");
    translations.insert("the item was removed", "objektet togs bort");
    translations.insert("the link expired", "giltighet för länken har gått ut");
    translations.insert("sharing is disabled", "delning är inaktiverat");
    translations.insert("For more info, please ask the person who sent this link.", "För mer information, kontakta den person som skickade den här länken.");
    translations.insert("%s shared the folder %s with you", "%s delade mappen %s med dig");
    translations.insert("%s shared the file %s with you", "%s delade filen %s med dig");
    translations.insert("Download", "Ladda ner");
    translations.insert("Upload", "Ladda upp");
    translations.insert("Cancel upload", "Avbryt uppladdning");
    translations.insert("No preview available for", "Ingen förhandsgranskning tillgänglig för");
    translations.insert("Direct link", "Direkt länk");
    translations
});

static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}