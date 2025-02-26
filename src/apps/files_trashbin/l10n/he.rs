use std::collections::HashMap;
use rust_i18n::t;

pub fn register_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "Couldn't delete %s permanently".to_string(),
        "לא ניתן למחוק את %s לצמיתות".to_string(),
    );
    translations.insert(
        "Couldn't restore %s".to_string(),
        "לא ניתן לשחזר את %s".to_string(),
    );
    translations.insert(
        "Error".to_string(),
        "שגיאה".to_string(),
    );
    translations.insert(
        "Nothing in here. Your trash bin is empty!".to_string(),
        "אין כאן שום דבר. סל המיחזור שלך ריק!".to_string(),
    );
    translations.insert(
        "Name".to_string(),
        "שם".to_string(),
    );
    translations.insert(
        "Restore".to_string(),
        "שחזור".to_string(),
    );
    translations.insert(
        "Deleted".to_string(),
        "נמחק".to_string(),
    );
    translations.insert(
        "Delete".to_string(),
        "מחיקה".to_string(),
    );
    translations.insert(
        "Deleted Files".to_string(),
        "קבצים שנמחקו".to_string(),
    );
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}