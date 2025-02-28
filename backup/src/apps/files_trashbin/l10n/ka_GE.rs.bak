use rust_gettext::prelude::*;
use std::collections::HashMap;

/// Translations for Karachay-Balkar (Georgia)
pub fn register_ka_ge_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Couldn't delete %s permanently", "ფაილი %s–ის სრულად წაშლა ვერ მოხერხდა");
    translations.insert("Couldn't restore %s", "%s–ის აღდგენა ვერ მოხერხდა");
    translations.insert("Error", "შეცდომა");
    translations.insert("Nothing in here. Your trash bin is empty!", "აქ არაფერი არ არის. სანაგვე ყუთი ცარიელია!");
    translations.insert("Name", "სახელი");
    translations.insert("Restore", "აღდგენა");
    translations.insert("Deleted", "წაშლილი");
    translations.insert("Delete", "წაშლა");
    translations.insert("Deleted Files", "წაშლილი ფაილები");
    
    translations
}

pub fn get_ka_ge_plural_form() -> PluralForm {
    PluralForm {
        nplurals: 1,
        plural_fn: Box::new(|_| 0),
    }
}