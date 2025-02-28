use rust_i18n::i18n;

i18n!("mk");

fn get_translations() -> std::collections::HashMap<&'static str, &'static str> {
    let mut translations = std::collections::HashMap::new();
    translations.insert("Password", "Лозинка");
    translations.insert("%s shared the folder %s with you", "%s ја сподели папката %s со Вас");
    translations.insert("%s shared the file %s with you", "%s ја сподели датотеката %s со Вас");
    translations.insert("Download", "Преземи");
    translations.insert("Upload", "Подигни");
    translations.insert("Cancel upload", "Откажи прикачување");
    translations.insert("No preview available for", "Нема достапно преглед за");
    translations
}

fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;"
}

pub fn init() -> (std::collections::HashMap<&'static str, &'static str>, &'static str) {
    (get_translations(), get_plural_forms())
}