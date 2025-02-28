use rust_i18n::translation_functions;
use std::collections::HashMap;
use once_cell::sync::Lazy;

translation_functions! {
    pub fn tr(message_id: &str, args: &[&str]) -> String;
    pub fn n_tr(singular_id: &str, plural_id: &str, count: u32, args: &[&str]) -> String;
}

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Couldn't delete %s permanently", "Не можеше трајно да се избрише %s");
    map.insert("Couldn't restore %s", "Не можеше да се поврати %s");
    map.insert("Error", "Грешка");
    map.insert("restored", "повратени");
    map.insert("Nothing in here. Your trash bin is empty!", "Тука нема ништо. Вашата корпа за отпадоци е празна!");
    map.insert("Name", "Име");
    map.insert("Restore", "Поврати");
    map.insert("Deleted", "Избришан");
    map.insert("Delete", "Избриши");
    map.insert("Deleted Files", "Избришани датотеки");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;";

pub fn get_plural_index(n: u32) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else {
        1
    }
}